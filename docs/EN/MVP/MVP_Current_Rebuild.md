**Document Status:** 🧩 Draft  
**Version:** 0.1.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07  

---

# 🧩 Arden Engine — MVP Current Rebuild

## 🎯 Purpose

This document records the current technical rebuild of the **Arden Engine** MVP direction.

It is not a changelog, not a final specification, and not an exact description of separate git patches.

The document is used to record:

- what is currently built in `src`;
- why these parts were added;
- which layers are already foundational;
- which parts are still raw or temporary;
- what will be refined later;
- how the current codebase connects to the next MVP stages.

The current rebuild is treated as a general implementation bundle, not as a set of perfectly separated patches.

---

## 🧭 General Rebuild Logic

The old active order was closer to a DUN-first MVP0:

```text
DUN prototype -> physics check -> stress test
```

The new development order:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
```

This means DUN is no longer the first MVP foundation.  
First, spatial truth is established, then a controlled lab sandbox is built, and only after that DUN becomes the next runtime/container layer.

---

# 1. Spatial Core

## Why it was added

Spatial Core is needed as the basic spatial truth of the engine.

It is responsible for:

- container sizes;
- coordinate structures;
- address forms;
- transitions between world-space and discrete topology;
- the foundation for debug/probe/HUD/tools.

This layer is not UI, not an editor, and not a DUN system.

---

## What it includes

- `Region`;
- `Chunk`;
- `Octochunk`;
- `Voxel`;
- `Sector`;
- `RuntimePosition`;
- `DensityKey`;
- `SimSectorKey`;
- `FullRoute`;
- validation helpers;
- mapping helpers.

---

## Source files

| File                     | Role                                                 |
| ------------------------ | ---------------------------------------------------- |
| `src/core/topology.rs`   | sizes, coordinates, `RuntimePosition`, bounds check  |
| `src/core/address.rs`    | address forms, validation, display helpers           |
| `src/core/mapping.rs`    | world/runtime/address conversion                     |
| `src/core/voxel/grid.rs` | dense voxel grid primitive                           |
| `src/core/mod.rs`        | public exports for the active core API               |

---

## Status

Foundational layer.

Requires further synchronization with architecture documentation and future tests for mapping/address helpers.

---

# 2. Lab Sandbox

## Why it was added

Lab Sandbox is needed as a controlled finite voxel environment before moving to DUN.

It is not a full editor UI.  
It is a technical sandbox for testing:

- voxel storage;
- selection workflows;
- edit tools;
- volume operations;
- copy/paste payload;
- save/load snapshot;
- dirty/rebuild flow.

---

## What it includes

- finite world profile;
- Edit / Runtime mode;
- chunk-backed voxel world;
- Paint;
- Erase;
- SelectBox;
- Fill selection;
- Delete selection;
- Clipboard copy/paste;
- save/load snapshot;
- lab HUD summary.

---

## Source files

| File                   | Role                                     |
| ---------------------- | ---------------------------------------- |
| `src/lab/sandbox.rs`   | finite world profile, Edit/Runtime mode  |
| `src/lab/world.rs`     | storage truth: `DensityKey -> VoxelGrid` |
| `src/lab/selection.rs` | select-box state and bounds              |
| `src/lab/clipboard.rs` | temporary copy/paste volume payload      |
| `src/lab/save.rs`      | lab snapshot save/load                   |
| `src/lab/scene.rs`     | Bevy resource/system wiring              |
| `src/lab/mod.rs`       | lab module map and responsibility notes  |

---

## Status

Working pre-DUN lab layer.

Some behavior is still lab-level and may later be moved into a cleaner engine API.

---

# 3. Volume Operation Backend

## Why it was added

The old `voxel-by-voxel` path does not scale well for large selection operations.

The volume backend is needed so operations are not executed as direct iteration over every voxel from the tool layer, but through a plan:

```text
intent -> operation plan -> chunk pass -> octochunk refine -> voxel frontier -> dirty queue
```

---

## What it includes

- `VolumeOpKind`;
- `VolumeIntent`;
- `OperationPlan`;
- `ChunkPlan`;
- `OctoPlan`;
- full chunk path;
- partial chunk path;
- octochunk refine;
- voxel frontier;
- dirty queue.

---

## Source files

| File                   | Role                                   |
| ---------------------- | -------------------------------------- |
| `src/lab/volume.rs`    | planner/executor for volume operations |
| `src/lab/world.rs`     | storage helpers used by executor       |
| `src/lab/selection.rs` | source bounds for volume intent        |

---

## Status

Architecturally correct direction, but currently still an MVP/lab backend.

Later, the dirty/rebuild flow may be rebuilt around more explicit render/collider/runtime queues.

---

# 4. Probe / HUD / Gizmos / Debug Tools

## Why it was added

This layer is needed so spatial truth is visible and verifiable in the scene.

Without it, the topology/mapping layer exists only as code, but not as a verifiable runtime picture.

---

## What it includes

- camera probe;
- inspect under crosshair;
- pinned target;
- machine/human notation;
- debug lenses;
- layered gizmos;
- current tool state;
- debug key bindings;
- fly camera controls.

---

## Source files

| File                             | Role                                                |
| -------------------------------- | --------------------------------------------------- |
| `src/lab/probe.rs`               | camera/inspect/pinned target resolving              |
| `src/lab/formatters.rs`          | machine/human formatting                            |
| `src/lab/hud.rs`                 | debug overlay                                       |
| `src/lab/gizmos.rs`              | Region/Sector/Chunk/Octo/Voxel/Selection wireframes |
| `src/tools/debug/config.rs`      | key bindings                                        |
| `src/tools/debug/input.rs`       | debug input handling                                |
| `src/tools/debug/state.rs`       | debug UI state                                      |
| `src/tools/camera_controller.rs` | fly camera                                          |

---

## Status

Working debug/tools layer.

DUN lens is still a stub and will be connected later.

---

# 5. Render Layer

## Why it was added

Render layer is needed to display current lab data.

Render is not the source of truth.  
Truth is stored in the lab/core layers, while render only builds a visual representation.

At the current stage, render serves two lab sources:

- `LabVoxelWorld` as world chunk storage;
- `LabVoxelObject` as pre-DUN object preview.

---

## Source files

| File                              | Role                                           |
| --------------------------------- | ---------------------------------------------- |
| `src/render/mesh_builder.rs`      | `VoxelGrid -> Bevy Mesh`                       |
| `src/render/lab_chunk_render.rs`  | chunk render rebuild from lab world            |
| `src/render/lab_object_render.rs` | object render preview from `LabVoxelObject`    |

---

## Status

Temporary lab render path.

Later, a separate rebuild policy will be needed for:

- mesh;
- collider;
- runtime objects;
- DUN objects.

---

# 6. Pre-DUN UnitNode Bridge

## Why it was added

Pre-DUN UnitNode Bridge is needed as the first practical bridge between the world voxel grid and the future DUN model.

This layer is not a full DUN yet.

It is needed to test the key idea:

```text
world voxels
-> extracted local voxel object
-> move / rotate as one unit
-> bake back into world
```

That means selected voxel mass stops being only part of `LabVoxelWorld` and can temporarily live as a separate object payload.

---

## What it includes

- `LabVoxelObject`;
- `VoxelPayload`;
- `LabObjectRegistry`;
- Extract Copy;
- Extract Cut;
- selected object switching;
- selected object delete;
- object bounds / pivot gizmo;
- object render preview;
- object move as one unit;
- C4 object orientation state;
- C4 yaw preview;
- rotated bake back to `LabVoxelWorld`;
- save/load detached objects inside lab snapshot.

---

## Source files

| File                              | Role                                                |
| --------------------------------- | --------------------------------------------------- |
| `src/lab/object.rs`               | extracted voxel objects, registry, move/rotate/bake |
| `src/lab/save.rs`                 | lab snapshot with world chunks + objects            |
| `src/lab/gizmos.rs`               | object bounds and pivot gizmos                      |
| `src/lab/hud.rs`                  | object summary and controls                         |
| `src/render/lab_object_render.rs` | object render preview                               |

---

## Current workflow

```text
SelectBox
-> Shift+X Extract Cut
-> LabVoxelObject
-> move object as unit
-> rotate C4 orientation
-> save/load object
-> bake rotated content back to world
```

`X` creates an object copy and keeps the original world voxels.

`Shift+X` creates an object and removes the source voxels from `LabVoxelWorld`.

`B` bakes the selected object payload back into `LabVoxelWorld`.

---

## Status

Working pre-DUN bridge layer.

This is not the final DUN contract, but an intermediate area for testing object/payload workflows.

---

## Important limitations

- object layer is still lab-level;
- object selection is temporary;
- object render preview is temporary;
- C4 rotation is supported only as yaw orientation;
- arbitrary quaternion Dynamic DUN rotation is not implemented yet;
- object physics/collider are not connected yet;
- save/load remains a lab snapshot, not a production format;
- UI/HUD needs a window rebuild.

---

# 7. DUN Stage / Future Prep

## Why it remains

DUN is now introduced not as the first MVP foundation, but as the next runtime/container stage above already validated layers:

```text
Spatial Truth
-> Pre-DUN Lab Sandbox
-> Pre-DUN UnitNode Bridge
-> DUN Stage
```

---

## Current state

- UnitNode workflow already tests extract / move / rotate / bake.
- DUN lens remains a future hook for now.
- Old DUN-first documentation may remain as historical reference for now.
- The real DUN stage should be documented after the current pre-DUN bridge.

---

## Later

- DUN lens contract;
- Static DUN;
- Dynamic DUN;
- route anchor;
- runtime transform bridge;
- generated mesh/collider pipeline;
- DUN physics integration;
- DUN documentation refresh.

---

# 🧰 Technical Debt / Cleanup Notes

## Temporary or raw parts

- DUN lens is still a stub.
- Save/load is a lab snapshot, not a production save format.
- Dirty/rebuild queue is a lab-level mechanism.
- Render rebuild is still connected to lab world.
- Some old documentation still describes the DUN-first order.
- Some of the `src` structure may still reflect old MVP0 logic.
- MVP documentation is not fully synchronized with the current code state yet.
- Pre-DUN UnitNode object layer is still lab-level and requires further DUN-contract cleanup.
- Object render preview is still a debug/runtime preview, not the final DUN render pipeline.
- C4 object rotation works for yaw preview and rotated bake, but it is not Dynamic DUN quaternion rotation.

---

## What to fix later

- Update `MVP_Structure.md` as a short index.
- Mark `MVP_0.1.md` as historical / archived.
- Update `meta/EN/project_status.md`.
- Update `meta/status_index.md`.
- Run portal/link cleanup.
- Later decide whether separate documents are needed for MVP 1 and MVP 2.
- Synchronize Concept after MVP documents.
- Later update EN versions.

---

# 🚫 Not in Scope

The current rebuild does not include:

- full editor UI;
- production save/load;
- collider rebuild pipeline;
- Dynamic DUN physics;
- HAOS scheduling;
- EQ runtime orchestration;
- streaming / LOD;
- final EN synchronization;
- public stable build.

---

# ➡ Next Documentation Step

Next order:

1. Reduce `MVP_Structure.md` to the role of an index.
2. Add an entry to `meta/log_2026.md`.
3. Later update `meta/EN/project_status.md`.
4. Then move to Concept refresh.
5. Then do portal/link cleanup separately.

---

# 📘 Related

[← Back to MVP Structure](./MVP_Structure.md)

[← Back to Roadmap](../roadmap.md)

[← Back to Meta](../../../meta/EN/README.md)

---

