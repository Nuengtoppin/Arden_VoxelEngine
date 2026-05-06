use bevy::prelude::*;

use super::config::DebugInputMap;
use super::state::{CurrentTool, DebugLens, DebugNotation, DebugPresentation, DebugUiState};

pub fn handle_debug_input(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    mut ui: ResMut<DebugUiState>,
) {
    // one-frame requests are reset every frame
    ui.pin_target_requested = false;
    ui.clear_pinned_target_requested = false;

    if keys.just_pressed(bindings.toggle_overlay) {
        ui.overlay_enabled = !ui.overlay_enabled;
    }

    if keys.just_pressed(bindings.toggle_gizmos) {
        ui.gizmos_enabled = !ui.gizmos_enabled;
    }

    if keys.just_pressed(bindings.cycle_lens_next) {
        ui.lens = next_lens(ui.lens);
    }

    if keys.just_pressed(bindings.cycle_lens_prev) {
        ui.lens = prev_lens(ui.lens);
    }

    if keys.just_pressed(bindings.toggle_notation) {
        ui.notation = match ui.notation {
            DebugNotation::Machine => DebugNotation::Human,
            DebugNotation::Human => DebugNotation::Machine,
        };
    }

    if keys.just_pressed(bindings.toggle_presentation) {
        ui.presentation = match ui.presentation {
            DebugPresentation::Compact => DebugPresentation::Detailed,
            DebugPresentation::Detailed => DebugPresentation::Compact,
        };
    }

    if keys.just_pressed(bindings.pin_target) {
        ui.pin_target_requested = true;
    }

    if keys.just_pressed(bindings.clear_pinned_target) {
        ui.clear_pinned_target_requested = true;
    }

    if keys.just_pressed(bindings.select_tool_inspect) {
        ui.current_tool = CurrentTool::Inspect;
    }

    if keys.just_pressed(bindings.select_tool_select_box) {
        ui.current_tool = CurrentTool::SelectBox;
    }

    if keys.just_pressed(bindings.select_tool_paint) {
        ui.current_tool = CurrentTool::Paint;
    }

    if keys.just_pressed(bindings.select_tool_erase) {
        ui.current_tool = CurrentTool::Erase;
    }
}

fn next_lens(lens: DebugLens) -> DebugLens {
    match lens {
        DebugLens::Runtime => DebugLens::Density,
        DebugLens::Density => DebugLens::Sim,
        DebugLens::Sim => DebugLens::Deep,
        DebugLens::Deep => DebugLens::Dun,
        DebugLens::Dun => DebugLens::Runtime,
    }
}

fn prev_lens(lens: DebugLens) -> DebugLens {
    match lens {
        DebugLens::Runtime => DebugLens::Dun,
        DebugLens::Density => DebugLens::Runtime,
        DebugLens::Sim => DebugLens::Density,
        DebugLens::Deep => DebugLens::Sim,
        DebugLens::Dun => DebugLens::Deep,
    }
}
