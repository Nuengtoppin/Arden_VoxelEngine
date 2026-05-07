**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-12-04

---

# 🧩 Arden Engine — Dynamic Unit Node (DUN)

## 🧭 Purpose

The **DUN (Dynamic Unit Node)** block describes dynamic world nodes in Arden —  
containers that can move, rotate, and participate in simulation  
without breaking the strict topology of the core (*Topology / Route / Rotation*).

DUN is a **world inside the world**: local topology, mesh, and state  
combined into a single container controlled through a transform.

---

## 🧩 Block Structure

| File | Purpose | Status |
|------|---------|--------|
| [`DUN.md`](./DUN.md) | Main document defining DUN, its invariants, and the Anchor Model | 🔬 Review |
| [`Container.md`](./Container.md) | Planned: containerization and data storage inside DUN | 🧩 Draft |
| [`Lifecycle.md`](./Lifecycle.md) | Planned: DUN lifecycle: creation, update, archiving | 🧩 Draft |
| [`Interaction.md`](./Interaction.md) | Planned: DUN interaction with simulation and physics | 🧩 Draft |
| [`Examples.md`](./Examples.md) | Planned: DUN examples and usage scenarios | 🧩 Draft |

---

## 🔗 Place in the Architecture

| Level | Purpose | Status |
|-------|---------|--------|
| **1.x** | Topology / Routing / Rotation — world foundation | 🧱 Stable |
| **2.x** | Dynamic Unit Node — dynamic nodes, this block | 🔬 Review |
| **3.x** | EQ-Core / EQ-Sim / HAOS — simulation and optimization | 🧩 Draft |

DUN is a **bridge** between static topology and dynamic simulation.

---

## 📘 Key Principles

- DUN does not change Topology, Route, or Rotation.
- Voxels always remain axis-aligned.
- The transform rotates, not the grid.
- Physics and walking operate on the surface mesh.
- Route remains the logical anchor.
- DUN exists in two modes: **Static** and **Dynamic**.

---

## 🧱 Context and Integration

- Uses EQ-Core for storage and consistency.  
- Managed by EQ-Sim during simulation ticks.  
- Optimized through HAOS and DTO.  
- Supports Blueprint / Instance for runtime variants.

---

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)

---
