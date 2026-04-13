use bevy::prelude::*;

use crate::core::topology::{
    DEFAULT_CHUNK_SIZE_VOXELS,
    DEFAULT_OCTO_SIZE_VOXELS,
};

/// Простейший плотный воксельный грид.
/// 0 = пусто, >0 = материал / тип вещества.
pub struct VoxelGrid {
    pub size: UVec3,
    pub data: Vec<u8>,
}

impl VoxelGrid {
    /// Создаёт грид указанного размера и заполняет нулями.
    pub fn new(size: UVec3) -> Self {
        let len = (size.x * size.y * size.z) as usize;
        Self {
            size,
            data: vec![0; len],
        }
    }

    /// Канонический дефолтный Octochunk-грид.
    pub fn new_octochunk_default() -> Self {
        Self::new(UVec3::splat(DEFAULT_OCTO_SIZE_VOXELS))
    }

    /// Канонический дефолтный Chunk-грид.
    pub fn new_chunk_default() -> Self {
        Self::new(UVec3::splat(DEFAULT_CHUNK_SIZE_VOXELS))
    }

    /// Временный дефолт для текущего DUN в lab.
    /// Сейчас DUN живёт как octochunk-sized контейнер.
    /// Позже можно будет переключить на chunk-sized одной строкой.
    pub fn new_dun_default() -> Self {
        Self::new_octochunk_default()
        // Если потом захочешь DUN по умолчанию = размер Chunk:
        // Self::new_chunk_default()
    }

    #[inline]
    fn index(&self, x: u32, z: u32, y: u32) -> Option<usize> {
        if x >= self.size.x || z >= self.size.z || y >= self.size.y {
            return None;
        }

        // Канонический порядок осей:
        // x + size_x * (z + size_z * y)
        let idx = x + self.size.x * (z + self.size.z * y);
        Some(idx as usize)
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32, z: u32) -> u8 {
        if let Some(i) = self.index(x, z, y) {
            self.data[i]
        } else {
            0
        }
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32, z: u32, value: u8) {
        if let Some(i) = self.index(x, z, y) {
            self.data[i] = value;
        }
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }
}