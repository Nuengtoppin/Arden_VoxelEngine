**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-11-16

# Architecture: Topology / Routing / Rotation

This document block describes the **core address model of the world** for the engine:

- how the world is divided into containers (topology),
- how we describe a path to a point or area (routing),
- how we rotate patterns and structures (rotation),
- and how all of this looks in practical examples.

Files in this folder:

- [`Topology.md`](./Topology.md)  
  The base **world topology**:  
  levels, coordinate canon, strides, index ranges, and Morton order.  
  This is the foundation on which addressing and all following documents are built.

- [`Routing.md`](./Routing.md)  
  **Routing** as the active address model above topology.  
  Address forms, full and shortened forms, invariants, string format,
  operations such as comparison, offset, containment, up/down, mapping API,  
  and how this layer can be used by DTO, HAOS, LOD/SVO, generation, and ECS.

- [`Rotation.md`](./Rotation.md)  
  **Rotation around the Y axis** (yaw, 0°/90°/180°/270°) for containers and patterns.  
  How to rotate local shapes inside Chunk / Octochunk / Blueprint,  
  how this works with Routing and topology, why the base layer intentionally  
  does not touch the Y axis, and why pitch/roll are left for later local systems.

- [`Examples.md`](./Examples.md)  
  **Human-readable examples** for the three documents above.

---

## How to Read

Recommended order:

1. `Topology.md` — understand **which building blocks the world is made of** and how coordinates work.
2. `Routing.md` — see **how these blocks become address forms / routes** understood by engine systems.
3. `Rotation.md` — see **how structures and patterns can be reused** by rotating them without breaking topology.
4. `Examples.md` — go through practical examples and connect everything intuitively.

---

## Why This Exists

The goal of this block is to provide a **shared language** for all engine subsystems:

- topology stores structure,
- routing defines addresses,
- rotation allows content and patterns to be reused,
- examples help make the whole model easier to understand in practice.

These documents are intentionally strict and demanding,  
but each system — generation, LOD/SVO, DTO/HAOS, ECS, render, tools —  
can optionally take only the depth it actually needs.

Feedback, comments, and notes on these files are welcome.  
This architectural triad, together with the examples, is intended as an open foundation  
that can be developed and refined as the project grows.

---

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  
