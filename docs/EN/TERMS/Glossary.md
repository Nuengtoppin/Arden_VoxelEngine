
**Document Status:** 🧩 Draft   
**Version:** 0.1.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-11-17  

---

[📚 Back to Roadmap](../roadmap.md)  

---

📚 [Back to main README →](../../../root/README_EN.md)

---

## MINI/README

This file contains the project glossary for **Arden Engine**.

The glossary is divided into:

- Custom Terms
- General Terms
- Mesh & Surface Extraction
- BVH Terms

As the project develops, this glossary will be expanded, changed, and edited.

---

## Custom Terms

Contains internal terms related to world architecture, modules, and subsystems.

---

### **Butler — core logger-validator**

**Description:** an integrity control service. It checks module states, collects logs, analyzes BUS overloads, reports errors, and suggests fixes. It may automatically resolve simple inconsistencies.

**Analogs:** closest to a watchdog / diagnostics system, but Butler is part of the architecture and acts as an “intelligent validator”.

---

### **Aspectrolog — semantic aspect library**

**Description:** a world semantics module at an early concept stage. It stores “aspects” — descriptions of properties, categories, and object behavior: *fire, water, gas, metal, biology, etc.*

It is used as an extensible property dictionary for HAOS, EQ-Sim, and systems such as:

- procedural behavior generation;
- optimization hints;
- custom world rules.

**Analogs:** no direct analog; partially similar to tags / traits in ECS or rule-based systems.

*(Note: the concept is raw and may be updated later.)*

---

### **DTO — Dormant Tick Observer**

**Description:** a concept of sleep and activity. A world element such as a container, octochunk, or subchunk is considered **ACTIVE** if it participates in ticks or changes, and **DORMANT** if it “sleeps” but can be awakened by HAOS or DUN.

**Analogs:** Unity sleeping bodies, ECS dirty flags.

---

### **HAOS — Hybrid Adaptive Optimization System**

**Description:** an optimization layer above EQ. It contains a modular library (`HAOS lib`) of algorithms that are enabled dynamically depending on world state. It works above EQ-Sim and optimizes data flow.

**Types of optimization:**

- **Spatial:** LOD, SVO, decimation
- **Temporal:** tick scheduling, batching
- **Culling:** distance / visibility
- **Memory:** archiving / awakening (DTO)

#### **HAOS-lib**

A collection of optimization algorithms available to the HAOS module.  
Includes Spatial / Temporal / Culling / Memory techniques.

---

## **DUN — Dynamic Unit Node**

**Description:**  
A basic movable architectural unit that combines voxel structure, mesh, physics, and runtime behavior.  
A DUN carries local data such as volume, bounds, and transform, and can be static or dynamic.  
It moves as a single whole without breaking the discrete topology of the world.

**Types:**

- **Static DUN** — anchored in space, part of static topology.
- **Dynamic DUN** — movable, has its own physics and activity state.

**Analogs:** dynamic chunk, rigid voxel body, but DUN is a formalized core node that connects topology and physics.

---

### **EQ — Equilibrium Kernel**

#### **EQ-Core — Equilibrium Core** = world data

**Description:** a pure model of world state. Stores hybrid data: voxels, vector structures, topology, containers, and system tags. Equivalent to “world memory” without executing logic.

**Analogs:** ECS World / Scene State, but with support for a voxel-vector hybrid.

#### **EQ-Sim — Equilibrium Simulator** = world behavior

**Description:** the executor of world rules. It reads EQ-Core, applies updates and logical processes, and forms a correct and consistent state before rendering and optimization.

**Analogs:** game executor, gameplay loop, but more deeply embedded into the core.

---

### **BUS — Coordinator Bus**

**Description:** the main data orchestrator between core modules. It does not execute heavy tasks — it only routes events and states.

**Analogs:** event bus / scheduler / orchestrator.

---

## **General Terms**

Contains terms used in graphics, rendering, and voxel engine development.

---

### **DAG — Directed Acyclic Graph**

**Description:** a dependency graph without cycles. Used for computation optimization, building LOD hierarchies, routes, and update chains.

**Analogs:** Blender node graph, task graphs in engines.

---

### **AABB — Axis-Aligned Bounding Box**

**Description:** a box aligned to the XYZ axes. Used for fast collision checks, broad-phase search, and spatial optimizations.

**Analogs:** classic PhysX / Havok / ECS structure.

---

### **OBB — Oriented Bounding Box**

**Description:** a box with arbitrary orientation. More accurate than AABB, but more expensive to calculate; used for complex dynamic shapes.

**Analogs:** SAT / Separating Axis Theorem.

---

### **LOD — Level of Detail**

**Description:** a system for replacing data such as meshes, voxels, or geometry with simplified levels based on distance, view angle, or load.

**Analogs:** Unity LODGroup, Unreal LODs.

---

### **Octree**

**Description:** a structure that divides space into 8 parts at each level. Optimal for voxels, spatial search, and LOD.

**Comment:** a foundation for the Region → Chunk → Octochunk → Subchunk hierarchy.

**Where it appears:** voxel engines, 3D navigation, physics, GI.

---

### **Loose Octree**

**Description:** a version of an octree where nodes are enlarged so dynamic objects do not constantly break boundaries.

**Comment:** suitable for movable containers such as DYN, reducing the number of rebuilds.

**Where it appears:** physics engines, dynamic object trees.

---

### **Cascaded Voxel Grid**

**Description:** a voxel grid stored at multiple scales for distant lighting, shadows, and GI.

**Comment:** a hybrid technique that can theoretically be combined with HAOS spatial optimization.

**Where it appears:** voxel cone tracing, VXGI, large GI systems.

---

### **Clipmap**

**Description:** a layered structure for storing space: dense layers nearby, sparse levels of detail farther away.

**Comment:** useful for huge worlds or multi-level LOD above a voxel scene.

**Where it appears:** VXGI, terrain engines, cascaded maps.

---

### **Clipmap LOD**

**Description:** a multi-level LOD system where each world zone is stored in several resolutions depending on distance from the player.

**Comment:** helps handle huge scenes and fits well above a voxel structure.

**Where it appears:** terrain engines, VXGI, large-world renderers.

---

### **Sparse Voxel Octree (SVO) — variants**

**Description:** sparse octrees that store only occupied nodes; variants may include compression, mip levels, and GPU acceleration.

**Comment:** suitable for GI, ray tracing, and large voxel worlds; can be used as an additional layer above Octochunk.

**Where it appears:** Atomontage, GigaVoxels, voxel GI.

---

### **Space-Filling Curves — Hilbert Curve**

**Description:** a curve that passes through all space so that neighboring points in 3D remain close in 1D order.

**Comment:** provides better locality than Morton, but is more expensive to compute; may be used in HAOS for spatial optimizations.

**Where it appears:** databases, GPU rendering, memory tiling.

---

### **SDF — Signed Distance Field**

**Description:**  
A distance field where each value stores the distance to the nearest surface, positive or negative. Used for smooth surfaces, collisions, and mesh generation.

**Where it appears:**  
Godot SDFGI, Unreal Distance Fields, VFX simulations.

**Comment:**  
May be used as an auxiliary format for the Mesh Layer or HAOS spatial optimizations.

---

### **Voxel GI — Voxel Global Illumination**

**Description:**  
A lighting method where light and reflections are stored in a voxel grid and updated across the scene.

**Where it appears:**  
Godot GIProbe (old), NVIDIA VXGI, sparse voxel GI techniques.

**Comment:**  
May be used as a separate module above Mesh Layer / SVO.

---

### **OpenVDB — sparse volume grid library**

**Description:** an open DreamWorks library for storing and processing large sparse voxel volumes such as smoke, SDFs, and density fields. Used in VFX and offline rendering.

**Analogs:** NanoVDB, the GPU version.

**Comment:** not suitable for real-time game engines; too heavy and not designed for dynamic small-block updates.

---

### **Z-Curve Hashing (Morton Hashing)**

**Description:** representation of 3D coordinates as a hash based on Morton code, preserving locality.

**Comment:** useful for fast dictionaries, spatial tables, and octochunk indexing.

**Where it appears:** LBVH, GPU voxel engines, SVO.

---

### **Morton Codes / Z-Order**

**Description:** a method for converting 3D coordinates into a single number by interleaving bits while preserving spatial proximity. Simplifies sorting and accelerates spatial structures.

**Comment:** used in Arden inside routing and topology for compact spatial addressing.

**Where it appears:** LBVH, SVO, GPU rendering, voxel engines.

---

### **KD-Tree (k-d tree)**

**Description:** a tree that divides space by cycling through the X → Y → Z axes, allowing fast nearest-point and nearest-object search.

**Comment:** useful for neighbor search, navigation, and selecting nearby meshes / containers.

**Where it appears:** AI, geometry, ray tracing, search accelerators.

---

### **ECS — Entity Component System**

**Description:**  
An architectural approach where the world is represented through entities, their data components, and systems that process them. It provides dense storage, separation of data and logic, and high performance for mass operations.

**Where it appears:**

- **Bevy** — a pure ECS engine and Arden’s current platform.
- **Unity DOTS** — an experimental ECS subsystem, not the standard runtime.
- **Frostbite (EA)** — uses an ECS-like data model.
- **Amethyst / Fyrox / Specs** — Rust engines or libraries with ECS cores.
- **EnTT, Flecs** — popular standalone ECS libraries.

**Comment:**  
In most classic engines such as Unreal, Godot, and standard Unity, ECS is not the main foundation and exists more as a concept or optional subsystem.

---

### **Mesh Layer — mesh generation and storage layer**

**Description:**  
An intermediate layer between voxel topology and GPU.

**Where it appears:**

- **Minecraft-like engines** — mesh extraction from chunks, such as Greedy Meshing, Marching Cubes, etc.
- **Godot / Unity / Unreal** — mesh builder APIs, but not usually as a separate layer.
- **GigaVoxels / Dual Contouring engines** — dense integration of voxels and meshes.

**Comment:**  
In classic engines, Mesh Layer is usually just a tool.

It may combine:

- vector layer;
- surface generation;
- GPU buffers;
- HAOS spatial / LOD optimizations.

It effectively acts as the Vox ↔ Render bridge.

---

#### **Triangulation**

**Description:**  
Conversion of a surface or voxels into a triangular mesh for rendering.

**Where it appears:**  
Marching Cubes, Dual Contouring, Surface Nets; used in voxel and mesh engines.

**Comment:**  
Used for generating GPU meshes from octochunks / subchunks.

---

#### **Marching Cubes**

**Description:**  
An algorithm for extracting a surface from a voxel field by sampling cubes and triangulating their boundaries.

**Where it appears:**  
Voxel engines, medical visualization, Blender.

**Comment:**  
One possible surface generation method in the Mesh Layer.

---

#### **Dual Contouring**

**Description:**  
An algorithm for producing a mesh from voxels while preserving sharp edges and topology.

**Where it appears:**  
Graphics and geometry engines, voxel GI.

**Comment:**  
Useful for smoother and more accurate surfaces.

---

#### **Surface Nets**

**Description:**  
A smoothed surface extraction method using a grid of points and simple connections between them.

**Where it appears:**  
Lightweight voxel renderers, scientific visualization.

**Comment:**  
A fast option for low-detail LOD.

---

#### **Greedy Meshing**

**Description:**  
An optimization that merges large voxel planes into larger polygons to reduce triangle count.

**Where it appears:**  
Minecraft-like engines, cubic voxel engines.

**Comment:**  
Used for dense block surfaces with cubic geometry.

---

### **GPU/WGPU Layer**

**Description:** the lower render layer that receives meshes, buffers, and textures.

**Where it appears:** WGPU, modern GPU engines.

---

### **BVH — Bounding Volume Hierarchy**

**Description:**  
A hierarchy of volumes, usually AABBs, that accelerates collision search, object picking, and ray tracing. It improves the speed of spatial operations through hierarchical rejection.

**Where it appears:**  
NVIDIA RTX, Unreal, Unity Physics, Blender, Embree, OptiX.  
A standard algorithm in graphics and physics.

**Comment:**  
Not part of Arden yet, but may be used as a HAOS-lib extension for spatial optimization / DYN.

---

## **Additional Terms** *(optional)*

---

#### **BVH2 / BVH4 / BVH8**

BVH variants that differ by the number of child nodes: 2, 4, or 8.  
Used in ray tracing and fast spatial structures such as RTX and Embree.

---

#### **SAH — Surface Area Heuristic**

A heuristic that determines how to split BVH nodes to minimize intersection cost.  
A standard BVH quality metric in renderers and ray tracers.

---

#### **LBVH — Linear BVH**

A method for building BVH from Morton codes for very fast GPU construction and updates.  
Used in GPU ray tracing and real-time voxel GI.

---

#### **HLBVH — Hierarchical LBVH**

An improved LBVH: the fast lower level is built linearly, while the upper level uses SAH.  
Provides a balance between GPU speed and tree quality.

---

#### **ABVH / Dynamic BVH**

A BVH that can be partially updated when objects move, without rebuilding the entire tree.  
Used in physics and games for dynamic scenes such as Havok and Bullet.

---

[📚 Back to Roadmap](../roadmap.md)