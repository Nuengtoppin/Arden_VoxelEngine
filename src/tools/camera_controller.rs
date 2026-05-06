use bevy::input::mouse::{MouseButton, MouseMotion, MouseWheel};
use bevy::prelude::*;

/// Простейшая fly-камера (полёт + вращение мышью).
#[derive(Component)]
pub struct FlyCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub move_speed: f32,
    pub move_speed_fast: f32,
    pub mouse_sensitivity: f32,
    pub wheel_dolly_step: f32,
}

impl FlyCamera {
    pub fn from_transform(transform: &Transform) -> Self {
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        Self {
            yaw,
            pitch,
            move_speed: 16.0,
            move_speed_fast: 64.0,
            mouse_sensitivity: 0.15,
            wheel_dolly_step: 40.0,
        }
    }
}

/// Поворот камеры правой кнопкой мыши.
pub fn fly_camera_look(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let (mut transform, mut flycam) = if let Ok(v) = query.get_single_mut() {
        v
    } else {
        for _ in mouse_motion.read() {}
        return;
    };

    if !mouse_buttons.pressed(MouseButton::Right) {
        for _ in mouse_motion.read() {}
        return;
    }

    let mut delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        delta += ev.delta;
    }
    if delta == Vec2::ZERO {
        return;
    }

    let dt = time.delta_seconds();
    flycam.yaw -= delta.x * flycam.mouse_sensitivity * dt;
    flycam.pitch -= delta.y * flycam.mouse_sensitivity * dt;

    flycam.pitch = flycam.pitch.clamp(-1.5, 1.5);

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, flycam.yaw) * Quat::from_axis_angle(Vec3::X, flycam.pitch);
}

/// Shift + Wheel = dolly camera forward/back.
pub fn fly_camera_dolly_wheel(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    if !shift_pressed {
        for _ in mouse_wheel.read() {}
        return;
    }

    let (mut transform, flycam) = if let Ok(v) = query.get_single_mut() {
        v
    } else {
        for _ in mouse_wheel.read() {}
        return;
    };

    let mut delta = 0.0;
    for ev in mouse_wheel.read() {
        delta += ev.y;
    }

    if delta == 0.0 {
        return;
    }

    let forward = *transform.forward();
    transform.translation += forward * (delta * flycam.wheel_dolly_step);
}

/// Движение: WASD + Space/Ctrl, Shift — ускорение.
pub fn fly_camera_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    let (mut transform, flycam) = if let Ok(v) = query.get_single_mut() {
        v
    } else {
        return;
    };

    let mut dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= *transform.forward();
    }

    if keys.pressed(KeyCode::KeyA) {
        dir -= *transform.right();
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += *transform.right();
    }

    if keys.pressed(KeyCode::Space) {
        dir += Vec3::Y;
    }
    if keys.pressed(KeyCode::ControlLeft) {
        dir -= Vec3::Y;
    }

    if dir.length_squared() == 0.0 {
        return;
    }

    dir = dir.normalize();
    let mut speed = flycam.move_speed;
    if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        speed = flycam.move_speed_fast;
    }

    transform.translation += dir * speed * time.delta_seconds();
}
