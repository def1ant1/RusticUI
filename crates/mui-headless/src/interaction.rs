//! Shared interaction primitives used by selection control state machines.
//!
//! Keeping keyboard semantics centralized ensures that each state machine
//! interprets navigation keys consistently which is critical for WCAG
//! compliance across frameworks.

/// Keys relevant to selection controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKey {
    /// Corresponds to the <Space> key which toggles most controls.
    Space,
    /// Corresponds to the <Enter> key.
    Enter,
    /// Arrow pointing up.
    ArrowUp,
    /// Arrow pointing down.
    ArrowDown,
    /// Arrow pointing left.
    ArrowLeft,
    /// Arrow pointing right.
    ArrowRight,
    /// Jump focus to the first item in a group.
    Home,
    /// Jump focus to the last item in a group.
    End,
}

impl ControlKey {
    /// Returns whether the key is considered a forward navigation request when
    /// iterating across a set of options.
    pub fn is_forward(self) -> bool {
        matches!(self, Self::ArrowRight | Self::ArrowDown)
    }

    /// Returns whether the key is considered a backward navigation request when
    /// iterating across a set of options.
    pub fn is_backward(self) -> bool {
        matches!(self, Self::ArrowLeft | Self::ArrowUp)
    }
}
