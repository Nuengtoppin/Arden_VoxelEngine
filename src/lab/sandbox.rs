use bevy::prelude::*;

use crate::core::RegionCoord;
use crate::tools::debug::{CurrentTool, DebugUiState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabMode {
    Edit,
    Runtime,
}

impl Default for LabMode {
    fn default() -> Self {
        Self::Runtime
    }
}

#[derive(Debug, Clone)]
pub struct LabWorldProfile {
    pub finite_world: bool,

    /// Минимальный Region-координатный угол finite sandbox.
    pub region_min: IVec3,

    /// Размер finite sandbox в регионах.
    pub region_dims: UVec3,
}

impl Default for LabWorldProfile {
    fn default() -> Self {
        Self {
            finite_world: true,
            // centered 3x3x3 around origin:
            // [-1..=1] по каждой оси
            region_min: IVec3::new(-1, -1, -1),
            region_dims: UVec3::new(3, 3, 3),
        }
    }
}

impl LabWorldProfile {
    pub fn region_max_exclusive(&self) -> IVec3 {
        self.region_min + self.region_dims.as_ivec3()
    }

    pub fn contains_region(&self, region: RegionCoord) -> bool {
        if !self.finite_world {
            return true;
        }

        let r = IVec3::new(region.rx, region.ry, region.rz);
        let max = self.region_max_exclusive();

        r.x >= self.region_min.x
            && r.x < max.x
            && r.y >= self.region_min.y
            && r.y < max.y
            && r.z >= self.region_min.z
            && r.z < max.z
    }
}

#[derive(Resource, Debug, Default)]
pub struct LabSandboxState {
    pub mode: LabMode,
    pub profile: LabWorldProfile,
}

impl LabSandboxState {
    #[inline]
    pub fn edit_tools_allowed(&self) -> bool {
        matches!(self.mode, LabMode::Edit)
    }
}

/// Step 2.2:
/// Runtime mode keeps tool layer in inspect-only mode.
/// Edit mode allows future edit tools to be selected.
pub fn enforce_lab_mode_policy(sandbox: Res<LabSandboxState>, mut debug_ui: ResMut<DebugUiState>) {
    if !sandbox.edit_tools_allowed() {
        debug_ui.current_tool = CurrentTool::Inspect;
    }
}
