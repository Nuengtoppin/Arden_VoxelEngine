// src/dun/dun.rs
use bevy::prelude::*;

use crate::dun::kind::DunKind;
use crate::core::VoxelGrid;
/// Базовый компонент DUN
/// Хранит только то, что реально нужно сейчас.
#[derive(Component)]
pub struct Dun {
    /// Тип DUN (для будущего различения Dynamic / Static и т.п.).
    pub kind: DunKind,
    /// Логическая координата чанка, к которому привязан DUN.
    pub chunk_coord: IVec3,
    /// Локальная воксельная решётка внутри контейнера.
    pub voxel: VoxelGrid,
}
