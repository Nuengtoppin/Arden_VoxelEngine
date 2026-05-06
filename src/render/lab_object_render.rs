use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;

use crate::core::VoxelGrid;
use crate::lab::object::{LabObjectRegistry, LabVoxelObject, LabVoxelObjectId};
use crate::render::mesh_builder::build_bevy_mesh;

const OBJECT_PREVIEW_SCALE: f32 = 1.0;

pub struct LabObjectRenderPlugin;

impl Plugin for LabObjectRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LabObjectRenderState>()
            .add_systems(Startup, setup_lab_object_render_material)
            .add_systems(Update, rebuild_lab_object_meshes);
    }
}

#[derive(Resource, Default)]
pub struct LabObjectRenderState {
    pub entities: HashMap<LabVoxelObjectId, Entity>,
}

#[derive(Resource, Clone)]
pub struct LabObjectRenderMaterial {
    pub handle: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct LabObjectRenderTag;

fn setup_lab_object_render_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        // Debug-preview color: intentionally different from world chunk material.
        base_color: Color::rgb(0.95, 0.45, 1.0),
        perceptual_roughness: 0.85,
        metallic: 0.0,
        ..default()
    });

    commands.insert_resource(LabObjectRenderMaterial { handle: material });
}

fn rebuild_lab_object_meshes(
    mut commands: Commands,
    objects: Res<LabObjectRegistry>,
    material: Res<LabObjectRenderMaterial>,
    mut render_state: ResMut<LabObjectRenderState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform, With<LabObjectRenderTag>>,
) {
    if !objects.is_changed() {
        return;
    }

    let mut still_alive: HashMap<LabVoxelObjectId, Entity> = HashMap::new();

    for object in objects.objects.iter() {
        if object.solid_voxels == 0 {
            continue;
        }

        let entity = if let Some(existing) = render_state.entities.get(&object.id).copied() {
            if let Ok(mut transform) = transforms.get_mut(existing) {
                *transform = object_render_transform(object);
                existing
            } else {
                spawn_object_entity(&mut commands, &mut meshes, &material, object)
            }
        } else {
            spawn_object_entity(&mut commands, &mut meshes, &material, object)
        };

        still_alive.insert(object.id, entity);
    }

    for (id, entity) in render_state.entities.drain() {
        if !still_alive.contains_key(&id) {
            commands.entity(entity).despawn_recursive();
        }
    }

    render_state.entities = still_alive;
}

fn spawn_object_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &LabObjectRenderMaterial,
    object: &LabVoxelObject,
) -> Entity {
    let grid = payload_to_grid(object);
    let mesh = build_bevy_mesh(&grid);
    let mesh_handle = meshes.add(mesh);

    commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: material.handle.clone(),
                transform: object_render_transform(object),
                ..default()
            },
            LabObjectRenderTag,
            NoFrustumCulling,
        ))
        .id()
}

fn object_render_transform(object: &LabVoxelObject) -> Transform {
    Transform {
        translation: object.world_origin.as_vec3() + object.payload.dims.as_vec3() * 0.5,
        rotation: Quat::from_rotation_y(object.orientation.yaw_radians()),
        scale: Vec3::splat(OBJECT_PREVIEW_SCALE),
        ..default()
    }
}

fn payload_to_grid(object: &LabVoxelObject) -> VoxelGrid {
    VoxelGrid {
        size: object.payload.dims,
        data: object.payload.data.clone(),
    }
}
