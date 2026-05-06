use bevy::prelude::*;

use crate::core::{
    chunk_flat_index, fmt_runtime_position, octo_flat_index, sector_packed_id, DensityKey,
    FullRoute, RuntimePosition, SimSectorKey, REGION_SIZE,
};

pub fn fmt_world(v: Vec3) -> String {
    format!("({:.2}, {:.2}, {:.2})", v.x, v.y, v.z)
}

pub fn fmt_runtime_machine(pos: RuntimePosition) -> String {
    match fmt_runtime_position(pos) {
        Ok(s) => s,
        Err(_) => format!(
            "R({}|{}|{}) / p(<invalid>|<invalid>|<invalid>)",
            pos.region.rx, pos.region.ry, pos.region.rz
        ),
    }
}

pub fn fmt_density_machine(key: DensityKey) -> String {
    key.to_string()
}

pub fn fmt_sim_machine(key: SimSectorKey) -> String {
    key.to_string()
}

pub fn fmt_full_route_machine(full: FullRoute) -> String {
    full.to_string()
}

#[derive(Debug, Clone)]
pub struct HumanProbeView {
    pub region_label: String,
    pub runtime_local_centered: String,
    pub sector_label: Option<String>,
    pub sector_compact: Option<String>,
    pub chunk_label: Option<String>,
    pub octo_label: Option<String>,
    pub voxel_label: Option<String>,
}

pub fn build_human_probe_view(
    runtime: Option<RuntimePosition>,
    density: Option<DensityKey>,
    sim_sector: Option<SimSectorKey>,
    full_route: Option<FullRoute>,
) -> HumanProbeView {
    let region_label = match runtime {
        Some(pos) => fmt_region_human(pos.region.rx, pos.region.ry, pos.region.rz),
        None => "Region [<none>]".to_string(),
    };

    let runtime_local_centered = match runtime {
        Some(pos) => fmt_local_centered(pos.local),
        None => "<none>".to_string(),
    };

    let sector_label = sim_sector.map(fmt_sector_human_label);
    let sector_compact =
        sim_sector.and_then(|s| sector_packed_id(s.sector).ok().map(|id| format!("S#{id}")));

    let chunk_label = density.and_then(|d| {
        chunk_flat_index(d.chunk)
            .ok()
            .map(|id| format!("Chunk #{id}"))
    });

    let octo_label =
        full_route.and_then(|f| octo_flat_index(f.octo).ok().map(|id| format!("Octo #{id}")));

    let voxel_label =
        full_route.map(|f| format!("Voxel ({}|{}|{})", f.voxel.vx, f.voxel.vy, f.voxel.vz));

    HumanProbeView {
        region_label,
        runtime_local_centered,
        sector_label,
        sector_compact,
        chunk_label,
        octo_label,
        voxel_label,
    }
}

pub fn fmt_region_human(rx: i32, ry: i32, rz: i32) -> String {
    format!("Region [{}|{}|{}]", rx, ry, rz)
}

pub fn fmt_local_centered(local: Vec3) -> String {
    let half = REGION_SIZE as f32 * 0.5;

    let cx = local.x - half;
    let cy = local.y - half;
    let cz = local.z - half;

    format!("({:+.2}|{:+.2}|{:+.2})", cx, cy, cz)
}

/// Human/debug sector label.
/// Machine truth remains numeric SectorCoord / S(...).
pub fn fmt_sector_human_label(key: SimSectorKey) -> String {
    let label = match (key.sector.sx, key.sector.sy, key.sector.sz) {
        (0, 1, 0) => "A",
        (1, 1, 0) => "B",
        (1, 1, 1) => "C",
        (0, 1, 1) => "D",
        (0, 0, 0) => "E",
        (1, 0, 0) => "F",
        (1, 0, 1) => "G",
        (0, 0, 1) => "I",
        _ => "?",
    };

    format!("Sector {label}")
}
