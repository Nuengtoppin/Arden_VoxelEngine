**Document Status:** 🔬 Review
**Version:** 0.4.0
**Maintainer:** Nuengtoppin
**Last update:** 2026-05-07

---

# 🧭 **Arden Engine — Roadmap (2025–2026)**

## 🎯 Purpose

This document describes the strategic development phases of **Arden Engine** —
from the research foundation to the first stable builds.

The goal is to capture the *architectural path*, not a strict release schedule.

Each phase builds on the previous one and is connected to the MVP system and `meta/project_status.md`.

---

## 🧩 Phase 0 — Research Foundation

**Goal:**  
test the basic ideas of topology, coordinates, routing, and the voxel/DUN model before active implementation.

**Includes:**

- first Region / Chunk / Voxel schemes;
- experiments with coordinates and nesting;
- initial formulation of DUN, EQ, HAOS, and Route;
- validation of the hybrid voxel + mesh approach.

**Result:**  
the project received its initial architectural language and a set of working hypotheses.

---

## 🧩 Phase 1 — Documentation and Repository Foundation

**Goal:**  
build the basic repository structure and capture the main architecture documents.

**Includes:**

- RU/EN documentation portal;
- Concept / Architecture / Terms;
- Topology / Routing / Rotation triad;
- DUN documentation draft;
- Meta/status system;
- licenses, README, and repository structure.

**Result:**  
the project received a documentation foundation and a base for further R&D development.

---

## 🧩 Phase 2 — Spatial Truth + Pre-DUN Lab Sandbox

**Goal:**  
build a stable spatial foundation for the engine and a finite voxel sandbox before moving to DUN.

**Includes:**

- active spatial core:
  - `Region`;
  - `Chunk`;
  - `Octochunk`;
  - `Voxel`;
- runtime bridge:
  - `RuntimePosition = Region + LocalFloat`;
- address forms:
  - `DensityKey`;
  - `SimSectorKey`;
  - `FullRoute`;
- mapping pipeline:
  - world position → runtime position;
  - runtime position → density key;
  - runtime position → full route;
- debug/probe layer:
  - camera probe;
  - inspect under crosshair;
  - pinned target;
  - machine/human notation;
- layered gizmos:
  - Region;
  - Sector;
  - Chunk;
  - Octochunk;
  - Voxel;
  - Selection;
- finite lab sandbox:
  - `LabWorldProfile`;
  - Edit / Runtime mode;
  - `LabVoxelWorld`;
  - Paint / Erase;
  - SelectBox;
  - Fill / Delete volume;
  - Clipboard copy/paste;
  - save/load snapshot;
- volume operation backend:
  - operation intent;
  - chunk-aware planning;
  - octochunk refine;
  - voxel frontier;
  - dirty/rebuild queue.

**Result:**  
the project receives a controllable pre-DUN sandbox where spatial truth, voxel editing, volume operations, payload workflows, and future DUN preparation can be tested.

---

## 🧩 Phase 3 — Pre-DUN UnitNode Bridge

**Goal:**  
test the object/payload workflow between the finite lab sandbox and the future DUN Stage.

This layer is not a full DUN runtime.  
It is needed as a practical bridge:

```text
World voxels -> UnitNode object -> move / rotate -> bake back to world
```

**Includes:**

* extract selected voxel mass into local payload object;
* `LabVoxelObject`;
* `VoxelPayload`;
* `LabObjectRegistry`;
* Extract Copy / Cut;
* selected object switching;
* selected object delete;
* object render preview;
* object bounds / pivot gizmo;
* object movement as one unit;
* lab snapshot save/load for detached objects;
* C4 orientation state;
* C4 yaw preview;
* rotated bake back into `LabVoxelWorld`.

**Result:**
the project receives its first working cycle:

```text
World voxels -> UnitNode object -> move / rotate -> bake back to world
```

This layer remains pre-DUN and is not the final DUN runtime.

---

## 🧩 Phase 4 — DUN Stage

**Goal:**
introduce DUN as a layer above the already working spatial truth, lab sandbox, and pre-DUN UnitNode bridge.

**Includes:**

* update DUN documentation for the current MVP canon;
* DUN lens contract;
* Static DUN preparation;
* Dynamic DUN preparation;
* route anchor + runtime transform bridge;
* generated mesh/collider pipeline;
* preparation for the transition to Dynamic DUN;
* future quaternion transform for Dynamic DUN.

**Result:**
DUN becomes the next runtime/container level above the validated topology/mapping/lab/object foundation.

---

## 🧩 Phase 5 — EQ / HAOS / Runtime Orchestration

**Goal:**
move from the lab sandbox and DUN stage toward a more explicit runtime orchestration system.

**Includes:**

* EQ-Core / EQ-Sim boundary;
* DTO activity/sleep policy;
* HAOS optimization layer;
* dirty/rebuild queues;
* runtime events and system coordination;
* preparation for streaming, LOD, and simulation scheduling.

**Result:**
the engine receives a foundation for controlled simulation, optimization, and future runtime architecture.

---

## 🧩 Phase 6 — Stable Research Build

**Goal:**
prepare Arden Engine for open R&D experiments and publication.

**Includes:**

* stabilization of core/lab/runtime layers;
* update RU/EN documentation;
* cleanup of examples and public-facing docs;
* preparation of demo scenes;
* license and contribution flow validation.

**Result:**
Arden Engine is ready for open R&D experiments, demonstration, and further open-collab development.

---

## 🔗 **Related Documents**

| Section           | File                                                                    |
| ----------------- | ----------------------------------------------------------------------- |
| 🧱 Project Status | [`meta/EN/project_status.md`](../../../meta/EN/project_status.md)       |
| 🧩 MVP Structure  | [`docs/EN/MVP/MVP_Structure.md`](../MVP/MVP_Structure.md)               |
| 📚 Architecture   | [`docs/EN/ARCHITECTURE/readme.md`](../ARCHITECTURE/readme.md)           |
| 📘 Concept        | [`docs/EN/CONCEPT/Concept_Overview.md`](../CONCEPT/Concept_Overview.md) |
| ⚙ Meta Panel      | [`meta/EN/README.md`](../../../meta/EN/README.md)                       |
| 🧾 Status Index   | [`meta/status_index.md`](../../../meta/status_index.md)                 |

---

[📚 Back to Roadmap](../roadmap.md)

---

📚 [Back to main README →](../../../root/README_EN.md)

---

