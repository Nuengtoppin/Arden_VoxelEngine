use bevy::prelude::*;

use crate::core::{
    full_route_to_runtime_position, runtime_position_to_world, AnchorMode, RuntimePosition,
    CHUNK_SIZE, OCTO_SIZE, REGION_SECTOR_SIZE, REGION_SIZE, VOXEL_SIZE,
};

use crate::lab::object::{LabObjectRegistry, LabVoxelObject};
use crate::lab::probe::LabProbeState;
use crate::lab::selection::{selection_world_bounds, SelectionBoxState};
use crate::tools::debug::DebugUiState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GizmoLayer {
    Region = 0,
    Sector = 1,
    Chunk = 2,
    Octochunk = 3,
    InspectVoxel = 4,
    PinnedVoxel = 5,
    SelectionVolume = 6,
    ObjectBounds = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
struct AxisSegment {
    axis: Axis,
    fixed_a: i32,
    fixed_b: i32,
    start: i32,
    end: i32,
    color: Color,
    layer: GizmoLayer,
}

impl AxisSegment {
    fn same_support_line(&self, other: &Self) -> bool {
        self.axis == other.axis && self.fixed_a == other.fixed_a && self.fixed_b == other.fixed_b
    }

    fn with_interval(&self, start: i32, end: i32) -> Self {
        Self {
            axis: self.axis,
            fixed_a: self.fixed_a,
            fixed_b: self.fixed_b,
            start,
            end,
            color: self.color,
            layer: self.layer,
        }
    }

    fn to_vec3(self) -> (Vec3, Vec3) {
        match self.axis {
            Axis::X => (
                Vec3::new(self.start as f32, self.fixed_a as f32, self.fixed_b as f32),
                Vec3::new(self.end as f32, self.fixed_a as f32, self.fixed_b as f32),
            ),
            Axis::Y => (
                Vec3::new(self.fixed_a as f32, self.start as f32, self.fixed_b as f32),
                Vec3::new(self.fixed_a as f32, self.end as f32, self.fixed_b as f32),
            ),
            Axis::Z => (
                Vec3::new(self.fixed_a as f32, self.fixed_b as f32, self.start as f32),
                Vec3::new(self.fixed_a as f32, self.fixed_b as f32, self.end as f32),
            ),
        }
    }
}

pub fn draw_lab_gizmos(
    mut gizmos: Gizmos,
    probe: Res<LabProbeState>,
    selection: Res<SelectionBoxState>,
    objects: Res<LabObjectRegistry>,
    debug_ui: Res<DebugUiState>,
) {
    if !debug_ui.gizmos_enabled {
        return;
    }

    let mut segments: Vec<AxisSegment> = Vec::new();

    if debug_ui.show_region_gizmo {
        collect_region_bounds(&mut segments, &probe);
    }

    if debug_ui.show_sector_gizmo {
        collect_sim_sector_bounds(&mut segments, &probe);
    }

    if debug_ui.show_chunk_gizmo {
        collect_chunk_bounds(&mut segments, &probe);
    }

    if debug_ui.show_octo_gizmo {
        collect_octo_bounds(&mut segments, &probe);
    }

    if debug_ui.show_hover_gizmo {
        collect_inspect_voxel_bounds(&mut segments, &probe);
    }

    if debug_ui.show_pinned_gizmo {
        collect_pinned_voxel_bounds(&mut segments, &probe);
    }

    if debug_ui.show_selection_gizmo {
        collect_selection_bounds(&mut segments, &selection);
    }

    if debug_ui.show_object_gizmo {
        collect_selected_object_bounds(&mut segments, &objects);
    }

    for seg in segments {
        let (a, b) = seg.to_vec3();
        gizmos.line(a, b, seg.color);
    }

    if debug_ui.show_object_gizmo {
        draw_selected_object_pivot(&mut gizmos, &objects);
    }
}

fn collect_region_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let Some(runtime) = probe.camera.runtime else {
        return;
    };

    let min = region_min_world(runtime);
    let max = min + Vec3::splat(REGION_SIZE as f32);

    let color = Color::rgba(1.0, 0.35, 0.35, 0.9);
    collect_wire_box(segments, min, max, color, GizmoLayer::Region);
}

fn collect_sim_sector_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let (Some(runtime), Some(sim)) = (probe.camera.runtime, probe.camera.sim_sector) else {
        return;
    };

    let region_min = region_min_world(runtime);

    let min = region_min
        + Vec3::new(
            sim.sector.sx as f32 * REGION_SECTOR_SIZE as f32,
            sim.sector.sy as f32 * REGION_SECTOR_SIZE as f32,
            sim.sector.sz as f32 * REGION_SECTOR_SIZE as f32,
        );

    let max = min + Vec3::splat(REGION_SECTOR_SIZE as f32);

    let color = Color::rgba(0.35, 0.75, 1.0, 0.85);
    collect_wire_box(segments, min, max, color, GizmoLayer::Sector);
}

fn collect_chunk_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let (Some(runtime), Some(density)) = (probe.camera.runtime, probe.camera.density) else {
        return;
    };

    let region_min = region_min_world(runtime);

    let min = region_min
        + Vec3::new(
            density.chunk.cx as f32 * CHUNK_SIZE as f32,
            density.chunk.cy as f32 * CHUNK_SIZE as f32,
            density.chunk.cz as f32 * CHUNK_SIZE as f32,
        );

    let max = min + Vec3::splat(CHUNK_SIZE as f32);

    let color = Color::rgba(0.45, 1.0, 0.45, 0.95);
    collect_wire_box(segments, min, max, color, GizmoLayer::Chunk);
}

fn collect_octo_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let (Some(runtime), Some(full)) = (probe.camera.runtime, probe.camera.full_route) else {
        return;
    };

    let region_min = region_min_world(runtime);

    let min = region_min
        + Vec3::new(
            full.chunk.cx as f32 * CHUNK_SIZE as f32 + full.octo.ox as f32 * OCTO_SIZE as f32,
            full.chunk.cy as f32 * CHUNK_SIZE as f32 + full.octo.oy as f32 * OCTO_SIZE as f32,
            full.chunk.cz as f32 * CHUNK_SIZE as f32 + full.octo.oz as f32 * OCTO_SIZE as f32,
        );

    let max = min + Vec3::splat(OCTO_SIZE as f32);

    // вдвое тусклее, чем chunk
    let color = Color::rgba(0.225, 0.50, 0.225, 0.55);
    collect_wire_box(segments, min, max, color, GizmoLayer::Octochunk);
}

fn collect_inspect_voxel_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let Some(full) = probe.inspect.full_route else {
        return;
    };

    let runtime_corner = match full_route_to_runtime_position(full, AnchorMode::Corner) {
        Ok(v) => v,
        Err(_) => return,
    };

    let min = match runtime_position_to_world(runtime_corner) {
        Ok(v) => v,
        Err(_) => return,
    };

    let max = min + Vec3::splat(VOXEL_SIZE);

    let color = Color::rgba(1.0, 0.95, 0.2, 1.0);
    collect_wire_box(segments, min, max, color, GizmoLayer::InspectVoxel);
}

fn collect_pinned_voxel_bounds(segments: &mut Vec<AxisSegment>, probe: &LabProbeState) {
    let Some(pinned) = &probe.pinned else {
        return;
    };

    let Some(full) = pinned.full_route else {
        return;
    };

    let runtime_corner = match full_route_to_runtime_position(full, AnchorMode::Corner) {
        Ok(v) => v,
        Err(_) => return,
    };

    let min = match runtime_position_to_world(runtime_corner) {
        Ok(v) => v,
        Err(_) => return,
    };

    let max = min + Vec3::splat(VOXEL_SIZE);

    let color = Color::rgba(1.0, 0.55, 0.15, 1.0);
    collect_wire_box(segments, min, max, color, GizmoLayer::PinnedVoxel);
}

fn region_min_world(runtime: RuntimePosition) -> Vec3 {
    Vec3::new(
        runtime.region.rx as f32 * REGION_SIZE as f32,
        runtime.region.ry as f32 * REGION_SIZE as f32,
        runtime.region.rz as f32 * REGION_SIZE as f32,
    )
}

fn collect_wire_box(
    segments: &mut Vec<AxisSegment>,
    min: Vec3,
    max: Vec3,
    color: Color,
    layer: GizmoLayer,
) {
    let lines = [
        (
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(max.x, min.y, min.z),
        ),
        (
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, min.y, max.z),
        ),
        (
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(min.x, min.y, max.z),
        ),
        (
            Vec3::new(min.x, min.y, max.z),
            Vec3::new(min.x, min.y, min.z),
        ),
        (
            Vec3::new(min.x, max.y, min.z),
            Vec3::new(max.x, max.y, min.z),
        ),
        (
            Vec3::new(max.x, max.y, min.z),
            Vec3::new(max.x, max.y, max.z),
        ),
        (
            Vec3::new(max.x, max.y, max.z),
            Vec3::new(min.x, max.y, max.z),
        ),
        (
            Vec3::new(min.x, max.y, max.z),
            Vec3::new(min.x, max.y, min.z),
        ),
        (
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(min.x, max.y, min.z),
        ),
        (
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, max.y, min.z),
        ),
        (
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(max.x, max.y, max.z),
        ),
        (
            Vec3::new(min.x, min.y, max.z),
            Vec3::new(min.x, max.y, max.z),
        ),
    ];

    for (a, b) in lines {
        insert_segment(segments, a, b, color, layer);
    }
}

fn insert_segment(
    segments: &mut Vec<AxisSegment>,
    a: Vec3,
    b: Vec3,
    color: Color,
    layer: GizmoLayer,
) {
    let Some(new_seg) = make_axis_segment(a, b, color, layer) else {
        return;
    };

    let full_start = new_seg.start;
    let full_end = new_seg.end;

    let mut pending: Vec<(i32, i32)> = vec![(full_start, full_end)];
    let mut rebuilt: Vec<AxisSegment> = Vec::with_capacity(segments.len() + 2);

    for existing in segments.drain(..) {
        if !existing.same_support_line(&new_seg) {
            rebuilt.push(existing);
            continue;
        }

        if existing.layer >= new_seg.layer {
            pending = subtract_intervals(&pending, existing.start, existing.end);
            rebuilt.push(existing);
        } else {
            let leftovers =
                subtract_single_interval(existing.start, existing.end, full_start, full_end);

            for (start, end) in leftovers {
                rebuilt.push(existing.with_interval(start, end));
            }
        }
    }

    for (start, end) in pending {
        rebuilt.push(new_seg.with_interval(start, end));
    }

    *segments = rebuilt;
}

fn make_axis_segment(a: Vec3, b: Vec3, color: Color, layer: GizmoLayer) -> Option<AxisSegment> {
    let qa = quantize_vec3(a);
    let qb = quantize_vec3(b);

    let dx = qa.0 != qb.0;
    let dy = qa.1 != qb.1;
    let dz = qa.2 != qb.2;

    match (dx, dy, dz) {
        (true, false, false) => {
            let (start, end) = sort_pair(qa.0, qb.0);
            Some(AxisSegment {
                axis: Axis::X,
                fixed_a: qa.1,
                fixed_b: qa.2,
                start,
                end,
                color,
                layer,
            })
        }
        (false, true, false) => {
            let (start, end) = sort_pair(qa.1, qb.1);
            Some(AxisSegment {
                axis: Axis::Y,
                fixed_a: qa.0,
                fixed_b: qa.2,
                start,
                end,
                color,
                layer,
            })
        }
        (false, false, true) => {
            let (start, end) = sort_pair(qa.2, qb.2);
            Some(AxisSegment {
                axis: Axis::Z,
                fixed_a: qa.0,
                fixed_b: qa.1,
                start,
                end,
                color,
                layer,
            })
        }
        _ => None,
    }
}

fn quantize_vec3(v: Vec3) -> (i32, i32, i32) {
    (v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
}

fn sort_pair(a: i32, b: i32) -> (i32, i32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn subtract_intervals(input: &[(i32, i32)], cut_start: i32, cut_end: i32) -> Vec<(i32, i32)> {
    let mut out = Vec::new();

    for &(start, end) in input {
        out.extend(subtract_single_interval(start, end, cut_start, cut_end));
    }

    out
}

fn subtract_single_interval(start: i32, end: i32, cut_start: i32, cut_end: i32) -> Vec<(i32, i32)> {
    let overlap_start = start.max(cut_start);
    let overlap_end = end.min(cut_end);

    if overlap_start >= overlap_end {
        return vec![(start, end)];
    }

    let mut out = Vec::new();

    if start < overlap_start {
        out.push((start, overlap_start));
    }

    if overlap_end < end {
        out.push((overlap_end, end));
    }

    out
}

fn collect_selection_bounds(segments: &mut Vec<AxisSegment>, selection: &SelectionBoxState) {
    let Some((min, max)) = selection_world_bounds(selection) else {
        return;
    };

    let color = Color::rgba(0.25, 0.95, 1.0, 0.95);
    collect_wire_box(segments, min, max, color, GizmoLayer::SelectionVolume);
}

fn collect_selected_object_bounds(segments: &mut Vec<AxisSegment>, objects: &LabObjectRegistry) {
    let Some(object) = objects.selected_object().or_else(|| objects.last_object()) else {
        return;
    };

    let center = object.world_origin.as_vec3() + object.payload.dims.as_vec3() * 0.5;

let dims = match object.orientation {
    crate::lab::object::ObjectOrientation::R0
    | crate::lab::object::ObjectOrientation::R180 => object.payload.dims.as_vec3(),

    crate::lab::object::ObjectOrientation::R90
    | crate::lab::object::ObjectOrientation::R270 => {
        Vec3::new(
            object.payload.dims.z as f32,
            object.payload.dims.y as f32,
            object.payload.dims.x as f32,
        )
    }
};

let min = center - dims * 0.5;
let max = center + dims * 0.5;

    let color = Color::rgba(1.0, 0.35, 1.0, 0.95);
    collect_wire_box(segments, min, max, color, GizmoLayer::ObjectBounds);
}

fn draw_selected_object_pivot(gizmos: &mut Gizmos, objects: &LabObjectRegistry) {
    let Some(object) = objects.selected_object().or_else(|| objects.last_object()) else {
        return;
    };

    draw_object_pivot(gizmos, object);
}

fn draw_object_pivot(gizmos: &mut Gizmos, object: &LabVoxelObject) {
    let center = object.world_origin.as_vec3() + object.pivot_local;

    let radius = 2.0;
    let color = Color::rgba(1.0, 0.75, 1.0, 1.0);

    gizmos.line(center - Vec3::X * radius, center + Vec3::X * radius, color);

    gizmos.line(center - Vec3::Y * radius, center + Vec3::Y * radius, color);

    gizmos.line(center - Vec3::Z * radius, center + Vec3::Z * radius, color);
}
