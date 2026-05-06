#![allow(unused_imports)]

pub mod address;
pub mod mapping;
pub mod topology;
pub mod voxel;

// legacy modules are intentionally disconnected from active MVP core:
// pub mod route;
// pub mod registry;

pub use topology::{
    is_region_local_in_bounds, ChunkCoord, OctochunkCoord, RegionCoord, RuntimePosition,
    SectorCoord, VoxelCoord, CHUNK_SIZE, OCTO_SIZE, OCTO_SPLIT_PER_AXIS, REGION_CHUNKS_PER_AXIS,
    REGION_SECTOR_SIZE, REGION_SECTOR_SPLIT, REGION_SIZE, VOXEL_SIZE,
};

pub use voxel::grid::VoxelGrid;

pub use address::{
    chunk_flat_index, fmt_runtime_position, full_route_local_voxel_origin, octo_flat_index,
    sector_packed_id, AddressAxis, AddressError, AddressLevel, DensityKey, FullRoute, SimSectorKey,
};

pub use mapping::{
    density_key_to_runtime_position, full_route_to_runtime_position, runtime_position_to_world,
    runtime_to_density_key, runtime_to_full_route, runtime_to_sim_sector_key,
    sim_sector_key_to_runtime_position, world_to_runtime_position, AnchorMode, MappingError,
};
