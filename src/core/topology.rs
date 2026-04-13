use bevy::prelude::*;

/// Канонический порядок осей во всей топологии:
/// x | z | y
///
/// x — поперечная ось
/// z — продольная ось
/// y — вертикаль

/// Размер одного вокселя в мировых единицах.
pub const VOXEL_SIZE: f32 = 1.0;

/// Region всегда состоит из 8×8×2 Blocks.
pub const REGION_BLOCKS_XZ: u32 = 8;
pub const REGION_BLOCKS_Y: u32 = 2;

/// Block всегда состоит из 8×8×8 Chunks.
pub const BLOCK_CHUNKS_XYZ: u32 = 8;

/// Chunk структурно всегда делится на 2×2×2 Octochunks.
/// Это именно топологическое дробление чанка на октанты.
pub const CHUNK_OCTO_SPLIT: u32 = 2;

/// Дефолтный канонический размер Chunk в вокселях.
/// Позже это можно будет переключать профилем/инструментом.
pub const DEFAULT_CHUNK_SIZE_VOXELS: u32 = 64;

/// Производный дефолтный размер Octochunk.
/// Octochunk = 1/8 Chunk, то есть половина стороны Chunk.
pub const DEFAULT_OCTO_SIZE_VOXELS: u32 =
    DEFAULT_CHUNK_SIZE_VOXELS / CHUNK_OCTO_SPLIT;

/// Размер Block в вокселях для дефолтного профиля.
pub const BLOCK_SIZE_XZ: u32 = BLOCK_CHUNKS_XYZ * DEFAULT_CHUNK_SIZE_VOXELS;
pub const BLOCK_SIZE_Y: u32 = BLOCK_CHUNKS_XYZ * DEFAULT_CHUNK_SIZE_VOXELS;

/// Размер Region в вокселях для дефолтного профиля.
pub const REGION_SIZE_XZ: u32 = REGION_BLOCKS_XZ * BLOCK_SIZE_XZ;
pub const REGION_SIZE_Y: u32 = REGION_BLOCKS_Y * BLOCK_SIZE_Y;

/// Вспомогательная функция:
/// вычислить размер Octochunk по размеру Chunk.
#[inline]
pub const fn octo_size_voxels(chunk_size_voxels: u32) -> u32 {
    chunk_size_voxels / CHUNK_OCTO_SPLIT
}

/// Вспомогательная функция:
/// вычислить размер Block по размеру Chunk.
#[inline]
pub const fn block_size_voxels(chunk_size_voxels: u32) -> u32 {
    BLOCK_CHUNKS_XYZ * chunk_size_voxels
}

/// Вспомогательная функция:
/// вычислить размер Region по XZ по размеру Chunk.
#[inline]
pub const fn region_size_xz_voxels(chunk_size_voxels: u32) -> u32 {
    REGION_BLOCKS_XZ * block_size_voxels(chunk_size_voxels)
}

/// Вспомогательная функция:
/// вычислить размер Region по Y по размеру Chunk.
#[inline]
pub const fn region_size_y_voxels(chunk_size_voxels: u32) -> u32 {
    REGION_BLOCKS_Y * block_size_voxels(chunk_size_voxels)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RegionCoord {
    pub rx: i32,
    pub rz: i32,
    pub ry: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockCoord {
    pub bx: u32,
    pub bz: u32,
    pub by: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkCoord {
    pub cx: u32,
    pub cz: u32,
    pub cy: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OctochunkCoord {
    pub ox: u32,
    pub oz: u32,
    pub oy: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VoxelCoord {
    pub vx: u32,
    pub vz: u32,
    pub vy: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Octant {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    I,
}

impl Default for Octant {
    fn default() -> Self {
        Self::A
    }
}

/// Позиция вида Region + LocalFloat.
/// Это ещё не Route, а мост между float-миром и дискретной топологией.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegionLocalPos {
    pub region: RegionCoord,
    pub local: Vec3,
}

/// Простая проверка: лежит ли local-позиция внутри одного Region
/// в дефолтном профиле.
#[inline]
pub fn is_region_local_in_bounds(local: Vec3) -> bool {
    local.x >= 0.0
        && local.z >= 0.0
        && local.y >= 0.0
        && local.x < REGION_SIZE_XZ as f32
        && local.z < REGION_SIZE_XZ as f32
        && local.y < REGION_SIZE_Y as f32
}