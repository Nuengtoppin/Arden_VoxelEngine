// src/app/setup.rs
use crate::tools::camera_controller::FlyCamera;
use bevy::prelude::*;

pub fn setup_camera_and_light(mut commands: Commands) {
    // Камера оставляем как есть
    let camera_transform = Transform::from_xyz(-80.0, 80.0, 120.0).looking_at(Vec3::ZERO, Vec3::Y);

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
        transform: Transform::from_xyz(180.0, 160.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        transform: Transform::from_xyz(-180.0, 40.0, -180.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.85, 0.9),
            illuminance: 4_000.0, // заметно слабее
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-180.0, -40.0, -180.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.85, 0.9),
            illuminance: 10_000.0, // заметно слабее
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(180.0, -160.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 3) Общий ambient, можно чуть приглушить, чтобы fill-свет работал
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.6, 0.6, 0.7),
        brightness: 1.3,
    });
}
