use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::core::{
    chunk_flat_index, octo_flat_index, runtime_to_density_key, runtime_to_full_route,
    runtime_to_sim_sector_key, sector_packed_id, world_to_runtime_position, DensityKey, FullRoute,
    RuntimePosition, SimSectorKey,
};
use crate::lab::world::LabVoxelWorld;
use crate::tools::debug::{CurrentTool, DebugUiState};

#[derive(Resource, Debug, Clone)]
pub struct InspectProbeSettings {
    pub enabled: bool,
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub wheel_step: f32,
}

impl Default for InspectProbeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            distance: 48.0,
            min_distance: 1.0,
            max_distance: 512.0,
            wheel_step: 4.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CameraProbe {
    pub world: Vec3,
    pub forward: Vec3,

    pub runtime: Option<RuntimePosition>,
    pub density: Option<DensityKey>,
    pub sim_sector: Option<SimSectorKey>,
    pub full_route: Option<FullRoute>,

    // helper/debug layer
    pub sector_id: Option<u8>,
    pub chunk_flat: Option<u32>,
    pub octo_flat: Option<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct InspectProbe {
    pub sample_world: Option<Vec3>,

    pub runtime: Option<RuntimePosition>,
    pub density: Option<DensityKey>,
    pub sim_sector: Option<SimSectorKey>,
    pub full_route: Option<FullRoute>,

    // Paint placement target:
    // last empty voxel before the first filled voxel on the ray.
    pub place_full_route: Option<FullRoute>,

    // helper/debug layer
    pub sector_id: Option<u8>,
    pub chunk_flat: Option<u32>,
    pub octo_flat: Option<u8>,
}

#[derive(Resource, Debug, Default)]
pub struct LabProbeState {
    pub camera: CameraProbe,
    pub inspect: InspectProbe,
    pub pinned: Option<InspectProbe>,
    pub last_error: Option<String>,
    pub inspect_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct ResolvedProbe {
    world: Vec3,
    runtime: RuntimePosition,
    density: DensityKey,
    sim_sector: SimSectorKey,
    full_route: FullRoute,
    sector_id: u8,
    chunk_flat: u32,
    octo_flat: u8,
}

#[derive(Debug, Clone)]
struct RaycastVoxelHit {
    filled: ResolvedProbe,
    place: Option<ResolvedProbe>,
}

pub fn adjust_inspect_probe_distance(
    keys: Res<ButtonInput<KeyCode>>,
    debug_ui: Res<DebugUiState>,
    mut wheel_events: EventReader<MouseWheel>,
    mut settings: ResMut<InspectProbeSettings>,
) {
    let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    if !settings.enabled || shift_pressed {
        for _ in wheel_events.read() {}
        return;
    }

    if debug_ui.current_tool != CurrentTool::Inspect {
        for _ in wheel_events.read() {}
        return;
    }

    let mut delta = 0.0;
    for ev in wheel_events.read() {
        delta += ev.y;
    }

    if delta == 0.0 {
        return;
    }

    // wheel up -> closer
    let next = settings.distance - delta * settings.wheel_step;
    settings.distance = next.clamp(settings.min_distance, settings.max_distance);
}

pub fn update_lab_probe(
    camera_q: Query<&Transform, With<Camera3d>>,
    inspect_settings: Res<InspectProbeSettings>,
    debug_ui: Res<DebugUiState>,
    world: Res<LabVoxelWorld>,
    mut probe: ResMut<LabProbeState>,
) {
    if debug_ui.clear_pinned_target_requested {
        probe.pinned = None;
    }

    let Ok(cam_transform) = camera_q.get_single() else {
        probe.last_error = Some("camera not found".to_string());
        probe.inspect_error = None;
        clear_camera_probe(&mut probe.camera);
        clear_inspect_probe(&mut probe.inspect);
        return;
    };

    let camera_world = cam_transform.translation;
    let camera_forward = (cam_transform.rotation * -Vec3::Z).normalize_or_zero();

    probe.last_error = None;
    probe.inspect_error = None;

    probe.camera.world = camera_world;
    probe.camera.forward = camera_forward;
    clear_camera_address_fields(&mut probe.camera);

    clear_inspect_probe(&mut probe.inspect);

    let resolved_camera = match resolve_probe(camera_world) {
        Ok(v) => v,
        Err(err) => {
            probe.last_error = Some(format!("camera resolve_probe failed: {err}"));
            return;
        }
    };

    apply_camera_resolved(&mut probe.camera, resolved_camera);

    if inspect_settings.enabled {
        match raycast_filled_voxel(
            camera_world,
            camera_forward,
            inspect_settings.max_distance,
            0.5,
            &world,
        ) {
            Some(hit) => {
                let place_full_route = hit.place.map(|p| p.full_route);
                apply_inspect_resolved(&mut probe.inspect, hit.filled);
                probe.inspect.place_full_route = place_full_route;
            }

            None => {
                let inspect_world = camera_world + camera_forward * inspect_settings.distance;
                probe.inspect.sample_world = Some(inspect_world);

                let resolved_inspect = match resolve_probe(inspect_world) {
                    Ok(v) => v,
                    Err(err) => {
                        probe.inspect_error = Some(format!("inspect resolve_probe failed: {err}"));
                        return;
                    }
                };

                apply_inspect_resolved(&mut probe.inspect, resolved_inspect);
            }
        }
    }

    if debug_ui.pin_target_requested {
        if probe.inspect.full_route.is_some() {
            probe.pinned = Some(probe.inspect.clone());
        } else {
            probe.inspect_error = Some("cannot pin target: no active inspect target".to_string());
        }
    }
}

fn resolve_probe(world: Vec3) -> Result<ResolvedProbe, String> {
    let runtime = world_to_runtime_position(world)
        .map_err(|err| format!("world_to_runtime_position: {err}"))?;

    let density =
        runtime_to_density_key(runtime).map_err(|err| format!("runtime_to_density_key: {err}"))?;

    let sim_sector = runtime_to_sim_sector_key(runtime)
        .map_err(|err| format!("runtime_to_sim_sector_key: {err}"))?;

    let sector_id =
        sector_packed_id(sim_sector.sector).map_err(|err| format!("sector_packed_id: {err}"))?;

    let full_route =
        runtime_to_full_route(runtime).map_err(|err| format!("runtime_to_full_route: {err}"))?;

    let chunk_flat =
        chunk_flat_index(density.chunk).map_err(|err| format!("chunk_flat_index: {err}"))?;

    let octo_flat =
        octo_flat_index(full_route.octo).map_err(|err| format!("octo_flat_index: {err}"))?;

    Ok(ResolvedProbe {
        world,
        runtime,
        density,
        sim_sector,
        full_route,
        sector_id,
        chunk_flat,
        octo_flat,
    })
}

fn apply_camera_resolved(camera: &mut CameraProbe, resolved: ResolvedProbe) {
    camera.world = resolved.world;
    camera.runtime = Some(resolved.runtime);
    camera.density = Some(resolved.density);
    camera.sim_sector = Some(resolved.sim_sector);
    camera.full_route = Some(resolved.full_route);
    camera.sector_id = Some(resolved.sector_id);
    camera.chunk_flat = Some(resolved.chunk_flat);
    camera.octo_flat = Some(resolved.octo_flat);
}

fn apply_inspect_resolved(inspect: &mut InspectProbe, resolved: ResolvedProbe) {
    inspect.sample_world = Some(resolved.world);
    inspect.runtime = Some(resolved.runtime);
    inspect.density = Some(resolved.density);
    inspect.sim_sector = Some(resolved.sim_sector);
    inspect.full_route = Some(resolved.full_route);
    inspect.place_full_route = None;
    inspect.sector_id = Some(resolved.sector_id);
    inspect.chunk_flat = Some(resolved.chunk_flat);
    inspect.octo_flat = Some(resolved.octo_flat);
}

fn clear_camera_probe(camera: &mut CameraProbe) {
    camera.world = Vec3::ZERO;
    camera.forward = Vec3::ZERO;
    clear_camera_address_fields(camera);
}

fn clear_camera_address_fields(camera: &mut CameraProbe) {
    camera.runtime = None;
    camera.density = None;
    camera.sim_sector = None;
    camera.full_route = None;
    camera.sector_id = None;
    camera.chunk_flat = None;
    camera.octo_flat = None;
}

fn clear_inspect_probe(inspect: &mut InspectProbe) {
    inspect.sample_world = None;
    inspect.runtime = None;
    inspect.density = None;
    inspect.sim_sector = None;
    inspect.full_route = None;
    inspect.place_full_route = None;
    inspect.sector_id = None;
    inspect.chunk_flat = None;
    inspect.octo_flat = None;
}

fn raycast_filled_voxel(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    step: f32,
    world: &LabVoxelWorld,
) -> Option<RaycastVoxelHit> {
    let dir = direction.normalize_or_zero();

    if dir.length_squared() == 0.0 {
        return None;
    }

    let mut distance = 0.0;
    let mut last_route: Option<FullRoute> = None;
    let mut previous_empty: Option<ResolvedProbe> = None;

    while distance <= max_distance {
        let sample_world = origin + dir * distance;

        let resolved = match resolve_probe(sample_world) {
            Ok(v) => v,
            Err(_) => {
                distance += step;
                continue;
            }
        };

        if last_route == Some(resolved.full_route) {
            distance += step;
            continue;
        }

        last_route = Some(resolved.full_route);

        if world.get_voxel(resolved.full_route) != 0 {
            return Some(RaycastVoxelHit {
                filled: resolved,
                place: previous_empty,
            });
        }

        previous_empty = Some(resolved);
        distance += step;
    }

    None
}
