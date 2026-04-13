// src/dun/kind.rs
/// Типы DUN для MVP0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DunKind {
    /// Динамический воксельный контейнер (наш текущий куб).
    DynamicVoxel,
}
