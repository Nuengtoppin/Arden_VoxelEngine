// src/render/mesh_builder.rs
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use crate::voxel::grid::VoxelGrid;
use crate::voxel::VOXEL_SIZE;

/// Строим "кубический" меш из вокселей.
/// - центрируем вокруг (0,0,0);
/// - рисуем только внешние грани (neighbor culling);
/// - нормали смотрят НАРУЖУ.
pub fn build_bevy_mesh(grid: &VoxelGrid) -> Mesh {
    let size = grid.size;

    let extent = Vec3::new(
        size.x as f32 * VOXEL_SIZE,
        size.y as f32 * VOXEL_SIZE,
        size.z as f32 * VOXEL_SIZE,
    );
    let center = extent * 0.5;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals:   Vec<[f32; 3]> = Vec::new();
    let mut uvs:       Vec<[f32; 2]> = Vec::new();
    let mut indices:   Vec<u32>      = Vec::new();

    // Хелпер: добавляет одну квадратную грань (2 треугольника).
    // ВЕРШИНЫ в порядке CCW (если смотреть С НАРУЖИ на грань).
    let mut push_face = |verts: [Vec3; 4], normal: Vec3| {
        let base = positions.len() as u32;

        for v in verts {
            let p = v - center; // центрируем вокруг (0,0,0)
            positions.push(p.to_array());
            normals.push(normal.to_array());
            uvs.push([0.0, 0.0]);
        }

        indices.extend_from_slice(&[
            base,     base + 1, base + 2,
            base,     base + 2, base + 3,
        ]);
    };

    // Сосед пустой, если:
    // - вне границ грида
    // - или воксель = 0
    let is_empty = |x: i32, y: i32, z: i32| -> bool {
        if x < 0 || y < 0 || z < 0 ||
           x >= size.x as i32 ||
           y >= size.y as i32 ||
           z >= size.z as i32
        {
            return true;
        }
        grid.get(x as u32, y as u32, z as u32) == 0
    };

    for x in 0..size.x {
        for y in 0..size.y {
            for z in 0..size.z {
                if grid.get(x, y, z) == 0 {
                    continue;
                }

                let fx = x as f32 * VOXEL_SIZE;
                let fy = y as f32 * VOXEL_SIZE;
                let fz = z as f32 * VOXEL_SIZE;

                let min = Vec3::new(fx, fy, fz);
                let max = min + Vec3::splat(VOXEL_SIZE);

                let ix = x as i32;
                let iy = y as i32;
                let iz = z as i32;

                // +X
                if is_empty(ix + 1, iy, iz) {
                    push_face(
                        [
                            Vec3::new(max.x, min.y, min.z),
                            Vec3::new(max.x, max.y, min.z),
                            Vec3::new(max.x, max.y, max.z),
                            Vec3::new(max.x, min.y, max.z),
                        ],
                        Vec3::X,
                    );
                }

                // -X
                if is_empty(ix - 1, iy, iz) {
                    push_face(
                        [
                            Vec3::new(min.x, min.y, max.z),
                            Vec3::new(min.x, max.y, max.z),
                            Vec3::new(min.x, max.y, min.z),
                            Vec3::new(min.x, min.y, min.z),
                        ],
                        -Vec3::X,
                    );
                }

                // +Y (верх) — нормаль вверх, вершины CCW при взгляде сверху
                if is_empty(ix, iy + 1, iz) {
                    push_face(
                        [
                            // v0: min x, max z
                            Vec3::new(min.x, max.y, max.z),
                            // v1: max x, max z
                            Vec3::new(max.x, max.y, max.z),
                            // v2: max x, min z
                            Vec3::new(max.x, max.y, min.z),
                            // v3: min x, min z
                            Vec3::new(min.x, max.y, min.z),
                        ],
                        Vec3::Y,
                    );
                }

                // -Y (низ) — нормаль вниз, вершины CCW при взгляде снизу
                if is_empty(ix, iy - 1, iz) {
                    push_face(
                        [
                            // v0: min x, min z
                            Vec3::new(min.x, min.y, min.z),
                            // v1: max x, min z
                            Vec3::new(max.x, min.y, min.z),
                            // v2: max x, max z
                            Vec3::new(max.x, min.y, max.z),
                            // v3: min x, max z
                            Vec3::new(min.x, min.y, max.z),
                        ],
                        -Vec3::Y,
                    );
                }

                // +Z (передняя грань) — нормаль вперёд (+Z)
                if is_empty(ix, iy, iz + 1) {
                    push_face(
                        [
                            // v0: min x, min y
                            Vec3::new(min.x, min.y, max.z),
                            // v1: max x, min y
                            Vec3::new(max.x, min.y, max.z),
                            // v2: max x, max y
                            Vec3::new(max.x, max.y, max.z),
                            // v3: min x, max y
                            Vec3::new(min.x, max.y, max.z),
                        ],
                        Vec3::Z,
                    );
                }

                // -Z (задняя грань) — нормаль назад (-Z)
                if is_empty(ix, iy, iz - 1) {
                    push_face(
                        [
                            // v0: max x, min y
                            Vec3::new(max.x, min.y, min.z),
                            // v1: min x, min y
                            Vec3::new(min.x, min.y, min.z),
                            // v2: min x, max y
                            Vec3::new(min.x, max.y, min.z),
                            // v3: max x, max y
                            Vec3::new(max.x, max.y, min.z),
                        ],
                        -Vec3::Z,
                    );
                }

            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
