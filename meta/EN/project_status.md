# 📊 Arden Engine — Project Status

---
**Document Status:** 🔬 Review  
**Version:** 0.2.1  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07

---

## 🎯 Purpose

This document reflects the current development state of **Arden Engine**, including progress across key subsystems, MVP stages, and the general development direction.

It is used for manual progress tracking and synchronization with the Roadmap, MVP documentation, and Meta system.

---

## 🔄 Current Focus Override — 2026-05-07

The current active project focus has shifted away from the old DUN-first MVP0 toward a new development sequence:

```text
Spatial Truth -> Pre-DUN Lab Sandbox -> DUN Stage -> EQ/HAOS
```

The current rebuild records the first three working layers:

```text
Step 1 — Spatial Truth Foundation
Step 2 — Pre-DUN Finite Lab Sandbox
Step 3 / DUN Stage
```

### Current Rebuild Documents

* `docs/RU/ROADMAP/roadmap_2025.md`
* `docs/RU/MVP/MVP_Structure.md`
* `docs/RU/MVP/MVP_Current_Rebuild.md`
* `meta/log_2026.md`

### Note

The tables below are temporarily considered **historical / pending refresh**.

They reflect the old December project model and will be updated in a separate pass after the MVP documentation, DUN Stage, and Meta/status system are stabilized.

---

## 🔄 Current Snapshot — May 2026

| Subsystem / Layer              | State          | Progress | Comment                                                                                                                                                                            |
| ------------------------------ | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Spatial Truth Foundation**   | 🧪 Stable-Test | 85%      | `Topology / Routing / Mapping`, `RuntimePosition`, `DensityKey`, `SimSectorKey`, `FullRoute`, probe/HUD/gizmos are working. Further tests and doc/code synchronization are needed. |
| **Pre-DUN Finite Lab Sandbox** | 🧪 Stable-Test | 80%      | Finite lab world, Edit/Runtime mode, Paint/Erase, SelectBox, Fill/Delete, chunk-aware volume backend, Copy/Paste, and Save/Load snapshot are working.                              |
| **Pre-DUN UnitNode Bridge**    | 🧪 Stable-Test | 60%      | The `World -> Object -> Move / Rotate -> Bake back` cycle works: `LabVoxelObject`, object registry, object render preview, C4 orientation, rotated bake, and save/load objects.    |
| **DUN Stage**                  | 🧩 Draft       | 25%      | DUN canon exists in documentation, but the current runtime code is not yet shaped as a full DUN layer. This is the next major architecture stage.                                  |
| **Render Layer**               | 🧩 Draft       | 35%      | A temporary lab render path exists for world chunks and object preview. Render is not the source of truth and later requires a rebuild policy.                                     |
| **Physics / Collider Layer**   | 🧩 Draft       | 10%      | Rapier is connected, but the DUN/object collider pipeline is not yet integrated into the current workflow.                                                                         |
| **EQ-Core / HAOS / DTO**       | 🧩 Draft       | 10%      | Currently a conceptual next layer after the DUN Stage. Runtime orchestration, sleep/awake, rebuild scheduling, and optimization policy are still ahead.                            |
| **Docs RU**                    | 🔬 Review      | 65%      | RU documentation is being rebuilt around the new MVP order. Step 1/2/3 are described; DUN Stage and Concept refresh are still ahead.                                               |
| **Docs EN**                    | 🧩 Draft       | 25%      | EN synchronization is postponed until the RU canon and current MVP rebuild stabilize.                                                                                              |
| **Meta System**                | 🧱 Stable      | 100%     | The status system and meta structure remain the normative foundation of the project.                                                                                               |

---

## 🧩 Current MVP Plan — May 2026

| Stage     | Goal                              | State          | Comment                                                                                             |
| --------- | --------------------------------- | -------------- | --------------------------------------------------------------------------------------------------- |
| **MVP 1** | Spatial Truth Foundation          | 🧪 Stable-Test | Spatial truth, address/mapping layer, probe, HUD, layered gizmos.                                   |
| **MVP 2** | Pre-DUN Finite Lab Sandbox        | 🧪 Stable-Test | Controlled voxel sandbox: edit tools, selection, volume backend, clipboard, save/load.              |
| **MVP 3** | Pre-DUN UnitNode Bridge           | 🧪 Stable-Test | Extract Copy/Cut, object registry, object move, C4 rotation, save/load objects, rotated bake back.  |
| **MVP 4** | DUN Stage                         | 🧩 Draft       | Full DUN contract in runtime: Static/Dynamic DUN, anchor, transform bridge, mesh/collider pipeline. |
| **MVP 5** | EQ / HAOS / Runtime Orchestration | 🧩 Draft       | EQ-Core/EQ-Sim boundary, DTO, HAOS, rebuild queues, runtime scheduling.                             |
| **MVP 6** | Stable Research Build             | 🧩 Draft       | Cleanup, demo scene, RU/EN docs sync, public-facing R&D build, contribution readiness.              |

---

## 🕯 Historical Note — December 2025

The December MVP0 model is preserved as a historical reference.

The old order was closer to:

```text
DUN prototype -> physics check -> stress test
```

The current active order has been replaced with:

```text
Spatial Truth
-> Pre-DUN Lab Sandbox
-> Pre-DUN UnitNode Bridge
-> DUN Stage
-> EQ/HAOS
```

The old December 2025 tables are no longer used as the active development plan.

---

## 📚 Related Documents

| File                                                                                   | Purpose                                 |
| -------------------------------------------------------------------------------------- | --------------------------------------- |
| [`focus_2025.md`](focus_2025.md)                                                       | Priorities and calendar focus           |
| [`../log_2025.md`](../log_2025.md)                                                     | Timeline of events and updates for 2025 |
| [`../log_2026.md`](../log_2026.md)                                                     | Timeline of events and updates for 2026 |
| [`../status_index.md`](../status_index.md)                                             | Registry of file statuses               |
| [`status_system.md`](status_system.md)                                                 | Status maturity reference               |
| [`../../docs/EN/roadmap.md`](../../docs/EN/roadmap.md)                                 | External portal and Roadmap             |
| [`../../docs/EN/MVP/MVP_Current_Rebuild.md`](../../docs/EN/MVP/MVP_Current_Rebuild.md) | Current technical MVP rebuild           |

---

## 🧭 Note

* This file is updated manually every 1–2 weeks or after major architectural shifts.
* The current `Current Focus Override` was added as a temporary update layer.
* Old tables are preserved as a historical snapshot until a full refresh.
* A full rebuild of this file will be done after Step 3 / DUN Stage is described and the MVP documentation is stabilized.
* All statuses should later be checked against [`../status_index.md`](../status_index.md).

---