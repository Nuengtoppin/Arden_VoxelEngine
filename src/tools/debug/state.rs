use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugLens {
    Runtime,
    Density,
    Sim,
    Deep,
    Dun,
}

impl Default for DebugLens {
    fn default() -> Self {
        Self::Runtime
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugNotation {
    Machine,
    Human,
}

impl Default for DebugNotation {
    fn default() -> Self {
        Self::Machine
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugPresentation {
    Compact,
    Detailed,
}

impl Default for DebugPresentation {
    fn default() -> Self {
        Self::Compact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentTool {
    Inspect,
    SelectBox,
    Paint,
    Erase,
}

impl Default for CurrentTool {
    fn default() -> Self {
        Self::Inspect
    }
}

#[derive(Resource, Debug)]
pub struct DebugUiState {
    pub overlay_enabled: bool,
    pub gizmos_enabled: bool,
    pub lens: DebugLens,
    pub notation: DebugNotation,
    pub presentation: DebugPresentation,
    pub current_tool: CurrentTool,

    pub show_region_gizmo: bool,
    pub show_sector_gizmo: bool,
    pub show_chunk_gizmo: bool,
    pub show_octo_gizmo: bool,
    pub show_hover_gizmo: bool,
    pub show_pinned_gizmo: bool,
    pub show_selection_gizmo: bool,
    pub show_object_gizmo: bool,

    // one-frame action requests
    pub pin_target_requested: bool,
    pub clear_pinned_target_requested: bool,
}

impl Default for DebugUiState {
    fn default() -> Self {
        Self {
            overlay_enabled: true,
            gizmos_enabled: true,
            lens: DebugLens::Runtime,
            notation: DebugNotation::Machine,
            presentation: DebugPresentation::Compact,
            current_tool: CurrentTool::Inspect,

            show_region_gizmo: true,
            show_sector_gizmo: true,
            show_chunk_gizmo: true,
            show_octo_gizmo: true,
            show_hover_gizmo: true,
            show_pinned_gizmo: true,
            show_selection_gizmo: true,
            show_object_gizmo: true,

            pin_target_requested: false,
            clear_pinned_target_requested: false,
        }
    }
}
