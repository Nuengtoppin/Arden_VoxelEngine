use std::collections::{HashMap, HashSet};

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::core::{DensityKey, FullRoute, VoxelGrid, CHUNK_SIZE, OCTO_SIZE};
use crate::lab::probe::{InspectProbe, LabProbeState};
use crate::lab::sandbox::LabSandboxState;
use crate::lab::volume::LocalVoxelBox;
use crate::tools::debug::{CurrentTool, DebugUiState};

#[derive(Resource, Default)]
pub struct LabVoxelWorld {
    pub chunks: HashMap<DensityKey, VoxelGrid>,
    pub dirty_chunks: HashSet<DensityKey>,
}

impl LabVoxelWorld {
    #[inline]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    #[inline]
    pub fn dirty_count(&self) -> usize {
        self.dirty_chunks.len()
    }

    #[inline]
    pub fn has_chunk(&self, key: DensityKey) -> bool {
        self.chunks.contains_key(&key)
    }

    #[inline]
    pub fn get_chunk(&self, key: DensityKey) -> Option<&VoxelGrid> {
        self.chunks.get(&key)
    }

    #[inline]
    pub fn get_chunk_mut(&mut self, key: DensityKey) -> Option<&mut VoxelGrid> {
        self.chunks.get_mut(&key)
    }

    pub fn ensure_chunk(&mut self, key: DensityKey) -> &mut VoxelGrid {
        self.chunks
            .entry(key)
            .or_insert_with(VoxelGrid::new_chunk_default)
    }

    #[inline]
    pub fn mark_dirty(&mut self, key: DensityKey) {
        self.dirty_chunks.insert(key);
    }

    pub fn clear_dirty(&mut self) {
        self.dirty_chunks.clear();
    }

    pub fn commit_dirty_from_queue(&mut self, touched: &HashSet<DensityKey>) {
        self.dirty_chunks.extend(touched.iter().copied());
    }

    pub fn take_dirty_chunks(&mut self) -> HashSet<DensityKey> {
        std::mem::take(&mut self.dirty_chunks)
    }

    pub fn get_voxel(&self, full: FullRoute) -> u8 {
        let key = DensityKey {
            region: full.region,
            chunk: full.chunk,
        };

        let (x, y, z) = full_route_chunk_local(full);

        match self.chunks.get(&key) {
            Some(chunk) => chunk.get(x, y, z),
            None => 0,
        }
    }

    pub fn set_voxel(&mut self, full: FullRoute, value: u8) {
        let key = DensityKey {
            region: full.region,
            chunk: full.chunk,
        };

        let (x, y, z) = full_route_chunk_local(full);

        if value == 0 {
            let mut remove_chunk = false;

            if let Some(chunk) = self.chunks.get_mut(&key) {
                chunk.set(x, y, z, 0);
                remove_chunk = chunk.data.iter().all(|&v| v == 0);
                self.mark_dirty(key);
            }

            if remove_chunk {
                self.chunks.remove(&key);
            }

            return;
        }

        let chunk = self.ensure_chunk(key);
        chunk.set(x, y, z, value);
        self.mark_dirty(key);
    }

    pub fn fill_chunk(&mut self, key: DensityKey, value: u8) {
        let chunk = self.ensure_chunk(key);
        chunk.fill(value);
        self.mark_dirty(key);
    }

    pub fn fill_chunk_bulk(&mut self, key: DensityKey, value: u8) {
        let chunk = self.ensure_chunk(key);
        chunk.fill(value);
    }

    pub fn remove_chunk(&mut self, key: DensityKey) -> bool {
        self.chunks.remove(&key).is_some()
    }

    pub fn fill_chunk_box(&mut self, key: DensityKey, local: LocalVoxelBox, value: u8) {
        let chunk = self.ensure_chunk(key);

        for z in local.min.z..local.max_exclusive.z {
            for y in local.min.y..local.max_exclusive.y {
                for x in local.min.x..local.max_exclusive.x {
                    chunk.set(x, y, z, value);
                }
            }
        }
    }

    pub fn clear_chunk_box(&mut self, key: DensityKey, local: LocalVoxelBox) {
        let Some(chunk) = self.chunks.get_mut(&key) else {
            return;
        };

        for z in local.min.z..local.max_exclusive.z {
            for y in local.min.y..local.max_exclusive.y {
                for x in local.min.x..local.max_exclusive.x {
                    chunk.set(x, y, z, 0);
                }
            }
        }
    }

    pub fn prune_chunk_if_empty(&mut self, key: DensityKey) -> bool {
        let should_remove = match self.chunks.get(&key) {
            Some(chunk) => chunk.data.iter().all(|&v| v == 0),
            None => false,
        };

        if should_remove {
            self.chunks.remove(&key);
            return true;
        }

        false
    }

    pub fn set_chunk_local_voxel(&mut self, key: DensityKey, local: UVec3, value: u8) {
        if value == 0 {
            if let Some(chunk) = self.chunks.get_mut(&key) {
                chunk.set(local.x, local.y, local.z, 0);
            }
            return;
        }

        let chunk = self.ensure_chunk(key);
        chunk.set(local.x, local.y, local.z, value);
    }
}

pub fn apply_lab_tool_actions(
    mut egui_contexts: EguiContexts,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    sandbox: Res<LabSandboxState>,
    debug_ui: Res<DebugUiState>,
    probe: Res<LabProbeState>,
    mut world: ResMut<LabVoxelWorld>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    if egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    let Some(target) = preferred_edit_target(&probe) else {
        return;
    };

    match debug_ui.current_tool {
        CurrentTool::Paint => {
            // Safe v1: one adjacent voxel per click.
            // This prevents held mouse from extruding a column toward the camera.
            if !mouse_buttons.just_pressed(MouseButton::Left) {
                return;
            }

            let Some(full) = target.place_full_route.or(target.full_route) else {
                return;
            };

            world.set_voxel(full, 1);
        }

        CurrentTool::Erase => {
            let Some(full) = target.full_route else {
                return;
            };

            world.set_voxel(full, 0);
        }

        _ => {}
    }
}

fn preferred_edit_target(probe: &LabProbeState) -> Option<&InspectProbe> {
    if let Some(pinned) = probe.pinned.as_ref() {
        Some(pinned)
    } else if probe.inspect.full_route.is_some() {
        Some(&probe.inspect)
    } else {
        None
    }
}

fn full_route_chunk_local(full: FullRoute) -> (u32, u32, u32) {
    let x = full.octo.ox * OCTO_SIZE + full.voxel.vx;
    let y = full.octo.oy * OCTO_SIZE + full.voxel.vy;
    let z = full.octo.oz * OCTO_SIZE + full.voxel.vz;

    debug_assert!(x < CHUNK_SIZE);
    debug_assert!(y < CHUNK_SIZE);
    debug_assert!(z < CHUNK_SIZE);

    (x, y, z)
}
