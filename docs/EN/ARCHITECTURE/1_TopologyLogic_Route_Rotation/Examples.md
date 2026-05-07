**Document Status:** 🔬 Review  
**Version:** 0.2.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-04-16  
**Purpose:** Practical examples for active MVP Topology / Routing / Rotation

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)

---

# Examples — how to use this file

This document is built on top of three base files:

- **Topology.md** — space structure, sizes, indices, and mapping,
- **Routing.md** — active address forms and readdressing,
- **Rotation.md** — rotations, orientation-state, and content reorientation.

This file does not introduce new rules.
Instead, it shows through examples
how these three layers work together in the active MVP.

If the base documents are the dry contract,
then this file is the practical layer:

- how to read these structures as a developer,
- how to think with them in runtime and tools,
- how to translate machine truth into compact / human form,
- how to use this in generation, copy/paste, movement, and debug.

This file exists not to repeat the canon again,
but to make it practical and alive.


## 1. Working profile

To keep the examples concrete,
we use one working project profile below:

```text
CHUNK_SIZE              = 64
OCTO_SIZE               = 32
REGION_CHUNKS_PER_AXIS  = 16
REGION_SIZE             = 1024
REGION_SECTOR_SPLIT     = 2
REGION_SECTOR_SIZE      = 512
```

This is only needed so that all following examples read as one concrete scene,
instead of spreading across different configurations.
Theory and experience from similar projects suggest
that this scale looks generally workable and convenient for explanation.

We get:

* Region as a `1024 × 1024 × 1024` cube
* Chunk as the main dense container `64³`
* Octochunk as an internal sublayer `32³`
* RegionSector as coarse simulation sections `512³`

This profile is convenient because
it remains large enough for practical examples,
but does not inflate into excessively heavy numbers.

Important:
this profile is not a hard dogma of the engine itself.
If needed, Chunk density can be changed,
for example using profiles:

```text
16³ / 32³ / 64³ / 128³
```

At the same time, the base logic does not break:

* `Chunk` remains the main density container;
* `Octochunk` remains its internal sublayer;
* `RegionSector` remains the coarse simulation subdivision `2 × 2 × 2`;
* the active hierarchy does not change.

Only density and profile scales change,
not the structure of space itself.

In other words, Examples uses one working profile for clarity,
but the system itself is not tied only to the number `64`.

---

## 2. One point across all layers

The same point in the world can be represented across several layers at once.

For live runtime, we usually think like this:

```text
RuntimePosition = Region + LocalFloat
```

This is a convenient form for movement, camera, navigation, rays, and other systems
that need a continuous position rather than a full discrete address.

If a system needs the main dense container,
the same position can be reduced to:

```text
DensityKey = Region + Chunk
```

If a system needs the coarse simulation level,
it can also be reduced to:

```text
SimSectorKey = Region + SectorCoord
```

And if the most precise discrete position is required,
the position can be decomposed deeper:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

So the active MVP does not rely on one overloaded address,
but on several representations of the same point for different tasks:

* runtime,
* density,
* simulation,
* deep debug / edit.

---

### Region and LocalFloat: where the local coordinate begins

When we say `Region + LocalFloat`,
it is important to understand:
the local coordinate inside a Region does not start from the center,
but from the origin corner of that Region.

That means each Region has its own local range:

```text
LocalFloat ∈ [0 .. REGION_SIZE)
```

And the local coordinate itself is calculated as:

```text
LocalFloat = WorldXYZ - RegionOriginWorld
```

This means:

* each Region has its own local coordinate system;
* when crossing a Region boundary, `RegionCoord` changes;
* the local coordinate does not “break”,
  it is simply rebuilt into the range of the Region
  that the world position entered.

This is exactly why the same object can live in runtime as
`Region + LocalFloat`,
and only later, on demand,
be decomposed into Chunk / Sector / Octo / Voxel.

---

### What this gives in practice

From the same runtime position,
a system can request:

* which Region the object is in;
* which Chunk it is in;
* which RegionSector it falls into;
* which Octochunk or Voxel it needs to descend into
  if deeper detail is required.

In other words:

```text
one world/runtime position
-> several address forms
-> each for its own task
```

This is one of the most useful ideas of the active MVP:
do not always carry one “super-address”,
but extract the needed depth only when it is actually needed.

---

## 3. Machine / compact / human representations

The same address information can be read on several layers.

The canonical machine form remains coordinate-based:

```text
R(rx|ry|rz) / S(sx|sy|sz) / C(cx|cy|cz) / O(ox|oy|oz) / v(vx|vy|vz)
```

This is the form that Topology, Routing, and internal transformations rely on.

But for logs, HUD, tools, and manual work,
shorter representations are often more convenient.

---

A compact helper layer may look like this:

```text
R(...) / S#n / C#n / O#n / v(...)
```

Here:

* `S#n` — compact sector id;
* `C#n` — compact Chunk id inside a Region;
* `O#n` — compact Octochunk id inside a Chunk.

This form does not replace machine truth.
It only makes addresses shorter and easier to read.

---

For the debug layer, I propose representing the address like this.
For the overlay layer:

```text
R(...)
Sector D
Chunk #17
Octo #3
Voxel (12|4|29)
```

This feels convenient for:

* logs;
* HUD;
* inspector;
* tool commands;
* copy/paste and selection scenarios.

But it is not the base machine form.

---

## 4. Compact 3-bit forms

In some places, small coordinate domains are convenient to pack into a compact id.

The simplest case:

```text
(0/1, 0/1, 0/1) <-> packed id 0..7
```

For the active MVP, this is especially natural where space is already divided as `2 × 2 × 2`.

This mainly applies to:

* `RegionSector`
* `Octochunk` inside a Chunk

That is:

* `SectorCoord = (sx, sy, sz)` can be packed into `S#n`
* `OctochunkCoord = (ox, oy, oz)` can be packed into `O#n`

Compact 3-bit form is useful for:

* helper ids in logs and HUD;
* lookup tables;
* bitmask / occupancy;
* child-slot indexing;
* small fixed-domain states inside `2 × 2 × 2`.

---

It is important not to confuse two different things:

1. **non-negative local coordinates**
2. **3-bit packing**

Non-negativity does not come from packing.
It comes from the container-local topology itself:

* local coordinates inside a Region live in `[0 .. REGION_SIZE)`
* local container indices are also non-negative

3-bit form is not the basis of topology.
It is only a compact helper form for a tiny domain.

---

For small fixed-domain levels `2 × 2 × 2`,
a packed id looks natural.

But for larger coordinates such as:

* `ChunkCoord`
* `VoxelCoord`
* full world/local mapping

the canonical form is still more convenient as regular `x / y / z`.

### Practical summary

It is useful to think this way:

```text
machine truth = coords
compact 3-bit = helper form for tiny fixed domains
```

This allows us to:

* keep the base coordinate canon intact;
* still have short `S#` and `O#`
  where they actually simplify reading and tooling.

---

## 5. Registration and where data goes

In the active MVP, one object does not have to live as one overloaded address object.

In practice, it is more useful to think like this:
the same entity can be decomposed
across several layers at the same time,
and each layer stores only what it actually needs for its role.

---

### Runtime layer

For live runtime, the base form looks like this:

```text
RuntimePosition = Region + LocalFloat
```

This is needed where a continuous position matters:

* movement;
* camera;
* navigation;
* rays and hit tests;
* general runtime state of an object.

This form is usually the “live” anchor of an object,
not a deep discrete address.

---

### Density layer

If a system needs the main dense container,
it works at the level of:

```text
DensityKey = Region + Chunk
```

This is convenient for:

* chunk storage;
* voxel payload;
* mesh build;
* local generation;
* compression / decompression;
* chunk-based processing.

So the density layer does not need to know live-position “like a camera does”.
It needs its own container key.

---

### Sim layer

If a system needs a coarse simulation bucket,
it uses:

```text
SimSectorKey = Region + SectorCoord
```

This is suitable for:

* coarse activity buckets;
* sleep / awake logic;
* simulation orchestration;
* broad-phase grouping.

The sim layer does not have to store a deep density address.
A coarse address form is enough,
answering the question:
which large sector of a Region the object or area currently lives in.

---

### Deep debug / edit / serialization

When deeper discrete precision is needed,
a system can descend to:

```text
FullRoute = Region + Chunk + Octochunk + Voxel
```

This is useful for:

* deep debug;
* voxel editing;
* point commands;
* copy and paste;
* serialization and checks.

But this does not mean
the whole engine must constantly live on `FullRoute`.

---

## 5. Orientation is stored separately

If an entity has orientation,
it is useful to think of it separately from the address form.

The practical model looks like this:

```text
AnchorAddress + Orientation + LocalContent
```

That means:

* address answers “where”;
* orientation answers “in which yaw orientation”;
* local content answers “what exactly is stored inside”.

This is especially important for copy/paste, preview, blueprint-like data, and rotation-aware workflows.

---

### Where the data “goes”

Roughly, without tying this to a specific backend,
it is convenient to think like this:

* **runtime registry**
  stores `RuntimePosition` and live state

* **density storage**
  stores `DensityKey` and chunk-local content

* **sim buckets**
  group objects by `SimSectorKey`

* **deep tool/debug layer**
  uses `FullRoute` when needed

* **orientation / content layer**
  stores orientation and local content separately from the address key

This is not a storage spec,
but a conceptual map:
it shows why the same object does not have to look the same everywhere.

---

### What this gives in practice

This separation gives several advantages at once:

* runtime is not overloaded with deep address details;
* density systems get their natural key;
* sim systems are not forced to work at chunk or voxel level;
* tool/debug layer can descend deeper only when it is actually needed.

The final idea here is simple:

```text
one object
-> several coordinated representation layers
-> each layer stores only its own role
```

This is what makes registration cleaner
and does not force the whole engine to live in one overloaded format.

---

## 6. Readdressing during movement

When an object moves in the world,
its address forms do not have to be rebuilt equally and constantly.

In the active MVP, it is useful to think like this:

* runtime lives on `RuntimePosition`
* density gets `DensityKey` on demand
* simulation gets `SimSectorKey` on demand
* `FullRoute` is only needed where deep discrete precision is actually required

So movement is not “carry the entire deep address every tick”.
It primarily updates the live-position,
and only after that extracts the required address depth from it.

---

### Basic chain

In practice, readdressing looks like this:

```text
WorldXYZ
-> RuntimePosition
-> DensityKey
-> SimSectorKey
-> FullRoute (on demand)
```

This means:

* the world first provides a continuous position;
* then it is bound to a specific Region;
* from there, more discrete forms are derived on demand.

---

### Movement inside the same Chunk

If the object remains inside the same Chunk,
usually only `LocalFloat` changes.

At the same time:

* `RuntimePosition` is updated continuously;
* `DensityKey` may remain the same;
* `SimSectorKey` may also remain the same;
* `FullRoute` may not be computed at all
  if deep discrete work is not required right now.

This is the cheapest scenario:
the object moves,
but its coarse address forms remain unchanged.

---

### Transition into a neighboring Chunk

If the object crosses a Chunk boundary,
the situation changes.

We still start from `RuntimePosition`,
but now it produces a new `ChunkCoord`,
which means this is updated too:

```text
DensityKey = Region + Chunk
```

At the same time:

* `RegionCoord` may remain the same;
* `SimSectorKey` may or may not change —
  depending on whether the object also crossed a coarse sector boundary.

So not every Chunk change
automatically means a sim-sector change.

---

### Transition into a neighboring Region

When the object crosses a Region boundary,
not only the local position changes,
but also `RegionCoord` itself.

This means:

* the object gets a new `Region`;
* `LocalFloat` is rebuilt into the range of the new Region;
* `DensityKey`, `SimSectorKey`, and any deeper forms
  are computed relative to the new Region.

This is where it becomes clear
why machine-local inside a Region begins from the origin corner of the Region:
when crossing the boundary,
the local coordinate does not “break”,
it is simply recalculated inside the new Region container.

---

### What is readdressed constantly and what is on demand

In practice, it is convenient to keep this rule:

* **always-live layer** —
  `RuntimePosition`

* **frequently updated coarse layer** —
  `DensityKey`, `SimSectorKey`

* **deep layer on demand** —
  `FullRoute`

This is especially useful
so runtime is not overloaded with unnecessary deep-address logic
where the system actually needs only coarse container info.

---

#### This approach gives several advantages at once:

* runtime does not carry extra discrete detail;
* density gets its natural key only when the object really crosses density-container boundaries;
* simulation gets its coarse bucket independently from deep density-address;
* deep tools and debug can descend deeper
  only where it is actually useful.

Final idea:

```text
movement first updates live-position,
and only then derives the address depth that is needed
```

That is, movement does not start with a deep route.
It ends with one only where it is actually needed.

---

## 7. Copy / Paste and rotation

Copy / paste is useful to understand not as one magical operation,
but as a small pipeline
where address, content, and orientation work together,
but are not mixed into one entity.

In practice, it looks like this:

1. choose source anchor;
2. determine local content bounds;
3. read content in local space;
4. apply rotation if needed;
5. choose target anchor;
6. normalize target address;
7. write content into the new area.

---

### Copy / Paste without rotation

The simplest case:

```text
copy source:  R(...) / C#17
paste target: R(...) / C#4
orientation:  R0
```

What happens here:

* source anchor defines
  which container we read local content from;
* target anchor defines
  where we write this content;
* source address and target address are independent from each other;
* content orientation remains the same,
  so rotation is not applied.

This mode is useful for:

* simple copying of fragments;
* reusing modules;
* rough assembly of structures without changing orientation.

---

### Copy / Paste with rotation

In a more interesting case, we want to paste the same local content,
but in another orientation.

For example:

```text
copy source:  R(...) / C#17
paste target: R(...) / C#4
orientation:  R90
mode:         content rotation
```

Here, it is not the source address itself that changes,
but the local shape of the content.

The pipeline becomes:

* take local content from source;
* rotate it in local space;
* keep target anchor separate;
* write rotated content into the target container.

So we rotate **content**,
and we do not have to rotate the source address itself as a world anchor.

---

### Anchor rotation and content rotation are not the same

This is one of the main practical points of the active MVP.

We need to distinguish:

#### **Anchor rotation**

when the anchor / position in the world itself rotates.

Then:

* `RuntimePosition` may change;
* `DensityKey` may change;
* `SimSectorKey` may change.

#### **Content rotation**

when the anchor remains the same,
but the local content inside the container is reoriented.

Then:

* the anchor address may remain the same;
* only local content changes;
* orientation-state is updated separately.

This is very important for:

* blueprint placement;
* copy/paste;
* preview tools;
* reuse of local structures.

---

### Address and orientation should not be mixed

In copy/paste, it becomes especially clear
why the active MVP keeps `Orientation` separate from address form.

In practice, this means:

```text
address != orientation
```

The same `DensityKey` can be used:

* with `R0`,
* with `R90`,
* with `R180`,
* with `R270`

if only content orientation changes,
and not the anchor itself.

This is exactly why it is more convenient to keep a model like:

```text
AnchorAddress + Orientation + LocalContent
```

rather than trying to pack everything into one overloaded route-like object.

---

### Tool command and machine pipeline

For a human and for the tool layer,
it is convenient to work with a softer language.

For example:

```text
copy C#17
paste to R(1|0|0) / C#4 with R90
```

But under the hood, this should still go through the machine pipeline:

```text
human/debug input
-> parser / translator
-> canonical machine form
-> validate / normalize
-> execute
```

This allows us to:

* keep the tool layer human-readable;
* still avoid breaking machine truth;
* use copy/paste consistently in editor, debug, and generation.

---

### What this gives in practice

Copy / paste in this form becomes not “special editor magic”,
but a natural consequence of the existing base:

* `Routing` provides address forms;
* `Rotation` provides orientation and local transform rules;
* `Topology` provides container-local math,
  which reads content and lays it back out.

The practical formula here is:

```text
copy/paste = address selection
           + local content extraction
           + optional rotation
           + normalized writeback
```

In this form, copy/paste fits well into:

* editor thinking,
* generation thinking,
* future blueprint workflows.

---

## 8. Octochunk as first coarse filter for SVO

In the active MVP, `Octochunk` is useful to understand not only as an internal sublayer of `Chunk`,
but also as the first practical step
before deeper hierarchical processing.

If `Chunk = 64³`,
then it naturally divides into:

```text
8 Octochunk = 2 × 2 × 2
```

where each Octochunk has size:

```text
32³
```

This gives a convenient coarse step
between a whole Chunk and deeper detail.

---

### Why this is needed at all

If we look at the whole `Chunk 64³` at once,
the algorithm is forced to process the entire volume immediately,
even if meaningful structure exists only in a small part of it.

Octochunk allows the system to make a very coarse classification first,
and only then decide whether it needs to go deeper.

So instead of this approach:

```text
analyse whole Chunk as one heavy unit
```

we get:

```text
Chunk
-> split into 8 Octochunk
-> classify each Octochunk
-> refine only where needed
```

---

### Basic coarse classification

For each Octochunk, a summary state can be obtained quickly.
For example:

* **empty** — empty, no need to descend;
* **solid / uniform** — everything is uniform, no need to descend;
* **mixed** — there is a boundary, surface, cavity, or non-uniformity,
  so it needs deeper inspection.

Example:

```text
Octo 0 = empty
Octo 1 = empty
Octo 2 = solid
Octo 3 = mixed
Octo 4 = solid
Octo 5 = mixed
Octo 6 = empty
Octo 7 = solid
```

After that, heavier processing runs
only for `mixed` areas.

---

### What this gives in practice

This coarse step is useful in several tasks at once.

#### **For SVO / refinement**

There is no need to descend equally into all of `64³`.
First, Octochunk acts as the first filter,
and only then deeper refinement starts.

#### **For meshing**

Empty and uniform Octochunks can be discarded quickly
or processed in a simplified way,
while `mixed` ones are treated as candidates for more detailed surface work.

#### **For summary / masks**

At Octochunk level, it is convenient to keep:

* occupancy,
* rough density summary,
* uniform / mixed flag,
* surface hints,
* coarse material hints.

#### **For future SVO-style processing**

Octochunk can be the first and very cheap “gate”
that a Chunk passes through
before deeper hierarchical processing.

---

### Why this is not the SVO itself

Important:
Octochunk summary is not the SVO itself yet.

It is rather the first coarse buffer / refinement gate
that helps decide:

* whether we need to go deeper at all;
* where exactly to descend;
* which parts of the Chunk can be skipped immediately.

That is:

```text
dense chunk data
-> octochunk summary
-> deeper refinement only where needed
```

This is why Octochunk is useful here not as an abstract “pretty middle layer”,
but as a real operational level.

---

### Practical role of Octochunk

If we compress this into one short idea,
then for Examples it is useful to record it like this:

```text
Octochunk = first coarse filter
Octochunk = first refinement gate
Octochunk = first summary domain inside Chunk
```

This is one of the strongest practical roles of Octochunk in the active MVP:
not just a structural sublayer,
but the first working layer before deeper SVO-style logic.

---

## 9. Mapping, flat index, Morton

When we talk about address forms and local coordinates,
it is useful to remember
that the same 3D structure can be reduced
to different index representations
depending on the task.

In Examples, three connected ideas matter:

* regular mapping through container-local coordinates;
* flat index for linear memory;
* Morton for spatial locality.

---

### Mapping: from world to local structure

The base active MVP logic looks like this:

```text
WorldXYZ
-> RegionCoord + LocalFloat
-> ChunkCoord
-> OctochunkCoord
-> VoxelCoord
```

This is the main machine mapping.

It answers the question:
how a continuous world position
is decomposed into discrete containers and local indices.

In practice, this means
the same position can be read at different depths:

* only as `RuntimePosition`,
* as `DensityKey`,
* as `SimSectorKey`,
* as `FullRoute`.

---

### Flat index: when linear order is needed

If level data needs to be placed in a regular linear array,
flat index is used.

For canonical XYZ order:

```text
flat_index =
    x +
    size_x * (
        y +
        size_y * z
    )
```

This formula is useful where data lives in regular dense memory.

For example:

* `ChunkCoord -> C#n`
* `OctochunkCoord -> O#n`
* `SectorCoord -> S#n`

So compact ids are convenient to build on this index logic
when a short and stable helper representation is needed.

---

### What flat index gives practically

Flat index is needed
when the system cares not about a spatial tree,
but about a regular linear layout:

* dense buffer;
* array of chunks / octos / sectors;
* helper ids in logs and HUD;
* compact indexing for tools.

It is not a “world address” by itself,
but only a way to place a local 3D structure in 1D order.

---

### Morton: when spatial locality matters

Morton is needed in another situation:
when it matters that elements close in space
remain relatively close in index order as well.

The rough idea is:

```text
(x, y, z) -> interleave_bits(x, y, z)
```

Unlike flat index,
Morton is not so much about a simple linear array,
but about spatial locality,
hierarchical structures,
and tree-like traversal.

---

### Flat and Morton do not compete

It is very useful not to confuse these two models.

```text
flat   = linear memory order
Morton = spatial-local order
```

That is:

* flat is convenient for dense storage and simple helper ids;
* Morton is convenient for spatial trees,
  hierarchical structures,
  and scenarios where spatial locality matters.

They do not replace each other.
They answer different questions.

---

### Where this appears in Examples

For the reader of Examples, this is useful in the following way:

* `mapping` explains
  how a position gets into the required container at all;

* `flat index` explains
  where convenient `S# / C# / O#` ids come from;

* `Morton` explains
  why more spatial-aware structures
  do not have to use the same order
  as a regular dense buffer.

---

### Practical summary

If we compress this into a short practical formula:

```text
mapping -> finds the required container and local coordinates
flat    -> gives linear helper/index order
Morton  -> gives spatial-local order
```

This makes it easier to understand
why the same 3D structure
can have several representations at once,
and why this is not a conflict,
but a normal part of the active MVP.

---

## 10. Conceptual buffers

In Examples, it is better to understand a buffer
not as a strict backend layout,
but as a convenient mental map:
which layer generally keeps which data
and why it needs it.

This is important
so live-position,
density-data,
sim buckets,
tool staging,
and helper indexing
are not mixed into one overloaded “super-buffer”.

---

### Runtime buffer / registry

The runtime layer primarily stores the live state of an object.

This is where these naturally live:

* `RuntimePosition = Region + LocalFloat`
* current live-state
* orientation-state
* temporary runtime fields
  such as velocity, flags, timers, and other operational logic

This buffer is not about deep discrete addressing,
but about what is actually alive, moving, and updating in runtime right now.

---

### Density buffer / chunk storage

The density layer is no longer thought through live-position,
but through a density container.

This is where these naturally live:

* `DensityKey = Region + Chunk`
* chunk-local voxel payload
* material / density data
* local data
  needed for generation, meshing, compression, and similar tasks

This is the main buffer for the Chunk itself as a dense container,
not for coarse simulation or tool debug.

---

### Octochunk summary buffer

If we look at a Chunk not as one heavy block,
but as a container with the first coarse step inside,
then it is useful to think separately about a summary layer per Octochunk.

For each of the 8 Octochunks, a very rough summary can be stored, for example:

* `empty`
* `solid / uniform`
* `mixed`

Or other summary traits:

* occupancy
* rough density summary
* surface hint
* material hint
* local activity flag

This does not have to be the SVO itself.
Such a buffer can be understood as:

```text
dense chunk data
-> octochunk summary
-> deeper refinement only where needed
```

So the Octochunk summary buffer is the first coarse filter
and the first refinement gate inside a Chunk.

---

### Sim buckets

The simulation layer is more convenient to think about separately from density storage.

This is where these naturally live:

* `SimSectorKey = Region + SectorCoord`
* coarse activity grouping
* awake / sleep flags
* orchestration queues
* broad-phase simulation grouping

This buffer does not answer the question
“which voxel lies here”,
but rather:
“which coarse simulation bucket does this object or area currently belong to”.

---

### Tool / staging buffer

It is useful to think of a temporary layer for tools separately.

This is where the following can live:

* selection bounds
* copy/paste payload
* rotated preview
* transformed local content
* import/export staging data
* temporary tool commands before writeback

This is especially convenient for editor thinking:

```text
source content
-> staging buffer
-> optional transform / rotation
-> target writeback
```

This way, the tool layer does not have to write everything directly into the main density buffer.
It can first collect, check, and transform data in the staging layer.

---

### Helper / index cache

Some data forms are useful to keep not as source of truth,
but as helper cache.

These may include:

* compact ids (`S#`, `C#`, `O#`)
* flat indices
* Morton helpers
* masks
* lookup tables
* derived preview/meta values

This is not a new addressing canon
and not a replacement for the machine form,
but simply an accelerating and simplifying layer
where it is actually needed.

---

### How to read all of this together

If we assemble this into one practical picture,
we get this mental scheme:

* **runtime buffer**
  stores live-position and current state

* **density buffer**
  stores Chunk and its local content

* **octochunk summary buffer**
  stores coarse classification inside Chunk

* **sim buckets**
  group objects and areas by the coarse sector layer

* **tool/staging buffer**
  keeps temporary data for selection, copy/paste, and transform

* **helper/index cache**
  keeps derived compact/index forms

---

### Important note

This section does not define a storage spec
and does not fix a required backend layout.

It is only a conceptual view
that helps explain:

* why different subsystems do not have to live in one format;
* why some data is more convenient to store as live-position,
  and other data as density / simulation / address summary;
* how the active MVP decomposes data by roles,
  instead of dumping everything into one structure.

The final idea here is simple:

```text
different layers keep different kinds of truth
```

This is what makes the system cleaner,
easier to understand for tools,
and more convenient for future growth.

---

[📚 Back](./README.md)

[🧱 Project Architecture Portal](../../ARCHITECTURE/readme.md)
