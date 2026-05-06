use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct DebugInputMap {
    pub toggle_overlay: KeyCode,
    pub toggle_gizmos: KeyCode,
    pub cycle_lens_next: KeyCode,
    pub cycle_lens_prev: KeyCode,
    pub toggle_notation: KeyCode,
    pub toggle_presentation: KeyCode,

    pub pin_target: KeyCode,
    pub clear_pinned_target: KeyCode,

    pub select_tool_inspect: KeyCode,
    pub select_tool_select_box: KeyCode,
    pub select_tool_paint: KeyCode,
    pub select_tool_erase: KeyCode,

    pub delete_selection: KeyCode,
    pub fill_selection: KeyCode,

    pub copy_selection: KeyCode,
    pub paste_clipboard: KeyCode,
    pub extract_object_copy: KeyCode,

    pub move_object_neg_x: KeyCode,
    pub move_object_pos_x: KeyCode,
    pub move_object_neg_y: KeyCode,
    pub move_object_pos_y: KeyCode,
    pub move_object_neg_z: KeyCode,
    pub move_object_pos_z: KeyCode,

    pub bake_object_to_world: KeyCode,

    pub select_next_object: KeyCode,
    pub select_prev_object: KeyCode,
    pub delete_selected_object: KeyCode,

    pub rotate_object_ccw: KeyCode,
    pub rotate_object_cw: KeyCode,

    pub save_lab_world: KeyCode,
    pub load_lab_world: KeyCode,
}

impl Default for DebugInputMap {
    fn default() -> Self {
        Self {
            toggle_overlay: KeyCode::F1,
            toggle_gizmos: KeyCode::F2,
            cycle_lens_next: KeyCode::PageDown,
            cycle_lens_prev: KeyCode::PageUp,
            toggle_notation: KeyCode::F3,
            toggle_presentation: KeyCode::F4,

            pin_target: KeyCode::KeyQ,
            clear_pinned_target: KeyCode::KeyE,

            select_tool_inspect: KeyCode::Digit1,
            select_tool_select_box: KeyCode::Digit2,
            select_tool_paint: KeyCode::Digit3,
            select_tool_erase: KeyCode::Digit4,

            delete_selection: KeyCode::Delete,
            fill_selection: KeyCode::KeyF,

            copy_selection: KeyCode::KeyC,
            paste_clipboard: KeyCode::KeyV,

            extract_object_copy: KeyCode::KeyX,

            move_object_neg_x: KeyCode::KeyJ,
            move_object_pos_x: KeyCode::KeyL,
            move_object_neg_y: KeyCode::KeyU,
            move_object_pos_y: KeyCode::KeyO,
            move_object_neg_z: KeyCode::KeyK,
            move_object_pos_z: KeyCode::KeyI,

            bake_object_to_world: KeyCode::KeyB,

            select_next_object: KeyCode::KeyN,
            select_prev_object: KeyCode::KeyM,
            delete_selected_object: KeyCode::Backspace,

            rotate_object_ccw: KeyCode::KeyG,
            rotate_object_cw: KeyCode::KeyH,

            save_lab_world: KeyCode::F5,
            load_lab_world: KeyCode::F9,
        }
    }
}
