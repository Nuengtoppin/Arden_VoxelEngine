**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16  
**Purpose:** Active MVP routing canon for Arden

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)

# **1.0. Overview / mini-README for the Routing section**

The Routing section defines the **active MVP addressing model of Arden**.

Routing is not spatial mathematics by itself.
It lives **on top of Topology** and uses the already defined:

* containers,
* sizes,
* strides,
* coordinate model,
* transformation formulas.

In the active MVP, Routing is no longer **treated** as a mandatory permanent runtime skeleton of the world.

Its role is now:

> **Routing = address protocol on demand**

That means Routing is not responsible for “how an object lives every millisecond in runtime”,
but for **how systems obtain and pass a discrete address at the required depth**.

---

## **1.0.1. What Routing defines**

Section 1.x defines:

* the difference between:
  * **RuntimePosition**
  * **DensityKey**
  * **SimSectorKey**
  * **FullRoute**

* the rules by which:
  * a runtime position becomes an address,
  * an address becomes local/world coordinates,
  * one system passes an address to another system without losing meaning;

* canonical notation forms:
  * structural,
  * string-based,
  * contextual shortened forms;

* basic operations on addresses:
  * comparison,
  * containment,
  * neighborhood,
  * moving up/down a level,
  * normalization,
  * transformations between addressing forms.

Routing does not introduce a new topology,
but uses the already fixed active MVP canon:

```text
Region → Chunk → Octochunk → Voxel
```

and a separate simulation overlay:

```text
RegionSector = 2 × 2 × 2
```

---

## **1.0.2. Structure of section 1.x**

**1.1. Routing concept and scope**  
Defines Routing as an address protocol layer,
not as permanent runtime state.

**1.2. Canonical address forms**  
Defines four main forms:

* RuntimePosition
* DensityKey
* SimSectorKey
* FullRoute

**1.3. Invariants and validity**  
Defines what counts as a correct address in the active MVP.

**1.4. Relationship between Routing, Topology, and coordinates**  
Shows transitions:

* WorldXYZ ↔ RuntimePosition
* RuntimePosition ↔ DensityKey
* RuntimePosition ↔ SimSectorKey
* RuntimePosition ↔ FullRoute

**1.5. Operations on addresses**  
Comparison, nesting, neighborhood, offset, Up/Down, normalize.

**1.6. String representation**  
Format for logs, inspectors, devtools, and debugging.

**1.7. Mapping / Address API**  
Minimal set of mapping and conversion operations.

**1.8. Using Routing in engine systems**  
How active MVP address forms are used in runtime, density, simulation, tooling, and ECS.

---

## **1.0.3. How to read this section**

* To understand **what Routing means now in the active MVP** — read `1.1–1.2`.
* If **correctness and invariants** matter — read `1.3`.
* If you work with coordinate and address transformations — read `1.4`.
* If you design operations on addresses — read `1.5`.
* If you need logs / inspector / CLI — read `1.6`.
* If you need a working conversion interface — read `1.7`.
* If you need the connection with engine subsystems — read `1.8`.

---

# **1.1. Routing concept and scope**

In the active MVP, Routing is **not one overloaded object**, but a rule layer
by which a system receives **an address at the required depth for its task**.

Main idea:

> Runtime lives on position.
> Routing lives on address.

That is:

* camera,
* player,
* physics,
* navigation,
* rays,
* movement tools

do not have to constantly carry a full route through every level.

Their base live state is:

```text
Region + LocalFloat
```

But when a system needs a discrete address,
Routing builds one of the canonical addressing forms.

---

## **1.1.1. Routing is not Runtime**

In the active MVP, the following must be strictly separated:

### **RuntimePosition**

Live object position:

```text
RegionCoord + LocalFloat
```

This is not a Route.

---

### **Route / Address**

A discrete address form
built from runtime position or another context
when a system needs:

* a container key,
* a voxel address,
* a sim-sector,
* a debug path,
* a serializable reference.

---

## **1.1.2. Why Routing is no longer one “super-object”**

The old approach overloaded one Route with several roles at once:

* runtime state,
* container address,
* deep voxel route,
* simulation key,
* debug string.

In the active MVP, these roles are separated:

* `RuntimePosition` — for live position,
* `DensityKey` — for the density level,
* `SimSectorKey` — for coarse simulation,
* `FullRoute` — for an exact discrete address.

This makes the model simpler,
cleaner for subsystems,
and cheaper to maintain.

---

## **1.1.3. Where Routing is used**

Routing is needed when systems need more than just a float position.

Main scenarios:

* **Density / streaming**

  * which Chunk is needed,
  * which Chunk to update,
  * which Chunk stores density / mesh / compression.

* **Sim / orchestration**

  * which RegionSector the object entered,
  * which coarse sim bucket should wake up or go to sleep.

* **Tools / debug**

  * inspector,
  * address log,
  * point command to the core,
  * deep debug.

* **Serialization**

  * exact or shortened discrete address,
  * reproducible reference to an area.

* **ECS / resource indexing**

  * stable keys for entity/meta/storage.

---

## **1.1.4. Routing as a shared language between subsystems**

Different subsystems may have different internal implementations,
but the address contract between them must remain shared.

For example:

* runtime says:
  “the object is in `Region + LocalFloat`”

* density system wants:
  `DensityKey = Region + Chunk`

* sim system wants:
  `SimSectorKey = Region + SectorCoord`

* debug/tooling wants:
  `FullRoute` or its string representation.

Routing defines exactly
how these forms are derived from one another
and how they remain consistent.

---

# **1.2. Canonical address forms**

In the active MVP, Routing uses **not one**, but several strict addressing forms.

---

## **1.2.1. RuntimePosition**

Base runtime form:

```text
RuntimePosition {
    region = RegionCoord
    local  = LocalFloat
}
```

Where:

* `RegionCoord = (rx, ry, rz)`
* `LocalFloat = (fx, fy, fz)`, `0 ≤ f* < REGION_SIZE`

RuntimePosition:

* is not a Route;
* does not store a discrete deep address;
* is the base live state for movement and physics.

---

## **1.2.2. DensityKey**

Density-level key:

```text
DensityKey {
    region = RegionCoord
    chunk  = ChunkCoord
}
```

Where:

* `ChunkCoord = (cx, cy, cz)`

This is the main active address form for:

* chunk streaming,
* density payload,
* mesh build,
* compression / decompression,
* chunk-based generation.

---

## **1.2.3. SimSectorKey**

Coarse simulation overlay key:

```text
SimSectorKey {
    region = RegionCoord
    sector = SectorCoord
}
```

Where:

* `SectorCoord = (sx, sy, sz)`, `sx, sy, sz ∈ {0,1}`

`SectorId ∈ [0..7]` is allowed as a packed/helper representation,
but is not the primary canonical form of `SimSectorKey`.

This form is needed for:

* coarse sim streaming,
* activity buckets,
* sleep / awake / degradation,
* broad-phase orchestration.

---

## **1.2.4. FullRoute**

Exact discrete address of the active density chain:

```text
FullRoute {
    region = RegionCoord
    chunk  = ChunkCoord
    octo   = OctochunkCoord
    voxel  = VoxelCoord
}
```

Where:

* `OctochunkCoord = (ox, oy, oz)`
* `VoxelCoord = (vx, vy, vz)`

FullRoute is needed for:

* deep debug,
* point editing,
* exact discrete commands,
* serialization,
* devtools,
* rare cross-system references.

---

## **1.2.5. Contextual Local Route**

Contextual local forms are also allowed
when the outer container is already known to the system.

For example:

```text
Chunk + Voxel
Octochunk + Voxel
Chunk + Octochunk
```

But importantly:

* this is not a global address form;
* such forms are valid **only in an explicitly defined context**;
* they do not replace `DensityKey`, `SimSectorKey`, or `FullRoute`.

---

## **1.2.6. What was removed from active MVP Routing**

In the active MVP:

* **Block** is not a required part of the address model;
* **Octant** is not part of the machine-truth route;
* letters `A/B/C/D/E/F/G/I` are not a required part of the discrete address.

They may live in:

* archived/legacy materials,
* human/debug overlay,
* visual sector labels,

but they do not define the active machine-addressing canon.

---

## **1.2.7. Route as an umbrella term**

For convenience, the word **Route** may be used as a general term
for discrete address forms.

But in the active MVP, this must not blur roles.

Recommended rule:

* if talking about live position — say `RuntimePosition`;
* if talking about a chunk key — say `DensityKey`;
* if talking about sim overlay — say `SimSectorKey`;
* if talking about deep discrete address — say `FullRoute`.

---

# **1.3. Invariants and validity**

Active MVP Routing requires
that every address form is consistent with Topology
and does not substitute one role for another.

---

## **1.3.1. A global address always contains Region**

Any global address form must contain:

```text
RegionCoord = (rx, ry, rz)
```

Without Region, an address is not globally interpretable.

Therefore:

* `DensityKey` without Region is invalid as a global key;
* `SimSectorKey` without Region is invalid as a global key;
* `FullRoute` without Region is invalid.

---

## **1.3.2. Active density level order is fixed**

The level order does not change:

```text
Region → Chunk → Octochunk → Voxel
```

Invariants:

* `Chunk` cannot appear after `Octochunk`;
* `Voxel` cannot exist as part of a FullRoute without `Chunk` and `Octochunk`;
* contextual local forms are allowed,
  but only when the outer container is explicitly defined.

---

## **1.3.3. Local coordinates are always non-negative**

Any local indices in address forms:

```text
cx, cy, cz ≥ 0
ox, oy, oz ≥ 0
vx, vy, vz ≥ 0
sx, sy, sz ≥ 0
```

And they must lie within active topology ranges.

---

## **1.3.4. Simulation truth is numeric, not alphabetic**

For `RegionSector`:

* machine truth = `SectorCoord` or `SectorId`;
* letter form (`A/B/C/D/E/F/G/I`) = human/debug overlay only.

Therefore:

* letters do not participate in machine validation;
* letters are not a required address field;
* letter labels must not replace `SectorCoord` / `SectorId`.

---

## **1.3.5. FullRoute must be internally consistent**

For a full address, the following is required:

* `voxel` belongs to `octo`,
* `octo` belongs to `chunk`,
* `chunk` belongs to `region`.

This means:

* indices must be in range;
* the resulting local point must lie inside the Region;
* no level may reference “outside” the container above it.

---

## **1.3.6. Partial / task-specific addresses must also be unambiguous**

`DensityKey` is valid if:

* Region exists,
* Chunk exists,
* Chunk coordinates are valid.

`SimSectorKey` is valid if:

* Region exists,
* sector is correct as `(sx,sy,sz)` or `sector_id`,
* sector belongs to the active Region overlay range.

Contextual local forms are valid only if:

* the outer container is explicitly known,
* address interpretation is unambiguous.

---

## **1.3.7. Reversibility with Topology**

An address form is correct
if it is consistent with active topology
and can be correctly converted:

* into local Region geometry,
* into a `RuntimePosition` anchor,
* into `WorldXYZ`,
* or into another allowed address form.

---

# **1.4. Relationship between Routing, Topology, and coordinates**

Routing relies on Topology,
but does not replace it.

Topology provides:

* sizes,
* strides,
* ranges,
* formulas `World ↔ Region ↔ local indices`.

Routing provides:

* address forms,
* rules for transitions between them,
* the semantic role of each form.

---

## **1.4.1. WorldXYZ ↔ RuntimePosition**

Base runtime bridge:

```text
WorldXYZ ↔ RegionCoord + LocalFloat
```

Where:

* `RegionCoord` defines the Region cube,
* `LocalFloat` defines the continuous position inside it.

This is the base runtime state of the active MVP.

---

## **1.4.2. RuntimePosition → DensityKey**

From `RuntimePosition`, the following can be computed at any moment:

```text
ChunkCoord = floor(LocalFloat / CHUNK_SIZE)
```

Thus:

```text
RuntimePosition → DensityKey
```

This is the main practical transition
from live position to density address.

---

## **1.4.3. RuntimePosition → SimSectorKey**

From the same `RuntimePosition`,
the sim-sector can be computed independently:

```text
SectorCoord = floor(LocalFloat / REGION_SECTOR_SIZE)
```

or packed:

```text
SectorId = pack(SectorCoord)
```

That is:

```text
RuntimePosition → SimSectorKey
```

without using chunk address.

---

## **1.4.4. RuntimePosition → FullRoute**

If an exact deep-addressing form is needed,
it is built from `RuntimePosition`:

1. `ChunkCoord`
2. remainder inside Chunk
3. `OctochunkCoord`
4. remainder inside Octochunk
5. `VoxelCoord`

Producing:

```text
RuntimePosition → FullRoute
```

This is an on-demand operation,
not required permanent runtime state.

---

## **1.4.5. FullRoute → RuntimePosition**

From `FullRoute`, a supporting runtime position inside Region can be restored.

Typical anchor modes:

* `corner`
* `center_of_voxel`
* `center_of_octo`
* `center_of_chunk`

This is important for:

* debug,
* tool selection,
* anchor points,
* transition from discrete address to visual/physical position.

---

## **1.4.6. DensityKey ↔ FullRoute**

`DensityKey` can be viewed as:

* a truncated chunk-level address;
* a container
  in which all possible `FullRoute` values
  starting with the same `Region + Chunk` may exist.

And conversely:

* any `FullRoute` collapses unambiguously into `DensityKey`
  by simply dropping lower levels.

---

## **1.4.7. SimSectorKey ↔ sector geometry**

`SimSectorKey` defines not a point,
but a coarse spatial bucket.

From it, the following can be recovered:

* the base corner of the sector inside Region,
* the sector bounding volume,
* its local AABB inside Region.

---

## **1.4.8. Contextual local addressing**

If a system is already working inside a known Chunk,
local forms such as these are allowed:

```text
C(...) / v(...)
```

But importantly:

* this is not a global Route;
* this is not a universal cross-system exchange form;
* when leaving the context, such an address must be expanded
  to `DensityKey` or `FullRoute`.

---

# **1.5. Operations on addresses**

Routing defines the minimal working set of operations.

---

## **1.5.1. Equality**

Addresses are equal
if all their required and explicitly present fields match.

Examples:

* two `DensityKey` values are equal if `region + chunk` match;
* two `SimSectorKey` values are equal if `region + sector` match;
* two `FullRoute` values are equal if `region + chunk + octo + voxel` match.

---

## **1.5.2. Ordering**

For deterministic sorting, lexicographic ordering by levels is recommended.

Examples:

* `DensityKey`: `region → chunk`
* `SimSectorKey`: `region → sector`
* `FullRoute`: `region → chunk → octo → voxel`

---

## **1.5.3. Contains / Within**

Allowed nesting relations:

* `Region` contains any `DensityKey`, `SimSectorKey`, or `FullRoute` with the same Region;
* `DensityKey` contains any `FullRoute` with the same `region + chunk`;
* `SimSectorKey` contains any local point / runtime position
  that falls inside the sector volume.

Important:

* `SimSectorKey` is not the parent of `Chunk` in active topology;
* `SimSectorKey` and `DensityKey` are different address slices of one Region.

---

## **1.5.4. Up / Down**

For density addressing:

* `FullRoute → DensityKey` = moving up to the chunk level;
* `DensityKey → FullRoute range` = moving down to a set of possible deep addresses inside the chunk.

For simulation overlay:

* `Region → SimSectorKey children`
* `SimSectorKey → Region parent`

`Block` does not participate in these operations in the active MVP.

---

## **1.5.5. Offset**

Offset makes sense only for forms
that allow geometrically unambiguous displacement.

### For `RuntimePosition`

displacement is natural:

```text
local/world vector addition
```

### For `DensityKey`

usually used as:

* neighboring chunk,
* offset on the chunk grid.

### For `FullRoute`

can be used as:

* offset on the voxel grid,
* followed by normalization into a new `FullRoute`.

### For `SimSectorKey`

usually used as:

* transition to a neighboring sector inside Region
  or across a Region boundary with an explicit carry policy.

---

## **1.5.6. Neighbor predicates**

Basic useful predicates:

* `same_region(a, b)`
* `same_chunk(a, b)`
* `same_sector(a, b)`
* `adjacent_chunks(a, b)`
* `adjacent_sectors(a, b)`

They are used in:

* streaming,
* sim orchestration,
* mesh updates,
* tooling,
* debug traversal.

---

## **1.5.7. Normalize**

Normalization checks
that the address form:

* lies within allowed ranges;
* does not violate level structure;
* is consistent with active topology;
* can be interpreted unambiguously.

Normalization is recommended after:

* manual address construction,
* deserialization,
* offset,
* transitions between forms.

---

## **1.5.8. Address → Range**

Some forms address not a point, but a volume:

* `DensityKey` → chunk volume
* `SimSectorKey` → sector volume

Therefore, any such form must be able to expand into:

* local bounds,
* voxel range,
* world/local AABB

depending on the task.

---

# **1.6. String representation**

String format is needed for:

* logs,
* devtools,
* inspector/debug,
* CLI,
* temporary R&D tools.

The canonical format must be:

* reversible,
* unambiguous,
* compact,
* consistent with active MVP address forms.

---

## **1.6.1. General structure**

Segments are separated by `/`.

Canonical prefixes:

* `R(...)` — Region
* `S(...)` — SectorCoord
* `C(...)` — ChunkCoord
* `O(...)` — OctochunkCoord
* `v(...)` — VoxelCoord

Axis order inside segments:

```text
x | y | z
```

That is:

* `R(rx|ry|rz)`
* `S(sx|sy|sz)`
* `C(cx|cy|cz)`
* `O(ox|oy|oz)`
* `v(vx|vy|vz)`

---

## **1.6.2. Canonical active MVP strings**

### RuntimePosition

Usually not serialized as a pure `RouteString`,
because it is a live position,
but if needed it may have a debug form:

```text
R(0|0|0) / p(123.5|42.0|87.25)
```

This is a debug/helper format,
not the main discrete address form.
The additional prefix `p(...)` may be used
only in a debug/helper representation of `RuntimePosition`.

---

### DensityKey

```text
R(0|0|0) / C(7|2|3)
```

---

### SimSectorKey

```text
R(0|0|0) / S(1|0|1)
```

or, if a packed form is more convenient for the project:

```text
R(0|0|0) / S#5
```

But one of the variants must be chosen as canonical.
For the active MVP, the base form is safer:

```text
S(sx|sy|sz)
```

---

### FullRoute

```text
R(0|0|0) / C(7|2|3) / O(1|0|1) / v(12|4|29)
```

---

## **1.6.3. Contextual local strings**

Local strings such as these are allowed:

```text
C(7|2|3) / v(12|4|29)
O(1|0|1) / v(12|4|29)
```

But they are valid only inside an explicitly known outer context.

For cross-system exchange, only global forms that include `R(...)` are recommended.

---

## **1.6.4. Required and optional segments**

For global address forms, the following is required:

* `R(...)`

Other segments depend on the form:

* `DensityKey` → `R / C`
* `SimSectorKey` → `R / S`
* `FullRoute` → `R / C / O / v`

In the active MVP:

* `Octant` is not part of the canonical machine string;
* `Block` is not part of the active route string.

---

## **1.6.5. Human/debug sector labels**

Letters `A/B/C/D/E/F/G/I` may be used in UI, logs, and overlays,
but must not be a required machine segment.

Allowed debug extension:

```text
R(0|0|0) / S(1|0|1) @label(D)
```

or:

```text
R(0|0|0) / S(1|0|1) #D
```

But the main truth part of the string remains numeric.

---

## **1.6.6. Parse / format invariant**

String format invariant:

```text
parse(format(Address)) == Address
```

Parser requirements:

* fixed segment order;
* correct index ranges;
* no implicit guessing of required levels;
* optional metadata must be ignored
  if it is not needed by the main logic.

---

## **1.6.7. Examples**

Correct:

```text
R(0|0|0) / C(7|2|3)
R(0|0|0) / S(1|0|1)
R(-1|0|2) / C(4|3|1) / O(0|1|0) / v(7|12|5)
C(7|2|3) / v(12|4|29)     // local context only
```

Incorrect:

```text
C(7|2|3)                  // as a global address without Region
R(0|0|0) / v(1|2|3)      // as FullRoute without Chunk/Octo
R(0|0|0) / O(1|0|1)      // missing Chunk
R(0|0|0) / S(2|0|0)      // sector out of range
```

---

# **1.7. Mapping / Address API**

This section defines the minimal interface for conversions and address data extraction.

It does not repeat Topology mathematics,
but wraps it into a usable address contract.

---

## **1.7.1. Basic extraction**

```text
region_of(runtime|density|sector|full) -> RegionCoord
chunk_of(density|full)                 -> ChunkCoord
sector_of(sim)                         -> SectorCoord 
octo_of(full)                          -> OctochunkCoord
voxel_of(full)                         -> VoxelCoord
```

---

## **1.7.2. Main conversions**

```text
world_to_runtime(WorldXYZ)                  -> RuntimePosition
runtime_to_world(RuntimePosition)           -> WorldXYZ

runtime_to_density_key(RuntimePosition)     -> DensityKey
runtime_to_sim_sector(RuntimePosition)      -> SimSectorKey
runtime_to_full_route(RuntimePosition)      -> FullRoute

full_route_to_runtime(FullRoute, anchor)    -> RuntimePosition
density_key_to_chunk_bounds(DensityKey)     -> ChunkBounds
sim_sector_to_bounds(SimSectorKey)          -> SectorBounds
```

---

## **1.7.3. Identity and neighborhood**

```text
same_region(a, b)       -> bool
same_chunk(a, b)        -> bool
same_sector(a, b)       -> bool

adjacent_chunks(a, b)   -> bool
adjacent_sectors(a, b)  -> bool
```

---

## **1.7.4. Level traversal**

```text
full_to_density(full)               -> DensityKey
density_children_octo(density)      -> iterator<OctochunkCoord>
octo_children_voxel(full|octo_ctx)  -> iterator<VoxelCoord>

region_sector_children(region)      -> iterator<SimSectorKey>
```

---

## **1.7.5. Optional index helpers**

If a subsystem needs:

```text
chunk_flat_index(DensityKey)        -> integer
octo_flat_index(FullRoute)          -> integer
voxel_flat_index(FullRoute)         -> integer

octo_morton(FullRoute)              -> integer
voxel_morton(FullRoute)             -> integer
sector_packed_id(SimSectorKey)      -> integer
```

But the formulas themselves remain part of Topology.

---

## **1.7.6. Cache policy**

Caching derived values is allowed:

* world anchor,
* chunk bounds,
* sector bounds,
* flat index,
* morton index,
* packed sector id.

But the sources of truth remain:

* `RuntimePosition`
* `DensityKey`
* `SimSectorKey`
* `FullRoute`

---

# **1.8. Using Routing in engine systems**

Routing is distributed by roles in the active MVP.

---

## **1.8.1. Runtime**

Runtime lives on:

```text
RuntimePosition = Region + LocalFloat
```

This is the base state for:

* camera,
* player,
* physics,
* navigation,
* rays,
* tools.

---

## **1.8.2. Density**

Density systems live on:

```text
DensityKey = Region + Chunk
```

This is the key for:

* voxel payload,
* density generation,
* chunk storage,
* mesh build,
* compression / decompression,
* density streaming.

---

## **1.8.3. Sim**

Simulation orchestration lives on:

```text
SimSectorKey = Region + Sector
```

This is the key for:

* coarse sim streaming,
* activity buckets,
* awake / sleep / degradation,
* broad-phase sim zoning.

---

## **1.8.4. Deep tools / edit / debug**

Deep point systems use:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

This is needed for:

* deep debug,
* voxel editing,
* inspector jump,
* point commands to the core,
* exact serialization.

---

## **1.8.5. Human/debug layer**

Human/debug overlay lives above machine truth:

* letter sectors,
* compact labels,
* centered anchors,
* inspector labels.

Machine truth must not depend on these labels.

---

## **1.8.6. Summary**

Active MVP Routing defines a simple hierarchy of roles:

```text
RuntimePosition = live position
DensityKey      = chunk address
SimSectorKey    = coarse sim address
FullRoute       = deep discrete address
```

This is the new active routing canon of Arden.

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)
