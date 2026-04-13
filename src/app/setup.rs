// src/app/setup.rs
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::tools::camera_controller::FlyCamera;


pub fn setup_camera_and_light(mut commands: Commands) {
    // Камера оставляем как есть
    let camera_transform = Transform::from_xyz(-80.0, 80.0, 120.0)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..default()
        },
        FlyCamera::from_transform(&camera_transform),
    ));

    // 1) Основной "солнечный" свет (key light)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 0.97, 0.92),
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(180.0, 160.0, 180.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 2) Заполняющий свет с обратной стороны, без теней (fill light)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.85, 0.9),
            illuminance: 4_000.0, // заметно слабее
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-180.0, 40.0, -180.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.85, 0.9),
            illuminance: 4_000.0, // заметно слабее
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-180.0, -40.0, -180.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.85, 0.9),
            illuminance: 10_000.0, // заметно слабее
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(180.0, -160.0, 180.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 3) Общий ambient, можно чуть приглушить, чтобы fill-свет работал
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.6, 0.6, 0.7),
        brightness: 1.3,
    });
}


/// Пол 512x512 + невидимые стены-борта высотой 512.
/// Верх аквариума открыт.
pub fn spawn_aquarium(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_size = 512.0;
    let wall_height = 512.0;
    let wall_thickness = 1.0;

    // --- Пол ---
    let floor_mesh = Mesh::from(shape::Plane::from_size(plane_size));
    let floor_collider = collider_from_mesh(&floor_mesh);
    let floor_mesh_handle = meshes.add(floor_mesh);

    let floor_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.12, 0.18, 0.12),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: floor_mesh_handle,
            material: floor_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        floor_collider,
    ));

    // --- Стены (ТОЛЬКО КОЛЛАЙДЕРЫ, без видимого меша) ---
    let half_thickness = wall_thickness * 0.5;
    let half_height = wall_height * 0.5;
    let half_size = plane_size * 0.5;

    // Стены вдоль оси X (поставлены по краям по Z)
    let wall_x_collider = Collider::cuboid(half_thickness, half_height, half_size);

    // +X
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            half_size + half_thickness,
            half_height,
            0.0,
        )),
        RigidBody::Fixed,
        wall_x_collider.clone(),
    ));

    // -X
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            -half_size - half_thickness,
            half_height,
            0.0,
        )),
        RigidBody::Fixed,
        wall_x_collider,
    ));

    // Стены вдоль оси Z
    let wall_z_collider = Collider::cuboid(half_size, half_height, half_thickness);

    // +Z
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            0.0,
            half_height,
            half_size + half_thickness,
        )),
        RigidBody::Fixed,
        wall_z_collider.clone(),
    ));

    // -Z
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            0.0,
            half_height,
            -half_size - half_thickness,
        )),
        RigidBody::Fixed,
        wall_z_collider,
    ));
        // --- Потолок (невидимый, чисто коллайдер) ---
    let ceiling_thickness = 1.0;
    let half_ceiling_thickness = ceiling_thickness * 0.5;

    // Центр потолка: на высоте верхнего края стен
    let ceiling_y = wall_height + half_ceiling_thickness;

    let ceiling_collider = Collider::cuboid(
        half_size,              // по X половина размера пола
        half_ceiling_thickness, // толщина по Y
        half_size,              // по Z половина размера пола
    );

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, ceiling_y, 0.0)),
        RigidBody::Fixed,
        ceiling_collider,
    ));

}
