use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// если Rapier пока нужен:
//use bevy_rapier3d::prelude::*;

mod app;
mod core;
mod lab;
mod render;
mod tools;
// mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(render::lab_chunk_render::LabChunkRenderPlugin)
        .add_plugins(render::lab_object_render::LabObjectRenderPlugin)
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(lab::scene::LabScenePlugin)
        .run();
}
