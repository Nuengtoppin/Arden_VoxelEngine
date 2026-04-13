// src/mvp0/scene.rs
use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::app::setup::setup_camera_and_light;
// use crate::app::setup::spawn_aquarium;
// use crate::tools::debug_grid::debug_grid_system;
use crate::tools::camera_controller::{fly_camera_look, fly_camera_move};
use crate::dun::spawn::spawn_single_dun;

pub struct LabScenePlugin;

impl Plugin for LabScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // Один раз: камера + свет
            .add_systems(Startup, (setup_camera_and_light, ))
            // Каждый кадр: сетка, управление камерой, спавн DUN по E
            .add_systems(
                Update,
                (
                    //debug_grid_system,
                    fly_camera_look,
                    fly_camera_move,
                    spawn_dun_on_e,
                ),
            );
    }
}

/// Временная система для MVP0:
/// по нажатию E спавним один DUN в направлении камеры,
/// с привязкой к сетке (шаг 16 world units).
fn spawn_dun_on_e(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_q: Query<&Transform, With<Camera3d>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Берём единственную 3D-камеру
    let Ok(cam_transform) = camera_q.get_single() else {
        return;
    };

    // Вектор взгляда камеры.
    // В Bevy по умолчанию камера смотрит вдоль -Z,
    // поэтому forward = rotation * (-Z).
    let forward = cam_transform.rotation * -Vec3::Z;

    // На каком расстоянии от камеры спавнить DUN (в мире, не в вокселях).
    // 32.0 ≈ "два контейнера вперёд" при VOXEL_SIZE=1.0 и size=32.
    let spawn_distance = 32.0;

    let raw_pos = cam_transform.translation + forward.normalize() * spawn_distance;

    // Привязка к сетке. Можно поменять шаг на 32.0, если хочешь
    // сразу снаппить по размеру контейнера DUN.
    let grid_step = 16.0;

    let snapped = Vec3::new(
        (raw_pos.x / grid_step).round() * grid_step,
        (raw_pos.y / grid_step).round() * grid_step,
        (raw_pos.z / grid_step).round() * grid_step,
    );

    let spawn_transform = Transform::from_translation(snapped);

    // Вызываем конструктор DUN: он сам строит шар и коллайдер по мешу.
    spawn_single_dun(
        &mut commands,
        &mut meshes,
        &mut materials,
        spawn_transform,
    );
}
