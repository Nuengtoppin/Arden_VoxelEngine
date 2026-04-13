// src/main.rs
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod core;
mod app;
mod lab;
mod tools;
mod voxel;
mod render;
mod physics;
mod dun;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Физика
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // Визуализация коллайдеров (можно закомментировать при желании)
        .add_plugins(RapierDebugRenderPlugin::default())
        // Наша сцена MVP0
        .add_plugins(lab::scene::LabScenePlugin)
        .run();
}
