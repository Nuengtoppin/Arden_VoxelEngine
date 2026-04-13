// src/dun/spawn.rs
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::core::VoxelGrid;
use crate::core::topology::VOXEL_SIZE;
use crate::render::mesh_builder::build_bevy_mesh;

use crate::physics::collider_builder::{
    collider_from_voxel_grid_box,
};

use crate::dun::dun::Dun;
use crate::dun::kind::DunKind;

/// Строим VoxelGrid 32×32×32 с шаром по центру.
/// Это и есть "наполнение" DUN для MVP.
fn build_dun_sphere_grid() -> VoxelGrid {
    // 32³ по умолчанию
    let mut grid = VoxelGrid::new_dun_default();
    let size = grid.size;

    // Центр в воксельных координатах (по центрам ячеек)
    let cx = size.x as f32 / 2.0;
    let cy = size.y as f32 / 2.0;
    let cz = size.z as f32 / 2.0;

    // Радиус — чуть меньше половины по минимальной оси,
    // чтобы шар не упирался в край
    let r = (size.x.min(size.y).min(size.z) as f32 * 0.5) - 1.0;
    let r2 = r * r;

    for x in 0..size.x {
        for y in 0..size.y {
            for z in 0..size.z {
                let vx = x as f32 + 0.5;
                let vy = y as f32 + 0.5;
                let vz = z as f32 + 0.5;

                let dx = vx - cx;
                let dy = vy - cy;
                let dz = vz - cz;

                let dist2 = dx * dx + dy * dy + dz * dz;

                if dist2 <= r2 {
                    grid.set(x, y, z, 1);
                }
            }
        }
    }

    grid
}

/// Спавн одного DUN (контейнер 32³ с шаром внутри) с ЗАДАННЫМ трансформом.
/// transform трактуем как центр контейнера DUN в мировых координатах.
pub fn spawn_single_dun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    // 1. Грид 32×32×32 с voxel-шаром
    let grid = build_dun_sphere_grid();

    // 2. Mesh из грида (он уже центрирован вокруг (0,0,0))
    let mesh = build_bevy_mesh(&grid);

    // 3. ПРОСТОЙ коллайдер по размеру контейнера DUN,
    //    а не по каждому треугольнику меша:
    let collider = collider_from_voxel_grid_box(&grid);

    // 4. Кладём меш в ресурсы
    let mesh_handle = meshes.add(mesh);

    // 5. Материал
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.6, 1.0),
        perceptual_roughness: 0.8,
        metallic: 0.0,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    // 6. Спавн сущности DUN
    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform,
            ..default()
        },
        RigidBody::Fixed,
        collider,
        Dun {
            kind: DunKind::DynamicVoxel,
            chunk_coord: IVec3::ZERO,
            voxel: grid,
        },
    ));
}
