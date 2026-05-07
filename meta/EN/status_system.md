# 🧾 Status System

---
**Document Status:** 🧱 Stable  
**Version:** 1.0.0  
**Maintainer:** Nuengtoppin  
**Last update:** 2025-12-01

---

## 🎯 Purpose

Defines the universal status system for all modules and documents of the **Arden Engine** project.  
The goal is to provide a unified format for tracking progress and making the state of code, architecture, and documentation transparent.

---

## 🧩 Maturity Levels

🧩 **Draft** → 🔬 **Review** → 🧪 **Stable-Test** → 🧰 **Validated** → 🧱 **Stable** → 🕯 **Archived**

---

## 📘 Usage Rules

| Stage | Meaning | When to use |
|------|---------|-------------|
| **🧩 Draft** | Idea, draft, incomplete text, or prototype. | Any file that is only being created. |
| **🔬 Review** | Content has been reviewed, but still requires comments. | When a document or code is ready for review. |
| **🧪 Stable-Test** | Material is stable and being tested in a real build. | When code/architecture works, but has not yet been validated. |
| **🧰 Validated** | Verified in MVP, behavior confirmed. | After stress tests and integrations. |
| **🧱 Stable** | Approved as the main version. | For fundamental components and final docs. |
| **🕯 Archived** | Outdated or replaced file. | Old revisions, previous variants. |

---

## 🧱 Usage by Category

| Category | Typical Statuses | Example Files |
|----------|------------------|---------------|
| **Architecture (`/docs/ARCHITECTURE`)** | Draft → Review → Stable | `TopologyLogic_Route_Rotation/Rotation.md` |
| **Source Code (`/src`)** | Draft → Stable-Test → Validated → Stable | `src/dun/spawn.rs`, `src/physics/mod.rs` |
| **Documentation (`/docs/RU`, `/docs/EN`)** | Draft → Review → Stable | `MVP_0.1.md`, `Glossary.md` |
| **Meta and Management (`/meta`)** | Draft → Review → Stable | `status_system.md`, `roles_and_structure.md` |
| **R&D Experiments (EQ-Sim, HAOS)** | Draft → Stable-Test | Research branches and prototypes |
| **Archive / History** | Archived | Old revisions, previous Roadmap versions |

---

## ⚙ Example Document Meta Header

```markdown
**Document Status:** 🧪 Stable-Test  
**Version:** 0.2.1  
**Maintainer:** Nuengtoppin  
**Reviewed by:** — (name)
**Last update:** 2025-12-02
```

---

## 📊 Summary and Automation

- Summary status data is collected manually or automatically in  
  `/meta/status_system.md`.

- If needed, a `/meta/status_index.md` file can be added, containing a table of all documents and their current statuses.

- In the future, a lightweight script in Rust or Python is planned. It will scan the repository, search for the `Document Status` block, and build a real-time project status table.

---

## 📋 Roles in the Meta Header

Each document or code file contains a **meta header** at the top, recording basic information about its state and responsibility.

### 🧩 Main Fields

| Field | Purpose | Example |
|------|---------|---------|
| **Document Status** | Reflects the current document state according to the status system. | `🧱 Stable` |
| **Version** | Version of the document or module. Used for change tracking. | `0.2.1` |
| **Maintainer** | The person responsible for content, relevance, and updates. | `Nuengtoppin` |
| **Reviewed by** | The person, team, or tool that reviewed or verified the document. | `Vitaly S.` / `Nuengtoppin (with GPT-5 assist)` |
| **Last update** | Date of the latest file change. | `2025-12-02` |

---

### 🧱 Filling Recommendations

| Situation | How to write it |
|----------|-----------------|
| Draft document | `Reviewed by: —` |
| Manually reviewed by the author | `Reviewed by: Nuengtoppin` |
| Reviewed with the help of an AI tool | `Reviewed by: Nuengtoppin (with GPT-5 assist)` |
| Reviewed by another participant | `Reviewed by: Vitaly S.` |
| Reviewed by an architecture group or team | `Reviewed by: Architecture Board` |
| Automatically checked | `Reviewed by: status_scan.rs (auto-check)` |

---

### ⚙ Notes

- The **Maintainer** field defines who owns responsibility for the document.
- The **Reviewed by** field does not transfer authorship rights and is used **only for transparency and revision tracking**.
- If a document goes through multi-level review, multiple names may be written separated by commas.
- If the review was performed with the help of tools, this should be stated explicitly, for example: *“with GPT-5 assist”*, *“auto-check”*.

---

## ⚙ Integration with File Statuses

Each `.md` or `.rs` file should include a meta header with a status.

Example for Rust code, as a comment at the beginning of the file:

```markdown
// Status: 🧪 Stable-Test
// Version: 0.2.1
// Last update: 2025-12-02
```

---

- Documentation files always use a markdown header, as shown in the example above.
- Files without a status are considered Draft by default.
- In the future, the `status_scan.rs` utility can automatically collect the status list of all files and output:

| Path | Status | Version | Date |
|------|--------|---------|------|
| src/dun/spawn.rs | 🧪 Stable-Test | 0.2.1 | 2025-12-02 |
| docs/RU/ARCHITECTURE/Topology.md | 🧱 Stable | 0.2.0 | 2025-11-25 |

---

## 🧭 Note

Statuses form a shared communication language between architects, developers, and documenters.  
The `status_system.md` file is considered the normative reference — any new symbols or changes to maturity levels must be recorded here.

---

