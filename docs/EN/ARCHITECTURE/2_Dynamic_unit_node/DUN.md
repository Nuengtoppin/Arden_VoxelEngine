**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2025-11-20

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  


# 0. Overview / mini-README

This section introduces the fundamental entity **DUN (Dynamic Unit Node)** —  
the first dynamic world unit built on top of  
**Topology → Routing → Rotation** (`1_TopologyLogic_Route_Rotation`, shortened as `1.x`).

DUN connects:

- the strict static topology of the world,
- Route addressing,
- runtime position and orientation (`float + quaternion`),
- local geometry (`voxels, SVO, meshes`),
- activity states (`ACTIVE / DORMANT / ARCHIVED`).

This document defines the **basic DUN invariants** and rules that will allow the project to:

- rotate containers through 360° without breaking topology,
- safely store local voxel structure,
- build physics and rendering on the surface representation,
- correctly connect Route and float-space.

This section **does not describe** EQ, HAOS, DTO, or the gameplay loop —  
those will be covered by the `3.x` documents (`EQ / Simulation Layer`).

---

### 0.1. Document strategy

Section `2.x` creates a **bridge between the static world (`1.x`)** and future dynamic subsystems (`3.x`).

At the first stage, the purpose of this document is to:

1. Clearly define what DUN is  
2. Define invariants — what it does and what it does not do  
3. Define Static vs Dynamic DUN  
4. Describe the anchoring model:  
   **Topology → LogicalAnchor → RuntimePosition → Transform**  
5. Explain what rotates and what remains static  
6. Define where voxels live and where meshes live  
7. Define DUN’s place in the general architecture order

This document contains **no simulation, sleep, wake-up, or network streaming logic** —  
that will come later in EQ-Core, EQ-Sim, and HAOS.

---

# 1. Purpose and role of DUN

## 1.1. Definition

**DUN (Dynamic Unit Node)** is an atomic node of the dynamic world that combines:

- a topology binding (**Route → discrete address**),
- runtime transformation (**position + quaternion**),
- local geometry (**voxels → mesh / SVO**),
- a basic activity state (`ACTIVE / DORMANT / ARCHIVED`).

DUN is **not a topology container**, but a layer above topology.  
It exists in dynamic space, but relies on the strict rules of the `1.x` triad.  
It does **not** change container sizes, strides, routing, or topology rotation.

DUN only introduces dynamics: transform, float-space, mesh, collider, and states.

## 1.2. Static DUN vs Dynamic DUN

DUN supports two operational modes: **Static** and **Dynamic**.  
Both modes use the same internal structure,  
but differ in transform behavior, rotation, simulation participation, and storage rules.

---

### **Static DUN**

Static DUN is a DUN that represents a **non-moving part of the world**,  
logically equal to a topological container or a consistent container volume  
(`Chunk / Octochunk / combinations of them according to active topology strides`).

Static DUN acts as a **dynamic wrapper around static topology**.  
It does not move and does not rotate smoothly, but it still has minimal dynamics  
(state, mesh rebuild) and is fully synchronized with the triad  
`Topology → Routing → Rotation`.

## 1.3. DUN responsibility area

It is important to separate **what DUN is** from **what DUN does**.

DUN describes:

- binding to topology (`Route`),
- its position and orientation (`Transform`),
- local volume (`DunVolume`),
- surface and colliders (`DunSurface`),
- bounding volumes (`DunBounds`),
- current participation state in the world (`DunState`).

At the same time, DUN does **not**:

- tick itself and does not contain gameplay loop logic;
- decide when it should be Active / Dormant / Archived;
- manage logging, profiling, or streaming;
- know about DTO, HAOS, or EQ algorithms.

These responsibilities belong to subsystems:

- **EQ-Core / EQ-Sim** — storage, ticking, simulation;
- **HAOS / DTO** — optimization, sleep/wake-up, archiving.

DUN remains a **transparent data and anchoring node** on top of which simulation and optimization systems operate.

---

#### **1. Static Transform**

```text
position: derived from logical anchor / container origin
rotation: discrete (0/90/180/270°)
scale: 1.0
```

Static DUN receives its position from a logical anchor  
(usually `DensityKey` or `FullRoute`, if a deeper binding is needed)  
and does not have its own float offsets.

Rotation is possible only through the Rotation Layer, discretely,  
without quaternion and without continuous angles.

---

#### **2. Static Geometry**

```text
voxel_grid: always available
svo_tree: optional (for LOD)
mesh[]: generated, but not rotated by a dynamic transform
aabb_local: constant
obb_world: same as AABB
```

Static DUN never has an angled OBB.  
It is always axis-aligned.

---

#### **3. Static Simulation Role**

```text
simulate_physics: false (except internal checks)
dynamic_move: false
dynamic_rotate: false
tick_rate: minimal (depends on HAOS/DTO)
```

Static DUN participates in simulation as:

- a voxel carrier,
- a mesh source,
- a world streaming participant,
- an object that may “sleep” or “wake up”.

It does **not** participate in physics as a body.

---

#### **4. Static State Rules**

```text
ACTIVE:   when mesh, generation, or loading is updated
DORMANT:  default state (almost always)
ARCHIVED: may be partially or fully unloaded
TEMP:     during chunk streaming
```

---

#### **5. Static Invariants**

1. Static DUN **never moves** in float-space.  
2. Rotation only goes through the **Rotation Layer**, discretely.  
3. The container fully matches its topological size.  
4. The Static DUN collider is an **axis-aligned mesh collider**.  
5. Static DUN is the foundation of **the whole world**, but not a moving subject.

---

### **Dynamic DUN**

Dynamic DUN is a DUN that is a **movable physical object**:  
a module, ship, platform, structural element, vehicle, NPC structure —  
anything that must:

- move,
- rotate smoothly,
- participate in physics,
- have a local voxel structure,
- have its own transform and behavior.

Dynamic DUN is a **world inside the world** (`mini-world`, local topology).

---

#### **1. Dynamic Transform**

```text
position: Vec3 (smooth movement)
rotation: Quat (smooth rotation)
scale: Vec3 (usually 1.0)
```

Dynamic DUN receives:

- full float position,
- quaternion rotation,
- world OBB,
- a dynamic physics body.

---

#### **2. Dynamic Geometry**

```text
voxel_grid / svo_tree: local topology
mesh[]: rotates and moves together with the transform
aabb_local: axis-aligned in local DUN space
obb_world: full Oriented Bounding Box
```

The key difference:

- **the voxel grid does NOT rotate** — invariant;
- **the mesh rotates as a whole** — surface representation.

This provides:

- stable strides,
- clean indexing,
- correct physics.

---

#### **3. Dynamic Simulation Role**

```text
simulate_physics: true
dynamic_move: true
dynamic_rotate: true
tick_rate: depends on EQ-Sim
```

Dynamic DUN participates in simulation as:

- a physical object,
- a source of a moving surface,
- a container for dynamic geometry,
- a participant in boosts / LOD / SVO inside HAOS.

---

#### **4. Dynamic State Rules**

```text
ACTIVE:   moving, in physics, being ticked
DORMANT:  asleep if the object has been stationary for a long time
ARCHIVED: may be packed into a blueprint/snapshot
TEMP:     when appearing in the world
```

---

#### **5. Dynamic Invariants**

1. Dynamic DUN may have **any position** in world float coordinates.  
2. Rotation is **not limited** to the discrete grid — quaternion is used.  
3. Voxels remain axis-aligned; the mesh rotates.  
4. Dynamic DUN **has a physical body / rigidbody**.  
5. Dynamic DUN may cross topological zones and chunk boundaries.  
6. LogicalAnchor remains a Core-side binding,  
   while `RuntimePosition + Transform` become the live source of truth for movement.

---

#### **6. Static vs Dynamic DUN — engineering summary**

| Property | Static DUN | Dynamic DUN |
|---------|-------------|--------------|
| Position | Derived from logical anchor | Float (`Vec3`), free |
| Rotation | Discrete 0/90/180/270 | Quaternion 0–360° |
| Collider | Axis-aligned | Oriented, rotates with object |
| Voxels | Constant | Constant |
| Mesh | Does not rotate | Rotates |
| Physics | No body | Has body |
| Tick | Almost no ticking | Full tick |
| Usage | World | Movable objects |
| AABB | = OBB | ≠ OBB |
| States | ACTIVE/DORM/ARCH | ACTIVE/DORM/ARCH |

---

### **Why both modes are part of one DUN**

Static and Dynamic are **operational modes**, not two different entity types.

They use:

- the same storage structure,
- the same topology invariants,
- the same anchor mechanism (`LogicalAnchor → RuntimePosition → Transform`),
- the same AABB/mesh pipeline,
- the same connection to EQ-Core/EQ-Sim.

The only difference is transform behavior.

This allows the project to have:

- one data format,
- one mesh generation pipeline,
- one sleep/wake-up mechanism,
- one interface for rendering, physics, and streaming.

---

# 2. DUN invariants

Strict invariants are introduced so that DUN does not break spatial logic.  
They define what DUN is allowed to do and what it cannot do.

---

## 2.1. DUN does not change topology

DUN:

- does not change the strides of active topology levels (`Region / Chunk / Octochunk / Voxel`);
- does not change container sizes;
- does not break the formulas of addressing and transformations in the active address model.

The world topology described in the `1.x` documents remains an unchanged foundation.  
DUN is layered on top of it and does not require any “special” containers or addresses.

---

## 2.2. DUN does not rotate the voxel grid

The local voxel grid of a DUN is always **axis-aligned**:

```text
(local_x, local_y, local_z) ∈ [0..Nx) × [0..Ny) × [0..Nz)
```

- there is no “rotated grid” inside DUN;
- SVO / voxel grid live in their own axis-aligned coordinate system;
- any rotation is applied above this layer, not by changing indices.

This preserves simple indexing, correct SVO/LOD behavior, and predictability for EQ.

---

## 2.3. DUN rotates as a whole through transform

DUN rotation is implemented through its transform:

```rust
struct DunTransform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3, // usually (1,1,1)
}
```

- the whole node rotates as one unit: mesh, OBB, visual representation;
- internal indices and voxel structure do not change.

For **Static DUN**, rotation may be limited to discrete values (`0/90/180/270°`).  
For **Dynamic DUN**, a full quaternion is allowed (`0–360°`).

---

## 2.4. Physics and walking happen on mesh, not raw voxels

Basic principle:

- voxels describe material/density;
- surface mesh describes the surface and is used for collisions.

Pipeline:

1. Inside DUN, a voxel grid / SVO is stored (material, destruction, LOD).
2. A surface mesh is built from this volume (triangles).
3. Physics and walking use the mesh or a derived collider, not direct iteration over every voxel.

This avoids “stair-step” collisions, keeps physics in a classic triangular/primitive form,  
and leaves voxel representation as an internal service layer.

---

## 2.5. AABB is local, OBB is world-space

DUN always contains two types of bounding volumes:

- **local AABB** — in DUN coordinates, axis-aligned;
- **world OBB** — the result of applying `DunTransform` to the local volume.

Local AABB:

- is used by HAOS/DTO and simple algorithms;
- does not depend on rotation.

World OBB:

- reflects the real shape of DUN in the world;
- is used by physics and precise intersection checks.

---

## 2.6. LogicalAnchor remains the logical DUN binding

DUN has two complementary descriptions of position:

- **LogicalAnchor** — where DUN is logically attached in the discrete world;
- **RuntimePosition / Transform** — where DUN currently is in runtime space.

For Static DUN, `LogicalAnchor` and runtime position usually almost match.

For Dynamic DUN, `LogicalAnchor` remains the Core-side binding,  
while `RuntimePosition + Transform` becomes the live source of truth for movement.

This allows the engine to:

- store and stream DUN through a stable address model;
- avoid forcing dynamic runtime to constantly live on a full deep address;
- extract `FullRoute` or other forms only on demand.

---

## 2.7. DUN content may change, size may not

DUN is a **fixed-volume container**, defined at creation time.

Fixed volume means:

- a fixed number of local grid cells (`Nx × Ny × Nz` voxels),
- a fixed binding of this volume to world strides  
  (for example: 1 Chunk, 1 Octochunk, 2 Chunks stacked vertically, a group of Octochunks, and so on).

Allowed:

- changing voxels inside it — destruction, adding voxels, changing materials;
- updating SVO / density / local data;
- designing different DUN types with different fixed sizes  
  (one chunk, two chunks, loose nodes, and so on).

Forbidden:

- dynamically changing the size of an existing DUN  
  (creating a “special” DUN of a different size on the fly);
- “stretching” or “shrinking” a DUN beyond its original volume.

If a task needs another size — for example, using 2 stacked Chunks instead of 1 Chunk —  
another DUN type or instance is created with its own topological binding and volume parameters.

### DUN size and topology strides

The “fixed DUN volume” invariant **does not mean** that every DUN must be  
strictly equal to one Chunk or one Octochunk.

The architecture allows different **DUN classes**, whose size is defined at design time:

- `DUN_Chunk` — one Chunk, for example `64×64×64` voxels;
- `DUN_Octo` — one Octochunk, for example `32×32×32`;
- `DUN_2Chunks_Vert` — two stacked Chunks, for example `64×128×64`;
- `DUN_4Octos_Tile` — a horizontal `2×2` Octochunk tile;
- `DUN_LooseNode` — a large loose node, for example `256×256×256`, above several base containers.

General rules:

- the size of a **specific DUN instance** is chosen at creation time  
  and remains unchanged throughout its lifecycle;
- DUN sizes must be consistent with topology  
  (based on base container strides: `Chunk / Octochunk / Region`,  
  or their integer combinations);
- large structures — ships, buildings, large platforms — are assembled **from multiple DUNs**  
  like tiles, not by dynamically “stretching” one DUN.

Thus:

- DUN remains a **fixed-volume container** at the instance level;
- the system remains flexible — a library of different DUN types can be designed,
  adapted to required strides, loose nodes, and usage patterns.

---

## 2.8. Summary: why DUN invariants are needed

DUN invariants guarantee that this entity:

- **adds dynamics on top of** the `Topology / Routing / Rotation` triad without changing its rules;
- remains **compatible with long-term saves and streaming** — logical anchor and topology are stable over time;
- gives a **clean foundation for physics and the mesh layer** — collision and walking happen on surface mesh, not on raw voxels;
- can be used by **EQ-Core / EQ-Sim / HAOS** without special cases;
- remains **simple and unambiguous for other developers**: it is clear what DUN does and what it cannot do.

---

# 3. Anchor Model

### Topology → LogicalAnchor → RuntimePosition → Transform

DUN exists simultaneously in four layers of spatial description:

1. **Topology Space** — discrete topology of the world  
2. **LogicalAnchor Space** — canonical DUN address in this topology  
3. **RuntimePosition Space** — continuous coordinates / runtime position  
4. **Transform Space** — full transform: position + rotation + scale

Anchor Model describes **how DUN is attached to all these layers at the same time**,  
and which of them are the “solid foundation” while others are dynamic layers above it.

---

## 3.1. Topology Space — discrete space

World topology (`1.x`) defines the discrete structure:

> Region → Chunk → Octochunk → Voxel

It defines:

- global subdivision of the world into containers,
- boundaries and sizes of each level,
- strides and indexing schemes, including Morton / flat,
- encode/decode formulas between indices and coordinates.

**DUN does not change Topology Space.**  
It only **references** already existing topological entities through a logical anchor.

---

## 3.2. LogicalAnchor Space — address

LogicalAnchor is the discrete binding of DUN in the active address model.

Depending on mode and task, the logical anchor can be:

- `DensityKey`
- `FullRoute`
- persistent container-aligned address

`SimSectorKey` is usually a derived coarse form,  
not the main DUN anchor.

LogicalAnchor defines:

- DUN’s logical membership in the world;
- its Core-side binding to the container structure;
- the entry point for EQ-Core, saves, and streaming.

At the same time, logical anchor:

- **does not store runtime-float position**;
- **does not describe rotation**;
- remains a **discrete binding identifier**, not a transform state.

---

## 3.3. RuntimePosition Space — runtime position

For physics, camera, render, and simulation, DUN has a  
**continuous float position in world space**:

```rust
position: Vec3
```

This position:

- is used by the physics engine — rigidbody, collisions;
- is used by rendering — world matrix for meshes;
- is used by simulation logic — DUN types, distances, triggers.

At the same time:

- changing `RuntimePosition` does not have to change `LogicalAnchor`;
- Topology/Routing remain the base discrete “map of the world”;
- RuntimePosition Space is a **live runtime projection** of DUN above this map.

---

## 3.4. Transform Space — position + quaternion + scale

The full DUN transform is described by:

```rust
struct DunTransform {
    position: Vec3,  // world position of the DUN anchor point
    rotation: Quat,  // DUN orientation, full 0–360° range
    scale: Vec3,     // usually (1.0, 1.0, 1.0)
}
```

Transform Space defines:

- DUN’s **world OBB**, through position + rotation;
- orientation and position of **surface meshes**;
- location of **physics colliders**;
- rotation of Dynamic DUN relative to the world.

For Static DUN:

- `position` strictly follows from the logical anchor — container origin;
- `rotation` is usually limited to discrete values (`0/90/180/270°`)
  and can be expressed through the Rotation Layer.

For Dynamic DUN:

- `position` freely changes over time (`Vec3`);
- `rotation` is a full quaternion — animation, physics, free rotation.

Transform **never changes topology** and **does not affect logical anchor**.  
It only describes how DUN “sits” in the float-world above the discrete structure.

---

## 3.5. Relationship between layers — conceptually

For any DUN, the chain can be understood as:

```text
Topology Space  ──►  LogicalAnchor Space  ──►  RuntimePosition Space  ──►  Transform Space
(world structure)    (Core-side binding)    (live address form)       (position+rotation)
```

- Topology gives the **framework** of the world.
- LogicalAnchor defines **which place in the framework** DUN is bound to.
- RuntimePosition defines DUN’s **live address position** in the world.
- Transform adds **rotation and scale** for render and physics.

In future documents (`EQ-Core`, `EQ-Sim`, `HAOS`), this model will be used  
as the base “language” for all DUN operations — streaming, simulation, optimization, saves.

## 3.6. Summary: why Anchor Model is needed

Anchor Model defines how DUN is simultaneously bound to four world description layers:

- **Topology Space** — defines the strict discrete world structure that DUN does not change.
- **LogicalAnchor Space** — gives DUN a stable Core-side binding and an entry point into topology.
- **RuntimePosition Space** — describes DUN’s live runtime position for addressing, physics, and logic.
- **Transform Space** — adds rotation and scale to position for Dynamic/Static modes.

Because of this:

- topology remains a stable foundation;
- DUN dynamics are described through a normal transform, not “special coordinates”;
- EQ-Core / EQ-Sim / HAOS systems can work with DUN using one coordinate language,
  without breaking the `Topology / Routing / Rotation` triad.

---

# 4. DUN rotation and topology stability

DUN rotation is one of the key places where the whole model can easily be “broken”  
if one tries to rotate voxels or containers at the topology level.

This section defines the fundamental rule:

> **only DUN rotates as a transform;  
> the internal voxel grid remains axis-aligned and static.**

---

## 4.1. The problem of rotating voxels

If one tries to rotate the DUN voxel grid itself:

- fixed container **strides** are broken  
  — voxels can no longer be indexed by simple linear formulas;
- **Morton encoding and flat indexing** are broken or become much more complex  
  — bit hierarchies no longer correspond to real geometry;
- the **container hierarchy** is destroyed  
  — Chunk / Octochunk / container levels no longer match “straight” blocks of space;
- it becomes difficult or impossible to:
  - search neighbors by integer coordinates,
  - cache areas — SVO, LOD,
  - use the same topology for different DUNs.

In other words:

> rotated voxel grid = dynamically distorted topology  
> that can no longer be treated as a simple discrete grid.

To prevent this, Arden architecture introduces a strict invariant:

> **the local DUN voxel grid is always axis-aligned and does not rotate.**

---

## 4.2. Solution through transform

DUN rotation is implemented not by changing topology or rebuilding the grid,  
but through a normal transform at the mesh and volume level.

Working sequence:

1. **Voxels are stored in local coordinates, axis-aligned**  
   Inside DUN there is its own local voxel grid / SVO:

   ```text
   (local_x, local_y, local_z) ∈ [0..Nx) × [0..Ny) × [0..Nz)
   ```

   It does not rotate and does not change orientation.

2. **A surface mesh is extracted from voxels**  
   Voxels are interpreted as material/density.  
   Based on them, one or more surface meshes are built:

   - marching cubes / dual contouring / greedy / another algorithm;
   - the result is a set of triangles in local DUN coordinates.

3. **Mesh follows the DUN transform — position + rotation**  
   For rendering and physics, the mesh receives:

   ```rust
   DunTransform { position: Vec3, rotation: Quat, scale: Vec3 }
   ```

   As a result, the mesh as a whole:

   - moves into world space (`position`),
   - rotates (`rotation`),
   - scales if needed (`scale`).

4. **Physics and render work only with the rotated mesh**

   - render uses the world transformation matrix,
   - the physics engine uses a collider built from the mesh or a simplified version of it,
   - characters and objects “stand” on the surface mesh, not on discrete voxel cubes.

5. **LogicalAnchor and topology remain untouched**  
   During any DUN rotation and movement:

   - LogicalAnchor remains the same logical binding,
   - Topology Space does not change,
   - encode/decode formulas, strides, and container sizes remain valid.

Result:

- the external world sees DUN as an object that can freely rotate and move;
- the internal voxel model remains simple, axis-aligned, and stable;
- all “rotation complexity” is concentrated in Transform Space, not in Topology Space.

## 4.3. Summary: why rotation goes through transform

DUN rotation is implemented only through transform (`position + rotation`),  
not through rotating the voxel grid. This guarantees that:

- topology and logical anchor remain strict and unchanged  
  — strides, encode/decode, and containers do not break;
- the internal voxel structure remains simple and axis-aligned, suitable for SVO/LOD and indexing;
- all continuous dynamics — rotations, physics, “sloped surfaces” — are solved at the mesh and collider level, where this is natural for an engine.

Thus, **DUN can freely rotate in the world without distorting the base discrete spatial model**.

---

# 5. Internal DUN structure — first level

This section describes the **minimal data skeleton** that every DUN should have.  
This is not the final code structure, but a **conceptual “field passport”**  
that EQ-Core, EQ-Sim, HAOS, and specialized DUN documents  
(`DUN.Mesh`, `DUN.Physics`, `DUN.Instance`, `DUN.Blueprint`) will rely on.

---

## 5.1. Conceptual DUN structure

At the conceptual level, DUN can be represented as:

```rust
DUN {
    anchor: DunAnchor,               // logical anchor in EQ-Core
    runtime_position: RuntimePosition,
    transform: DunTransform,
    volume: DunVolume,
    surface: DunSurface,
    bounds: DunBounds,
    state: DunState,
}
```

Where:

```rust
enum DunAnchor {
    DensityKey(DensityKey),
    FullRoute(FullRoute),
}
```

---

## 5.2. Binding to topology

```rust
anchor: DunAnchor
```

**DunAnchor** is the logical DUN anchor in EQ-Core.

It defines a stable discrete binding of DUN to the world  
and is used for storage, streaming, grouping, and persistence logic.

In the active MVP, this may be:

- `DensityKey`
- `FullRoute`

Practically, this means:

- `anchor` defines which place in the discrete world DUN is bound to;
- `anchor` does not store runtime-float position;
- `anchor` does not describe rotation;
- `anchor` remains the Core-side truth for logical binding,
  while runtime movement lives in `RuntimePosition + Transform`.

---

## 5.3. DUN Transform

```rust
struct DunTransform {
    position: Vec3,  // world position of the DUN anchor point
    rotation: Quat,  // DUN orientation (0–360°, for Dynamic DUN)
    scale: Vec3,     // usually (1.0, 1.0, 1.0)
}
```

Transform describes:

- where DUN is in **float-space**;
- how it is oriented, especially for Dynamic DUN;
- what scale is applied to its surface representation.

For **Static DUN**:

- `position` matches the origin computed from the logical anchor;
- `rotation` is either zero or discrete (`0/90/180/270°`, through Rotation Layer);
- `scale` is almost always `(1,1,1)`.

For **Dynamic DUN**:

- `position` moves freely;
- `rotation` is a full quaternion;
- `scale` is usually also unit, but may be expanded by future documents.

---

## 5.4. Local volumes: voxels and/or SVO

```rust
struct DunVolume {
    voxel_grid: Option<VoxelGrid>,  // regular grid, if used
    svo_tree:   Option<SvoTree>,    // sparse structure for LOD/SVO, if used
    local_aabb: Aabb,               // AABB in local DUN coordinates
}
```

**DunVolume** describes the “inside” of DUN:

- `voxel_grid` — local voxel grid, axis-aligned, if needed;
- `svo_tree` — sparse tree for optimizations, LOD, and complex shapes;
- `local_aabb` — minimal AABB covering the whole DUN volume
  in its **local coordinate system**.

The key point:

- voxel structure does not rotate;
- DunVolume lives in local DUN space.

---

## 5.5. Surface representation — meshes and colliders

```rust
struct DunSurface {
    meshes: Vec<MeshId>,        // one or more surface meshes
    collider: PhysicsCollider,  // physics shape based on mesh or primitives
}
```

**DunSurface** is responsible for everything that interacts with:

- render — models, surfaces, LOD meshes;
- physics — collisions, character standing, raycasts.

Key points:

- meshes are built from DunVolume — voxels / SVO;
- during render and physics, `DunTransform` is applied to the mesh;
- the character “stands” on the mesh, not on raw voxels.

---

## 5.6. Bounding volumes

```rust
struct DunBounds {
    aabb_local: Aabb, // axis-aligned in local DUN space
    obb_world: Obb,   // oriented box in world space
}
```

**DunBounds** connects local and world geometry:

- `aabb_local`:

  - used by HAOS / DTO and simple algorithms;
  - always axis-aligned, independent of rotation;
  - may match `DunVolume.local_aabb` or be aggregated.

- `obb_world`:

  - computed from `aabb_local + DunTransform`;
  - describes the real DUN volume in world space;
  - used by physics and precise broad-phase.

---

## 5.7. DUN state

```rust
enum DunState {
    Active,      // participates in simulation and updates
    Dormant,     // exists, but does not tick
    Archived,    // unloaded/saved, not present in memory as a full object
    TempLoaded,  // just loaded, not yet in the active cycle
}
```

**DunState** describes DUN participation in:

- simulation,
- streaming,
- optimization.

State transition algorithms are not described in this document  
and will be defined in EQ-Sim / HAOS. Here, only the **set of allowed statuses** is fixed.

---

## 5.8. Summary: role of the internal DUN structure

The internal DUN structure defines a **minimal unified format** that can be used by:

- EQ-Core — as a structure of storage and state,
- EQ-Sim — as a simulation unit,
- HAOS/DTO — as an object of optimization and sleep/wake-up,
- Mesh/Render subsystems — as a carrier of surface geometry,
- Physics — as a carrier of colliders and volumes.

This is **not the final data structure in code**, but an architectural skeleton:  
specific fields and types may be expanded in separate documents (`DUN.Mesh`, `DUN.Physics`, `DUN.Instance`),  
but the basic field groups and their meaning should remain unchanged.

---

# 6. DUN states

DUN state defines **how actively this node participates in simulation and streaming**,  
and which subsystems are allowed to work with it at the current moment.

At the structure level (see `5.7`), this is represented as `DunState`;  
here, the **semantics** of these states are defined.

---

## 6.1. Purpose of states

DUN states are needed to:

- separate **active** and **sleeping** parts of the world;
- allow HAOS/DTO to manage node loading and unloading;
- give EQ-Sim a clear signal: this DUN should tick / should not tick;
- reduce CPU/GPU load when there are many DUNs.

State is an **architecture-level flag**,  
not a concrete implementation of sleep/wake-up algorithms.

---

## 6.2. Semantics of DunState

```rust
enum DunState {
    Active,
    Dormant,
    Archived,
    TempLoaded,
}
```

### **Active**

DUN is in an active phase:

- participates in simulation ticks;
- its Transform may change, especially for Dynamic DUN;
- its surface/mesh may update;
- physics and interaction logic take this DUN into account.

Typical cases:

- movable objects — Dynamic DUN;
- chunks near the player;
- areas where changes are currently happening.

---

### **Dormant**

DUN is in a “sleeping” state:

- DUN data is present in memory — volume, surface, bounds;
- Transform does not change, or changes very rarely;
- DUN does not tick every frame / simulation step;
- physics may treat it as static geometry through mesh/collider;
- HAOS/DTO may wake DUN if needed.

Typical cases:

- distant world areas that are visible but not actively changing;
- static objects that have not been accessed for a long time.

---

### **Archived**

DUN is unloaded from active memory:

- full data — volume, surface, bounds — may be saved to disk or in compressed form;
- only a “thin” record may remain in RAM — Route, ID, metadata;
- DUN participates in neither simulation nor rendering;
- it can be restored into TempLoaded / Active during streaming.

Typical cases:

- regions the player has not approached for a long time;
- server world snapshots.

---

### **TempLoaded**

Intermediate state:

- DUN has just been pulled from `Archived`;
- data has been restored partially or fully;
- it is not yet included in the main simulation cycle;
- HAOS/DTO may prepare it and then move it to Active or Dormant.

Typical cases:

- chunk loading before the player appears nearby;
- restoring a dynamic object from a save.

---

## 6.3. Subsystem responsibilities

- **EQ-Core** is responsible for DUN data consistency in each state  
  — what exactly must be in memory: only Route, Route+Volume, or full object.

- **EQ-Sim** is responsible for which states participate in ticks  
  — usually only `Active`, sometimes partly `Dormant` for static objects.

- **HAOS/DTO** are responsible for transitions between states  
  — when to “put to sleep”, when to “wake up”, when to “archive”.

This document defines only **the state set and their meaning**.  
Specific transition rules will be described in EQ documents.

---

# 7. Why DUN does not break Topology / Routing / Rotation

DUN is designed from the start as a **layer above the `1.x` triad**,  
not as a modification of it. This section defines architectural guarantees  
for why adding DUN does not break:

- Topology Space — container structure;
- Routing Space / active address forms;
- Rotation Layer — discrete rotations in topology.

---

## 7.1. Guarantees for Topology Space

DUN:

- does **not change container sizes** of active topology;
- does **not change strides** or indexing schemes;
- does **not interfere with encode/decode formulas** of coordinates and active address forms;
- does **not rotate** the voxel grid — `voxel_grid` and `svo_tree` remain axis-aligned.

Consequence:

- all properties proven and described in the Topology/Routing documents  
  remain valid regardless of the number and behavior of DUNs;
- any algorithms relying on strict topology — neighbor search, Morton, SVO/LOD —  
  can operate without knowing that DUN exists.

---

## 7.2. Guarantees for LogicalAnchor Space

DUN:

- always has **one canonical Route** — logical address in the world;
- does not change Route structure or construction rules;
- does not require “special” Route types for Static / Dynamic modes;
- uses Route only as an **anchor** for:

  - storage,
  - streaming,
  - grouping by regions.

Consequence:

- save and load systems can work with DUN like with any other entities,
  relying on logical anchor and active address model;
- Topology + Routing remain the “single map of the world” to which DUN is only attached.

---

## 7.3. Guarantees for Rotation Layer

Rotation Layer from `1.x`:

- describes discrete container rotations (`0/90/180/270°`),
- is used where “hard” rotations compatible with topology are needed.

DUN:

- does not change the set of discrete rotations in Rotation Layer;
- does not require introducing new discrete states into topology;
- uses **Transform Space** (`Quat`) only at the level of mesh / collider / OBB.

Static DUN:

- may use Rotation Layer — discrete rotations — as its orientation.

Dynamic DUN:

- uses quaternion rotation above existing topology,
  without adding new states to Rotation Layer.

Consequence:

- Rotation Layer remains a self-sufficient system for discrete rotations;
- continuous DUN rotations do not require changes in the triad.

---

## 7.4. Guarantee of layer separation

In total, DUN provides **strict layer separation**:

- Topology / Routing / Rotation describe **global discrete space**;
- DUN describes **local dynamic nodes** in this space;
- Transform / Mesh / Physics describe **concrete DUN behavior and geometry**.

DUN works **above** the triad, using it as a foundation,  
but does not change its definitions, formulas, or invariants.

That is why:

> adding DUN makes the world dynamic and interactive,  
> but does not turn topology into a “floating” or unstable system.

---

# 8. DUN place in architecture and future documents

Section `2.x` defines DUN as a **minimal dynamic world unit**, built **above** the triad:

- Topology / Routing / Rotation (`1.x`) — strict discrete foundation;
- DUN (`2.x`) — local dynamic nodes;
- Transform / Mesh / Physics — continuous behavior and geometry above DUN.

This document defines:

- DUN definition — Static / Dynamic modes;
- invariants — what DUN does and what it does not do;
- Anchor Model — `Topology → LogicalAnchor → RuntimePosition → Transform`;
- transform-based rotation rules;
- basic internal structure and DUN states.

Further architecture levels do not change these rules; they rely on them.

---

## 8.1. Relationship with `1.x` documents — Topology / Routing / Rotation

The `1.x` documents describe:

- discrete world topology — `Region → Chunk → Octochunk → Voxel`;
- active address forms in Routing;
- discrete Rotation Layer.

DUN:

- does not change container sizes, strides, or encode/decode active address forms;
- does not rotate the voxel grid;
- uses logical anchor as a Core-side binding and an entry point into topology;
- only adds runtime-position, transform, mesh, collider, and states above this layer.

Thus, **`1.x` remains the “laws of the universe”**,  
while DUN is a controlled “inhabitant” of this world.

---

## 8.2. Relationship with future EQ documents — `3.x`

The next major architecture block is the **`3.x` document series (`EQ / Simulation Layer`)**:

- **EQ-Core**  
  EQ-Core stores the stable truth model of DUN:
  - `anchor`,
  - `volume`,
  - `surface` refs,
  - `bounds`,
  - `state`,
  - persistent metadata.

- **EQ-Sim**  
  EQ-Sim executes runtime DUN behavior:
  - `runtime_position`,
  - `transform`,
  - physics participation,
  - tick logic,
  - sim grouping.

- **HAOS (Hybrid Adaptive Optimization System)**  
  Describes how DUN activity is optimized:
  - transitions between `Active / Dormant / Archived / TempLoaded`,
  - DUN budgets by regions, classes, and families,
  - interaction with DTO — sleep/wake-up systems — and bounding volumes.

All these subsystems work **on top of the already established DUN invariants**  
and do not change its base contract.

---

## 8.3. Specialized DUN documents

On top of this base document, separate clarifying specifications are planned:

- **DUN.Mesh**  
  - how surface meshes are built from `DunVolume` (`voxel_grid / SVO`),
  - LOD policy and mesh updates when voxels change,
  - connection between the mesh layer, Transform, and Render.

- **DUN.Physics**  
  - how colliders are formed from `DunBounds` and mesh,
  - which body types are used for Static / Dynamic DUN,
  - rules for using AABB/OBB in physics and broad-phase.

- **DUN.Blueprint**  
  - DUN templates — walls, modules, ships, structures,
  - destruction/fragmentation parameters, prebuilt mesh/volume assets,
  - connection between blueprints and EQ-Core/streaming.

- **DUN.Instance**  
  - runtime representation of Dynamic DUN,
  - connection between an instance and blueprints,
  - additional fields needed only during simulation — timers, local controllers, and so on.

Each of these documents **does not redefine** the base DUN contract,  
but only details separate aspects: geometry, physics, templates, and instances.

---

## 8.4. What is stable and what may change

Considered **stable** and not subject to arbitrary changes:

- definition of DUN as Dynamic Unit Node;
- separation of Static / Dynamic modes;
- Anchor Model (`Topology → LogicalAnchor → RuntimePosition → Transform`);
- `2.x` invariants — topology, logical anchor, rotation, mesh / voxel, AABB / OBB, fixed volume;
- base DUN structure — anchor, runtime_position, transform, volume, surface, bounds, state;
- `DunState` set.

May evolve in future versions:

- specific fields inside `DunVolume / DunSurface / DunBounds`;
- extensions for `DUN.Instance / DUN.Blueprint`;
- DUN state management strategies in EQ-Sim and HAOS;
- additional DUN subclasses for specific tasks — water, large structures, FX, and so on.

---

## 8.5. Summary

This document defines DUN as:

> **an atomic node of Arden’s dynamic world,  
> which adds movement, rotation, and physics  
> without breaking the strict discrete topology of the core.**

All subsequent architecture documents (`1.x`, `3.x`, and specialized DUN modules)  
will rely on the invariants and anchoring model described here  
to preserve project integrity and a clear contract for developers and researchers.

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  
