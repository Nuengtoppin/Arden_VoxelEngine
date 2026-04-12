// src/physics/collider_builder.rs
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy_rapier3d::prelude::*;

use crate::voxel::grid::VoxelGrid;
use crate::voxel::VOXEL_SIZE;

pub fn collider_from_mesh(mesh: &Mesh) -> Collider {
    let positions: Vec<Vec3> = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float32x3(verts)) => verts
            .iter()
            .map(|[x, y, z]| Vec3::new(*x, *y, *z))
            .collect(),
        _ => {
            return Collider::cuboid(0.5, 0.5, 0.5);
        }
    };

    let raw_indices: Vec<u32> = match mesh.indices() {
        Some(Indices::U32(ind)) => ind.clone(),
        Some(Indices::U16(ind)) => ind.iter().map(|&i| i as u32).collect(),
        None => (0..positions.len() as u32).collect(),
    };

    let mut indices_triangles = Vec::new();
    for chunk in raw_indices.chunks(3) {
        if let [a, b, c] = chunk {
            indices_triangles.push([*a, *b, *c]);
        }
    }

    if positions.is_empty() || indices_triangles.is_empty() {
        Collider::cuboid(0.5, 0.5, 0.5)
    } else {
        Collider::trimesh(positions, indices_triangles)
    }
}

/// Коробка по размерам воксельной решётки.
pub fn collider_from_voxel_grid_box(grid: &VoxelGrid) -> Collider {
    let size = grid.size;

    let hx = size.x as f32 * VOXEL_SIZE * 0.5;
    let hy = size.y as f32 * VOXEL_SIZE * 0.5;
    let hz = size.z as f32 * VOXEL_SIZE * 0.5;

    Collider::cuboid(hx, hy, hz)
}
