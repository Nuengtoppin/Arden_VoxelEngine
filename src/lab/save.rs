use std::fs;
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::{ChunkCoord, DensityKey, RegionCoord, VoxelGrid};

use crate::lab::object::{
    LabObjectRegistry, LabVoxelObject, LabVoxelObjectId, ObjectOrientation, VoxelPayload,
};

use crate::lab::sandbox::{LabSandboxState, LabWorldProfile};
use crate::lab::world::LabVoxelWorld;
use crate::tools::debug::DebugInputMap;

const LAB_SAVE_VERSION: u32 = 4;
const LAB_SAVE_PATH: &str = "saves/lab_snapshot.ron";

#[derive(Resource, Debug, Clone)]
pub struct LabSaveStatus {
    pub path: String,
    pub last_message: String,
}

impl Default for LabSaveStatus {
    fn default() -> Self {
        Self {
            path: LAB_SAVE_PATH.to_string(),
            last_message: "not saved yet".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabSaveFile {
    pub version: u32,
    pub profile: LabWorldProfileSave,
    pub chunks: Vec<LabChunkSave>,
    pub objects: Vec<LabObjectSave>,
    pub selected_object: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabWorldProfileSave {
    pub finite_world: bool,
    pub region_min: [i32; 3],
    pub region_dims: [u32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabChunkSave {
    pub region: [i32; 3],
    pub chunk: [u32; 3],
    pub size: [u32; 3],
    pub data_rle: Vec<RleRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabObjectSave {
    pub id: u64,
    pub world_origin: [i32; 3],
    pub pivot_local: [f32; 3],
    pub orientation_c4: u8,
    pub dims: [u32; 3],
    pub data_rle: Vec<RleRun>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RleRun {
    pub value: u8,
    pub len: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LabLoadSummary {
    pub chunks: usize,
    pub objects: usize,
}

pub fn apply_save_load_actions(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<DebugInputMap>,
    mut world: ResMut<LabVoxelWorld>,
    mut objects: ResMut<LabObjectRegistry>,
    mut sandbox: ResMut<LabSandboxState>,
    mut status: ResMut<LabSaveStatus>,
) {
    if keys.just_pressed(bindings.save_lab_world) {
        match save_lab_world_to_file(&world, &objects, &sandbox.profile, LAB_SAVE_PATH) {
            Ok(()) => {
                status.last_message = format!("saved lab v{LAB_SAVE_VERSION} to {LAB_SAVE_PATH}");
            }
            Err(err) => {
                status.last_message = format!("save failed: {err}");
            }
        }
    }

    if keys.just_pressed(bindings.load_lab_world) {
        match load_lab_world_from_file(
            &mut world,
            &mut objects,
            &mut sandbox.profile,
            LAB_SAVE_PATH,
        ) {
            Ok(summary) => {
                status.last_message = format!(
                    "loaded {} chunks, {} objects from {}",
                    summary.chunks, summary.objects, LAB_SAVE_PATH,
                );
            }
            Err(err) => {
                status.last_message = format!("load failed: {err}");
            }
        }
    }
}

pub fn save_lab_world_to_file(
    world: &LabVoxelWorld,
    objects: &LabObjectRegistry,
    profile: &LabWorldProfile,
    path: impl AsRef<Path>,
) -> Result<(), String> {
    let save = LabSaveFile {
        version: LAB_SAVE_VERSION,
        profile: save_profile(profile),
        chunks: save_chunks(world),
        objects: save_objects(objects),
        selected_object: objects.selected.map(|id| id.0),
    };

    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).map_err(|err| format!("create save dir: {err}"))?;
    }

    let pretty = ron::ser::PrettyConfig::default();

    let text =
        ron::ser::to_string_pretty(&save, pretty).map_err(|err| format!("serialize ron: {err}"))?;

    fs::write(path.as_ref(), text).map_err(|err| format!("write file: {err}"))?;

    Ok(())
}

pub fn load_lab_world_from_file(
    world: &mut LabVoxelWorld,
    objects: &mut LabObjectRegistry,
    profile: &mut LabWorldProfile,
    path: impl AsRef<Path>,
) -> Result<LabLoadSummary, String> {
    let text = fs::read_to_string(path.as_ref()).map_err(|err| format!("read file: {err}"))?;

    let save: LabSaveFile = ron::de::from_str(&text).map_err(|err| format!("parse ron: {err}"))?;

    if save.version != LAB_SAVE_VERSION {
        return Err(format!(
            "unsupported save version: {}, expected {}",
            save.version, LAB_SAVE_VERSION,
        ));
    }

    restore_profile(profile, save.profile);

    world.chunks.clear();
    world.dirty_chunks.clear();

    let mut loaded_chunks = 0usize;

    for chunk_save in save.chunks {
        let Some((key, grid)) = restore_chunk(chunk_save) else {
            continue;
        };

        if key.validate().is_err() {
            continue;
        }

        world.chunks.insert(key, grid);
        world.mark_dirty(key);
        loaded_chunks += 1;
    }

    let restored_objects = restore_objects(save.objects);
    let loaded_objects = restored_objects.len();

    let selected = save.selected_object.map(LabVoxelObjectId);
    objects.replace_all(restored_objects, selected);

    Ok(LabLoadSummary {
        chunks: loaded_chunks,
        objects: loaded_objects,
    })
}

fn save_profile(profile: &LabWorldProfile) -> LabWorldProfileSave {
    LabWorldProfileSave {
        finite_world: profile.finite_world,
        region_min: [
            profile.region_min.x,
            profile.region_min.y,
            profile.region_min.z,
        ],
        region_dims: [
            profile.region_dims.x,
            profile.region_dims.y,
            profile.region_dims.z,
        ],
    }
}

fn restore_profile(profile: &mut LabWorldProfile, save: LabWorldProfileSave) {
    profile.finite_world = save.finite_world;

    profile.region_min = IVec3::new(save.region_min[0], save.region_min[1], save.region_min[2]);

    profile.region_dims = UVec3::new(
        save.region_dims[0],
        save.region_dims[1],
        save.region_dims[2],
    );
}

fn save_chunks(world: &LabVoxelWorld) -> Vec<LabChunkSave> {
    let mut chunks = Vec::new();

    for (&key, grid) in world.chunks.iter() {
        if grid.data.iter().all(|&v| v == 0) {
            continue;
        }

        chunks.push(LabChunkSave {
            region: [key.region.rx, key.region.ry, key.region.rz],
            chunk: [key.chunk.cx, key.chunk.cy, key.chunk.cz],
            size: [grid.size.x, grid.size.y, grid.size.z],
            data_rle: encode_rle(&grid.data),
        });
    }

    chunks
}

fn restore_chunk(save: LabChunkSave) -> Option<(DensityKey, VoxelGrid)> {
    let size = UVec3::new(save.size[0], save.size[1], save.size[2]);

    let expected_len = checked_grid_len(size)?;

    let data = decode_rle(&save.data_rle, expected_len)?;

    let key = DensityKey {
        region: RegionCoord {
            rx: save.region[0],
            ry: save.region[1],
            rz: save.region[2],
        },
        chunk: ChunkCoord {
            cx: save.chunk[0],
            cy: save.chunk[1],
            cz: save.chunk[2],
        },
    };

    let mut grid = VoxelGrid::new(size);
    grid.data = data;

    Some((key, grid))
}

fn save_objects(objects: &LabObjectRegistry) -> Vec<LabObjectSave> {
    let mut out = Vec::new();

    for object in objects.objects.iter() {
        if object.payload.data.iter().all(|&v| v == 0) {
            continue;
        }

        out.push(LabObjectSave {
            id: object.id.0,
            world_origin: [
                object.world_origin.x,
                object.world_origin.y,
                object.world_origin.z,
            ],
            pivot_local: [
                object.pivot_local.x,
                object.pivot_local.y,
                object.pivot_local.z,
            ],
            orientation_c4: object.orientation.as_u8(),
            dims: [
                object.payload.dims.x,
                object.payload.dims.y,
                object.payload.dims.z,
            ],
            data_rle: encode_rle(&object.payload.data),
        });
    }

    out
}

fn restore_objects(saves: Vec<LabObjectSave>) -> Vec<LabVoxelObject> {
    let mut objects = Vec::new();

    for save in saves {
        let Some(object) = restore_object(save) else {
            continue;
        };

        objects.push(object);
    }

    objects
}

fn restore_object(save: LabObjectSave) -> Option<LabVoxelObject> {
    if save.id == 0 {
        return None;
    }

    let dims = UVec3::new(save.dims[0], save.dims[1], save.dims[2]);
    let expected_len = checked_grid_len(dims)?;

    let data = decode_rle(&save.data_rle, expected_len)?;

    let payload = VoxelPayload { dims, data };

    let solid_voxels = payload.solid_count();

    if solid_voxels == 0 {
        return None;
    }

    let orientation = ObjectOrientation::from_u8(save.orientation_c4)?;

    Some(LabVoxelObject {
        id: LabVoxelObjectId(save.id),
        world_origin: IVec3::new(
            save.world_origin[0],
            save.world_origin[1],
            save.world_origin[2],
        ),
        pivot_local: Vec3::new(
            save.pivot_local[0],
            save.pivot_local[1],
            save.pivot_local[2],
        ),
        orientation,
        payload,
        solid_voxels,
    })
}

fn checked_grid_len(size: UVec3) -> Option<usize> {
    let xy = size.x.checked_mul(size.y)?;
    let xyz = xy.checked_mul(size.z)?;
    Some(xyz as usize)
}

fn encode_rle(data: &[u8]) -> Vec<RleRun> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut runs = Vec::new();

    let mut current = data[0];
    let mut len: u32 = 1;

    for &value in &data[1..] {
        if value == current && len < u32::MAX {
            len += 1;
        } else {
            runs.push(RleRun {
                value: current,
                len,
            });

            current = value;
            len = 1;
        }
    }

    runs.push(RleRun {
        value: current,
        len,
    });

    runs
}

fn decode_rle(runs: &[RleRun], expected_len: usize) -> Option<Vec<u8>> {
    let mut data = Vec::with_capacity(expected_len);

    for run in runs {
        if run.len == 0 {
            return None;
        }

        let next_len = data.len().checked_add(run.len as usize)?;

        if next_len > expected_len {
            return None;
        }

        data.resize(next_len, run.value);
    }

    if data.len() != expected_len {
        return None;
    }

    Some(data)
}
