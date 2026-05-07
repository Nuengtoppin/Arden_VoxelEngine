**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16  
**Purpose:** Active MVP rotation canon for Arden

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  


# **1.0. Overview / mini-README for the Rotation section**

The **Rotation** section defines the active MVP model for reorienting space in Arden.

Rotation does not describe
how the world is divided into containers,
and it does not describe
how systems address those containers.
Those roles are already assigned to:

* **Topology** — the mathematics of space;
* **Routing** — the address protocol on demand.

Rotation lives **above** them
and answers a different question:

> **how to correctly rotate coordinates, directions, addresses, and container contents
> without breaking their structure and meaning.**

In the active MVP, Rotation defines:

* discrete rotations around the **Y** axis:
  `R0 / R90 / R180 / R270`;
* shared rules for:
  * world-space coordinates,
  * local coordinates inside containers,
  * active Routing address forms,
  * directions and orientation-state,
  * local container contents.

Rotation does not add new topology
and does not replace Routing.
It introduces a **mathematical reorientation contract** for already defined space and addresses.

---

## **1.0.1. What Rotation defines**

Section 1.x defines:

* **the canonical set of allowed rotations**
  * the discrete group `C4` around the Y axis:
    `R0 / R90 / R180 / R270`;

* **the layers affected by Rotation**
  * world coordinates `WorldXYZ`,
  * local container coordinates,
  * local float coordinates,
  * active Routing address forms,
  * directions and orientation-state;

* **the separation between two types of rotation**
  * rotation of an **anchor / position** in the world,
  * rotation of **content** inside a container;

* **Rotation invariants**
  * reversibility,
  * preservation of address validity,
  * preservation of container membership,
  * consistency with Topology and Routing.

Rotation does not define:
* a new container hierarchy,
* a new addressing method,
* gameplay,
* physical rules,
* a full 3D orientation system with arbitrary rotations.

---

## **1.0.2. Place of Rotation in the triad**

In the active MVP, the document triad divides responsibilities as follows:

* **Topology** — what world space is:
  levels, sizes, indices, strides, transformations.

* **Routing** — how systems obtain an address of the needed depth:
  `RuntimePosition`, `DensityKey`, `SimSectorKey`, `FullRoute`.

* **Rotation** — how all of this can be **reoriented**
  without breaking topology, addressing, and internal consistency.

That is:

```text
Topology = structure of space
Routing  = addressing protocol
Rotation = reorientation protocol
```

In this triad, Rotation is not responsible for “where something is”
and not for “how it is addressed”,
but for:

* how world-space is rotated,
* how local coordinates are rotated,
* how address forms are rotated,
* how anchor rotation is separated from content rotation,
* how orientation-state is stored separately from the address key itself.

---

## **1.0.3. Active MVP foundation of Rotation**

Rotation relies on the already defined active MVP canon:

```text
Axes: X | Y | Z
Active topology: Region → Chunk → Octochunk → Voxel
Sim overlay: RegionSector = 2 × 2 × 2
```

And on the active address forms:

```text
RuntimePosition = Region + LocalFloat
DensityKey      = Region + Chunk
SimSectorKey    = Region + Sector
FullRoute       = Region + Chunk + Octochunk + Voxel
```

Therefore, in the active MVP, Rotation:

* does not use `Octant` as part of the machine-truth rotation/address model;
* does not treat orientation as part of the base address key itself;
* treats orientation as a **separate state**
  that can be applied to position, direction, or content.

---

# **1.1. Purpose of the Rotation layer**

The purpose of Rotation is to provide one strict way to answer the question:

> **what exactly does “rotate” mean in Arden,**
> when talking about world-space, a container, an address, or content.

In the active MVP, Rotation is needed so that all subsystems
interpret rotation in the same way.

---

## **1.1.1. What exactly rotates**

Rotation distinguishes several independent rotation targets.

### **1. World position**

A world-space position or a runtime anchor attached to it can rotate.

Example:

* rotating a world vector,
* recalculating `RuntimePosition`,
* possible transition into another `DensityKey` or `SimSectorKey`.

---

### **2. Address form**

A discrete address can rotate
if a system needs to obtain a correct new address
after a world rotation.

Examples:

* `DensityKey`
* `SimSectorKey`
* `FullRoute`

---

### **3. Container content**

It is possible to rotate not the container address,
but only its local content.

Examples:

* rotating a voxel pattern inside a Chunk,
* rotating a blueprint inside a fixed anchor,
* reorienting a local shape without moving the container.

---

### **4. Orientation-state**

It is possible to change only the orientation state of an object,
without changing the anchor address itself.

This is especially important for:

* templates,
* blueprints,
* future DUN-like containers,
* oriented runtime objects.

---

## **1.1.2. What Rotation does not do**

Rotation **does not**:

* change world topology;
* change the active hierarchy;
* turn one address type into another “by meaning” without an explicit conversion;
* replace Topology or Routing;
* introduce arbitrary 3D rotations into the active MVP.

In other words:

* Rotation does not decide
  which container is the density unit;
* Rotation does not decide
  which address form is used by a system;
* Rotation only decides
  **how the selected entity is correctly reoriented**.

---

## **1.1.3. Why Rotation is a separate section**

Rotation is separated into its own layer
because rotation is not part of topology itself
and not part of addressing itself.

The same topology can exist without rotation.
The same address can be valid without knowing rotation.

But as soon as a system needs to:

* rotate chunk-local content,
* rotate a template,
* interpret orientation-state,
* recalculate an address after a world rotation,

a single mathematical contract is needed.
That is exactly what Rotation defines.

---

# **1.2. Scope**

Rotation describes the **mathematical application of rotations** in Arden systems.

It is needed not only for “nice object rotation”,
but for the consistent work of several engine layers.

---

## **1.2.1. Runtime and world-space**

Rotation applies to:

* world-space coordinates;
* runtime positions of objects;
* orientation-state of the camera, player, or dynamic node;
* local float coordinates inside a Region or another container.

This gives a unified language for:

* movement,
* orientation,
* rotating anchors,
* interpreting world/local-space.

---

## **1.2.2. Density and containers**

Rotation applies to:

* contents of a Chunk / Octochunk / voxel range;
* local indices inside containers;
* rotation of templates and patterns inside a fixed density anchor.

This is important for:

* mesh build,
* generation patterns,
* reusable structures,
* compression-friendly local transformations.

---

## **1.2.3. Routing and address forms**

Rotation applies to the address layer through active forms:

* `RuntimePosition`
* `DensityKey`
* `SimSectorKey`
* `FullRoute`

At the same time, it is important that:

* rotation does not have to be part of the address key itself;
* address and orientation-state are different entities;
* address rotation is defined as a consistent transformation
  through world/local math and active topology.

---

## **1.2.4. Sim overlay**

Rotation also applies to the coarse sim-layer,
but not as a “magical rotation of a sim key by itself”.

The correct logic is:

* an object has runtime-position and orientation-state;
* after rotation/movement it may enter another `RegionSector`;
* `SimSectorKey` is recalculated from the new position.

That is, sim-addressing remains derived from position,
not an independent rotation system for its own sake.

---

## **1.2.5. Tools / debug / blueprints**

Rotation is especially important for:

* inspector,
* debug visualization,
* local pattern tools,
* copy/paste with orientation,
* blueprint placement,
* anchor vs content rotation.

This is exactly where it is especially important to distinguish:

* rotation of the **anchor address**,
* rotation of the **local content**,
* rotation of **orientation-state only**.

---

## **1.2.6. Future extensions**

In the active MVP, Rotation is limited to discrete Y-rotation `C4`.

This is an intentional limitation.

In the future, extensions may appear above it:

* a broader symmetry/orientation layer,
* richer blueprint orientation,
* DUN/local-world orientation models,
* possibly the full set of cubic orientations.

But in the active MVP, none of this is part of the base Rotation contract.

---

## **1.2.7. Practical result**

In the active document triad, Rotation is responsible for one thing:

> **how to correctly rotate an already existing world,
> an already existing address,
> and already existing content,
> without breaking the structure of Topology and the Routing contract.**

This is the role of Rotation in the active MVP of Arden.

---

# **2. Discrete rotation model**

Section 2 defines the mathematical model of rotations
on which all later Rotation operations are built:

* rotation of world-space coordinates,
* rotation of local container coordinates,
* rotation of directions,
* rotation of active address forms,
* rotation of container contents.

In the active MVP, Rotation is treated as a **discrete yaw-rotation model**
around the vertical **Y** axis.

---

## **2.1. Canonical set of rotations (C4 group)**

The active MVP uses a finite set of discrete rotations:

```text
R0   =   0°
R90  =  90°
R180 = 180°
R270 = 270°
```

General notation:

```text
Rot = { R0, R90, R180, R270 }
```

These rotations form a cyclic group of order 4:

```text
C4
```

with the usual composition operation.

Examples:

```text
R0   * θ    = θ
R90  * R90  = R180
R90  * R180 = R270
R90  * R270 = R0

R180 * R180 = R0
R270 * R90  = R0
```

Group properties:

* there is an identity element `R0`;
* every angle has an inverse:

  * `inv(R0)   = R0`
  * `inv(R90)  = R270`
  * `inv(R180) = R180`
  * `inv(R270) = R90`
* composition is closed inside `{R0,R90,R180,R270}`.

In the active MVP, this is the **only canonical set of rotations**.
All Rotation operations must accept exactly one element of `Rot`.

---

## **2.2. Why C4**

In the active MVP, Rotation is intentionally limited to the `C4` group,
not to broader sets of spatial symmetries.

Reasons:

* the current world/runtime layer of Arden is built around an **invariant vertical Y**;
* the main practical need is yaw rotations:

  * north / east / south / west,
  * 4 blueprint orientations,
  * 4 anchor/content orientations,
  * 4 rotation states of a local template;
* `C4` provides:

  * strict reversibility,
  * predictable mathematics,
  * low implementation cost,
  * enough coverage for the active MVP.

Important:

* `C4` is the **current active Rotation contract**;
* broader symmetry systems may appear later
  as a separate future layer;
* they must not replace the base MVP model of the Rotation section.

---

## **2.3. Y axis and XZ plane**

Rotation in the active MVP is defined **strictly around the Y axis**.

This means:

* the **Y** axis remains invariant;
* only the **XZ** plane rotates;
* “up / down” orientation is not changed by yaw rotation itself.

This is consistent with the active MVP canon:

```text
Axes: X | Y | Z
```

and with the fact that:

* `X/Z` form the horizontal working plane;
* `Y` defines verticality.

---

## **2.4. Canonical world rotation (WorldXYZ)**

Given a world-space vector:

```text
P = (X, Y, Z)
```

Rotation acts on it as follows.

### **R0**

```text
rot_world(P, R0) = ( X,  Y,  Z )
```

### **R90**

```text
rot_world(P, R90) = (  Z,  Y, -X )
```

### **R180**

```text
rot_world(P, R180) = ( -X,  Y, -Z )
```

### **R270**

```text
rot_world(P, R270) = ( -Z,  Y,  X )
```

Properties:

* `Y` is preserved without changes;
* length of the XZ projection is preserved;
* the transformation is reversible:

```text
rot_world(rot_world(P, θ), inv(θ)) = P
```

These formulas are the **world-space rotation canon**
for the whole document.

---

## **2.5. Rotation direction and convention**

In the active MVP, the following convention is accepted:

* `R90` is the canonical next step after `R0`;
* `R180` is the second step;
* `R270` is the third step;
* `R270 = inv(R90)`.

In practice, this means
that the system works not through word labels like “left/right”,
but through a strict discrete set of states and their composition.

In other words:

* the **state algebra** `R0/R90/R180/R270` matters,
* not a verbal description of the angle detached from the formulas.

This removes confusion between:

* “clockwise”,
* “counterclockwise”,
* “when looking from above / below”.

The source of truth here is specifically the formulas `rot_world(...)`.

---

## **2.6. Composition law**

If an object has orientation-state `a`
and an additional rotation `b` is applied to it,
the resulting state is defined as composition:

```text
rot_compose(a, b) = a * b
```

where `*` is the operation of the `C4` group.

Examples:

```text
R0   * R90  = R90
R90  * R90  = R180
R180 * R90  = R270
R270 * R90  = R0
```

This is important for:

* blueprint placement,
* runtime orientation-state,
* container content rotation,
* gradual accumulation of rotations.

---

## **2.7. Rotation invariants**

All Rotation operations in the active MVP follow a common set of invariants.

### **2.7.1. Reversibility**

For any rotated entity `T` and any `θ ∈ Rot`:

```text
rot(rot(T, θ), inv(θ)) == T
```

This must hold for:

* world-space coordinates,
* local coordinates,
* directions,
* orientation-state,
* active address forms,
* local container contents.

---

### **2.7.2. Validity preservation**

After applying Rotation:

* coordinates remain valid for their level;
* the address form remains valid;
* the container remains a container of the same level;
* rotation does not break active MVP invariants.

That is, Rotation cannot:

* turn `Chunk` into another level type;
* break index ranges;
* produce an invalid `FullRoute`;
* break `Region + LocalFloat` as a valid runtime anchor.

---

### **2.7.3. Consistency with Topology**

Rotation does not change:

* active hierarchy:

```text
Region → Chunk → Octochunk → Voxel
```

* `RegionSector = 2 × 2 × 2` as sim-overlay;
* axis canon `X | Y | Z`;
* sizes and strides of levels.

Rotation only **reorients** already existing entities inside this contract.

---

### **2.7.4. Consistency with Routing**

Rotation must be consistent with active address forms:

* `RuntimePosition`
* `DensityKey`
* `SimSectorKey`
* `FullRoute`

At the same time:

* orientation-state does not have to be part of the address key itself;
* address and orientation are different entities;
* if needed, Rotation can recalculate an address form,
  but does not replace orientation-state with it.

---

### **2.7.5. Independence from data carrier**

Rotation is defined:

* at the coordinate level,
* at the index level,
* at the address-form level,
* at the local-content level,

not at the level of a concrete storage format.

This means:

* Rotation does not depend on whether data is stored in a CPU array, GPU buffer, ECS storage, or another container;
* it only defines the rule
  **for how coordinates and indices should change**;
* physical memory reordering is an implementation task.

---

## **2.8. Two independent types of rotation**

The active MVP Rotation immediately distinguishes two different rotation types.

### **1. Anchor / position rotation**

Changes:

* world-space position,
* runtime anchor,
* possibly — the computed address.

Examples:

* rotating a world vector;
* recalculating `RuntimePosition`;
* transition into another `DensityKey` or `SimSectorKey`.

---

### **2. Content rotation**

Changes:

* local orientation of container contents,
* local coordinates inside the container,
* orientation-state of a local template.

At the same time:

* the anchor address may remain unchanged;
* the container may stay at the same `DensityKey`
  or the same `RuntimePosition` anchor.

This distinction is one of the main points of active MVP Rotation
and will be used in all applied sections below.

---

## **2.9. Summary of the Rotation model**

In the active MVP, the Rotation model defines:

```text
Rotation = C4 around Y
world canonical rotation = rot_world(X,Y,Z)
orientation-state is separate from address
anchor rotation != content rotation
```

This is the minimal mathematical foundation
for building:

* coordinate rotation,
* direction rotation,
* address-form rotation,
* rotation of containers and their contents.

---

# **3. Coordinate rotation**

This section describes how the discrete rotations `R0 / R90 / R180 / R270`
act on active MVP coordinate representations.

Rotation applies to three connected layers:

* **world-space coordinates** `WorldXYZ`,
* **local index coordinates** inside containers,
* **local float coordinates** inside containers.

Here Rotation works **purely with coordinates**.
The address forms themselves (`RuntimePosition`, `DensityKey`, `SimSectorKey`, `FullRoute`)
will be covered separately in later sections.

---

## **3.1. Global world coordinates (WorldXYZ)**

Global coordinates describe a position in world space
and may be integer or real-valued:

```text
P = (X, Y, Z),   X, Y, Z ∈ ℤ  or  ℝ
```

Rotation in the active MVP acts around the **Y** axis,
therefore the `Y` component remains invariant,
and only the `XZ` plane rotates.

---

### **3.1.1. Canonical world-space rotation formulas**

For any `P = (X, Y, Z)` and any `θ ∈ {R0, R90, R180, R270}`:

* **R0**

  ```text
  rot_world(P, R0) = ( X,  Y,  Z )
  ```

* **R90**

  ```text
  rot_world(P, R90) = (  Z,  Y, -X )
  ```

* **R180**

  ```text
  rot_world(P, R180) = ( -X,  Y, -Z )
  ```

* **R270**

  ```text
  rot_world(P, R270) = ( -Z,  Y,  X )
  ```

These formulas are the **world-space rotation canon**
for the entire Rotation section.

---

### **3.1.2. Properties of world-space rotation**

For `rot_world`:

* `Y` does not change;
* length of the XZ projection is preserved;
* the transformation is fully reversible:

```text
rot_world(rot_world(P, θ), inv(θ)) = P
```

The same mathematics is also used for float vectors in world-space:

```text
(X, Y, Z) ∈ ℝ³
```

meaning the same canon applies to:

* physics,
* navigation,
* camera,
* tools,
* oriented runtime objects.

---

## **3.2. Local index coordinates of containers**

Local indices of active topology are always non-negative
and live inside containers of fixed size.

Examples:

```text
ChunkCoord     = (cx, cy, cz)
OctochunkCoord = (ox, oy, oz)
VoxelCoord     = (vx, vy, vz)
SectorCoord    = (sx, sy, sz)
```

Rotation at this level means:

* rotate the coordinate **inside the container**,
* keep it **inside the same container**,
* do not change the container type and do not break ranges.

---

### **3.2.1. General model for a square XZ grid**

Let the container have dimensions:

```text
size_x = size_z = N
size_y = H
```

and a local coordinate:

```text
p = (x, y, z),
0 ≤ x < N,
0 ≤ y < H,
0 ≤ z < N
```

Then discrete rotation inside the container is defined as follows.

* **R0**

  ```text
  x' = x
  y' = y
  z' = z
  ```

* **R90**

  ```text
  x' = z
  y' = y
  z' = N - 1 - x
  ```

* **R180**

  ```text
  x' = N - 1 - x
  y' = y
  z' = N - 1 - z
  ```

* **R270**

  ```text
  x' = N - 1 - z
  y' = y
  z' = x
  ```

This is the canon for **discrete index rotation inside a container**
under yaw rotation around the `Y` axis.

---

### **3.2.2. Where this model applies in the active MVP**

This scheme applies to all active MVP levels
where the XZ base of the container is square.

#### **Voxel inside Octochunk**

```text
N = OCTO_SIZE
(vx', vy', vz') = rot_local(vx, vy, vz, θ; N)
```

#### **Octochunk inside Chunk**

```text
N = 2
(ox', oy', oz') = rot_local(ox, oy, oz, θ; N)
```

#### **Chunk inside Region**

```text
N = REGION_CHUNKS_PER_AXIS
(cx', cy', cz') = rot_local(cx, cy, cz, θ; N)
```

#### **RegionSector inside Region**

```text
N = 2
(sx', sy', sz') = rot_local(sx, sy, sz, θ; N)
```

Important:

* `RegionSector` is rotated here as an **overlay coordinate inside Region**;
* this does not make the sim-layer part of the density path;
* this is simply the same mathematical rotation operation in a local XZ grid.

---

### **3.2.3. Invariants of local index rotation**

For any correct index `p`
and any `θ ∈ Rot`, the following is guaranteed:

1. **Container membership is preserved**

```text
0 ≤ x' < N
0 ≤ y' < H
0 ≤ z' < N
```

2. **Reversibility**

```text
rot_local(rot_local(p, θ; N), inv(θ); N) = p
```

3. **Level is preserved**

* `VoxelCoord` remains `VoxelCoord` after rotation;
* `OctochunkCoord` remains `OctochunkCoord`;
* `ChunkCoord` remains `ChunkCoord`;
* `SectorCoord` remains `SectorCoord`.

Rotation does not change the coordinate type —
it only changes its orientation inside the level.

---

## **3.3. Local float coordinates inside a container**

Besides discrete indices,
systems may work with a continuous local coordinate inside a container.

General form:

```text
P_local = (x_f, y_f, z_f),   x_f, y_f, z_f ∈ ℝ
```

With a range consistent with container dimensions:

```text
0 ≤ x_f < N
0 ≤ y_f < H
0 ≤ z_f < N
```

For float coordinates in the active MVP,
the canonical model is **centered rotation around the local center of the container**.

---

### **3.3.1. Centered local rotation**

Let the container center be:

```text
C = (cx, cy, cz)
```

For a container of size `N × H × N` in the active MVP,
it is enough to use:

```text
cx = N / 2
cz = N / 2
```

and along the `Y` axis rotation changes nothing,
so `cy` may be omitted from the yaw rotation formula.

Then:

1. Move into centered space:

```text
p_centered = (x_f - cx, y_f, z_f - cz)
```

2. Apply canonical `rot_world` to the `(x, y, z)` components:

```text
p_centered' = rot_world(p_centered, θ)
```

3. Move back into the local container:

```text
p_local' = (p_centered'.x + cx, p_centered'.y, p_centered'.z + cz)
```

This is the canonical float rotation inside a container.

---

### **3.3.2. Why float rotation is defined through the center**

For discrete indices, formulas with `N - 1` are convenient
because they rotate **grid cells**.

For float coordinates, the active MVP uses a cleaner model:

* rotation is defined as geometric reorientation of a point
  around the container center;
* the canon is built through world-rotation formulas,
  not through a separate ad-hoc table;
* this preserves consistency between:

  * world-space,
  * local float,
  * orientation-state.

---

### **3.3.3. Where this applies**

Local float rotation is needed for:

* anchored local tools,
* blueprint placement,
* content preview,
* local shape transforms,
* future DUN-like containers,
* debug / visualization of local orientation.

It is especially useful when a system wants to rotate **not a cell index**,
but an arbitrary point, vector, or support anchor inside a container.

---

## **3.4. Relationship between the three coordinate layers**

Rotation in the active MVP must be consistent
between three coordinate layers.

### **Layer 1 — WorldXYZ**

```text
(X, Y, Z)
```

### **Layer 2 — LocalFloat**

```text
(x_f, y_f, z_f)
```

### **Layer 3 — Local indices**

```text
(x, y, z)
```

The correct logic is:

* world-space can be rotated directly through `rot_world`;
* local float can be rotated through centered local rotation;
* local indices can be rotated through `rot_local`.

All three variants must be **semantically consistent**:
they describe the same yaw rotation `C4`,
only at different levels of representation.

---

## **3.5. Practical distinction: anchor-space vs content-space**

Already at the coordinate level, two scenarios must be distinguished.

### **1. Anchor-space rotation**

The system rotates:

* world-position,
* runtime-anchor,
* orientation-state of the anchor.

This affects:

* global position,
* possible transition between `Chunk` / `RegionSector`,
* new address after normalization.

---

### **2. Content-space rotation**

The system rotates:

* local indices,
* local float coordinates,
* internal container content.

At the same time:

* the anchor address may not change;
* `DensityKey` or `RuntimePosition` of the anchor may remain the same;
* only the local orientation of what is stored inside changes.

This distinction will be critical later
for address and container rotation.

---

## **3.6. Summary of the coordinate section**

Section 3 defines a unified coordinate language for Rotation in the active MVP:

```text
world-space   → rot_world(X,Y,Z)
local indices → rot_local(x,y,z; N)
local floats  → centered local rotation
```

This gives Rotation a consistent basis for:

* directions,
* address forms,
* container anchors,
* local content,
* future orientation-aware tools.

---

# **4. Rotation of directions and orientation-state**

This section defines
how discrete rotations `R0 / R90 / R180 / R270`
act on:

* discrete directions,
* orientation-state of objects and containers,
* derived compact direction forms.

In the active MVP, this section **does not use `Octant` as a machine-truth layer**.
Old letter labels `A/B/C/D/E/F/G/I`
may exist as human/debug overlay,
but they are not the canonical machine model of Rotation.

---

## **4.1. What counts as direction in the active MVP**

In the active MVP, it is useful to distinguish three direction layers.

### **1. Orientation-state**

This is a discrete orientation state:

```text id="3y0qea"
Orientation ∈ { R0, R90, R180, R270 }
```

It describes
which of the four yaw orientations an object,
container, or template currently has.

It is not a vector and not an address.
It is specifically an **orientation state**.

---

### **2. Dir6**

Minimal axial direction set:

```text id="e5ezva"
+X, -X, +Y, -Y, +Z, -Z
```

It is needed for:

* axial normals,
* face-based logic,
* simple discrete adjacency,
* direction of “where a face / axis is looking”.

---

### **3. DirN / signed vector direction**

Any direction
that can be represented through a signed vector:

```text id="rpe4pm"
D = (dx, dy, dz),   dx, dy, dz ∈ ℤ  or  ℝ
```

This covers:

* Dir26,
* arbitrary local directions,
* vector orientations of templates,
* derived directional helpers.

---

## **4.2. Orientation-state as a separate entity**

Orientation-state is not part of the address key itself
and not part of the machine sector label.

It is a separate state layer.

Examples:

* `RuntimePosition + Orientation`
* `DensityKey + Orientation`
* `FullRoute + Orientation`
* `Anchor + ContentOrientation`

In the active MVP this is especially important,
because:

* address answers **“where?”**
* orientation answers **“in which yaw orientation?”**

These answers are related,
but should not merge into one overloaded entity.

---

## **4.3. Composition and orientation-state update**

If an object already has orientation-state `a`
and an additional rotation `b` is applied,
the result is defined through `C4` composition:

```text
orientation' = a * b
```

where `*` is the operation from section 2.

Examples:

```text
R0   * R90  = R90
R90  * R90  = R180
R180 * R90  = R270
R270 * R90  = R0
```

Properties:

* the state always remains inside `{R0,R90,R180,R270}`;
* the result is fully reversible:
  `orientation * inv(orientation) = R0`;
* orientation-state is convenient to store as a compact enum / id / tag.

---

## **4.4. Dir6 rotation**

Dir6 is the minimal discrete set of axial directions:

```text
PosX, NegX, PosY, NegY, PosZ, NegZ
```

Rotation around the `Y` axis:

* leaves `PosY / NegY` unchanged;
* rotates `PosX / NegX / PosZ / NegZ` according to the `C4` canon.

---

### **4.4.1. Dir6 rotation table**

#### **R0**

```text
PosX → PosX
NegX → NegX
PosY → PosY
NegY → NegY
PosZ → PosZ
NegZ → NegZ
```

#### **R90**

```text
PosX → PosZ
PosZ → NegX
NegX → NegZ
NegZ → PosX

PosY → PosY
NegY → NegY
```

#### **R180**

```text
PosX → NegX
NegX → PosX
PosZ → NegZ
NegZ → PosZ

PosY → PosY
NegY → NegY
```

#### **R270**

```text
PosX → NegZ
NegZ → NegX
NegX → PosZ
PosZ → PosX

PosY → PosY
NegY → NegY
```

---

### **4.4.2. Properties of Dir6 rotation**

For any direction `d ∈ Dir6`:

```text id="4v4a5z"
rot_dir6(rot_dir6(d, θ), inv(θ)) = d
```

Most importantly:

* the Dir6 table must be consistent with `rot_world(X,Y,Z)`;
* if a direction is interpreted as an axial vector,
  its rotation must match the rotation of that vector in world-space.

---

## **4.5. Rotation of arbitrary directions (DirN / vectors)**

For denser direction sets,
there is no need for a separate table for every system.

The general principle is:

1. Represent the direction as a signed vector:

```text
D = (dx, dy, dz)
```

2. Apply canonical `rot_world`:

```text
D' = rot_world(D, θ)
```

3. If needed:

   * either keep the result as a vector;
   * or normalize it back into the required discrete set (`Dir26`, `DirN`, etc.).

This makes Rotation:

* independent from the concrete number of directions;
* compatible with any direction-layers above the active MVP;
* consistent with base world-math.

---

## **4.6. Compact direction IDs and helper representations**

If a subsystem or tool layer needs compact direction labels,
they are allowed as **derived helper forms**.

Examples:

* `Dir6Id`
* `OrientationId`
* `FacingIndex`
* local debug labels

But the canon remains:

* source of truth = `Orientation` or signed/vector direction;
* compact id = derived packing;
* human label = another layer above it.

That is, the active MVP in the Rotation section follows the same rule
as Topology/Routing:

```text
machine truth <-> compact helper <-> human/debug alias
```

But the machine truth remains the architectural source of truth,
not the other way around.

---

## **4.7. Relationship with RegionSector human/debug labels**

In the active MVP, letters `A/B/C/D/E/F/G/I`
live in the human/debug layer of Region sections
and are not the base machine layer of Rotation.

Therefore:

* Rotation does not have to define an `A→D→...` table as its main contract;
* orientation and sector-label are different entities;
* if a tool layer wants to show a direction or sector as a letter,
  this is done through a separate display table.

This is important
so as not to mix:

* orientation-state,
* sim-sector identity,
* human/debug notation.

---

## **4.8. Direction rotation invariants**

For all direction layers and orientation-state, common requirements apply.

### **1. Reversibility**

```text
rot_dir(rot_dir(D, θ), inv(θ)) = D
```

### **2. Consistency with world-math**

Rotation of a symbolic direction
must match rotation of an equivalent vector through `rot_world`.

### **3. Independence from address key**

Direction and orientation-state
must not implicitly replace an address form.

### **4. Stability of machine model**

Human/debug labels may change,
but the canonical machine model of Rotation
remains based on `Orientation`, `Dir6`, and signed/vector directions.

---

## **4.9. Practical result**

In the active MVP, the direction section defines the following hierarchy of meaning:

```text
Orientation = discrete yaw state
Dir6        = minimal axis directions
DirN        = derived vector directions
Labels      = human/debug overlay only
```
---

# **5. Rotation of address forms and containers**

Section 5 describes
how the discrete rotations `R0 / R90 / R180 / R270`
act on active Arden address forms
and on the containers they anchor.

Here Rotation becomes a bridge between:

* **Topology** — the structure of space,
* **Routing** — address forms,
* **Rotation** — reorientation of anchor, address, and content.

In the active MVP, Rotation no longer works with the old global Route
of the form `Region + Octant + Block + Chunk + ...`.
Instead, it must be consistent with the active forms:

```text
RuntimePosition
DensityKey
SimSectorKey
FullRoute
```

---

## **5.1. Main principle: address != orientation**

In the active MVP, the following must be strictly separated:

* **address form** — answers the question:
  `where is the anchor / container / addressable point?`

* **orientation-state** — answers the question:
  `in which yaw orientation is the object, container, or content?`

Therefore:

* the same address can exist in different orientation-states;
* the same orientation-state does not define an address by itself;
* rotation can:

  * change address,
  * change orientation,
  * change content,
  * or change several of these entities together —
    but these operations must not be mixed.

---

## **5.2. Canonical address-form rotation scheme**

Any canonical world rotation of an addressable entity
is defined through the coordinate layer.

General scheme:

```text
Address
→ anchor coordinates
→ rotate around pivot
→ normalize back to address form
```

Where:

* **anchor coordinates** — world-space or local anchor
  that unambiguously represents the address form;
* **pivot** — point or frame
  around which the rotation is performed;
* **normalize** — conversion back into a correct active address form.

This is the general canon:

* do not rotate an address “in a vacuum”;
* first define its anchor;
* then perform rotation;
* then return the result to a valid address form.

---

## **5.3. Pivot as a required part of world rotation**

For address forms, world rotation without a pivot is ambiguous.

Therefore, the active MVP requires
rotation of address forms to always be interpreted
relative to one of the explicitly defined pivot modes.

Typical variants:

### **1. World-origin pivot**

```text id="jjpffm"
pivot = (0, 0, 0)
```

Used as a pure mathematical canon
or for global symmetries.

### **2. Region-local pivot**

Rotation is performed relative to the center or origin of a specific Region.

This is useful for:

* local world transforms,
* editor actions,
* bounded tools.

### **3. Container pivot**

Rotation is performed relative to the anchor of a specific container:

* center_of_chunk
* center_of_octo
* center_of_voxel
* corner_of_container

### **4. Explicit pivot**

The system itself passes an explicit rotation point:

```text 
pivot = (Px, Py, Pz)
```

This is the most general mode.

---

## **5.4. Rotation of RuntimePosition**

`RuntimePosition` is a live-position:

```text 
RuntimePosition = RegionCoord + LocalFloat
```

Rotation here can mean two different operations.

---

### **5.4.1. World rotation of runtime-position**

If a rotating object actually changes position in world-space,
the canonical scheme is:

1. `RuntimePosition → WorldXYZ`
2. `WorldXYZ → rotate around pivot`
3. `WorldXYZ' → RuntimePosition'`

Notation:

```text 
rotate_runtime_world(runtime, θ, pivot) -> RuntimePosition'
```

Properties:

* result — new live-position;
* new `DensityKey` and new `SimSectorKey`
  are computed **from the result** if needed;
* orientation-state may change together with position,
  but this is a separate system decision.

---

### **5.4.2. Orientation-state rotation with fixed position**

Another case is also allowed:

* address and runtime-position remain the same;
* only the yaw-state of the object changes.

Notation:

```text 
rotate_runtime_orientation(runtime, orientation, θ)
    -> (runtime, orientation')
```

This is important for:

* camera,
* oriented object state,
* preview placement,
* anchored runtime nodes.

---

## **5.5. Rotation of DensityKey**

`DensityKey` is a chunk-level address:

```text 
DensityKey = Region + Chunk
```

It addresses a **density container**,
not orientation by itself.

---

### **5.5.1. What “rotate DensityKey” means**

The correct interpretation is:

* first choose the container anchor
  such as `corner_of_chunk` or `center_of_chunk`;
* convert the anchor into coordinates;
* rotate the anchor around pivot;
* normalize coordinates back into a new `DensityKey`.

Notation:

```text 
rotate_density_world(key, θ, pivot, anchor_mode) -> DensityKey'
```

---

### **5.5.2. What Rotation should not do with DensityKey**

Rotation should not implicitly turn `DensityKey`
into “address plus orientation in one object”.

That is, in the active MVP the recommended model is:

```text id="x9jbnp"
DensityKey + Orientation
```

not:

```text 
RotatedDensityKeyAsSpecialType
```

Otherwise, the same chunk-address
starts carrying several meanings at once:
as an address, as orientation-state, and as a container transform.

---

### **5.5.3. Content rotation with fixed DensityKey**

A frequent scenario:

* `DensityKey` remains the same;
* Chunk content is rotated locally;
* template orientation-state changes;
* anchor address does not change.

Notation:

```text 
rotate_chunk_content_local(key, θ) -> same key + rotated local content
```

This is the canonical way to rotate
a local density-pattern without moving the container.

---

## **5.6. Rotation of SimSectorKey**

`SimSectorKey` is a coarse sim-overlay address:

```text id="sfxmkr"
SimSectorKey = Region + SectorCoord
```

It defines a sim bucket,
not orientation-state.

---

### **5.6.1. Correct logic for rotating SimSectorKey**

In the active MVP, sim-sector **usually is not rotated as an independent entity for rotation itself**.

The correct logic is:

1. there is an object runtime-position;
2. position and/or orientation changes;
3. `SimSectorKey` is recalculated from the new runtime-position.

That is:

```text id="qugshq"
RuntimePosition' -> SimSectorKey'
```

not:

```text id="yy53tr"
SimSectorKey -> magically rotated SimSectorKey
```

---

### **5.6.2. When direct SimSectorKey rotation is still allowed**

Direct rotation is allowed
if a system explicitly works with coarse sector geometry,
for example:

* sector debug tool,
* region-local sim visualizer,
* bounded orchestration experiment.

Then the scheme is the same:

1. `SimSectorKey → sector bounds / anchor`
2. rotate around pivot
3. normalize back to sector coordinates

But this must explicitly be treated as a
**geometric transform of a coarse volume**,
not as a required everyday sim-layer operation.

---

## **5.7. Rotation of FullRoute**

`FullRoute` is a deep discrete address:

```text id="35g66j"
FullRoute = Region + Chunk + Octochunk + Voxel
```

It is the most precise address form of the active MVP.

---

### **5.7.1. Canonical world rotation of FullRoute**

Basic definition:

1. `FullRoute → world anchor`
2. `world anchor → rotate around pivot`
3. `world coordinates' → normalize back to FullRoute`

Notation:

```text id="kulw4c"
rotate_full_route_world(full, θ, pivot, anchor_mode) -> FullRoute'
```

This is the active equivalent of the old formula
“route → xyz → rotate → route”,
but now for the new `FullRoute`.

---

### **5.7.2. FullRoute as a point address and as a content address**

For `FullRoute`, it is especially important to distinguish two modes.

#### **Mode A — rotate address**

The addressed world point itself changes.

Result:

* new `FullRoute'`,
* possibly — new `DensityKey`,
* possibly — new `SimSectorKey`.

#### **Mode B — rotate local content under fixed anchor**

`FullRoute` or container anchor remains the same,
only local content orientation changes.

These are two different actions,
and the document must keep them separate.

---

## **5.8. Rotation of containers**

Rotation for a container must always be understood in two layers.

---

### **5.8.1. Anchor rotation**

Container as a world anchor:

* has position/address;
* this address can be transformed by rotation.

Examples:

* `RuntimePosition`
* `DensityKey`
* `FullRoute`

This is rotation of **where the container is located**.

---

### **5.8.2. Content rotation**

Container as a carrier of local content:

* has local indices,
* local float coordinates,
* orientation-state of content.

This is rotation of **what rotated inside the container**.

---

### **5.8.3. Combined rotation**

Both operations can be combined:

1. rotate local content;
2. rotate container anchor;
3. update orientation-state;
4. normalize address forms if needed.

This is especially important for:

* blueprint placement,
* copy/paste with orientation,
* future DUN-like local worlds,
* anchored rotated chunks.

---

## **5.9. Registering orientation-state**

To prevent rotation from dissolving into address forms,
the active MVP recommends registering orientation-state separately.

Typical model:

```text id="5cdiu7"
AnchorAddress + Orientation + LocalContent
```

Examples:

```text id="nv8olz"
RuntimePosition + Orientation
DensityKey      + Orientation
FullRoute       + Orientation
```

At the same time:

* address remains address;
* orientation remains orientation;
* local content can rotate independently;
* human/debug alias can be built above this model.

This makes registration cleaner
and does not force the address key to store extra semantic load.

---

## **5.10. Address-form rotation invariants**

For all address-layer operations in this section, common requirements apply.

### **1. Validity after normalization**

Any rotation result must return
to a correct active address form.

### **2. Consistency with Topology**

Rotation does not change:

* container level,
* container size,
* active hierarchy,
* valid index ranges.

### **3. Consistency with Routing**

Rotation must not replace:

* `RuntimePosition`,
* `DensityKey`,
* `SimSectorKey`,
* `FullRoute`

with each other without an explicitly specified conversion.

### **4. Separation of address and orientation**

Rotation may change both address and orientation,
but they must remain different model entities.

### **5. Reversibility**

If the same pivot and the same anchor_mode are used,
the following should hold:

```text id="n8zyw4"
rotate(rotate(A, θ, pivot), inv(θ), pivot) = A
```

for a correctly defined address form `A`.

---

## **5.11. Practical result**

In the active MVP, the address/container rotation section defines this canon:

```text id="5mu6wo"
RuntimePosition = live anchor
DensityKey      = density anchor
SimSectorKey    = derived sim bucket
FullRoute       = deep discrete address

anchor rotation != content rotation
address != orientation
sim key is usually derived from rotated position
```

This is the correct foundation
for further building
rotating blueprints, registration logic,
anchored content transforms,
and future DUN-like orientation workflows.

---

# **6. Summary of the Rotation section**

The Rotation section defines a **single and minimal reorientation contract**
for the active MVP of Arden.

It does not change Topology and does not replace Routing.
It describes **how already existing space, already existing addresses,
and already existing content can be rotated
without breaking their structure and meaning**.
Rotation defines the reorientation canon for the active MVP of Arden.

It describes
how coordinates, directions, address forms, and local content
can be rotated discretely and reversibly,
without breaking Topology and Routing.

In the active MVP, Rotation defines only yaw rotations around the Y axis:

R0 / R90 / R180 / R270

That is, Rotation is responsible not for the structure of the world
and not for addressing as such,
but for correct reorientation of already existing space and addresses.

This opens a foundation for:

* rotating blueprint and pattern insertions;
* rotation-aware tools and debug representations;
* separation of anchor rotation and content rotation;
* orientation-state as a separate state layer;
* future DUN-like containers and richer orientation/symmetry extensions.

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  
