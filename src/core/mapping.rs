use bevy::prelude::Vec3;

use crate::core::address::{AddressError, DensityKey, FullRoute, SimSectorKey};
use crate::core::topology::{
    is_region_local_in_bounds, ChunkCoord, OctochunkCoord, RegionCoord, RuntimePosition,
    SectorCoord, VoxelCoord, CHUNK_SIZE, OCTO_SIZE, REGION_SECTOR_SIZE, REGION_SIZE,
};

/// Какую точку возвращать внутри адресуемой области.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorMode {
    /// Угол / origin адресуемой области.
    Corner,
    /// Геометрический центр deepest-уровня.
    Center,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MappingError {
    InvalidAddress(AddressError),
    LocalOutOfBounds { local: Vec3 },
    NonFinitePosition { value: Vec3 },
}

impl From<AddressError> for MappingError {
    fn from(value: AddressError) -> Self {
        Self::InvalidAddress(value)
    }
}

impl std::fmt::Display for MappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingError::InvalidAddress(err) => {
                write!(f, "invalid address: {err}")
            }
            MappingError::LocalOutOfBounds { local } => {
                write!(
                    f,
                    "region-local position is out of bounds: ({}, {}, {})",
                    local.x, local.y, local.z
                )
            }
            MappingError::NonFinitePosition { value } => {
                write!(
                    f,
                    "position contains non-finite values: ({}, {}, {})",
                    value.x, value.y, value.z
                )
            }
        }
    }
}

impl std::error::Error for MappingError {}

/// World-space float position -> RuntimePosition = Region + LocalFloat.
pub fn world_to_runtime_position(world: Vec3) -> Result<RuntimePosition, MappingError> {
    if !world.x.is_finite() || !world.y.is_finite() || !world.z.is_finite() {
        return Err(MappingError::NonFinitePosition { value: world });
    }

    let region_size = REGION_SIZE as f32;

    let rx = (world.x / region_size).floor() as i32;
    let ry = (world.y / region_size).floor() as i32;
    let rz = (world.z / region_size).floor() as i32;

    let local_x = world.x - rx as f32 * region_size;
    let local_y = world.y - ry as f32 * region_size;
    let local_z = world.z - rz as f32 * region_size;

    let local = Vec3::new(local_x, local_y, local_z);

    if !is_region_local_in_bounds(local) {
        return Err(MappingError::LocalOutOfBounds { local });
    }

    Ok(RuntimePosition {
        region: RegionCoord { rx, ry, rz },
        local,
    })
}

/// RuntimePosition -> world-space float position.
pub fn runtime_position_to_world(pos: RuntimePosition) -> Result<Vec3, MappingError> {
    if !pos.local.x.is_finite() || !pos.local.y.is_finite() || !pos.local.z.is_finite() {
        return Err(MappingError::NonFinitePosition { value: pos.local });
    }

    if !is_region_local_in_bounds(pos.local) {
        return Err(MappingError::LocalOutOfBounds { local: pos.local });
    }

    let region_size = REGION_SIZE as f32;

    let world_x = pos.region.rx as f32 * region_size + pos.local.x;
    let world_y = pos.region.ry as f32 * region_size + pos.local.y;
    let world_z = pos.region.rz as f32 * region_size + pos.local.z;

    Ok(Vec3::new(world_x, world_y, world_z))
}

/// RuntimePosition -> DensityKey = Region + Chunk.
pub fn runtime_to_density_key(pos: RuntimePosition) -> Result<DensityKey, MappingError> {
    if !is_region_local_in_bounds(pos.local) {
        return Err(MappingError::LocalOutOfBounds { local: pos.local });
    }

    let cx = (pos.local.x.floor() as u32) / CHUNK_SIZE;
    let cy = (pos.local.y.floor() as u32) / CHUNK_SIZE;
    let cz = (pos.local.z.floor() as u32) / CHUNK_SIZE;

    let key = DensityKey {
        region: pos.region,
        chunk: ChunkCoord { cx, cy, cz },
    };

    key.validate()?;
    Ok(key)
}

/// RuntimePosition -> SimSectorKey = Region + SectorCoord.
pub fn runtime_to_sim_sector_key(pos: RuntimePosition) -> Result<SimSectorKey, MappingError> {
    if !is_region_local_in_bounds(pos.local) {
        return Err(MappingError::LocalOutOfBounds { local: pos.local });
    }

    let sx = (pos.local.x.floor() as u32) / REGION_SECTOR_SIZE;
    let sy = (pos.local.y.floor() as u32) / REGION_SECTOR_SIZE;
    let sz = (pos.local.z.floor() as u32) / REGION_SECTOR_SIZE;

    let key = SimSectorKey {
        region: pos.region,
        sector: SectorCoord { sx, sy, sz },
    };

    key.validate()?;
    Ok(key)
}

/// RuntimePosition -> FullRoute = Region + Chunk + Octochunk + Voxel.
pub fn runtime_to_full_route(pos: RuntimePosition) -> Result<FullRoute, MappingError> {
    if !is_region_local_in_bounds(pos.local) {
        return Err(MappingError::LocalOutOfBounds { local: pos.local });
    }

    let lx = pos.local.x.floor() as u32;
    let ly = pos.local.y.floor() as u32;
    let lz = pos.local.z.floor() as u32;

    let cx = lx / CHUNK_SIZE;
    let cy = ly / CHUNK_SIZE;
    let cz = lz / CHUNK_SIZE;

    let rx = lx % CHUNK_SIZE;
    let ry = ly % CHUNK_SIZE;
    let rz = lz % CHUNK_SIZE;

    let ox = rx / OCTO_SIZE;
    let oy = ry / OCTO_SIZE;
    let oz = rz / OCTO_SIZE;

    let vx = rx % OCTO_SIZE;
    let vy = ry % OCTO_SIZE;
    let vz = rz % OCTO_SIZE;

    let full = FullRoute {
        region: pos.region,
        chunk: ChunkCoord { cx, cy, cz },
        octo: OctochunkCoord { ox, oy, oz },
        voxel: VoxelCoord { vx, vy, vz },
    };

    full.validate()?;
    Ok(full)
}

/// FullRoute -> RuntimePosition at selected anchor.
pub fn full_route_to_runtime_position(
    full: FullRoute,
    anchor: AnchorMode,
) -> Result<RuntimePosition, MappingError> {
    full.validate()?;

    let base_x = full.chunk.cx * CHUNK_SIZE + full.octo.ox * OCTO_SIZE + full.voxel.vx;
    let base_y = full.chunk.cy * CHUNK_SIZE + full.octo.oy * OCTO_SIZE + full.voxel.vy;
    let base_z = full.chunk.cz * CHUNK_SIZE + full.octo.oz * OCTO_SIZE + full.voxel.vz;

    let local = match anchor {
        AnchorMode::Corner => Vec3::new(base_x as f32, base_y as f32, base_z as f32),
        AnchorMode::Center => Vec3::new(
            base_x as f32 + 0.5,
            base_y as f32 + 0.5,
            base_z as f32 + 0.5,
        ),
    };

    if !is_region_local_in_bounds(local) {
        return Err(MappingError::LocalOutOfBounds { local });
    }

    Ok(RuntimePosition {
        region: full.region,
        local,
    })
}

/// DensityKey -> RuntimePosition anchor for whole Chunk.
pub fn density_key_to_runtime_position(
    key: DensityKey,
    anchor: AnchorMode,
) -> Result<RuntimePosition, MappingError> {
    key.validate()?;

    let base_x = key.chunk.cx * CHUNK_SIZE;
    let base_y = key.chunk.cy * CHUNK_SIZE;
    let base_z = key.chunk.cz * CHUNK_SIZE;

    let local = match anchor {
        AnchorMode::Corner => Vec3::new(base_x as f32, base_y as f32, base_z as f32),
        AnchorMode::Center => Vec3::new(
            base_x as f32 + CHUNK_SIZE as f32 * 0.5,
            base_y as f32 + CHUNK_SIZE as f32 * 0.5,
            base_z as f32 + CHUNK_SIZE as f32 * 0.5,
        ),
    };

    if !is_region_local_in_bounds(local) {
        return Err(MappingError::LocalOutOfBounds { local });
    }

    Ok(RuntimePosition {
        region: key.region,
        local,
    })
}

/// SimSectorKey -> RuntimePosition anchor for whole Sector volume.
#[allow(dead_code)]
pub fn sim_sector_key_to_runtime_position(
    key: SimSectorKey,
    anchor: AnchorMode,
) -> Result<RuntimePosition, MappingError> {
    key.validate()?;

    let base_x = key.sector.sx * REGION_SECTOR_SIZE;
    let base_y = key.sector.sy * REGION_SECTOR_SIZE;
    let base_z = key.sector.sz * REGION_SECTOR_SIZE;

    let local = match anchor {
        AnchorMode::Corner => Vec3::new(base_x as f32, base_y as f32, base_z as f32),
        AnchorMode::Center => Vec3::new(
            base_x as f32 + REGION_SECTOR_SIZE as f32 * 0.5,
            base_y as f32 + REGION_SECTOR_SIZE as f32 * 0.5,
            base_z as f32 + REGION_SECTOR_SIZE as f32 * 0.5,
        ),
    };

    if !is_region_local_in_bounds(local) {
        return Err(MappingError::LocalOutOfBounds { local });
    }

    Ok(RuntimePosition {
        region: key.region,
        local,
    })
}
