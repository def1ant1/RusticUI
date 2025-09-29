//! Responsive visibility controller used by the `Hidden` primitive.
//!
//! Headless adapters often need to hide content on specific breakpoints while
//! keeping it available to assistive technology.  This module centralises the
//! responsive bookkeeping and provides ergonomic helpers for computing the
//! current visibility flag.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// ARIA role defaults for hidden regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HiddenRole {
    /// Presentation role when the element is purely structural.
    Presentation,
    /// Group role for content that still forms a logical grouping when visible.
    Group,
}

impl HiddenRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presentation => "presentation",
            Self::Group => "group",
        }
    }
}

impl Default for HiddenRole {
    fn default() -> Self {
        Self::Presentation
    }
}

/// State describing when an element should be hidden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenState {
    visibility: ResponsiveValue<bool>,
    breakpoints: BreakpointConfig,
    role: HiddenRole,
    inert: bool,
}

impl HiddenState {
    /// Create a new hidden state machine.
    pub fn new(visibility: ResponsiveValue<bool>, breakpoints: BreakpointConfig) -> Self {
        Self {
            visibility,
            breakpoints,
            role: HiddenRole::default(),
            inert: false,
        }
    }

    /// Override the role to something more descriptive.
    pub fn with_role(mut self, role: HiddenRole) -> Self {
        self.role = role;
        self
    }

    /// Configure whether hidden content should also be inert (aria-disabled + tabindex).
    pub fn inert(mut self, inert: bool) -> Self {
        self.inert = inert;
        self
    }

    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> HiddenEvaluation {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> HiddenEvaluation {
        let hidden = *self.visibility.value_for(breakpoint);
        HiddenEvaluation {
            breakpoint,
            hidden,
            role: self.role,
            inert: self.inert,
        }
    }

    #[inline]
    pub fn attributes(&self) -> HiddenAttributes<'_> {
        HiddenAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Result of resolving the hidden state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenEvaluation {
    pub breakpoint: Breakpoint,
    pub hidden: bool,
    pub role: HiddenRole,
    pub inert: bool,
}

/// Attribute builder for hidden containers.
#[derive(Debug, Clone)]
pub struct HiddenAttributes<'a> {
    state: &'a HiddenState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> HiddenAttributes<'a> {
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    #[inline]
    pub fn class(mut self, value: &'a str) -> Self {
        self.class = Some(value);
        self
    }

    #[inline]
    pub fn role(&self) -> (&'static str, &'static str) {
        ("role", self.state.role.as_str())
    }

    #[inline]
    pub fn hidden(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let evaluation = self.state.evaluate(viewport_width);
        (
            "data-hidden",
            if evaluation.hidden { "true" } else { "false" },
        )
    }

    #[inline]
    pub fn inert(&self) -> Option<(&'static str, &'static str)> {
        if self.state.inert {
            Some(("data-inert", "true"))
        } else {
            None
        }
    }

    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    #[inline]
    pub fn class_attr(&self) -> Option<(&'static str, &str)> {
        self.class.map(|value| ("class", value))
    }
}
