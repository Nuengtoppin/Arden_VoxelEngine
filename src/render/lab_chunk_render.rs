use std::collections::HashMap;

use bevy::prelude::*;

use crate::core::density_key_to_runtime_position;
use crate::core::{runtime_position_to_world, AnchorMode, DensityKey, VoxelGrid};
use crate::lab::world::LabVoxelWorld;
use crate::render::mesh_builder::build_bevy_mesh;
use bevy::render::view::NoFrustumCulling;

pub struct LabChunkRenderPlugin;

impl Plugin for LabChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LabChunkRenderState>()
            .add_systems(Startup, setup_lab_chunk_render_material)
            .add_systems(Update, rebuild_visible_lab_chunks);
    }
}

#[derive(Resource, Default)]
pub struct LabChunkRenderState {
    pub entities: HashMap<DensityKey, Entity>,
}

#[derive(Resource, Clone)]
pub struct LabChunkRenderMaterial {
    pub handle: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct LabChunkRenderTag;

fn setup_lab_chunk_render_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.72, 0.78, 0.88),
        perceptual_roughness: 0.9,
        metallic: 0.0,
        ..default()
    });

    commands.insert_resource(LabChunkRenderMaterial { handle: material });
}

fn rebuild_visible_lab_chunks(
    mut commands: Commands,
    world: Res<LabVoxelWorld>,
    material: Res<LabChunkRenderMaterial>,
    mut render_state: ResMut<LabChunkRenderState>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, With<LabChunkRenderTag>>,
) {
    if !world.is_changed() {
        return;
    }

    let mut still_alive: HashMap<DensityKey, Entity> = HashMap::new();

    for (&key, grid) in world.chunks.iter() {
        if chunk_is_effectively_empty(grid) {
            continue;
        }

        let mesh = build_bevy_mesh(grid);

        let runtime = match density_key_to_runtime_position(key, AnchorMode::Corner) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let world_pos = match runtime_position_to_world(runtime) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let chunk_extent = Vec3::new(grid.size.x as f32, grid.size.y as f32, grid.size.z as f32);

        let render_pos = world_pos + chunk_extent * 0.5;

        let mesh_handle = meshes.add(mesh);

        let entity = if let Some(existing) = render_state.entities.get(&key).copied() {
            commands.entity(existing).insert((
                PbrBundle {
                    mesh: mesh_handle,
                    material: material.handle.clone(),
                    transform: Transform::from_translation(render_pos),
                    ..default()
                },
                NoFrustumCulling,
            ));
            existing
        } else {
            commands
                .spawn((
                    PbrBundle {
                        mesh: mesh_handle,
                        material: material.handle.clone(),
                        transform: Transform::from_translation(render_pos),
                        ..default()
                    },
                    LabChunkRenderTag,
                    NoFrustumCulling,
                ))
                .id()
        };

        still_alive.insert(key, entity);
    }

    for (key, entity) in render_state.entities.drain() {
        if !still_alive.contains_key(&key) {
            if query.get(entity).is_ok() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    render_state.entities = still_alive;
}

fn chunk_is_effectively_empty(grid: &VoxelGrid) -> bool {
    grid.data.iter().all(|&v| v == 0)
}
