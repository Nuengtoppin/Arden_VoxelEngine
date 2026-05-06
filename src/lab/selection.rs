use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::core::{
    full_route_to_runtime_position, runtime_position_to_world, AnchorMode, FullRoute, VOXEL_SIZE,
};
use crate::lab::probe::LabProbeState;
use crate::lab::sandbox::LabSandboxState;
use crate::tools::debug::{CurrentTool, DebugUiState};

#[derive(Debug, Clone, Copy)]
pub struct VoxelSelection {
    pub start: FullRoute,
    pub end: FullRoute,
}

#[derive(Resource, Debug, Default)]
pub struct SelectionBoxState {
    pub start: Option<FullRoute>,
    pub end: Option<FullRoute>,
    pub selecting: bool,
}

impl SelectionBoxState {
    pub fn clear(&mut self) {
        self.start = None;
        self.end = None;
        self.selecting = false;
    }

    pub fn selection(&self) -> Option<VoxelSelection> {
        Some(VoxelSelection {
            start: self.start?,
            end: self.end?,
        })
    }

    pub fn is_ready(&self) -> bool {
        self.start.is_some() && self.end.is_some() && !self.selecting
    }

    pub fn is_open(&self) -> bool {
        self.start.is_some() && self.end.is_some() && self.selecting
    }
}

pub fn update_select_box_skeleton(
    mut egui_contexts: EguiContexts,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    sandbox: Res<LabSandboxState>,
    debug_ui: Res<DebugUiState>,
    probe: Res<LabProbeState>,
    mut selection: ResMut<SelectionBoxState>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if debug_ui.current_tool != CurrentTool::SelectBox {
        return;
    }

    if egui_contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    let current_target = preferred_selection_target(&probe);

    if selection.selecting {
        if let Some(route) = current_target {
            selection.end = Some(route);
        }
    }

    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(route) = current_target else {
        return;
    };

    if !selection.selecting {
        selection.start = Some(route);
        selection.end = Some(route);
        selection.selecting = true;
    } else {
        selection.end = Some(route);
        selection.selecting = false;
    }
}

fn preferred_selection_target(probe: &LabProbeState) -> Option<FullRoute> {
    if let Some(pinned) = probe.pinned.as_ref() {
        pinned.full_route
    } else {
        probe.inspect.full_route
    }
}

pub fn selection_world_bounds(selection: &SelectionBoxState) -> Option<(Vec3, Vec3)> {
    let sel = selection.selection()?;

    let start_min = route_world_min(sel.start)?;
    let end_min = route_world_min(sel.end)?;

    let min = Vec3::new(
        start_min.x.min(end_min.x),
        start_min.y.min(end_min.y),
        start_min.z.min(end_min.z),
    );

    let max_corner = Vec3::new(
        start_min.x.max(end_min.x),
        start_min.y.max(end_min.y),
        start_min.z.max(end_min.z),
    ) + Vec3::splat(VOXEL_SIZE);

    Some((min, max_corner))
}

pub fn selection_voxel_dims(selection: &SelectionBoxState) -> Option<UVec3> {
    let sel = selection.selection()?;

    let start_min = route_world_min(sel.start)?;
    let end_min = route_world_min(sel.end)?;

    let dx = (start_min.x.round() as i32 - end_min.x.round() as i32).abs() as u32 + 1;
    let dy = (start_min.y.round() as i32 - end_min.y.round() as i32).abs() as u32 + 1;
    let dz = (start_min.z.round() as i32 - end_min.z.round() as i32).abs() as u32 + 1;

    Some(UVec3::new(dx, dy, dz))
}

fn route_world_min(full: FullRoute) -> Option<Vec3> {
    let runtime = full_route_to_runtime_position(full, AnchorMode::Corner).ok()?;
    runtime_position_to_world(runtime).ok()
}
