use bevy::prelude::*;

use crate::core::topology::{CHUNK_SIZE, OCTO_SIZE};

/// Simple dense voxel grid.
/// 0 = empty, >0 = material / substance id.
pub struct VoxelGrid {
    pub size: UVec3,
    pub data: Vec<u8>,
}

impl VoxelGrid {
    /// Create grid of requested size, filled with zeros.
    pub fn new(size: UVec3) -> Self {
        let len = (size.x * size.y * size.z) as usize;
        Self {
            size,
            data: vec![0; len],
        }
    }

    /// Canonical default Octochunk-sized grid.
    pub fn new_octochunk_default() -> Self {
        Self::new(UVec3::splat(OCTO_SIZE))
    }

    /// Canonical default Chunk-sized grid.
    pub fn new_chunk_default() -> Self {
        Self::new(UVec3::splat(CHUNK_SIZE))
    }

    /// Temporary DUN default for current lab.
    /// For now, DUN uses octochunk-sized container by default.
    pub fn new_dun_default() -> Self {
        Self::new_octochunk_default()
        // If later you want DUN default = Chunk-sized container:
        // Self::new_chunk_default()
    }

    #[inline]
    fn index(&self, x: u32, y: u32, z: u32) -> Option<usize> {
        if x >= self.size.x || y >= self.size.y || z >= self.size.z {
            return None;
        }

        // Canonical flat index for X | Y | Z:
        // x + size_x * (y + size_y * z)
        let idx = x + self.size.x * (y + self.size.y * z);
        Some(idx as usize)
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32, z: u32) -> u8 {
        if let Some(i) = self.index(x, y, z) {
            self.data[i]
        } else {
            0
        }
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32, z: u32, value: u8) {
        if let Some(i) = self.index(x, y, z) {
            self.data[i] = value;
        }
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
