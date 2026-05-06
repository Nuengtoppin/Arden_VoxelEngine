use std::collections::HashSet;

use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::core::{
    runtime_to_density_key, world_to_runtime_position, DensityKey, CHUNK_SIZE, OCTO_SIZE,
};
use crate::lab::sandbox::LabSandboxState;
use crate::lab::selection::{selection_world_bounds, SelectionBoxState};
use crate::lab::world::LabVoxelWorld;
use crate::tools::debug::{CurrentTool, DebugInputMap, DebugUiState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeOpKind {
    Fill(u8),
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldVoxelBox {
    pub min: IVec3,
    pub max_exclusive: IVec3,
}

impl WorldVoxelBox {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max_exclusive.x
            || self.min.y >= self.max_exclusive.y
            || self.min.z >= self.max_exclusive.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalVoxelBox {
    pub min: UVec3,
    pub max_exclusive: UVec3,
}

impl LocalVoxelBox {
    #[inline]
    pub fn full_chunk() -> Self {
        Self {
            min: UVec3::ZERO,
            max_exclusive: UVec3::splat(CHUNK_SIZE),
        }
    }

    #[inline]
    pub fn is_full_chunk(&self) -> bool {
        *self == Self::full_chunk()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolumeIntent {
    pub kind: VolumeOpKind,
    pub bounds: WorldVoxelBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkCoverage {
    Full,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctoCoverage {
    Empty,
    Full,
    Frontier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OctoPlan {
    pub octo_index: u8,
    pub local_box: LocalVoxelBox,
    pub coverage: OctoCoverage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPlan {
    pub key: DensityKey,
    pub local_box: LocalVoxelBox,
    pub coverage: ChunkCoverage,
    pub octos: Option<[OctoPlan; 8]>,
}

#[derive(Debug, Clone, Default)]
pub struct OperationPlan {
    pub chunks: Vec<ChunkPlan>,
}

#[derive(Resource, Debug, Default)]
pub struct VolumeDirtyQueue {
    pub touched_chunks: HashSet<DensityKey>,
}

pub fn apply_selection_volume_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    debug_ui: Res<DebugUiState>,
    selection: Res<SelectionBoxState>,
    mut world: ResMut<LabVoxelWorld>,
    mut dirty_queue: ResMut<VolumeDirtyQueue>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if debug_ui.current_tool != CurrentTool::SelectBox {
        return;
    }

    if !selection.is_ready() {
        return;
    }

    if egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    let Some(kind) = requested_volume_op(&keys, &bindings) else {
        return;
    };

    let Some(intent) = selection_to_volume_intent(&selection, kind) else {
        return;
    };

    dirty_queue.touched_chunks.clear();

    let plan = build_operation_plan(&intent, &sandbox);
    execute_operation_plan(&plan, intent.kind, &mut world, &mut dirty_queue);
}

#[inline]
fn requested_volume_op(
    keys: &ButtonInput<KeyCode>,
    bindings: &DebugInputMap,
) -> Option<VolumeOpKind> {
    if keys.just_pressed(bindings.delete_selection) {
        return Some(VolumeOpKind::Delete);
    }

    if keys.just_pressed(bindings.fill_selection) {
        return Some(VolumeOpKind::Fill(1));
    }

    None
}

pub fn selection_to_volume_intent(
    selection: &SelectionBoxState,
    kind: VolumeOpKind,
) -> Option<VolumeIntent> {
    let (min, max_exclusive) = selection_world_bounds(selection)?;

    let bounds = WorldVoxelBox {
        min: IVec3::new(
            min.x.round() as i32,
            min.y.round() as i32,
            min.z.round() as i32,
        ),
        max_exclusive: IVec3::new(
            max_exclusive.x.round() as i32,
            max_exclusive.y.round() as i32,
            max_exclusive.z.round() as i32,
        ),
    };

    if bounds.is_empty() {
        return None;
    }

    Some(VolumeIntent { kind, bounds })
}

pub fn build_operation_plan(intent: &VolumeIntent, sandbox: &LabSandboxState) -> OperationPlan {
    let mut plan = OperationPlan::default();

    if intent.bounds.is_empty() {
        return plan;
    }

    let size = CHUNK_SIZE as i32;

    let chunk_min = IVec3::new(
        intent.bounds.min.x.div_euclid(size),
        intent.bounds.min.y.div_euclid(size),
        intent.bounds.min.z.div_euclid(size),
    );

    let last_voxel = intent.bounds.max_exclusive - IVec3::ONE;

    let chunk_max = IVec3::new(
        last_voxel.x.div_euclid(size),
        last_voxel.y.div_euclid(size),
        last_voxel.z.div_euclid(size),
    );

    for cz in chunk_min.z..=chunk_max.z {
        for cy in chunk_min.y..=chunk_max.y {
            for cx in chunk_min.x..=chunk_max.x {
                let chunk_world_min = IVec3::new(cx * size, cy * size, cz * size);
                let chunk_world_max = chunk_world_min + IVec3::splat(size);

                let overlap_min = ivec3_max(intent.bounds.min, chunk_world_min);
                let overlap_max = ivec3_min(intent.bounds.max_exclusive, chunk_world_max);

                if overlap_min.x >= overlap_max.x
                    || overlap_min.y >= overlap_max.y
                    || overlap_min.z >= overlap_max.z
                {
                    continue;
                }

                let Some(key) = chunk_origin_to_density_key(chunk_world_min) else {
                    continue;
                };

                if !sandbox.profile.contains_region(key.region) {
                    continue;
                }

                let local_min_i = overlap_min - chunk_world_min;
                let local_max_i = overlap_max - chunk_world_min;

                let local_box = LocalVoxelBox {
                    min: UVec3::new(
                        local_min_i.x as u32,
                        local_min_i.y as u32,
                        local_min_i.z as u32,
                    ),
                    max_exclusive: UVec3::new(
                        local_max_i.x as u32,
                        local_max_i.y as u32,
                        local_max_i.z as u32,
                    ),
                };

                let coverage = if local_box.is_full_chunk() {
                    ChunkCoverage::Full
                } else {
                    ChunkCoverage::Partial
                };

                let octos = match coverage {
                    ChunkCoverage::Full => None,
                    ChunkCoverage::Partial => Some(build_octo_plans(local_box)),
                };

                plan.chunks.push(ChunkPlan {
                    key,
                    local_box,
                    coverage,
                    octos,
                });
            }
        }
    }

    plan
}

pub fn execute_operation_plan(
    plan: &OperationPlan,
    kind: VolumeOpKind,
    world: &mut LabVoxelWorld,
    dirty_queue: &mut VolumeDirtyQueue,
) {
    for chunk in &plan.chunks {
        match (kind, chunk.coverage) {
            (VolumeOpKind::Fill(value), ChunkCoverage::Full) => {
                world.fill_chunk_bulk(chunk.key, value);
            }

            (VolumeOpKind::Delete, ChunkCoverage::Full) => {
                world.remove_chunk(chunk.key);
            }

            (_, ChunkCoverage::Partial) => {
                let Some(octos) = chunk.octos.as_ref() else {
                    continue;
                };

                for octo in octos {
                    match (kind, octo.coverage) {
                        (_, OctoCoverage::Empty) => {}

                        (VolumeOpKind::Fill(value), OctoCoverage::Full) => {
                            world.fill_chunk_box(chunk.key, octo.local_box, value);
                        }

                        (VolumeOpKind::Delete, OctoCoverage::Full) => {
                            world.clear_chunk_box(chunk.key, octo.local_box);
                        }

                        (op_kind, OctoCoverage::Frontier) => {
                            execute_frontier_octo(world, chunk.key, octo.local_box, op_kind);
                        }
                    }
                }

                if matches!(kind, VolumeOpKind::Delete) {
                    world.prune_chunk_if_empty(chunk.key);
                }
            }
        }

        dirty_queue.touched_chunks.insert(chunk.key);
    }

    world.commit_dirty_from_queue(&dirty_queue.touched_chunks);
}

#[inline]
fn octo_origin_from_index(idx: u8) -> UVec3 {
    let ox = (idx & 1) as u32;
    let oy = ((idx >> 1) & 1) as u32;
    let oz = ((idx >> 2) & 1) as u32;

    UVec3::new(ox * OCTO_SIZE, oy * OCTO_SIZE, oz * OCTO_SIZE)
}

#[inline]
fn octo_local_box(idx: u8) -> LocalVoxelBox {
    let origin = octo_origin_from_index(idx);

    LocalVoxelBox {
        min: origin,
        max_exclusive: origin + UVec3::splat(OCTO_SIZE),
    }
}

fn build_octo_plans(chunk_local_box: LocalVoxelBox) -> [OctoPlan; 8] {
    std::array::from_fn(|i| {
        let idx = i as u8;
        let octo_box = octo_local_box(idx);

        match intersect_local_boxes(chunk_local_box, octo_box) {
            None => OctoPlan {
                octo_index: idx,
                local_box: octo_box,
                coverage: OctoCoverage::Empty,
            },
            Some(hit) if hit == octo_box => OctoPlan {
                octo_index: idx,
                local_box: octo_box,
                coverage: OctoCoverage::Full,
            },
            Some(hit) => OctoPlan {
                octo_index: idx,
                local_box: hit,
                coverage: OctoCoverage::Frontier,
            },
        }
    })
}

fn intersect_local_boxes(a: LocalVoxelBox, b: LocalVoxelBox) -> Option<LocalVoxelBox> {
    let min = uvec3_max(a.min, b.min);
    let max_exclusive = uvec3_min(a.max_exclusive, b.max_exclusive);

    if min.x >= max_exclusive.x || min.y >= max_exclusive.y || min.z >= max_exclusive.z {
        return None;
    }

    Some(LocalVoxelBox { min, max_exclusive })
}

fn chunk_origin_to_density_key(chunk_world_min: IVec3) -> Option<DensityKey> {
    let world = Vec3::new(
        chunk_world_min.x as f32,
        chunk_world_min.y as f32,
        chunk_world_min.z as f32,
    );

    let runtime = world_to_runtime_position(world).ok()?;
    let key = runtime_to_density_key(runtime).ok()?;
    Some(key)
}

#[inline]
fn ivec3_min(a: IVec3, b: IVec3) -> IVec3 {
    IVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}

#[inline]
fn ivec3_max(a: IVec3, b: IVec3) -> IVec3 {
    IVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}

#[inline]
fn uvec3_min(a: UVec3, b: UVec3) -> UVec3 {
    UVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}

#[inline]
fn uvec3_max(a: UVec3, b: UVec3) -> UVec3 {
    UVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}

fn execute_frontier_octo(
    world: &mut LabVoxelWorld,
    chunk_key: DensityKey,
    octo_box: LocalVoxelBox,
    kind: VolumeOpKind,
) {
    for z in octo_box.min.z..octo_box.max_exclusive.z {
        for y in octo_box.min.y..octo_box.max_exclusive.y {
            for x in octo_box.min.x..octo_box.max_exclusive.x {
                let local = UVec3::new(x, y, z);

                match kind {
                    VolumeOpKind::Fill(value) => {
                        world.set_chunk_local_voxel(chunk_key, local, value);
                    }
                    VolumeOpKind::Delete => {
                        world.set_chunk_local_voxel(chunk_key, local, 0);
                    }
                }
            }
        }
    }
}
