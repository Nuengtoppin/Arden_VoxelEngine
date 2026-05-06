use std::fmt;

use crate::core::topology::{
    is_region_local_in_bounds, ChunkCoord, OctochunkCoord, RegionCoord, RuntimePosition,
    SectorCoord, VoxelCoord, CHUNK_SIZE, OCTO_SIZE, REGION_CHUNKS_PER_AXIS, REGION_SECTOR_SPLIT,
    REGION_SIZE,
};

/// Chunk-level address used by density systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DensityKey {
    pub region: RegionCoord,
    pub chunk: ChunkCoord,
}

/// Coarse sim overlay address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimSectorKey {
    pub region: RegionCoord,
    pub sector: SectorCoord,
}

/// Deep discrete address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FullRoute {
    pub region: RegionCoord,
    pub chunk: ChunkCoord,
    pub octo: OctochunkCoord,
    pub voxel: VoxelCoord,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressLevel {
    Runtime,
    Density,
    Sector,
    Octo,
    Voxel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressError {
    RuntimeLocalOutOfBounds {
        local: bevy::prelude::Vec3,
    },
    OutOfRange {
        level: AddressLevel,
        axis: AddressAxis,
        value: u32,
        max_exclusive: u32,
    },
}

impl DensityKey {
    pub fn validate(&self) -> Result<(), AddressError> {
        validate_chunk(self.chunk)
    }
}

impl SimSectorKey {
    pub fn validate(&self) -> Result<(), AddressError> {
        validate_sector(self.sector)
    }
}

impl FullRoute {
    pub fn validate(&self) -> Result<(), AddressError> {
        validate_chunk(self.chunk)?;
        validate_octo(self.octo)?;
        validate_voxel(self.voxel)?;
        Ok(())
    }
}

impl fmt::Display for DensityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R({}|{}|{}) / C({}|{}|{})",
            self.region.rx,
            self.region.ry,
            self.region.rz,
            self.chunk.cx,
            self.chunk.cy,
            self.chunk.cz,
        )
    }
}

impl fmt::Display for SimSectorKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R({}|{}|{}) / S({}|{}|{})",
            self.region.rx,
            self.region.ry,
            self.region.rz,
            self.sector.sx,
            self.sector.sy,
            self.sector.sz,
        )
    }
}

impl fmt::Display for FullRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R({}|{}|{}) / C({}|{}|{}) / O({}|{}|{}) / v({}|{}|{})",
            self.region.rx,
            self.region.ry,
            self.region.rz,
            self.chunk.cx,
            self.chunk.cy,
            self.chunk.cz,
            self.octo.ox,
            self.octo.oy,
            self.octo.oz,
            self.voxel.vx,
            self.voxel.vy,
            self.voxel.vz,
        )
    }
}

impl fmt::Display for AddressAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AddressAxis::X => "X",
            AddressAxis::Y => "Y",
            AddressAxis::Z => "Z",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for AddressLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AddressLevel::Runtime => "Runtime",
            AddressLevel::Density => "Density",
            AddressLevel::Sector => "Sector",
            AddressLevel::Octo => "Octo",
            AddressLevel::Voxel => "Voxel",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::RuntimeLocalOutOfBounds { local } => {
                write!(
                    f,
                    "runtime local out of bounds: ({}, {}, {})",
                    local.x, local.y, local.z
                )
            }
            AddressError::OutOfRange {
                level,
                axis,
                value,
                max_exclusive,
            } => {
                write!(
                    f,
                    "{} axis {} is out of range: {} >= {}",
                    level, axis, value, max_exclusive
                )
            }
        }
    }
}

impl std::error::Error for AddressError {}

/// Debug/helper string for runtime position.
/// This is not a deep discrete route string.
pub fn fmt_runtime_position(pos: RuntimePosition) -> Result<String, AddressError> {
    if !is_region_local_in_bounds(pos.local) {
        return Err(AddressError::RuntimeLocalOutOfBounds { local: pos.local });
    }

    Ok(format!(
        "R({}|{}|{}) / p({:.3}|{:.3}|{:.3})",
        pos.region.rx, pos.region.ry, pos.region.rz, pos.local.x, pos.local.y, pos.local.z,
    ))
}

#[inline]
fn validate_chunk(chunk: ChunkCoord) -> Result<(), AddressError> {
    if chunk.cx >= REGION_CHUNKS_PER_AXIS {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Density,
            axis: AddressAxis::X,
            value: chunk.cx,
            max_exclusive: REGION_CHUNKS_PER_AXIS,
        });
    }
    if chunk.cy >= REGION_CHUNKS_PER_AXIS {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Density,
            axis: AddressAxis::Y,
            value: chunk.cy,
            max_exclusive: REGION_CHUNKS_PER_AXIS,
        });
    }
    if chunk.cz >= REGION_CHUNKS_PER_AXIS {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Density,
            axis: AddressAxis::Z,
            value: chunk.cz,
            max_exclusive: REGION_CHUNKS_PER_AXIS,
        });
    }

    Ok(())
}

#[inline]
fn validate_sector(sector: SectorCoord) -> Result<(), AddressError> {
    if sector.sx >= REGION_SECTOR_SPLIT {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Sector,
            axis: AddressAxis::X,
            value: sector.sx,
            max_exclusive: REGION_SECTOR_SPLIT,
        });
    }
    if sector.sy >= REGION_SECTOR_SPLIT {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Sector,
            axis: AddressAxis::Y,
            value: sector.sy,
            max_exclusive: REGION_SECTOR_SPLIT,
        });
    }
    if sector.sz >= REGION_SECTOR_SPLIT {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Sector,
            axis: AddressAxis::Z,
            value: sector.sz,
            max_exclusive: REGION_SECTOR_SPLIT,
        });
    }

    Ok(())
}

#[inline]
fn validate_octo(octo: OctochunkCoord) -> Result<(), AddressError> {
    let max = CHUNK_SIZE / OCTO_SIZE;

    if octo.ox >= max {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Octo,
            axis: AddressAxis::X,
            value: octo.ox,
            max_exclusive: max,
        });
    }
    if octo.oy >= max {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Octo,
            axis: AddressAxis::Y,
            value: octo.oy,
            max_exclusive: max,
        });
    }
    if octo.oz >= max {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Octo,
            axis: AddressAxis::Z,
            value: octo.oz,
            max_exclusive: max,
        });
    }

    Ok(())
}

#[inline]
fn validate_voxel(voxel: VoxelCoord) -> Result<(), AddressError> {
    if voxel.vx >= OCTO_SIZE {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Voxel,
            axis: AddressAxis::X,
            value: voxel.vx,
            max_exclusive: OCTO_SIZE,
        });
    }
    if voxel.vy >= OCTO_SIZE {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Voxel,
            axis: AddressAxis::Y,
            value: voxel.vy,
            max_exclusive: OCTO_SIZE,
        });
    }
    if voxel.vz >= OCTO_SIZE {
        return Err(AddressError::OutOfRange {
            level: AddressLevel::Voxel,
            axis: AddressAxis::Z,
            value: voxel.vz,
            max_exclusive: OCTO_SIZE,
        });
    }

    Ok(())
}

/// Optional helper: packed sector id for 2 x 2 x 2 domain.
#[inline]
pub fn sector_packed_id(sector: SectorCoord) -> Result<u8, AddressError> {
    validate_sector(sector)?;

    let id = sector.sx | (sector.sy << 1) | (sector.sz << 2);
    Ok(id as u8)
}

/// Optional helper: flat chunk id inside Region.
#[inline]
pub fn chunk_flat_index(chunk: ChunkCoord) -> Result<u32, AddressError> {
    validate_chunk(chunk)?;

    Ok(chunk.cx + REGION_CHUNKS_PER_AXIS * (chunk.cy + REGION_CHUNKS_PER_AXIS * chunk.cz))
}

/// Optional helper: flat octo id inside Chunk.
#[inline]
pub fn octo_flat_index(octo: OctochunkCoord) -> Result<u8, AddressError> {
    validate_octo(octo)?;

    let max = CHUNK_SIZE / OCTO_SIZE;
    let id = octo.ox + max * (octo.oy + max * octo.oz);
    Ok(id as u8)
}

/// Optional helper: recover Region-local voxel origin from FullRoute.
#[allow(dead_code)]
#[inline]
pub fn full_route_local_voxel_origin(full: FullRoute) -> Result<(u32, u32, u32), AddressError> {
    full.validate()?;

    let lx = full.chunk.cx * CHUNK_SIZE + full.octo.ox * OCTO_SIZE + full.voxel.vx;
    let ly = full.chunk.cy * CHUNK_SIZE + full.octo.oy * OCTO_SIZE + full.voxel.vy;
    let lz = full.chunk.cz * CHUNK_SIZE + full.octo.oz * OCTO_SIZE + full.voxel.vz;

    debug_assert!(lx < REGION_SIZE);
    debug_assert!(ly < REGION_SIZE);
    debug_assert!(lz < REGION_SIZE);

    Ok((lx, ly, lz))
}
