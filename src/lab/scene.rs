use bevy::prelude::*;

use crate::app::setup::setup_camera_and_light;
use crate::lab::clipboard::{apply_clipboard_actions, LabClipboard};
use crate::lab::gizmos::draw_lab_gizmos;
use crate::lab::hud::draw_lab_hud;

use crate::lab::object::{
    apply_object_actions,
    apply_object_bake_actions,
    apply_object_move_actions,
    apply_object_orientation_actions,
    apply_object_registry_actions,
    LabObjectRegistry,
};

use crate::lab::probe::{
    adjust_inspect_probe_distance, update_lab_probe, InspectProbeSettings, LabProbeState,
};
use crate::lab::sandbox::{enforce_lab_mode_policy, LabSandboxState};
use crate::lab::save::{apply_save_load_actions, LabSaveStatus};
use crate::lab::selection::{update_select_box_skeleton, SelectionBoxState};
use crate::lab::volume::{apply_selection_volume_actions, VolumeDirtyQueue};
use crate::lab::world::{apply_lab_tool_actions, LabVoxelWorld};
use crate::tools::camera_controller::{fly_camera_dolly_wheel, fly_camera_look, fly_camera_move};
use crate::tools::debug::{handle_debug_input, DebugInputMap, DebugUiState};

pub struct LabScenePlugin;

impl Plugin for LabScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LabProbeState>()
            .init_resource::<InspectProbeSettings>()
            .init_resource::<LabSandboxState>()
            .init_resource::<LabVoxelWorld>()
            .init_resource::<SelectionBoxState>()
            .init_resource::<VolumeDirtyQueue>()
            .init_resource::<LabClipboard>()
            .init_resource::<LabObjectRegistry>()
            .init_resource::<LabSaveStatus>()
            .init_resource::<DebugUiState>()
            .init_resource::<DebugInputMap>()
            .add_systems(Startup, (setup_camera_and_light,))
            .add_systems(
                Update,
                (
                    handle_debug_input,
                    enforce_lab_mode_policy.after(handle_debug_input),
                    adjust_inspect_probe_distance.after(enforce_lab_mode_policy),
                    fly_camera_dolly_wheel,
                    fly_camera_look,
                    fly_camera_move,
                    update_lab_probe
                        .after(enforce_lab_mode_policy)
                        .after(fly_camera_dolly_wheel)
                        .after(fly_camera_look)
                        .after(fly_camera_move),
                    update_select_box_skeleton.after(update_lab_probe),
                    apply_lab_tool_actions.after(update_select_box_skeleton),
                    apply_selection_volume_actions.after(apply_lab_tool_actions),
                    apply_clipboard_actions.after(apply_selection_volume_actions),
                    apply_object_actions.after(apply_clipboard_actions),
                    apply_object_move_actions.after(apply_object_actions),
                    apply_object_bake_actions.after(apply_object_move_actions),
                    apply_object_orientation_actions.after(apply_object_actions),
                    apply_object_registry_actions.after(apply_object_actions),
                    apply_save_load_actions.after(apply_object_actions),
                    draw_lab_hud.after(apply_save_load_actions),
                    draw_lab_gizmos.after(apply_object_actions),
                ),
            );
    }
}
