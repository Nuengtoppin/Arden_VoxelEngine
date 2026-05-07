
# 🧾 Status Index

---
**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-12-03
---

## 🎯 Purpose

Creates a centralized status registry for all key files of the **Arden Engine** project.  
Used for manual and/or automatic tracking of the state of architecture, code, and documentation.

---

## 📊 Status Table

| Path | Status | Version | Updated | Comment |
|------|--------|---------|---------|---------|
| **/docs/RU/ARCHITECTURE/1_TopologyLogic_Route_Rotation/README.md** | 🔬 Review | 0.1.0 | 2025-11-25 | Foundation of architecture 0.1 |
| **/docs/RU/ARCHITECTURE/2_Dynamic_unit_node/DUN.md** | 🔬 Review | 0.1.0 | 2025-11-30 | DUN description development |
| **/docs/RU/MVP/MVP_0.1.md** | 🔬 Review | 0.1.0 | 2025-11-30 | MVP 0.1 concept |
| **/src/dun/spawn.rs** | 🧪 Stable-Test | 0.2.1 | 2025-12-02 | DUN spawn testing |
| **/src/physics/mod.rs** | 🧩 Draft | 0.2.0 | 2025-11-28 | Physics module foundation |
| **/meta/status_system.md** | 🧱 Stable | 1.0.0 | 2025-12-01 | Normative status reference |
| **/meta/project_status.md** | 🔬 Review | 0.2.0 | 2025-11-30 | State of MVP, code, and docs |
| **/meta/log_2025.md** | 🧩 Draft | 0.1.1 | 2025-11-30 | 2025 change log |
| **/meta/log_2026.md** | 🧩 Draft | 0.1.0 | 2026-01-01 | Preparation for the next stage |
| **/meta/focus_2025.md** | 🧩 Draft | 0.1.0 | 2025-11-30 | Annual priorities and goals |
| **/meta/roles_and_structure.md** | 🔬 Review | 0.1.0 | 2025-11-30 | Roles and organizational model |
| **/meta/origin_and_rights.md** | 🧱 Stable | 1.0.0 | 2025-11-30 | Licenses, authorship, and project origin |

---

## ⚙ Entry Format

| Field | Meaning |
|------|---------|
| **Path** | Absolute or relative path to the file. |
| **Status** | One of the statuses from [Status System](status_system.md). |
| **Version** | Current file version. |
| **Updated** | Date of the latest edit. |
| **Comment** | Short description of the file purpose or changes. |

---

## 🧩 Updating

- Updated manually when files change or after commits.
- When `status_scan.rs` is run, this file will be rewritten automatically.
- Files without a status are considered **🧩 Draft** by default.

---

## ⚙ Auto-Scan Structure

1. Reads all `.md` and `.rs` files.
2. Extracts lines with the following keys:
   - `Document Status:`
   - `Version:`
   - `Last update:`
3. Updates the table above according to the folder hierarchy.
4. Saves a backup in `/meta/_status_index_backup/`.

---

## 🧭 Notes

- `status_index.md` is a **working tool** and is updated more often than `status_system.md`.
- It is used to track the current development state and analyze the pace of progress.
- Before a release, it is recommended to compare it with `project_status.md` and `log_20XX.md`.
- See `Status System` for status definitions.

---
