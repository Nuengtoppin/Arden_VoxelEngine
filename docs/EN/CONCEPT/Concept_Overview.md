
**Document Status:** 🧩 Draft   
**Version:** 0.1.0  
**Maintainer:** Nuengtoppin  
**Reviewed by:** —  
**Last update:** 2026-05-07

---

# Arden Engine — Concept Overview

## 1. Engine Philosophy

**Arden Engine** is an attempt to build a viable core for a hybrid voxel / mesh engine.

By “hybrid”, this project does not mean a simple mix of cubes and polygons. It means an architecture where world logic lives deeper than rendering: voxel data, topology, materials, density, addressing, and object state exist as independent truth, while mesh / render / collider layers are derived representations.

In direction, this is close to the idea behind engines such as **Atomontage**: the world does not have to be only an image or only a set of meshes. It can have an internal volumetric structure that can be read, modified, streamed, optimized, and transformed into different representations.

Arden is built with **Rust** not only for performance, but also for the possibility of using the core separately from a specific game or visual layer. Ideally, the core should be usable not only in a Bevy scene, but also by other upper-level stacks:

- game scene;
- editor / lab tools;
- server simulation;
- headless world processing;
- world generation;
- testing and debug scenarios.

The main idea: **the world should exist before rendering**.

Render shows the state of the world, but it is not the source of truth.  
The same applies to UI, debug tools, collider layers, and future game systems: they work on top of the core, they do not replace it.

**Arden** represents a workspace for these kinds of architectural experiments.

**The Arden Cube** is the project mascot and a symbol of a supporting form: structure, stability, nesting, and spatial clarity. It does not mean the engine must be “cubic”; it represents a core that can be inspected, tested, taken apart, and assembled again.

---

## 2. Why Hybrid Voxel / Mesh

Arden is built around the idea of a **hybrid voxel / mesh core**, because different data types are useful for different tasks.

The **voxel layer** is useful where the internal structure of the world matters:

- destruction;
- procedural generation;
- density and materials;
- hidden internal layers of an object;
- volume logic, not only surface logic;
- storing world state in a discrete form.

But this does not mean the whole world must be 100% voxel-based.  
Not every object has to be fully filled with voxel data, and not every surface has to look cubic.

The **mesh layer** is useful where shape matters:

- smooth surface;
- convenient rendering;
- collider representation;
- external visual layer;
- models that do not require full voxel filling;
- work with familiar polygon-based tools.

The goal of the hybrid approach is not to choose “voxels versus meshes”, but to make these two layers work together.

Working idea:

```text
Voxel / volume truth -> Mesh representation -> Render / Collider / Tools
```

Voxels can store structure, material, destruction, and procedural logic.  
Mesh can be a derived representation of that structure, or a separate external layer connected to the core through a spatial/address system.

This approach makes it possible to build a world where:

- some objects can be voxel-driven;
- some objects can be regular mesh models;
- some systems can have voxel logic inside, but a smooth mesh representation outside;
- destruction, generation, and runtime changes do not have to break the visual layer directly;
- render and collider layers can be rebuildable representations, not the source of truth.

The main question for Arden is not “which method is better”, but how to connect both approaches without conflict:

```text
voxels provide internal structure,
mesh provides external shape,
spatial truth connects them into one system.
```

Similar ideas already exist in different forms: voxel destruction, volumetric engines, sparse volume libraries, terrain/procedural systems, and hybrid rendering pipelines. But I look at this not as one rendering or destruction technology, but as an attempt to build an independent world core where logic, topology, and volumetric data exist before rendering and can be used by different upper-level stacks.

---

## 3. Arden Core

The heart of Arden development is a minimal viable core for a hybrid voxel / mesh engine.

Without this layer, there is no point in moving into DUN, complex rendering, optimization, or game systems, because first the project needs to answer a basic question: where is the world, how is it addressed, how does it change, and how can its state be verified?

The working name for this layer is **EQ Kernel** / **Equilibrium Kernel**.

EQ Kernel can be understood as a single independent core block with two separated responsibilities inside:

```text
EQ-Core = world state
EQ-Sim  = rule application and state changes
```

**EQ-Core** is responsible for the “memory of the world”: spatial truth, topology, address forms, voxel/storage data, route logic, active areas, and container state.

**EQ-Sim** is responsible for runtime behavior: applying changes, movement, transform/rotation, activity, basic simulation rules, and preparing a consistent state for external layers.

Between these parts, a minimal internal coordinator is needed — **Core BUS**.

In this context, BUS is not a large universal task bus for the entire engine.  
It is more like a simple “postman” inside the core: it passes events, dirty states, requests, and results between layers so that storage, spatial truth, streaming, tools, and sim do not access each other directly.

Simplified:

```text
EQ-Core  -> stores state
EQ-Sim   -> applies changes
Core BUS -> passes messages between layers
```

The goal of the core is to be a simple but coherent machine: not to solve everything for the project, but to reliably store, address, change, and log the state of the world.

Such a core should be usable not only for a Bevy scene, but also for other upper-level stacks:

- lab/debug environment;
- game scene;
- editor tools;
- headless/server simulation;
- world generation;
- test scenarios.

The main goal of Arden at this level is to build a minimal viable core for standard hybrid-world tasks: spatial truth, addressing, data storage, state changes, logging, and passing updates to higher layers.

---

## 4. Butler — Core Logger and Validator

**Butler** is an optional but useful branch of the project focused on diagnostics and development convenience.

In its minimal form, Butler can be understood as an internal state logger: a tool that helps quickly collect a technical picture of what is happening inside the core right now.

Its basic tasks:

- collect the state of key layers;
- show active / dirty areas;
- highlight inconsistent data;
- help understand which layer changed what;
- give the developer a compact report instead of forcing manual searching through code and logs.

At the early stage, Butler should not be a “smart manager” and should not interfere with gameplay or runtime logic.  
It should help the developer see a slice of the system: what changed, where there is a conflict, and what needs to be checked.

In the future, Butler may grow into a more complete local project assistant:

- diagnostic layer;
- architectural rule validator;
- local RAG assistant for code and documentation;
- tool for explaining errors, dirty/rebuild states, and conflicts between layers.

The Butler concept is still early and raw.  
Its role is not to replace the core, BUS, or HAOS, but to help observe, verify, and explain the state of the system during development.

---

## 5. HAOS — Adaptive Optimization Layer

**HAOS** — Hybrid Adaptive Optimization System.

HAOS is a future layer for optimization discipline.  
Its task is not to “magically speed up the engine”, but to avoid spreading optimization decisions across the entire codebase.

In Arden, optimization affects more than just render or mesh.  
It touches multiple types of data:

- voxel storage;
- mesh representation;
- dirty/rebuild queues;
- visibility;
- LOD;
- SVO / octree structures;
- tick frequency;
- memory / dormancy;
- particles and visual effects;
- collider / physics rebuild.

Because of that, HAOS can be understood as a separate “toolbox” of optimizations: a place where rules, configs, modes, and algorithms are collected and applied to different engine layers.

Basic idea:

```text
HAOS-lib = a set of manually described optimization rules and algorithms
HAOS     = the layer that applies these rules according to world state
```

At the early stage, HAOS should remain as simple as possible:

- manual configs;
- debug overlay for tuning;
- enabling / disabling individual optimizations;
- displaying current budgets;
- understandable quality / performance modes;
- connection to dirty/rebuild states.

Basic HAOS directions:

```text
Spatial  -> LOD, SVO, decimation, clipmaps
Temporal -> tick scheduling, batching
Culling  -> distance / visibility
Memory   -> archiving / awakening / DTO
Render   -> particle budget, effect quality, mesh detail
```

HAOS should not be a “magical AI”.  
At first, it is just a library of rules:

```text
what should be calculated now?
what can be delayed?
what can be simplified?
what can be put to sleep?
what needs to be awakened?
what needs to be rebuilt?
```

In the future, HAOS may work together with Butler.

Example:

```text
Butler -> collects logs, checks budgets, shows overloads
HAOS   -> applies an optimization mode
```

If the system is overloaded, HAOS may reduce quality more aggressively: lower particle count, reduce detail, update distant areas less often, or delay less important rebuild tasks.

If there is performance headroom, HAOS may loosen restrictions and increase quality.

The core idea is simple:

```text
performance <-> quality
```

HAOS is needed so this balance does not become a random set of hacks scattered across different files, but instead becomes a separate observable layer that can be tuned, extended, and tested.

---

## 6. DTO — Dormant Tick Observer

**DTO** — Dormant Tick Observer.

DTO is not a “super-algorithm” and not a separate artificial intelligence for the world.  
At the early stage, it is better understood as an activity discipline: a layer of rules that helps decide which parts of the world participate in updates right now, and which can temporarily sleep.

Simple idea:

```text
ACTIVE  = participates in ticks / changes
DORMANT = sleeps, but can be awakened
```

DTO is needed precisely because the project is hybrid.

In regular ECS logic, it is possible to simply enable and disable systems, components, or entities.  
But in Arden, activity may apply not only to an entity, but to different layers of the world:

- voxel storage;
- chunk;
- octochunk;
- region sector;
- DUN;
- object;
- runtime-interest zone;
- dirty/rebuild state;
- render/collider representation.

Because of that, DTO is more convenient to describe as a separate layer above ECS sleep/activity logic.  
It does not replace ECS, but defines rules: what is active, what can be skipped, and what should wake up when density changes, a player approaches, a collision happens, a tool operation runs, or a simulation request appears.

In the future, DTO may work together with HAOS:

```text
DTO  -> marks what is active / asleep
HAOS -> decides how often and how expensive it is to update
```

Real alternatives and close analogs:

- **sleeping bodies in physics** — a rigidbody goes to sleep if it does not move for a while;
- **dirty flags** — an object or chunk is marked as changed and requires rebuild;
- **active set / inactive set** — a system stores active objects separately from sleeping ones;
- **interest management** — a server or simulation updates only zones important to the player or camera;
- **tick throttling** — distant or less important objects are updated less frequently;
- **LOD / streaming zones** — data is loaded, simplified, or unloaded based on distance;
- **ECS scheduling filters** — systems run only on entities with the required components/states.

DTO in Arden is an attempt to collect these ideas into a clear internal project term.

The main role of DTO:

```text
do not update the whole world all the time,
but explicitly manage
what is active,
what is sleeping,
what became dirty,
and what should wake up.
```

It is not only an optimization.  
It is a discipline of world activity.

---

## 7. Render Layer

Render in Arden should not be the source of truth.

It does not decide what exists in the world.  
It shows representations of data that are already defined by the core, lab/world layer, or future runtime systems.

Basic scheme:

```text
Core / Lab / World data
-> representation pipeline
-> mesh / surface extraction
-> GPU buffers
-> materials / shaders
-> frame output
```

At the early stage, the main path is converting voxel / volume data into mesh representation, because Bevy and GPU pipelines work naturally with triangular geometry.

```text
Voxel / volume truth
-> triangulated mesh
-> render / collider / tools representation
```

But this does not mean the render layer must be hardwired to a single display method.

In the future, the representation may change:

- cuboid voxel mesh;
- rough surface mesh;
- smoothed surface;
- debug wireframe;
- LOD / simplified mesh;
- specialized representation for tools;
- separate experimental render path.

Because of that, the render layer should remain a replaceable module.

The main boundary:

```text
world/core/lab = what exists
mesh extraction = how to obtain shape
render = how to show it
```

If render starts owning world logic, tools, or UI, the system will quickly stick together: tools will depend on the visual layer, UI will bypass the core, and changing the rendering method will break storage and simulation.

The logic is partly similar to a mesh editor: data becomes shape, and shape is prepared for display.  
But in Arden, this should work as a runtime pipeline: with dirty/rebuild states, budget limits, debug visibility, and the ability to change representation without breaking the core.

The main role of the render layer:

```text
not to be the truth of the world,
but to be a replaceable representation of the truth of the world.
```

---

## 8. PFO — Presentation / Post Filter Layer

**PFO** can be understood as a future visual presentation layer above the Render Layer.

Render is responsible for preparing and outputting geometry:

```text
World / Core data
-> mesh / render representation
-> GPU draw
```

PFO is closer to the final frame:

```text
Core truth
-> Render data
-> PFO
-> Final frame
```

Its task is not to decide what exists in the world, but to control **how it appears to the observer**.

PFO can be a place for:

- post-processing;
- stylization;
- debug overlays;
- visibility filters;
- visual lenses;
- highlighting points of interest;
- effects around active / selected / inspected areas;
- surface presentation;
- particles and visual effects that do not change world truth.

A good example of PFO’s role is a situation where a point of interest does not change the world, but changes how the world is displayed:

```text
the object exists in core/lab/world
render builds its representation
PFO adds a visual lens / filter / highlight / effect
```

So PFO is not a new data source and not part of spatial truth.  
It is a replaceable presentation layer that can support render, debug tools, and visual analysis without interfering with core logic.

At the early stage, PFO can be treated as a conceptual placeholder layer.  
Its exact form will depend on the future render pipeline, debug needs, and visual style of the project.

---

## 9. Development Environment Around the Core

Arden is currently being built not as a finished game and not as a full editor, but as a development environment around the core.

The main task of the current stage is to build the internal foundation: spatial truth, addressing, voxel data storage, operation pipeline, debug tools, and minimal visual representation. All of this is needed so the core can not only be written, but also tested in a live scene.

The game layer appears later. It should not dictate the architecture of the core; it should connect on top of it as a testing periphery: a small environment where destruction, assets, physics, simple game logic, and future DUN scenarios can be tested.

---

### Core Debug Environment

**Core Debug Environment** is the current working laboratory for the core.

It is needed to check basic questions:

```text
where is the data?
how is it addressed?
what can be changed?
what became dirty?
what needs to be rebuilt?
what does the machine see?
what does the human see?
```

This layer includes:

```text
Core Debug Environment
├─ Spatial Truth
├─ Topology / Routing / Mapping
├─ RuntimePosition / DensityKey / FullRoute
├─ LabVoxelWorld
├─ Probe
├─ HUD
├─ Gizmos
├─ Selection
├─ Volume Backend
├─ Dirty / Rebuild state
├─ Save / Load snapshot
└─ Butler / logs / validation later
```

This is not a game scene in the usual sense.  
It is an instrumental environment where the core can be tested by hand: whether it truly understands space, changes, selections, volumes, and rebuild states.

HUD, gizmos, probe tools, and the lab scene are not there for decoration.  
They exist for core observability.

---

### 2.5D / Periphery Stack

**2.5D / Periphery Stack** is a future small game environment above the core.

It is not intended as the final game, but as the first applied layer where the core can be tested in more alive scenarios:

- destruction;
- simple assets;
- basic physics;
- object interaction;
- game logic;
- connection between core and render / physics / UI;
- future DUN scenarios.

Approximate composition:

```text
2.5D / Periphery Stack
├─ Camera / Player / Input
├─ Small game scene rules
├─ Simple assets
├─ Destruction tests
├─ DUN / UnitNode later
├─ Physics / Colliders
├─ Render / PFO
├─ Gameplay UI / HUD
├─ Materials / effects
└─ Save / scenario / gameplay loop
```

This layer can be considered periphery: it uses the core, tests it, and helps find weak points, but it should not become the source of architectural rules.

---

### Why Bottom-Up

Arden follows a **bottom-up** path: first the core, then the debug/lab environment, then a small game periphery, and only after that a more complex 3D representation, assets, and extended world logic.

If development starts immediately from a full 3D game scene, the project quickly begins accumulating private solutions:

```text
graphics,
assets,
UI,
physics,
controllers,
collisions,
game rules,
exceptions for one specific scene.
```

With that approach, it is easy to end up not with a hybrid core, but with a set of hacks for one specific game.

Because of that, the current strategy is simpler:

```text
first build a viable core,
then test it in a lab environment,
then connect a small game periphery,
and only after that expand the visual and gameplay layers.
```

---

## 10. Aspectrolog — Semantic Aspect Library

**Aspectrolog** is an early concept for a future library of world aspects.

Its task is to help describe materials, forms, properties, and behavioral traits through composition, instead of chaotic special-case code for every new strange object.

Aspectrolog can be understood as an editor/lab layer for semantic assembly of objects and materials.  
It is not part of the current MVP core and is not needed for the nearest stages, but it helps describe how Arden may work with more complex hybrid entities in the future.

Examples:

```text
Ice + Flammable + MeltingCycle = burning ice
Stone + Edible + Heavy = edible stone
Tree + Metal + GrowthCycle = iron tree
Gas + Metal + Volatile = gaseous metal
Crystal + Biological + GrowthCycle = biological crystal
```

The main idea is not to “magically invent” behavior, but to build an object from already described semantic traits.

```text
aspect = semantic trait
material = set of aspects
object = shape + material + state + rules
system = reads aspects and applies its own logic
```

Aspectrolog does not replace **EQ-Sim**, ECS, or gameplay logic.  
It only helps describe what properties an object, material, or world zone has, so that other systems can read those traits and apply their own rules.

A conditional connection with other layers:

```text
Spatial Truth = where it is
Voxel / Mesh data = what it consists of and how it is represented
Aspectrolog = what properties it has
EQ-Sim = how properties participate in behavior
HAOS = how properties may affect update / optimization mode
Butler = how to explain state and possible conflicts
```

In the future, Aspectrolog may be useful for:

- materials;
- procedural generation;
- describing unusual assets;
- EQ-Sim rules;
- HAOS hints;
- editor/lab tools;
- Butler diagnostics;
- checking conflicts between properties.

There are risks in this layer too: combinatorial explosion, conflicting properties, too-early editor UI, and attempts to replace real simulation with nice words.

Because of that, at the current stage Aspectrolog is considered a **raw, distant, and optional concept**.  
Its current role is to define a direction: in the future, Arden should be able to describe the meaning of objects in a disciplined way, without turning every new material or strange asset into a separate piece of manual code.

## Result

The **Concept** section serves as a starting point for understanding what Arden Engine is meant to become in the future.

It connects:

- architectural design (`/ARCHITECTURE`);
- MVP slices and working tests (`/MVP`);
- terminology and the internal project language (`/TERMS`);
- future runtime layers, tools, and game experiments.

The main desire is to build not just a set of technologies, but an environment that feels good to **touch, study, test, and destroy**.

Arden should be observable enough to understand, flexible enough to add new layers, and coherent enough so that destruction, materials, tools, visual representation, and internal logic do not feel cardboard-like or bolted on from the side.

The ideal goal is to reach a state where, when a new idea appears, the question is not:

```text
how do I even fit this in here?
```

but rather:

```text
I want to try this as soon as possible — the base environment is no longer in the way.
```

In terms of ambition, this sounds loud, and much of it is still a raw concept.  
But this kind of assembled environment — hybrid core, lab tools, observability, voxel/mesh connection, DUN, optimization, and future game periphery — feels interesting enough to keep analyzing, testing, and gradually building further.

This section will be updated and split into separate parts as the project evolves.

---

📚 [Back](./readme.md)

---
