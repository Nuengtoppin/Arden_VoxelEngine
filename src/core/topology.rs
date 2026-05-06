use bevy::prelude::*;

/// Active MVP topology canon:
/// Axes: X | Y | Z
/// Hierarchy: Region -> Chunk -> Octochunk -> Voxel
/// Sim overlay: RegionSector = 2 x 2 x 2

/// Size of one voxel in world units.
pub const VOXEL_SIZE: f32 = 1.0;

/// Main dense container size.
pub const CHUNK_SIZE: u32 = 64;

/// Chunk is always split into 2 x 2 x 2 Octochunks.
pub const OCTO_SPLIT_PER_AXIS: u32 = 2;
pub const OCTO_SIZE: u32 = CHUNK_SIZE / OCTO_SPLIT_PER_AXIS;

/// Region profile used by active MVP examples / lab.
pub const REGION_CHUNKS_PER_AXIS: u32 = 16;
pub const REGION_SIZE: u32 = REGION_CHUNKS_PER_AXIS * CHUNK_SIZE;

/// Coarse sim overlay inside Region.
pub const REGION_SECTOR_SPLIT: u32 = 2;
pub const REGION_SECTOR_SIZE: u32 = REGION_SIZE / REGION_SECTOR_SPLIT;

#[inline]
pub const fn octo_size_voxels(chunk_size: u32) -> u32 {
    chunk_size / OCTO_SPLIT_PER_AXIS
}

#[inline]
pub const fn region_size_voxels(chunk_size: u32, region_chunks_per_axis: u32) -> u32 {
    chunk_size * region_chunks_per_axis
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RegionCoord {
    pub rx: i32,
    pub ry: i32,
    pub rz: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkCoord {
    pub cx: u32,
    pub cy: u32,
    pub cz: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OctochunkCoord {
    pub ox: u32,
    pub oy: u32,
    pub oz: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VoxelCoord {
    pub vx: u32,
    pub vy: u32,
    pub vz: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SectorCoord {
    pub sx: u32,
    pub sy: u32,
    pub sz: u32,
}

/// RuntimePosition = Region + LocalFloat.
/// This is runtime truth bridge between world-space and discrete topology.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuntimePosition {
    pub region: RegionCoord,
    pub local: Vec3,
}

/// Simple local bounds check for Region-local float position.
#[inline]
pub fn is_region_local_in_bounds(local: Vec3) -> bool {
    local.x >= 0.0
        && local.y >= 0.0
        && local.z >= 0.0
        && local.x < REGION_SIZE as f32
        && local.y < REGION_SIZE as f32
        && local.z < REGION_SIZE as f32
}
