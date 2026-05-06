#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugAction {
    ToggleOverlay,
    ToggleGizmos,
    CycleLensNext,
    CycleLensPrev,
    ToggleNotation,
    TogglePresentation,
    PinTarget,
    ClearPinnedTarget,

    SelectToolInspect,
    SelectToolSelectBox,
    SelectToolPaint,
    SelectToolErase,
}
