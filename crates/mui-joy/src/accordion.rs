//! Joy accordion scaffolding that wraps the shared headless state machine.
//!
//! Rendering adapters can construct [`AccordionController`] and wire its
//! [`AccordionGroupState`](mui_headless::accordion::AccordionGroupState) into
//! component templates.  Centralising the boilerplate here keeps future Joy
//! components focused on styling rather than state orchestration.

pub use mui_headless::accordion::{
    AccordionGroupState, AccordionItemChange,
};

/// Convenience wrapper around [`AccordionGroupState`] so Joy renderers can
/// instantiate accordions without touching the headless crate directly.
#[derive(Debug, Clone)]
pub struct AccordionController {
    /// Headless state machine powering the accordion group.
    pub state: AccordionGroupState,
}

impl AccordionController {
    /// Construct a new controller mirroring the `AccordionGroup` defaults from
    /// the TypeScript implementation.
    pub fn new(item_count: usize, allow_multiple: bool, default_expanded: &[usize]) -> Self {
        Self {
            state: AccordionGroupState::new(item_count, allow_multiple, default_expanded),
        }
    }
}
