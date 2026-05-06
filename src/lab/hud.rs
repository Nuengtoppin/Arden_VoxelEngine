use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::core::{DensityKey, FullRoute, RuntimePosition, SimSectorKey};
use crate::lab::formatters::{
    build_human_probe_view, fmt_density_machine, fmt_full_route_machine, fmt_runtime_machine,
    fmt_sim_machine, fmt_world,
};

use crate::lab::probe::{InspectProbeSettings, LabProbeState};
use crate::tools::debug::{CurrentTool, DebugLens, DebugNotation, DebugPresentation, DebugUiState};

use crate::lab::clipboard::LabClipboard;
use crate::lab::object::LabObjectRegistry;
use crate::lab::sandbox::{LabMode, LabSandboxState};
use crate::lab::save::LabSaveStatus;
use crate::lab::selection::{selection_voxel_dims, SelectionBoxState};
use crate::lab::world::LabVoxelWorld;

#[derive(Debug, Clone, Copy)]
struct DeepHudTarget {
    world: Vec3,
    runtime: Option<RuntimePosition>,
    density: Option<DensityKey>,
    sim_sector: Option<SimSectorKey>,
    full_route: Option<FullRoute>,
    sector_id: Option<u8>,
    chunk_flat: Option<u32>,
    octo_flat: Option<u8>,
    source: &'static str,
}

pub fn draw_lab_hud(
    mut contexts: EguiContexts,
    probe: Res<LabProbeState>,
    inspect_settings: Res<InspectProbeSettings>,
    world: Res<LabVoxelWorld>,
    selection: Res<SelectionBoxState>,
    clipboard: Res<LabClipboard>,
    objects: Res<LabObjectRegistry>,
    save_status: Res<LabSaveStatus>,
    mut sandbox: ResMut<LabSandboxState>,
    mut debug_ui: ResMut<DebugUiState>,
) {
    if !debug_ui.overlay_enabled {
        return;
    }

    let ctx = contexts.ctx_mut();
    let max_hud_height = (ctx.available_rect().height() - 24.0).max(240.0);

    egui::Area::new("lab_truth_overlay")
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .order(egui::Order::Foreground)
        .movable(false)
        .interactable(true)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 10, 14, 210))
                .stroke(egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(90, 90, 110, 180),
                ))
                .rounding(egui::Rounding::same(6.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    let min_width = match debug_ui.presentation {
                        DebugPresentation::Compact => 320.0,
                        DebugPresentation::Detailed => 420.0,
                    };

                    ui.set_min_width(min_width);
                    ui.set_max_width(420.0);

                    egui::ScrollArea::vertical()
                        .max_height(max_hud_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            {
                                let debug_view = &*debug_ui;

                                draw_overlay_header(ui, debug_view);

                                match debug_view.notation {
                                    DebugNotation::Machine => draw_machine_overlay(
                                        ui,
                                        &probe,
                                        &inspect_settings,
                                        &world,
                                        debug_view,
                                    ),
                                    DebugNotation::Human => draw_human_overlay(
                                        ui,
                                        &probe,
                                        &inspect_settings,
                                        &world,
                                        debug_view,
                                    ),
                                }
                            }

                            if let Some(err) = &probe.last_error {
                                ui.add_space(6.0);
                                ui.separator();
                                ui.add_space(4.0);
                                ui.colored_label(
                                    egui::Color32::from_rgb(255, 140, 140),
                                    format!("camera error  {err}"),
                                );
                            }

                            if let Some(err) = &probe.inspect_error {
                                ui.add_space(4.0);
                                ui.colored_label(
                                    egui::Color32::from_rgb(255, 210, 120),
                                    format!("inspect error  {err}"),
                                );
                            }

                            draw_sandbox_controls(ui, sandbox.as_mut());
                            draw_world_summary(ui, &world);
                            draw_selection_summary(ui, &selection);
                            draw_clipboard_summary(ui, &clipboard);
                            draw_object_summary(ui, &objects);
                            draw_save_summary(ui, &save_status);
                            draw_gizmo_controls(ui, debug_ui.as_mut());
                        });
                });
        });
}

fn draw_overlay_header(ui: &mut egui::Ui, debug_ui: &DebugUiState) {
    let lens = match debug_ui.lens {
        DebugLens::Runtime => "Runtime",
        DebugLens::Density => "Density",
        DebugLens::Sim => "Sim",
        DebugLens::Deep => "Deep",
        DebugLens::Dun => "Dun",
    };

    let notation = match debug_ui.notation {
        DebugNotation::Machine => "Machine",
        DebugNotation::Human => "Human",
    };

    let presentation = match debug_ui.presentation {
        DebugPresentation::Compact => "Compact",
        DebugPresentation::Detailed => "Detailed",
    };

    let tool = match debug_ui.current_tool {
        CurrentTool::Inspect => "Inspect",
        CurrentTool::SelectBox => "SelectBox",
        CurrentTool::Paint => "Paint",
        CurrentTool::Erase => "Erase",
    };

    ui.monospace(format!("TRUTH / {lens} / {notation} / {presentation}"));
    ui.monospace(format!("TOOL  / {tool}"));
    ui.add_space(6.0);
}

fn draw_machine_overlay(
    ui: &mut egui::Ui,
    probe: &LabProbeState,
    inspect_settings: &InspectProbeSettings,
    world: &LabVoxelWorld,
    debug_ui: &DebugUiState,
) {
    match debug_ui.lens {
        DebugLens::Runtime => draw_runtime_machine(ui, probe, debug_ui.presentation),
        DebugLens::Density => draw_density_machine(ui, probe, debug_ui.presentation),
        DebugLens::Sim => draw_sim_machine(ui, probe, debug_ui.presentation),
        DebugLens::Deep => {
            draw_deep_machine(ui, probe, inspect_settings, world, debug_ui.presentation)
        }
        DebugLens::Dun => draw_dun_stub(ui),
    }
}

fn draw_human_overlay(
    ui: &mut egui::Ui,
    probe: &LabProbeState,
    inspect_settings: &InspectProbeSettings,
    world: &LabVoxelWorld,
    debug_ui: &DebugUiState,
) {
    match debug_ui.lens {
        DebugLens::Runtime => draw_runtime_human(ui, probe, debug_ui.presentation),
        DebugLens::Density => draw_density_human(ui, probe, debug_ui.presentation),
        DebugLens::Sim => draw_sim_human(ui, probe, debug_ui.presentation),
        DebugLens::Deep => {
            draw_deep_human(ui, probe, inspect_settings, world, debug_ui.presentation)
        }
        DebugLens::Dun => draw_dun_stub(ui),
    }
}

fn draw_runtime_machine(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    ui.monospace(format!("world    {}", fmt_world(probe.camera.world)));
    ui.monospace(format!("forward  {}", fmt_world(probe.camera.forward)));

    match probe.camera.runtime {
        Some(v) => ui.monospace(format!("runtime  {}", fmt_runtime_machine(v))),
        None => ui.monospace("runtime  <none>"),
    };

    if matches!(presentation, DebugPresentation::Detailed) {
        match probe.camera.density {
            Some(v) => ui.monospace(format!("density  {}", fmt_density_machine(v))),
            None => ui.monospace("density  <none>"),
        };

        match probe.camera.sim_sector {
            Some(v) => ui.monospace(format!("sim      {}", fmt_sim_machine(v))),
            None => ui.monospace("sim      <none>"),
        };
    }
}

fn draw_density_machine(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    match probe.camera.density {
        Some(v) => ui.monospace(format!("density  {}", fmt_density_machine(v))),
        None => ui.monospace("density  <none>"),
    };

    match probe.camera.chunk_flat {
        Some(v) => ui.monospace(format!("C#       {}", v)),
        None => ui.monospace("C#       <none>"),
    };

    if matches!(presentation, DebugPresentation::Detailed) {
        match probe.camera.full_route {
            Some(v) => ui.monospace(format!("full     {}", fmt_full_route_machine(v))),
            None => ui.monospace("full     <none>"),
        };
    }
}

fn draw_sim_machine(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    match probe.camera.sim_sector {
        Some(v) => ui.monospace(format!("sim      {}", fmt_sim_machine(v))),
        None => ui.monospace("sim      <none>"),
    };

    match probe.camera.sector_id {
        Some(v) => ui.monospace(format!("S#       {}", v)),
        None => ui.monospace("S#       <none>"),
    };

    if matches!(presentation, DebugPresentation::Detailed) {
        match probe.camera.runtime {
            Some(v) => ui.monospace(format!("runtime  {}", fmt_runtime_machine(v))),
            None => ui.monospace("runtime  <none>"),
        };
    }
}

fn draw_deep_machine(
    ui: &mut egui::Ui,
    probe: &LabProbeState,
    inspect_settings: &InspectProbeSettings,
    world: &LabVoxelWorld,
    presentation: DebugPresentation,
) {
    let target = pick_deep_target(probe);

    ui.monospace(format!("source     {}", target.source));

    if target.source == "hover" {
        ui.monospace(format!("inspect d  {:.1}", inspect_settings.distance));
    }

    ui.monospace(format!("world      {}", fmt_world(target.world)));

    match target.runtime {
        Some(v) => ui.monospace(format!("runtime    {}", fmt_runtime_machine(v))),
        None => ui.monospace("runtime    <none>"),
    };

    match target.full_route {
        Some(v) => ui.monospace(format!("voxel      {}", world.get_voxel(v))),
        None => ui.monospace("voxel      <none>"),
    };

    match target.octo_flat {
        Some(v) => ui.monospace(format!("O#         {}", v)),
        None => ui.monospace("O#         <none>"),
    };

    if matches!(presentation, DebugPresentation::Detailed) {
        match target.density {
            Some(v) => ui.monospace(format!("density    {}", fmt_density_machine(v))),
            None => ui.monospace("density    <none>"),
        };

        match target.sim_sector {
            Some(v) => ui.monospace(format!("sim        {}", fmt_sim_machine(v))),
            None => ui.monospace("sim        <none>"),
        };

        match target.chunk_flat {
            Some(v) => ui.monospace(format!("C#         {}", v)),
            None => ui.monospace("C#         <none>"),
        };

        match target.sector_id {
            Some(v) => ui.monospace(format!("S#         {}", v)),
            None => ui.monospace("S#         <none>"),
        };
    }
}

fn draw_runtime_human(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    let view = build_human_probe_view(
        probe.camera.runtime,
        probe.camera.density,
        probe.camera.sim_sector,
        probe.camera.full_route,
    );

    ui.monospace(format!("world   {}", fmt_world(probe.camera.world)));
    ui.monospace(view.region_label);
    ui.monospace(format!("Local   {}", view.runtime_local_centered));

    if matches!(presentation, DebugPresentation::Detailed) {
        if let Some(v) = view.sector_label {
            ui.monospace(v);
        }
        if let Some(v) = view.chunk_label {
            ui.monospace(v);
        }
    }
}

fn draw_density_human(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    let view = build_human_probe_view(
        probe.camera.runtime,
        probe.camera.density,
        probe.camera.sim_sector,
        probe.camera.full_route,
    );

    ui.monospace(view.region_label);

    if let Some(v) = view.chunk_label {
        ui.monospace(v);
    } else {
        ui.monospace("Chunk <none>");
    }

    if matches!(presentation, DebugPresentation::Detailed) {
        if let Some(v) = view.octo_label {
            ui.monospace(v);
        }
        if let Some(v) = view.voxel_label {
            ui.monospace(v);
        }
    }
}

fn draw_sim_human(ui: &mut egui::Ui, probe: &LabProbeState, presentation: DebugPresentation) {
    let view = build_human_probe_view(
        probe.camera.runtime,
        probe.camera.density,
        probe.camera.sim_sector,
        probe.camera.full_route,
    );

    ui.monospace(view.region_label);

    if let Some(v) = view.sector_label {
        ui.monospace(v);
    } else {
        ui.monospace("Sector <none>");
    }

    if let Some(v) = view.sector_compact {
        ui.monospace(v);
    }

    if matches!(presentation, DebugPresentation::Detailed) {
        ui.monospace(format!("Local   {}", view.runtime_local_centered));
    }
}

fn draw_deep_human(
    ui: &mut egui::Ui,
    probe: &LabProbeState,
    inspect_settings: &InspectProbeSettings,
    world: &LabVoxelWorld,
    presentation: DebugPresentation,
) {
    let target = pick_deep_target(probe);

    ui.monospace(format!("Source  {}", target.source));

    if target.source == "hover" {
        ui.monospace(format!("Dist    {:.1}", inspect_settings.distance));
    }

    ui.monospace(format!("World   {}", fmt_world(target.world)));

    match target.full_route {
        Some(v) => ui.monospace(format!("Voxel   {}", world.get_voxel(v))),
        None => ui.monospace("Voxel   <none>"),
    };

    let view = build_human_probe_view(
        target.runtime,
        target.density,
        target.sim_sector,
        target.full_route,
    );

    ui.monospace(view.region_label);

    if let Some(v) = view.chunk_label {
        ui.monospace(v);
    }
    if let Some(v) = view.octo_label {
        ui.monospace(v);
    }
    if let Some(v) = view.voxel_label {
        ui.monospace(v);
    }

    if matches!(presentation, DebugPresentation::Detailed) {
        if let Some(v) = view.sector_label {
            ui.monospace(v);
        }
        ui.monospace(format!("Local   {}", view.runtime_local_centered));
    }
}

fn draw_dun_stub(ui: &mut egui::Ui) {
    ui.monospace("DUN lens: not connected yet");
}

fn pick_deep_target(probe: &LabProbeState) -> DeepHudTarget {
    if let Some(pinned) = &probe.pinned {
        DeepHudTarget {
            world: pinned.sample_world.unwrap_or(probe.camera.world),
            runtime: pinned.runtime,
            density: pinned.density,
            sim_sector: pinned.sim_sector,
            full_route: pinned.full_route,
            sector_id: pinned.sector_id,
            chunk_flat: pinned.chunk_flat,
            octo_flat: pinned.octo_flat,
            source: "pinned",
        }
    } else if probe.inspect.sample_world.is_some() {
        DeepHudTarget {
            world: probe.inspect.sample_world.unwrap_or(probe.camera.world),
            runtime: probe.inspect.runtime,
            density: probe.inspect.density,
            sim_sector: probe.inspect.sim_sector,
            full_route: probe.inspect.full_route,
            sector_id: probe.inspect.sector_id,
            chunk_flat: probe.inspect.chunk_flat,
            octo_flat: probe.inspect.octo_flat,
            source: "hover",
        }
    } else {
        DeepHudTarget {
            world: probe.camera.world,
            runtime: probe.camera.runtime,
            density: probe.camera.density,
            sim_sector: probe.camera.sim_sector,
            full_route: probe.camera.full_route,
            sector_id: probe.camera.sector_id,
            chunk_flat: probe.camera.chunk_flat,
            octo_flat: probe.camera.octo_flat,
            source: "camera",
        }
    }
}

fn draw_gizmo_controls(ui: &mut egui::Ui, debug_ui: &mut DebugUiState) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("GIZMOS");
    ui.checkbox(&mut debug_ui.gizmos_enabled, "Show Gizmos");

    if !debug_ui.gizmos_enabled {
        return;
    }

    ui.add_space(4.0);

    ui.checkbox(&mut debug_ui.show_region_gizmo, "Region");
    ui.checkbox(&mut debug_ui.show_sector_gizmo, "Sector");
    ui.checkbox(&mut debug_ui.show_chunk_gizmo, "Chunk");
    ui.checkbox(&mut debug_ui.show_octo_gizmo, "Octochunk");
    ui.checkbox(&mut debug_ui.show_hover_gizmo, "Hover Target");
    ui.checkbox(&mut debug_ui.show_pinned_gizmo, "Pinned Target");
    ui.checkbox(&mut debug_ui.show_selection_gizmo, "Selection");
    ui.checkbox(&mut debug_ui.show_object_gizmo, "Objects");
}

fn draw_sandbox_controls(ui: &mut egui::Ui, sandbox: &mut LabSandboxState) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("SANDBOX");

    ui.horizontal(|ui| {
        ui.radio_value(&mut sandbox.mode, LabMode::Edit, "Edit");
        ui.radio_value(&mut sandbox.mode, LabMode::Runtime, "Runtime");
    });

    ui.checkbox(&mut sandbox.profile.finite_world, "Finite World");

    ui.monospace(format!(
        "Edit Tools  {}",
        if sandbox.edit_tools_allowed() {
            "enabled"
        } else {
            "runtime locked"
        }
    ));

    let min = sandbox.profile.region_min;
    let max = sandbox.profile.region_max_exclusive();
    let dims = sandbox.profile.region_dims;

    ui.monospace(format!("Region Min   [{}|{}|{}]", min.x, min.y, min.z));
    ui.monospace(format!("Region Max   [{}|{}|{}] excl", max.x, max.y, max.z));
    ui.monospace(format!("Region Dims  [{}|{}|{}]", dims.x, dims.y, dims.z));
}

fn draw_world_summary(ui: &mut egui::Ui, world: &LabVoxelWorld) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("WORLD");
    ui.monospace(format!("Stored Chunks  {}", world.chunk_count()));
    ui.monospace(format!("Dirty Chunks   {}", world.dirty_count()));
    ui.monospace("Storage        Chunk -> VoxelGrid");
}

fn draw_selection_summary(ui: &mut egui::Ui, selection: &SelectionBoxState) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("SELECTION");

    let state = if selection.is_open() {
        "open"
    } else if selection.is_ready() {
        "ready"
    } else {
        "idle"
    };

    ui.monospace(format!("State          {}", state));

    match (selection.start, selection.end) {
        (Some(start), Some(end)) => {
            ui.monospace(format!("Start          {}", start));
            ui.monospace(format!("End            {}", end));

            if let Some(dims) = selection_voxel_dims(selection) {
                ui.monospace(format!("Voxel Dims     [{}|{}|{}]", dims.x, dims.y, dims.z));
            }

            if selection.is_ready() {
                ui.monospace("Actions        Delete / F");
            }
        }
        _ => {
            ui.monospace("Start          <none>");
            ui.monospace("End            <none>");
        }
    }
}

fn draw_clipboard_summary(ui: &mut egui::Ui, clipboard: &LabClipboard) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("CLIPBOARD");

    let Some(volume) = clipboard.volume.as_ref() else {
        ui.monospace("State          empty");
        ui.monospace("Actions        C / V");
        return;
    };

    ui.monospace("State          ready");
    ui.monospace(format!(
        "Dims           [{}|{}|{}]",
        volume.dims.x, volume.dims.y, volume.dims.z,
    ));
    ui.monospace(format!("Solid Voxels   {}", volume.non_empty_count()));
    ui.monospace("Actions        C / V");
}

fn draw_save_summary(ui: &mut egui::Ui, save_status: &LabSaveStatus) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("SAVE / LOAD");
    ui.monospace(format!("Path           {}", save_status.path));
    ui.monospace(format!("Status         {}", save_status.last_message));
    ui.monospace("Actions        F5 / F9");
}

fn draw_object_summary(ui: &mut egui::Ui, objects: &LabObjectRegistry) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    ui.monospace("OBJECTS");
    ui.monospace(format!("Count          {}", objects.object_count()));
    ui.monospace(format!("Status         {}", objects.last_message));
    ui.monospace("Actions        X Copy / Shift+X Cut");
    ui.monospace("Move           J/L X, U/O Y, K/I Z");
    ui.monospace("Rotate         G/H C4 yaw");
    ui.monospace("Bake           B to World");
    ui.monospace("Select         N next / M prev");
    ui.monospace("Delete         Backspace");

    let Some(object) = objects.selected_object().or_else(|| objects.last_object()) else {
        ui.monospace("Selected       <none>");
        return;
    };

    ui.monospace(format!("Selected       #{}", object.id.0));
    ui.monospace(format!(
        "Origin         [{}|{}|{}]",
        object.world_origin.x, object.world_origin.y, object.world_origin.z,
    ));
    ui.monospace(format!("Orientation    {}", object.orientation.label()));
    ui.monospace(format!(
        "Dims           [{}|{}|{}]",
        object.payload.dims.x, object.payload.dims.y, object.payload.dims.z,
    ));
    ui.monospace(format!(
        "Pivot Local    ({:.2}|{:.2}|{:.2})",
        object.pivot_local.x, object.pivot_local.y, object.pivot_local.z,
    ));
    ui.monospace(format!("Solid Voxels   {}", object.solid_voxels));
}
