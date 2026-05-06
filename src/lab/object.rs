use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::core::{runtime_to_full_route, world_to_runtime_position, FullRoute};
use crate::lab::sandbox::LabSandboxState;
use crate::lab::selection::{selection_world_bounds, SelectionBoxState};

use crate::lab::volume::{
    build_operation_plan, execute_operation_plan, selection_to_volume_intent, VolumeDirtyQueue,
    VolumeOpKind,
};

use crate::lab::world::LabVoxelWorld;
use crate::tools::debug::{CurrentTool, DebugInputMap, DebugUiState};

const OBJECT_MOVE_STEP: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabVoxelObjectId(pub u64);

#[derive(Debug, Clone)]
pub struct VoxelPayload {
    pub dims: UVec3,
    pub data: Vec<u8>,
}

impl VoxelPayload {
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
    pub fn solid_count(&self) -> usize {
        self.data.iter().filter(|&&v| v != 0).count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ObjectOrientation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl ObjectOrientation {
    #[inline]
    pub fn as_u8(self) -> u8 {
        match self {
            Self::R0 => 0,
            Self::R90 => 1,
            Self::R180 => 2,
            Self::R270 => 3,
        }
    }

    #[inline]
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::R0),
            1 => Some(Self::R90),
            2 => Some(Self::R180),
            3 => Some(Self::R270),
            _ => None,
        }
    }

    #[inline]
    pub fn label(self) -> &'static str {
        match self {
            Self::R0 => "R0",
            Self::R90 => "R90",
            Self::R180 => "R180",
            Self::R270 => "R270",
        }
    }

    #[inline]
    pub fn yaw_radians(self) -> f32 {
        match self {
            Self::R0 => 0.0,
            Self::R90 => std::f32::consts::FRAC_PI_2,
            Self::R180 => std::f32::consts::PI,
            Self::R270 => std::f32::consts::PI * 1.5,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn rotate_cw(self) -> Self {
        match self {
            Self::R0 => Self::R90,
            Self::R90 => Self::R180,
            Self::R180 => Self::R270,
            Self::R270 => Self::R0,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::R0 => Self::R270,
            Self::R90 => Self::R0,
            Self::R180 => Self::R90,
            Self::R270 => Self::R180,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LabVoxelObject {
    pub id: LabVoxelObjectId,

    /// World-space integer min corner where this payload was extracted from.
    pub world_origin: IVec3,

    /// Local pivot inside payload bounds.
    /// v0 policy: bounds center.
    pub pivot_local: Vec3,

    /// C4 yaw orientation state.
    /// This is separate from world_origin/address and does not rotate voxel data by itself.
    pub orientation: ObjectOrientation,

    /// Local axis-aligned voxel payload.
    /// This grid is not rotated.
    pub payload: VoxelPayload,

    pub solid_voxels: usize,
}

#[derive(Resource, Debug)]
pub struct LabObjectRegistry {
    next_id: u64,
    pub objects: Vec<LabVoxelObject>,
    pub selected: Option<LabVoxelObjectId>,
    pub last_message: String,
}

impl Default for LabObjectRegistry {
    fn default() -> Self {
        Self {
            next_id: 1,
            objects: Vec::new(),
            selected: None,
            last_message: "no objects".to_string(),
        }
    }
}

impl LabObjectRegistry {
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn selected_object(&self) -> Option<&LabVoxelObject> {
        let selected = self.selected?;
        self.objects.iter().find(|object| object.id == selected)
    }

    pub fn last_object(&self) -> Option<&LabVoxelObject> {
        self.objects.last()
    }

    fn push_object(
        &mut self,
        world_origin: IVec3,
        pivot_local: Vec3,
        payload: VoxelPayload,
    ) -> LabVoxelObjectId {
        let id = LabVoxelObjectId(self.next_id);
        self.next_id += 1;

        let solid_voxels = payload.solid_count();

        let object = LabVoxelObject {
            id,
            world_origin,
            pivot_local,
            orientation: ObjectOrientation::R0,
            payload,
            solid_voxels,
        };

        self.objects.push(object);
        self.selected = Some(id);

        id
    }

    pub fn move_selected_by(&mut self, delta: IVec3) -> bool {
        let Some(selected) = self.selected else {
            self.last_message = "move failed: no selected object".to_string();
            return false;
        };

        let Some(index) = self.objects.iter().position(|object| object.id == selected) else {
            self.last_message = format!("move failed: selected object #{} not found", selected.0);
            return false;
        };

        let (id, origin) = {
            let object = &mut self.objects[index];
            object.world_origin += delta;
            (object.id.0, object.world_origin)
        };

        self.last_message = format!(
            "moved object #{} by [{}|{}|{}] -> origin [{}|{}|{}]",
            id, delta.x, delta.y, delta.z, origin.x, origin.y, origin.z,
        );

        true
    }

    pub fn remove_selected_object(&mut self) -> Option<LabVoxelObject> {
        let selected = self.selected?;

        let index = self
            .objects
            .iter()
            .position(|object| object.id == selected)?;

        let removed = self.objects.remove(index);

        self.selected = self.objects.last().map(|object| object.id);

        Some(removed)
    }

    pub fn select_next_object(&mut self) -> Option<LabVoxelObjectId> {
        if self.objects.is_empty() {
            self.selected = None;
            self.last_message = "select next failed: no objects".to_string();
            return None;
        }

        let current_index = self
            .selected
            .and_then(|selected| self.objects.iter().position(|object| object.id == selected));

        let next_index = match current_index {
            Some(index) => (index + 1) % self.objects.len(),
            None => 0,
        };

        let id = self.objects[next_index].id;
        self.selected = Some(id);
        self.last_message = format!("selected object #{}", id.0);

        Some(id)
    }

    pub fn select_prev_object(&mut self) -> Option<LabVoxelObjectId> {
        if self.objects.is_empty() {
            self.selected = None;
            self.last_message = "select previous failed: no objects".to_string();
            return None;
        }

        let current_index = self
            .selected
            .and_then(|selected| self.objects.iter().position(|object| object.id == selected));

        let prev_index = match current_index {
            Some(0) | None => self.objects.len() - 1,
            Some(index) => index - 1,
        };

        let id = self.objects[prev_index].id;
        self.selected = Some(id);
        self.last_message = format!("selected object #{}", id.0);

        Some(id)
    }

    pub fn delete_selected_object(&mut self) -> Option<LabVoxelObject> {
        let removed = self.remove_selected_object();

        match removed.as_ref() {
            Some(object) => {
                self.last_message = format!("deleted object #{}", object.id.0);
            }
            None => {
                self.last_message = "delete failed: no selected object".to_string();
            }
        }

        removed
    }

    pub fn rotate_selected_ccw(&mut self) -> bool {
    let Some(selected) = self.selected else {
        self.last_message = "rotate failed: no selected object".to_string();
        return false;
    };

    let Some(index) = self.objects.iter().position(|object| object.id == selected) else {
        self.last_message = format!("rotate failed: selected object #{} not found", selected.0);
        return false;
    };

    let (id, orientation) = {
        let object = &mut self.objects[index];
        object.orientation = object.orientation.rotate_ccw();
        (object.id.0, object.orientation)
    };

    self.last_message = format!("rotated object #{} -> {}", id, orientation.label());
    true
}

pub fn rotate_selected_cw(&mut self) -> bool {
    let Some(selected) = self.selected else {
        self.last_message = "rotate failed: no selected object".to_string();
        return false;
    };

    let Some(index) = self.objects.iter().position(|object| object.id == selected) else {
        self.last_message = format!("rotate failed: selected object #{} not found", selected.0);
        return false;
    };

    let (id, orientation) = {
        let object = &mut self.objects[index];
        object.orientation = object.orientation.rotate_cw();
        (object.id.0, object.orientation)
    };

    self.last_message = format!("rotated object #{} -> {}", id, orientation.label());
    true
}

    pub fn replace_all(
        &mut self,
        objects: Vec<LabVoxelObject>,
        selected: Option<LabVoxelObjectId>,
    ) {
        let max_id = objects.iter().map(|object| object.id.0).max().unwrap_or(0);

        self.objects = objects;

        self.selected = selected
            .filter(|id| self.objects.iter().any(|object| object.id == *id))
            .or_else(|| self.objects.last().map(|object| object.id));

        self.next_id = max_id + 1;

        self.last_message = format!(
            "loaded {} objects, selected {}",
            self.objects.len(),
            self.selected
                .map(|id| format!("#{}", id.0))
                .unwrap_or_else(|| "<none>".to_string())
        );
    }
}

pub fn apply_object_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    debug_ui: Res<DebugUiState>,
    mut selection: ResMut<SelectionBoxState>,
    mut world: ResMut<LabVoxelWorld>,
    mut dirty_queue: ResMut<VolumeDirtyQueue>,
    mut registry: ResMut<LabObjectRegistry>,
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

    if !keys.just_pressed(bindings.extract_object_copy) {
        return;
    }

    let cut_mode = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    let Some((world_origin, pivot_local, payload)) =
        extract_selection_payload(&selection, &world, &sandbox)
    else {
        registry.last_message = "extract failed: empty or invalid selection".to_string();
        return;
    };

    let dims = payload.dims;
    let solid = payload.solid_count();

    let id = registry.push_object(world_origin, pivot_local, payload);

    if cut_mode {
        cut_selection_from_world(&selection, &sandbox, &mut world, &mut dirty_queue);

        registry.last_message = format!(
            "cut object #{} dims [{}|{}|{}], solid {}",
            id.0, dims.x, dims.y, dims.z, solid
        );
    } else {
        registry.last_message = format!(
            "copied object #{} dims [{}|{}|{}], solid {}",
            id.0, dims.x, dims.y, dims.z, solid
        );
    }

    selection.clear();
}

pub fn apply_object_bake_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    mut world: ResMut<LabVoxelWorld>,
    mut registry: ResMut<LabObjectRegistry>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if egui_contexts.ctx_mut().wants_keyboard_input() {
        return;
    }

    if !keys.just_pressed(bindings.bake_object_to_world) {
        return;
    }

    let Some(object) = registry.selected_object().cloned() else {
        registry.last_message = "bake failed: no selected object".to_string();
        return;
    };

    let baked = bake_object_payload_to_world(&object, &sandbox, &mut world);

    if baked == 0 {
        registry.last_message = format!("bake failed: object #{} wrote 0 voxels", object.id.0);
        return;
    }

    let removed = registry.remove_selected_object();

    registry.last_message = match removed {
        Some(removed) => format!(
            "baked object #{} to world, wrote {} voxels",
            removed.id.0, baked
        ),
        None => format!(
            "baked object #{} to world, wrote {}, remove failed",
            object.id.0, baked
        ),
    };
}

pub fn apply_object_registry_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    mut registry: ResMut<LabObjectRegistry>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if egui_contexts.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(bindings.select_next_object) {
        registry.select_next_object();
    }

    if keys.just_pressed(bindings.select_prev_object) {
        registry.select_prev_object();
    }

    if keys.just_pressed(bindings.delete_selected_object) {
        registry.delete_selected_object();
    }
}

pub fn apply_object_orientation_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    mut registry: ResMut<LabObjectRegistry>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if egui_contexts.ctx_mut().wants_keyboard_input() {
        return;
    }

    if keys.just_pressed(bindings.rotate_object_ccw) {
        registry.rotate_selected_ccw();
    }

    if keys.just_pressed(bindings.rotate_object_cw) {
        registry.rotate_selected_cw();
    }
}

fn bake_object_payload_to_world(
    object: &LabVoxelObject,
    sandbox: &LabSandboxState,
    world: &mut LabVoxelWorld,
) -> usize {
    let mut written = 0usize;

    let bake_origin = rotated_bake_origin(object);

    for z in 0..object.payload.dims.z {
        for y in 0..object.payload.dims.y {
            for x in 0..object.payload.dims.x {
                let src_local = UVec3::new(x, y, z);
                let value = object.payload.get(src_local);

                // v0 bake policy:
                // write only solid payload voxels.
                // Empty cells do not erase destination world voxels yet.
                if value == 0 {
                    continue;
                }

                let dst_local =
                    rotate_payload_local(src_local, object.payload.dims, object.orientation);

                let target_world = bake_origin + dst_local.as_ivec3();

                let Some(full) = resolve_world_voxel(target_world) else {
                    continue;
                };

                if !sandbox.profile.contains_region(full.region) {
                    continue;
                }

                world.set_voxel(full, value);
                written += 1;
            }
        }
    }

    written
}

fn rotated_payload_dims(dims: UVec3, orientation: ObjectOrientation) -> UVec3 {
    match orientation {
        ObjectOrientation::R0 | ObjectOrientation::R180 => dims,

        ObjectOrientation::R90 | ObjectOrientation::R270 => {
            UVec3::new(dims.z, dims.y, dims.x)
        }
    }
}

fn rotated_bake_origin(object: &LabVoxelObject) -> IVec3 {
    let src_dims = object.payload.dims.as_vec3();
    let dst_dims = rotated_payload_dims(object.payload.dims, object.orientation).as_vec3();

    // Object preview rotates around the payload center.
    // Bake uses the rotated AABB min corner as integer world origin.
    let center = object.world_origin.as_vec3() + src_dims * 0.5;
    let min = center - dst_dims * 0.5;

    IVec3::new(
        min.x.round() as i32,
        min.y.round() as i32,
        min.z.round() as i32,
    )
}

fn rotate_payload_local(
    src: UVec3,
    dims: UVec3,
    orientation: ObjectOrientation,
) -> UVec3 {
    match orientation {
        ObjectOrientation::R0 => src,

        ObjectOrientation::R90 => {
            UVec3::new(
                src.z,
                src.y,
                dims.x - 1 - src.x,
            )
        }

        ObjectOrientation::R180 => {
            UVec3::new(
                dims.x - 1 - src.x,
                src.y,
                dims.z - 1 - src.z,
            )
        }

        ObjectOrientation::R270 => {
            UVec3::new(
                dims.z - 1 - src.z,
                src.y,
                src.x,
            )
        }
    }
}

pub fn apply_object_move_actions(
    mut egui_contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    sandbox: Res<LabSandboxState>,
    mut registry: ResMut<LabObjectRegistry>,
) {
    if !sandbox.edit_tools_allowed() {
        return;
    }

    if egui_contexts.ctx_mut().wants_keyboard_input() {
        return;
    }

    let mut delta = IVec3::ZERO;

    if keys.just_pressed(bindings.move_object_neg_x) {
        delta.x -= OBJECT_MOVE_STEP;
    }

    if keys.just_pressed(bindings.move_object_pos_x) {
        delta.x += OBJECT_MOVE_STEP;
    }

    if keys.just_pressed(bindings.move_object_neg_y) {
        delta.y -= OBJECT_MOVE_STEP;
    }

    if keys.just_pressed(bindings.move_object_pos_y) {
        delta.y += OBJECT_MOVE_STEP;
    }

    if keys.just_pressed(bindings.move_object_neg_z) {
        delta.z -= OBJECT_MOVE_STEP;
    }

    if keys.just_pressed(bindings.move_object_pos_z) {
        delta.z += OBJECT_MOVE_STEP;
    }

    if delta == IVec3::ZERO {
        return;
    }

    registry.move_selected_by(delta);
}

fn cut_selection_from_world(
    selection: &SelectionBoxState,
    sandbox: &LabSandboxState,
    world: &mut LabVoxelWorld,
    dirty_queue: &mut VolumeDirtyQueue,
) {
    let Some(intent) = selection_to_volume_intent(selection, VolumeOpKind::Delete) else {
        return;
    };

    dirty_queue.touched_chunks.clear();

    let plan = build_operation_plan(&intent, sandbox);
    execute_operation_plan(&plan, intent.kind, world, dirty_queue);
}

fn extract_selection_payload(
    selection: &SelectionBoxState,
    world: &LabVoxelWorld,
    sandbox: &LabSandboxState,
) -> Option<(IVec3, Vec3, VoxelPayload)> {
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

    let mut payload = VoxelPayload::new(dims);

    for z in 0..dims.z {
        for y in 0..dims.y {
            for x in 0..dims.x {
                let local = UVec3::new(x, y, z);
                let world_voxel = min + local.as_ivec3();

                let value = resolve_world_voxel(world_voxel)
                    .filter(|full| sandbox.profile.contains_region(full.region))
                    .map(|full| world.get_voxel(full))
                    .unwrap_or(0);

                payload.set(local, value);
            }
        }
    }

    if payload.solid_count() == 0 {
        return None;
    }

    let pivot_local = Vec3::new(dims.x as f32, dims.y as f32, dims.z as f32) * 0.5;

    Some((min, pivot_local, payload))
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
