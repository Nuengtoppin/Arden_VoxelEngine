use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::core::{
    full_route_to_runtime_position, runtime_position_to_world, runtime_to_full_route,
    world_to_runtime_position, AnchorMode, FullRoute,
};
use crate::lab::probe::{InspectProbe, LabProbeState};
use crate::lab::sandbox::LabSandboxState;
use crate::lab::selection::{selection_world_bounds, SelectionBoxState};
use crate::lab::world::LabVoxelWorld;
use crate::tools::debug::DebugInputMap;

#[derive(Debug, Clone)]
pub struct ClipboardVolume {
    pub dims: UVec3,
    pub data: Vec<u8>,
}

impl ClipboardVolume {
    pub fn new(dims: UVec3) -> Self {
        let len = (dims.x * dims.y * dims.z) as usize;

        Self {
            dims,
            data: vec![0; len],
        }
    }

    #[inline]
    fn index(&self, local: UVec3) -> Option<usize> {
        if local.x >= self.dims.x || local.y >= self.dims.y || local.z >= self.dims.z {
            return None;
        }

        let idx = local.x + self.dims.x * (local.y + self.dims.y * local.z);
        Some(idx as usize)
    }

    #[inline]
    pub fn get(&self, local: UVec3) -> u8 {
        self.index(local)
            .and_then(|i| self.data.get(i).copied())
            .unwrap_or(0)
    }

    #[inline]
    pub fn set(&mut self, local: UVec3, value: u8) {
        if let Some(i) = self.index(local) {
            self.data[i] = value;
        }
    }

    #[inline]
    pub fn non_empty_count(&self) -> usize {
        self.data.iter().filter(|&&v| v != 0).count()
    }
}

#[derive(Resource, Debug, Default)]
pub struct LabClipboard {
    pub volume: Option<ClipboardVolume>,
}

pub fn apply_clipboard_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    selection: Res<SelectionBoxState>,
    probe: Res<LabProbeState>,
    mut world: ResMut<LabVoxelWorld>,
    mut clipboard: ResMut<LabClipboard>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if keys.just_pressed(bindings.copy_selection) {
        if let Some(volume) = copy_selection_to_clipboard(&selection, &world) {
            clipboard.volume = Some(volume);
        }
    }

    if keys.just_pressed(bindings.paste_clipboard) {
        let Some(volume) = clipboard.volume.clone() else {
            return;
        };

        let Some(target) = preferred_clipboard_target(&probe) else {
            return;
        };

        let Some(anchor) = target.full_route else {
            return;
        };

        paste_clipboard_at(anchor, &volume, &mut world, &sandbox);
    }
}

fn copy_selection_to_clipboard(
    selection: &SelectionBoxState,
    world: &LabVoxelWorld,
) -> Option<ClipboardVolume> {
    let (min, max_exclusive) = selection_world_bounds(selection)?;

    let min = IVec3::new(
        min.x.round() as i32,
        min.y.round() as i32,
        min.z.round() as i32,
    );

    let max_exclusive = IVec3::new(
        max_exclusive.x.round() as i32,
        max_exclusive.y.round() as i32,
        max_exclusive.z.round() as i32,
    );

    if min.x >= max_exclusive.x || min.y >= max_exclusive.y || min.z >= max_exclusive.z {
        return None;
    }

    let dims_i = max_exclusive - min;
    let dims = UVec3::new(dims_i.x as u32, dims_i.y as u32, dims_i.z as u32);

    let mut volume = ClipboardVolume::new(dims);

    for z in 0..dims.z {
        for y in 0..dims.y {
            for x in 0..dims.x {
                let local = UVec3::new(x, y, z);
                let world_voxel = min + local.as_ivec3();

                let value = resolve_world_voxel(world_voxel)
                    .map(|full| world.get_voxel(full))
                    .unwrap_or(0);

                volume.set(local, value);
            }
        }
    }

    Some(volume)
}

fn paste_clipboard_at(
    anchor: FullRoute,
    volume: &ClipboardVolume,
    world: &mut LabVoxelWorld,
    sandbox: &LabSandboxState,
) {
    let Some(anchor_world) = route_world_min(anchor) else {
        return;
    };

    for z in 0..volume.dims.z {
        for y in 0..volume.dims.y {
            for x in 0..volume.dims.x {
                let local = UVec3::new(x, y, z);
                let value = volume.get(local);

                // v0 mode:
                // paste only solid payload voxels.
                // Empty cells are stored in clipboard but do not erase destination yet.
                if value == 0 {
                    continue;
                }

                let target_world = anchor_world + local.as_ivec3();

                let Some(full) = resolve_world_voxel(target_world) else {
                    continue;
                };

                if !sandbox.profile.contains_region(full.region) {
                    continue;
                }

                world.set_voxel(full, value);
            }
        }
    }
}

fn preferred_clipboard_target(probe: &LabProbeState) -> Option<&InspectProbe> {
    if let Some(pinned) = probe.pinned.as_ref() {
        Some(pinned)
    } else if probe.inspect.full_route.is_some() {
        Some(&probe.inspect)
    } else {
        None
    }
}

fn resolve_world_voxel(world_voxel: IVec3) -> Option<FullRoute> {
    let world = Vec3::new(
        world_voxel.x as f32,
        world_voxel.y as f32,
        world_voxel.z as f32,
    );

    let runtime = world_to_runtime_position(world).ok()?;
    runtime_to_full_route(runtime).ok()
}

fn route_world_min(full: FullRoute) -> Option<IVec3> {
    let runtime = full_route_to_runtime_position(full, AnchorMode::Corner).ok()?;
    let world = runtime_position_to_world(runtime).ok()?;

    Some(IVec3::new(
        world.x.round() as i32,
        world.y.round() as i32,
        world.z.round() as i32,
    ))
}
