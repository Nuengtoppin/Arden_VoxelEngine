//! Lab module map.
//!
//! Step 2 = finite pre-DUN voxel sandbox.
//!
//! Responsibilities:
//! - `world`     = storage truth: DensityKey -> VoxelGrid, voxel/chunk read-write.
//! - `volume`    = volume operation backend: selection -> plan -> chunk/octo/frontier execution.
//! - `clipboard` = temporary payload layer: copy/paste selected voxel volumes.
//! - `probe`     = targeting only: camera, inspect, surface raymarch, pinned target.
//! - `selection` = select-box state and bounds only.
//! - `render`    = visualization hook outside lab: LabVoxelWorld -> mesh entities.
//! - `hud`       = state visibility/debug controls, not operation execution.
//! - `scene`     = Bevy resource/system wiring only.
//! - `save`      = lab snapshot serialization, not runtime storage policy.
//!
//! Hard rule:
//! Do not move planning/execution into `world.rs`.
//! Do not mutate world from `probe.rs` or `hud.rs`.
//! Render is never source of truth.
//! - `object`    = extracted local voxel objects / pre-DUN unit nodes.

pub mod clipboard;
pub mod formatters;
pub mod gizmos;
pub mod hud;
pub mod object;
pub mod probe;
pub mod sandbox;
pub mod save;
pub mod scene;
pub mod selection;
pub mod volume;
pub mod world;
