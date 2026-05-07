**Document Status:** 🔬 Review  
**Version:** 0.2.0 
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16

---
[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  


# **1.0. Overview / mini-README for the topology section**

This section defines the **active MVP world topology of Arden** — the strict language
used to describe engine space at the current stage.

The section defines:

* the active hierarchy of density containers:
  `Region → Chunk → Octochunk → Voxel`;
* a separate simulation subdivision of a Region into 8 equal sections:
  `2 × 2 × 2`;
* the classic axis order:
  `X | Y | Z`;
* the relationship between world coordinates, coordinates inside a Region,
  and local container indices;
* the basic rules of strides, indexing, and transformations.

This section **does not describe** routing, gameplay, DTO/HAOS as a behavior system,
or specific runtime algorithms.
It only defines the **pure mathematics of the active MVP topology**.

---

## 1.0.1. What topology defines

Section 1.x forms:

* **A unified spatial language**
  * active container hierarchy:
    `Region → Chunk → Octochunk → Voxel`;
  * separate overlay layer of Region simulation sections:
    `RegionSector = 2 × 2 × 2`;
  * fixed nesting and divisibility rules between levels.

* **Coordinate model**
  * canonical axis order `X | Y | Z`;
  * separation into:
    * world coordinates (`WorldXYZ`, signed / float),
    * local coordinates inside a Region,
    * local container indices (`Chunk / Octochunk / Voxel`);
  * bridge:
    `WorldXYZ ↔ RegionCoord + LocalFloat ↔ local indices`.

* **Addressing and indexing**
  * hierarchical indices of active levels;
  * strides and offsets;
  * flat index;
  * Morton order for spatial structures.

This is enough to:

* unambiguously address any active container or voxel,
* recover local indices from a position inside a Region,
* use one shared mathematical model in density, simulation, and tooling layers.

---

## 1.0.2. Structure of section 1.x by blocks

**1.1. Active topology levels**  
Defines the active MVP hierarchy:

> `Region → Chunk → Octochunk → Voxel`

and separately defines `RegionSector` as a simulation overlay layer,
not as part of the density chain.

---

**1.2. Dimensions and constants**  
Defines:

* sizes of active levels,
* derived strides,
* Region size as a cube,
* Region sections `2 × 2 × 2`,
* allowed density scaling for Chunk / Octochunk.

---

**1.3. Canonical axis system**  
Defines:

* axis order `X | Y | Z`;
* no negative local indices;
* separation between world coordinates and local indices;
* RegionSector as a uniform 3D subdivision of the Region cube.

---

**1.4. Coordinate types and the bridge between them**  
Separates two layers:

* **Type A** — world coordinates (`WorldXYZ`) for movement, physics, navigation, cameras;
* **Type B** — local topology indices (`Chunk / Octochunk / Voxel`) for storage and density.

Describes the model:

> `RegionCoord + LocalFloat`

as the main bridge between runtime space and topology.

---

**1.5. Hierarchical indices and offset structure**  
Formalizes:

* active level indices;
* Region simulation section indices;
* index ranges;
* the rule `offset = index * stride`;
* hierarchical offset summation.

---

**1.6. Topology transformation formulas**  
Defines direct and inverse formulas:

* from `Chunk / Octochunk / Voxel` to a local coordinate inside a Region;
* from a local coordinate inside a Region to active level indices.

---

**1.7. Flat Indexing**  
Shows how local coordinates of a level `(x, y, z)` are converted into one number `index`
for linear memory, and back.

---

**1.8. Morton order (Z-order)**  
Introduces alternative addressing `3D → 1D` through bit interleaving
of `x / y / z` coordinates, suitable for spatial structures and future LOD/SVO layers.

---

## 1.0.3. Legacy level status

For the active MVP:

* **Block has been removed from the active addressing strategy**;
* Block is not a required level of the current topology;
* old materials where Block appears as part of the main path
  are considered a legacy layer and do not describe the active MVP canon.

This means:

* the history of Block can be preserved in archived documents;
* but active MVP topology no longer has to pass through `Block`
  in formulas, indices, or Region sizes.

---

# **1.1. Active topology levels**

Below is the fixed set of logical containers
that form the active MVP structure of Arden 3D topology.

---

## **1.1.1. List of active levels**

| Level          | Purpose |
| -------------- | ------- |
| **Region**     | Root cubic world container. |
| **Chunk**      | Main container of dense data. |
| **Octochunk**  | Internal structural sub-container of a Chunk. |
| **Voxel**      | Atomic spatial unit. |

---

## **1.1.2. Simulation overlay level**

In addition to the density hierarchy, each Region has a separate subdivision:

> `RegionSector = 2 × 2 × 2`

`RegionSector`:

* is not part of the density chain;
* does not replace `Chunk`;
* is used as a coarse simulation / orchestration overlay;
* can be represented as:
  * `SectorCoord = (sx, sy, sz)`, where each coordinate ∈ `{0, 1}`,
  * or as `sector_id ∈ [0..7]` in a stable project-defined mapping.

In the active MVP:

* `Chunk` = density unit
* `RegionSector` = simulation unit

---

## **1.1.3. Properties of the active hierarchy**

The nesting of the active density chain is fixed:

```text
Region → Chunk → Octochunk → Voxel
```

At the same time:

* `RegionSector` exists **in parallel** as an overlay subdivision of a Region;
* `Octochunk` remains part of the internal Chunk structure;
* `Voxel` remains the smallest addressable density unit;
* application systems may skip levels,
  but this does not change the topology itself.

---

## **1.1.4. Logical roles of levels**

* **Region** — base cubic world container.
* **Chunk** — main carrier of density, voxel payload, and local geometry.
* **Octochunk** — internal subdivision of a Chunk into 8 equal parts.
* **Voxel** — smallest spatial unit.
* **RegionSector** — coarse overlay cell for simulation and orchestration inside a Region.

---

# **1.2. Dimensions and constants**

This section defines the sizes of active topology levels
and the derived constants used by all subsequent mathematics.

---

## **1.2.1. Active constants**

Below are the base constants of the active MVP.

```text
VOXEL_SIZE              = 1
CHUNK_SIZE              = project density constant
OCTO_SPLIT_PER_AXIS     = 2
OCTO_SIZE               = CHUNK_SIZE / 2

REGION_CHUNKS_PER_AXIS  = project topology constant
REGION_SIZE             = REGION_CHUNKS_PER_AXIS * CHUNK_SIZE

REGION_SECTOR_SPLIT     = 2
REGION_SECTOR_SIZE      = REGION_SIZE / 2
```

Invariants:

* `Region` is always cubic:
  `REGION_SIZE_X = REGION_SIZE_Y = REGION_SIZE_Z = REGION_SIZE`;
* `Octochunk` always splits a Chunk in half along each axis;
* `RegionSector` always splits a Region in half along each axis;
* `REGION_CHUNKS_PER_AXIS` must be even
  so that RegionSector divides the Region without remainder.

---

## **1.2.2. Canonical sizes of levels**

Sizes are given in units of the directly lower level.

| Level            | Dimension                                                                         |
| ---------------- | --------------------------------------------------------------------------------- |
| **Region**       | `REGION_CHUNKS_PER_AXIS × REGION_CHUNKS_PER_AXIS × REGION_CHUNKS_PER_AXIS Chunks` |
| **Chunk**        | `2 × 2 × 2 Octochunks`                                                            |
| **Octochunk**    | `OCTO_SIZE × OCTO_SIZE × OCTO_SIZE vox`                                           |
| **Voxel**        | `1 × 1 × 1`                                                                       |
| **RegionSector** | `1/8 Region` using the `2 × 2 × 2` scheme                                         |

---

## **1.2.3. Active density profile**

For the active MVP, one working density profile may be fixed,
for example:

```text
CHUNK_SIZE = 64
OCTO_SIZE  = 32
```

At the same time, this document does not have to hardcode the Region size in voxels
until the value of `REGION_CHUNKS_PER_AXIS` is separately approved.

Important:

* changing `CHUNK_SIZE` changes container density;
* changing `REGION_CHUNKS_PER_AXIS` changes the size of the Region cube;
* but the active hierarchy itself remains the same:

```text
Region → Chunk → Octochunk → Voxel
```

---

## **1.2.4. Allowed scaling**

Chunk / Octochunk density may be scaled:

* `Chunk = 32³ → Octochunk = 16³`
* `Chunk = 64³ → Octochunk = 32³`
* `Chunk = 128³ → Octochunk = 64³`

At the same time:

* Region remains a cube;
* RegionSector remains a `2 × 2 × 2` subdivision;
* the active hierarchy does not change;
* formulas remain linear — only constants and strides change.

---

## **1.2.5. Base strides**

For later sections, the following base strides are fixed:

| Stride            | Value                |
| ----------------- | -------------------- |
| **VOXEL_STRIDE**  | `1`                  |
| **OCTO_STRIDE**   | `OCTO_SIZE`          |
| **CHUNK_STRIDE**  | `CHUNK_SIZE`         |
| **REGION_STRIDE** | `REGION_SIZE`        |
| **SECTOR_STRIDE** | `REGION_SECTOR_SIZE` |

These strides define:

* ranges of local coordinates;
* offsets during transitions between levels;
* direct and inverse transformations;
* flat / Morton indexing.

---

# **1.3. Canonical axis system**

This section defines the base coordinate system of the active MVP topology.

---

## **1.3.1. Axis order**

Topology uses the classic order:

```text
X | Y | Z
```

Where:

* **X** — left/right horizontal axis;
* **Y** — vertical axis;
* **Z** — depth / longitudinal axis.

This is the canonical order for the active MVP.

---

## **1.3.2. Why XYZ**

Reasons for choosing `XYZ`:

* it is the classic and expected order for most participants;
* it lowers the entry threshold;
* it reduces the chance of confusion in formulas, field names, and debugging;
* it allows documentation, code, and the human/debug layer to be aligned in one format.

---

## **1.3.3. Local coordinates are always non-negative**

All local coordinates inside active topology containers are non-negative:

```text
x_local ≥ 0
y_local ≥ 0
z_local ≥ 0
```

The same applies to local level indices:

```text
cx, cy, cz ≥ 0
ox, oy, oz ≥ 0
vx, vy, vz ≥ 0
sx, sy, sz ≥ 0
```

World coordinates may be signed,
but local container indices always remain non-negative.

---

## **1.3.4. Region as a cube**

In the active MVP, `Region` is defined as a cube.

This means:

```text
REGION_SIZE_X = REGION_SIZE_Y = REGION_SIZE_Z
```

Consequences:

* mathematics becomes simpler;
* formulas become symmetric across axes;
* Region naturally splits in half along each axis;
* RegionSector `2 × 2 × 2` becomes a direct and uniform subdivision;
* future trees and symmetric spatial schemes do not receive artificial vertical asymmetry.

---

## **1.3.5. Region simulation sections**

Each Region is divided into 8 equal sections:

```text
2 × 2 × 2
```

These sections:

* are not a replacement for Chunk;
* are not part of the density path;
* are used as a coarse spatial overlay inside the Region cube.

Canonical machine form:

```text
SectorCoord = (sx, sy, sz),  sx, sy, sz ∈ {0,1}
```

Packed form:

```text
sector_id ∈ [0..7]
```

Human/debug labels (`A/B/C/D/E/F/G/I`) may exist on top of this model,
but they are not the topology machine truth.

---

## **1.3.6. Human/debug layer of sections**

In the active MVP:

* primary machine truth = `SectorCoord`;
* `sector_id` is allowed as a packed/helper representation;
* human/debug layer = section letters, compact labels, visual markers.

Therefore:

* section letters must not replace the canonical machine form;
* packed/helper form must not replace the primary topology truth;
* the human layer is built on top of topology, not the other way around.

---

# **1.4. Coordinate types and the bridge between them**

Topology separates two coordinate layers:

---

## **1.4.1. Type A — world coordinates (WorldXYZ)**

World coordinates describe the position of an object in world space
and are used by physics, movement, navigation, cameras, and tools.

They may be:

* signed integer,
* float.

In general form:

```text
WorldPos = (X, Y, Z)
```

Where:

```text
X, Y, Z ∈ ℝ   or   ℤ
```

---

## **1.4.2. Type B — local topology indices**

Topology works with local container indices:

```text
ChunkCoord     = (cx, cy, cz)
OctochunkCoord = (ox, oy, oz)
VoxelCoord     = (vx, vy, vz)
SectorCoord    = (sx, sy, sz)
```

All these indices are:

* integer,
* non-negative,
* limited by the size ranges of their corresponding level.

---

## **1.4.3. Main bridge of the active MVP**

The base bridge between world-space and topology in the active MVP:

```text
RegionCoord + LocalFloat
```

Where:

```text
RegionCoord = (rx, ry, rz)
LocalFloat  = (fx, fy, fz)
```

With constraints:

```text
fx, fy, fz ∈ [0 .. REGION_SIZE)
```

That is:

* `RegionCoord` tells which Region cube the object is in;
* `LocalFloat` defines a continuous position inside that cube.

This is the base runtime-space for the active MVP.

---

## **1.4.4. Relationship with WorldXYZ**

Given a world position:

```text
(X, Y, Z)
```

Then:

```text
rx = floor(X / REGION_SIZE)
ry = floor(Y / REGION_SIZE)
rz = floor(Z / REGION_SIZE)

fx = X - rx * REGION_SIZE
fy = Y - ry * REGION_SIZE
fz = Z - rz * REGION_SIZE
```

This produces:

```text
WorldXYZ → RegionCoord + LocalFloat
```

Inverse transformation:

```text
X = rx * REGION_SIZE + fx
Y = ry * REGION_SIZE + fy
Z = rz * REGION_SIZE + fz
```

---

## **1.4.5. Relationship with local indices**

From `LocalFloat`, local indices of active levels can be recovered at any time:

```text
chunk_x = floor(fx / CHUNK_SIZE)
chunk_y = floor(fy / CHUNK_SIZE)
chunk_z = floor(fz / CHUNK_SIZE)
```

The remainder inside the Chunk is then decomposed into `OctochunkCoord` and `VoxelCoord`
using the usual strides.

Thus:

* runtime lives on `Region + LocalFloat`;
* density topology is recovered on demand;
* topology does not have to constantly store the full discrete path for every object.

---

## **1.4.6. Separation between sim-sector and density address**

From the same `LocalFloat`, `RegionSector` can also be computed:

```text
sx = floor(fx / REGION_SECTOR_SIZE)
sy = floor(fy / REGION_SECTOR_SIZE)
sz = floor(fz / REGION_SECTOR_SIZE)
```

With subsequent clamp / range guarantee:

```text
sx, sy, sz ∈ {0,1}
```

This means:

* the same runtime position inside a Region
  can be decomposed
  into both `ChunkCoord`
  and `SectorCoord`;
* `Chunk` and `RegionSector` describe **different tasks**,
  not competing addresses of the same level.

---

# **1.5. Hierarchical indices and offset structure**

Hierarchical indices are numeric coordinates inside each active topology level.

---

## **1.5.1. Level indices**

| Level            | Indices                                         |
| ---------------- | ----------------------------------------------- |
| **Region**       | `(rx, ry, rz)` — Region indices in the world    |
| **Chunk**        | `(cx, cy, cz)` — Chunk indices inside a Region  |
| **Octochunk**    | `(ox, oy, oz)` — Octochunk indices inside Chunk |
| **Voxel**        | `(vx, vy, vz)` — Voxel indices inside Octochunk |
| **RegionSector** | `(sx, sy, sz)` — section indices inside Region  |

---

## **1.5.2. Index ranges**

For the active MVP, ranges are defined as:

```text
cx, cy, cz ∈ [0 .. REGION_CHUNKS_PER_AXIS - 1]

ox, oy, oz ∈ [0 .. 1]

vx, vy, vz ∈ [0 .. OCTO_SIZE - 1]

sx, sy, sz ∈ [0 .. 1]

fx, fy, fz ∈ [0 .. REGION_SIZE)
```

Here:

* `ChunkCoord` covers the entire Region volume;
* `OctochunkCoord` is always 2×2×2;
* `VoxelCoord` covers the volume of one Octochunk;
* `SectorCoord` covers the coarse simulation overlay of a Region.

---

## **1.5.3. “index → stride → offset” rule**

Each level defines its own stride.

General rule:

```text
level offset = level index * level stride
```

Examples:

```text
chunk_offset_x = cx * CHUNK_SIZE
octo_offset_x  = ox * OCTO_SIZE
voxel_offset_x = vx * VOXEL_SIZE

sector_offset_x = sx * REGION_SECTOR_SIZE
region_offset_x = rx * REGION_SIZE
```

The same applies to axes `Y` and `Z`.

---

## **1.5.4. Hierarchical offset summation**

The absolute local coordinate inside a Region for the density path is obtained as a sum:

```text
local_x = cx * CHUNK_SIZE + ox * OCTO_SIZE + vx
local_y = cy * CHUNK_SIZE + oy * OCTO_SIZE + vy
local_z = cz * CHUNK_SIZE + oz * OCTO_SIZE + vz
```

For the simulation overlay:

```text
sector_local_x = sx * REGION_SECTOR_SIZE
sector_local_y = sy * REGION_SECTOR_SIZE
sector_local_z = sz * REGION_SECTOR_SIZE
```

That is:

* density and simulation overlay use the same Region geometry;
* but different strides and different index types.

---

## **1.5.5. Role of indices in the active MVP**

Indices provide:

* bijectivity of a local address;
* O(1) transformations;
* independence of runtime position from the full discrete path;
* compatibility with flat index and Morton;
* a strict foundation for:

  * density storage,
  * mesh build,
  * compression / decompression,
  * sim-sector orchestration,
  * tooling and debug.

---

# **1.6. Topology transformation formulas**

This section defines strict mathematical rules for transformation between:

* local indices of active topology
* and absolute local coordinates inside a Region

The formulas are based on the active MVP hierarchy:

```text
Region → Chunk → Octochunk → Voxel
```

and the overlay subdivision:

```text
RegionSector = 2 × 2 × 2
```

All transformations are:

* deterministic,
* linear,
* O(1),
* fully reversible,
* do not require constant storage of FullRoute in runtime.

---

# **1.6.1. Linear offsets and strides**

Each level defines its own index and its own stride.

General rule:

```text
level offset = level index * level stride
```

For the active MVP:

| Level            | Indices      | Stride along X / Y / Z |
| ---------------- | ------------ | ---------------------- |
| **Region**       | `rx, ry, rz` | `REGION_SIZE`          |
| **Chunk**        | `cx, cy, cz` | `CHUNK_SIZE`           |
| **Octochunk**    | `ox, oy, oz` | `OCTO_SIZE`            |
| **Voxel**        | `vx, vy, vz` | `VOXEL_SIZE = 1`       |
| **RegionSector** | `sx, sy, sz` | `REGION_SECTOR_SIZE`   |

In active topology, we care about two different decompositions of the same Region volume:

1. **density decomposition**
   `Chunk → Octochunk → Voxel`

2. **simulation overlay**
   `RegionSector`

---

# **1.6.2. Direct density address transformation downward**

### *Chunk / Octochunk / Voxel → absolute local coordinates inside Region*

Given the index structure:

```text
Chunk:     (cx, cy, cz)
Octochunk: (ox, oy, oz)
Voxel:     (vx, vy, vz)
```

The absolute local coordinates inside a Region are calculated as:

```text
X_local =
    cx * CHUNK_SIZE +
    ox * OCTO_SIZE  +
    vx

Y_local =
    cy * CHUNK_SIZE +
    oy * OCTO_SIZE  +
    vy

Z_local =
    cz * CHUNK_SIZE +
    oz * OCTO_SIZE  +
    vz
```

Properties:

* each coordinate is the sum of three independent offsets;
* formulas are fully linear;
* each level contributes according to its stride.

Constraints:

```text
0 ≤ X_local < REGION_SIZE
0 ≤ Y_local < REGION_SIZE
0 ≤ Z_local < REGION_SIZE
```

---

# **1.6.3. Inverse density address transformation upward**

### *Absolute local coordinates inside Region → Chunk / Octochunk / Voxel*

Given:

```text
X_local, Y_local, Z_local
```

Need to recover:

```text
cx, cy, cz
ox, oy, oz
vx, vy, vz
```

General rule:

```text
index = value // SIZE
value = value % SIZE
```

---

### **1) Chunk**

```text
cx = X_local // CHUNK_SIZE
cy = Y_local // CHUNK_SIZE
cz = Z_local // CHUNK_SIZE

X1 = X_local % CHUNK_SIZE
Y1 = Y_local % CHUNK_SIZE
Z1 = Z_local % CHUNK_SIZE
```

---

### **2) Octochunk**

```text
ox = X1 // OCTO_SIZE
oy = Y1 // OCTO_SIZE
oz = Z1 // OCTO_SIZE

X2 = X1 % OCTO_SIZE
Y2 = Y1 % OCTO_SIZE
Z2 = Z1 % OCTO_SIZE
```

---

### **3) Voxel**

```text
vx = X2
vy = Y2
vz = Z2
```

This is enough to recover the full density address of the active topology
from any point inside a Region.

---

# **1.6.4. Simulation overlay transformation**

### *Local coordinate inside Region ↔ RegionSector*

Region overlay sections are computed independently from the density chain.

Given a local coordinate inside a Region:

```text
X_local, Y_local, Z_local
```

The sector coordinate is:

```text
sx = X_local // REGION_SECTOR_SIZE
sy = Y_local // REGION_SECTOR_SIZE
sz = Z_local // REGION_SECTOR_SIZE
```

With correct Region configuration:

```text
sx, sy, sz ∈ {0, 1}
```

The inverse transformation gives the base corner of the sector inside the Region:

```text
SectorBaseX = sx * REGION_SECTOR_SIZE
SectorBaseY = sy * REGION_SECTOR_SIZE
SectorBaseZ = sz * REGION_SECTOR_SIZE
```

Important:

* `RegionSector` does not replace `Chunk`;
* `RegionSector` and `Chunk` are two different decompositions of the same volume;
* the first serves coarse simulation / orchestration,
  the second serves density and geometry.

---

# **1.6.5. Relationship with world coordinates**

A full world coordinate is built on top of `RegionCoord`:

```text
X_world = rx * REGION_SIZE + X_local
Y_world = ry * REGION_SIZE + Y_local
Z_world = rz * REGION_SIZE + Z_local
```

Inverse transformation:

```text
rx = floor(X_world / REGION_SIZE)
ry = floor(Y_world / REGION_SIZE)
rz = floor(Z_world / REGION_SIZE)

X_local = X_world - rx * REGION_SIZE
Y_local = Y_world - ry * REGION_SIZE
Z_local = Z_world - rz * REGION_SIZE
```

Thus, active topology uses a simple two-step scheme:

```text
WorldXYZ
→ RegionCoord + LocalPositionInRegion
→ Chunk / Octochunk / Voxel
```

The simulation overlay is extracted from the same `LocalPositionInRegion` separately.

---

# **1.6.6. Properties of direct and inverse transformations**

### ✔ **1. Bijectivity**

Each correct index set
`(Chunk, Octochunk, Voxel)`
corresponds to exactly one local point inside a Region.

### ✔ **2. Reversibility**

For any correct values:

```text
to_indices(to_local(p)) = p
```

and

```text
to_local(to_indices(X_local, Y_local, Z_local)) = (X_local, Y_local, Z_local)
```

### ✔ **3. Linearity**

All formulas:

* have no branching,
* have no signed logic inside local topology,
* are fully O(1).

### ✔ **4. Scalability**

If `CHUNK_SIZE` and `OCTO_SIZE` are changed,
the formulas remain the same —
only constants change.

### ✔ **5. Separation of roles**

The same local point inside a Region
may simultaneously be represented as:

* density address (`Chunk / Octochunk / Voxel`),
* simulation address (`RegionSector`).

This is not a conflict, but a normal coexistence of two address slices.

---

# **1.6.7. Summary**

Section 1.6 formed the mathematical basis of the active MVP topology:

* strict linear formulas,
* full reversibility,
* separation of density and simulation overlay decompositions,
* compatibility with `Region + LocalFloat`,
* readiness for flat index and Morton code.

---

# **1.7. Flat Indexing**

A flat index maps a triple of local coordinates at a level
into a single integer representing an element position in a linear array.

Flat index provides:

* strict O(1) transformation between 3D coordinates and linear memory,
* deterministic storage structure,
* compatibility with CPU/GPU buffers,
* direct work with arrays, tables, and spatial structures.

---

# **1.7.1. Linear memory model**

Any active topology level
can be represented as a linear memory segment
if its axis sizes are known and fixed.

Total array length:

```text
len = size_x * size_y * size_z
```

This applies to:

* **Voxel** inside Octochunk,
* **Octochunk** inside Chunk,
* **Chunk** inside Region,
* **RegionSector** inside Region.

---

# **1.7.2. Transformation formula (x, y, z → index)**

For canonical order, the formula is:

```text
flat_index =
    x +
    size_x * (
        y +
        size_y * z
    )
```

Where:

* `x, y, z` — local coordinates of the level,
* `size_x, size_y, size_z` — dimensions of the level.

Meaning of the order:

* **X** — fastest step,
* **Y** — middle step,
* **Z** — slowest step.

---

# **1.7.3. Inverse transformation (index → x, y, z)**

Inverse operation:

```text
x = index % size_x
t = index / size_x

y = t % size_y
z = t / size_y
```

Guarantees:

* full reversibility;
* no loss of information;
* O(1) computation;
* strict correspondence of the triple `(x, y, z)` to the original array element.

---

# **1.7.4. Flat index for active MVP levels**

### **Voxel inside Octochunk**

If `Octochunk = OCTO_SIZE³`, then:

```text
voxel_index =
    vx +
    OCTO_SIZE * (
        vy +
        OCTO_SIZE * vz
    )
```

---

### **Octochunk inside Chunk, 2×2×2**

```text
octo_index =
    ox +
    2 * (
        oy +
        2 * oz
    )
```

---

### **Chunk inside Region**

```text
chunk_index =
    cx +
    REGION_CHUNKS_PER_AXIS * (
        cy +
        REGION_CHUNKS_PER_AXIS * cz
    )
```

---

### **RegionSector inside Region, 2×2×2**

```text
sector_index =
    sx +
    2 * (
        sy +
        2 * sz
    )
```

---

# **1.7.5. Relationship between flat index and hierarchical levels**

Flat index works **inside one specific level**.

Hierarchical addressing from `1.6` defines where the level is located in a larger container,
while flat index defines where an element is located **inside the volume of that level**.

That is:

```text
hierarchical offset  → where the container is
flat index           → where the element is inside the container
```

Both mechanisms complement each other:

* hierarchy provides structural decomposition,
* flat index provides linear memory layout.

---

# **1.7.6. Invariants and constraints**

* flat index uses only non-negative coordinates;
* formulas are reversible for any active MVP level;
* canonical order remains strictly `X | Y | Z`;
* linear memory always has size `size_x * size_y * size_z`;
* reordering axes breaks compatibility with active topology.

---

# **1.7.7. Note about storage profiles**

If the project later needs a different memory order
for cache, GPU, or specialized structures,
this must be described as a **storage profile** or **implementation detail**,
not as a replacement of the canonical topology order.

In other words:

* topology canon = `X | Y | Z`
* storage optimization = separate layer above the canon

---

# **1.8. Morton order (Z-order)**

Morton order is a method of encoding three-dimensional coordinates
into a one-dimensional index by bit-interleaving the coordinates.

In the active MVP, Morton code follows canonical axis order:

```text
X | Y | Z
```

That is, bits are interleaved as:

```text
x0 y0 z0 x1 y1 z1 x2 y2 z2 ...
```

This order:

* preserves spatial locality;
* works well for spatial structures;
* is useful for future LOD/SVO profiles;
* is compatible with local active topology indices.

---

# **1.8.1. Basic idea of Morton code**

Given a local coordinate of a level:

```text
(x, y, z)
```

Morton code is a number where:

* **X** bits occupy positions `0, 3, 6, 9, ...`
* **Y** bits occupy positions `1, 4, 7, 10, ...`
* **Z** bits occupy positions `2, 5, 8, 11, ...`

That is:

```text
bitstream = interleave_bits(x, y, z)
```

Each bit triplet defines a position in one of 8 `2×2×2`
sub-blocks at the corresponding spatial subdivision level.

---

# **1.8.2. Morton encode (3D → 1D)**

Normalize level coordinates:

```text
x, y, z ∈ [0 .. N)
```

Then interleave bits:

```text
morton =
    part_bits(x) |
    (part_bits(y) << 1) |
    (part_bits(z) << 2)
```

Where `part_bits` is a function that spreads the bits of a number
so that two empty positions remain between adjacent bits of the original coordinate.

Example signature:

```rust
fn part_bits(x: u32) -> u32
```

The concrete implementation may differ by language,
but the encode logic must remain reversible.

---

# **1.8.3. Morton decode (1D → 3D)**

For inverse transformation, `compact_bits` is used,
which collects every third bit back.

Algorithmically:

```text
x = compact_bits(morton >> 0)
y = compact_bits(morton >> 1)
z = compact_bits(morton >> 2)
```

Where `compact_bits` is the full inverse of `part_bits`.

Decode properties:

* strictly restores the original coordinates;
* preserves value ranges;
* guarantees determinism.

---

# **1.8.4. Where Morton order is appropriate in the active MVP**

Morton code is useful where spatial locality matters:

### **Voxel inside Octochunk**

Fits naturally,
because `OCTO_SIZE` is usually a power-of-two multiple.

### **Octochunk inside Chunk**

Ideal,
because this is a fixed `2×2×2` subdivision.

### **RegionSector**

Ideal,
because this is also a fixed `2×2×2` subdivision.

### **Chunk inside Region**

Suitable if `REGION_CHUNKS_PER_AXIS` is chosen as a power of two
or if a normalized/padded domain is used for Morton representation.

---

# **1.8.5. Morton order constraints and invariants**

Morton order is correct under the following conditions:

* coordinates are non-negative;
* axis sizes are consistent with the used bit range;
* for “clean” Morton without padding, the level size along each axis should preferably be a power of two;
* canonical bit-interleave order remains strictly `X | Y | Z`.

Fully compatible:

* with local coordinates of the active topology;
* with strides from `1.6`;
* with flat index from `1.7`;
* with Chunk / Octochunk scaling,
  if sizes remain correctly normalizable.

---

# **1.8.6. Summary of the Morton section**

Morton order provides:

* spatial locality;
* compactness;
* a convenient basis for hierarchical spatial structures;
* compatibility with active MVP coordinates;
* natural work with `Octochunk` and `RegionSector` as `2×2×2` subdivisions.

It does not replace flat index,
but complements it:
flat index is needed for linear memory,
Morton is needed for spatially local order.

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)  
