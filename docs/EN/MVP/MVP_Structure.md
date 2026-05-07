
**Document Status:** 🧩 Draft  
**Version:** 0.3.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07  

---

# 🧩 Arden Engine — MVP Structure

## 🎯 Purpose

This document describes the general MVP structure of **Arden Engine**.

In the context of Arden Engine, an MVP is not a final release and not a strict calendar build.  
It is a technical layer for testing architecture, code, and research hypotheses.

This document is used to define:

- the general order of MVP layers;
- the connection between Roadmap, codebase, and documentation;
- the status of the current rebuild;
- historical MVP prototypes;
- future development directions.

The detailed description of the current `src` state is placed in:

[`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

---

## 🧭 Current MVP Logic

After revising the development order, the active MVP path is no longer based on a DUN-first prototype.

New order:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
```

This means:

- first, spatial truth is established;
- then a controlled pre-DUN voxel sandbox is built;
- then DUN is introduced as the next runtime/container layer;
- after that, the project can move toward EQ / HAOS / runtime orchestration.

---

## 🕯 MVP 0 — Historical DUN Prototype

**Status:** historical reference / to be archived later  
**Document:** [`MVP_0.1.md`](./MVP_0.1.md)

### Purpose

An early DUN-first prototype for testing the idea:

```text
VoxelGrid -> Surface Mesh -> Collider / Physics check
```

### Included

- Single DUN Prototype;
- Dynamic DUN stress test;
- surface mesh collider;
- sphere / physics check;
- first validation of the voxel grid → mesh → collider connection.

### Comment

This MVP is preserved as a historical project layer.

It is no longer the active development foundation, because the current architecture first requires:

- spatial truth;
- address / mapping layer;
- finite lab sandbox;
- volume operation backend.

DUN returns later as a separate stage above this foundation.

---

## 🧩 MVP Current Rebuild

**Status:** 🧩 Draft / Active  
**Document:** [`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

### Purpose

To record the current general rebuild of the codebase and documentation after moving away from DUN-first MVP0 toward the new sequence:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> Pre-DUN UnitNode Bridge -> DUN Stage -> EQ/HAOS
```

### Includes

The current rebuild describes the first three working layers:

```text
Step 1 — Spatial Truth Foundation
Step 2 — Pre-DUN Finite Lab Sandbox
Step 3 — Pre-DUN UnitNode Bridge
```

It also leaves room for the next full DUN pass:

```text
Step 4 — DUN Stage
```

### Comment

`MVP_Current_Rebuild.md` is not a final specification.  
It is a technical map of the current `src` state, recording:

- what has already been built;
- why it was added;
- which parts are foundational;
- which parts are temporary or raw;
- what will be refined later.

---

## 🧩 MVP 1 — Spatial Truth Foundation

**Status:** Draft / active base  
**Detailed document:** later, if needed

### Purpose

To establish the spatial truth of the engine.

### Includes

- topology layer;
- `Region`;
- `Chunk`;
- `Octochunk`;
- `Voxel`;
- `RuntimePosition`;
- `DensityKey`;
- `SimSectorKey`;
- `FullRoute`;
- world/runtime/address mapping;
- debug/probe/HUD/gizmos visibility.

### Result

The engine receives a verifiable spatial truth layer that can be read through runtime tools, HUD, and gizmos.

---

## 🧩 MVP 2 — Pre-DUN Finite Lab Sandbox

**Status:** Draft / active  
**Detailed document:** later, if needed

### Purpose

To create a finite voxel sandbox before moving to DUN.

### Includes

- finite lab world profile;
- Edit / Runtime mode;
- chunk-backed voxel storage;
- Paint / Erase;
- SelectBox;
- Fill / Delete volume;
- Clipboard copy/paste;
- lab save/load snapshot;
- volume operation backend;
- temporary lab render path.

### Result

The project receives a controlled sandbox environment for testing voxel editing, selection workflows, payload operations, and future DUN preparation.

---

## 🧩 MVP 3 — Pre-DUN UnitNode / DUN Bridge

**Status:** Draft / active local checkpoint  
**Detailed document:** [`MVP_Current_Rebuild.md`](./MVP_Current_Rebuild.md)

### Purpose

To test the transition from a world voxel grid to a separate object/unit-node mass before full DUN.

This layer is not the final DUN.  
It is needed as a preparatory bridge:

```text
World voxels -> Local payload object -> Unit movement -> C4 orientation -> Bake back
```

### Includes

- `LabVoxelObject`;
- local `VoxelPayload`;
- `LabObjectRegistry`;
- Extract Copy / Cut;
- object render preview;
- object bounds / pivot gizmo;
- selected object switching;
- selected object delete;
- object movement as one unit;
- object save/load inside lab snapshot;
- C4 orientation state;
- C4 yaw preview;
- rotated bake back to world.

### Result

The project receives its first working pre-DUN workflow:

```text
World -> Object -> Move / Rotate -> World
```

This prepares the future DUN Stage, but does not replace it.

---

## 🧩 MVP 4 — DUN Stage

**Status:** planned / next architecture stage  
**Detailed document:** later

### Purpose

To introduce DUN as a runtime/container layer above the already working spatial truth, lab sandbox, and pre-DUN UnitNode bridge.

### Planned

- DUN lens contract;
- Static DUN;
- Dynamic DUN;
- route anchor;
- runtime transform bridge;
- generated mesh/collider pipeline;
- DUN documentation refresh;
- eventual quaternion transform for Dynamic DUN.

### Result

DUN becomes not the first foundation of MVP, but the next runtime/container layer above the validated spatial architecture and object workflow.

---

## 🧩 MVP 5 — EQ / HAOS / Runtime Orchestration

**Status:** planned

### Purpose

To prepare runtime orchestration, optimization, and simulation rules.

### Planned

- EQ-Core / EQ-Sim boundary;
- DTO activity/sleep logic;
- HAOS optimization layer;
- rebuild scheduling;
- collider/mesh rebuild policy;
- event/intent pipeline;
- future streaming and LOD hooks.

---

## 🧩 MVP 6 — Stable Research Build

**Status:** planned

### Purpose

To prepare Arden Engine for open R&D experiments, demonstration, and further open-collab development.

### Planned

- stabilization of core/lab/runtime layers;
- publicly readable documentation;
- RU/EN sync;
- cleaned examples;
- demo scene;
- contribution/license readiness.

---

## 🧰 Current TODO

- Clean up `MVP_Current_Rebuild.md` according to the current `src` state.
- Step 3 / Pre-DUN UnitNode Bridge is described in the current rebuild.
- Later, document the full DUN Stage as a separate pass.
- Later, mark `MVP_0.1.md` as historical / archived.
- Synchronize `meta/log_2026.md`.
- Later, update `meta/EN/project_status.md`.
- Later, update `meta/status_index.md`.
- Run portal/link cleanup as a separate pass.
- Later, update Concept and EN documents.

---

## 📘 Related Materials

[📄 MVP Current Rebuild](./MVP_Current_Rebuild.md)

[📚 Back to Roadmap](../roadmap.md)

[🧾 Project Management Panel / Meta](../../../meta/EN/README.md)

[⚙ Glossary](../TERMS/Glossary.md)

---

📚 [Back to main README →](../../../root/README_EN.md)

---
