//! Responsive Divider state centralising orientation and inset logic.
//!
//! Dividers bridge multiple layout primitives so the responsive behaviour needs
//! to stay deterministic across adapters.  The state machine exposes ARIA role
//! helpers (defaulting to `separator`) and responsive tokens for thickness,
//! insets and orientation.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Orientation of the divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// ARIA roles supported by the divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerRole {
    /// Default separator semantics.
    Separator,
    /// Presentation for purely visual dividers.
    Presentation,
}

impl DividerRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Separator => "separator",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for DividerRole {
    fn default() -> Self {
        Self::Separator
    }
}

/// Tokens driving responsive divider rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DividerTokens {
    /// Orientation overrides per breakpoint.
    pub orientation: ResponsiveValue<DividerOrientation>,
    /// Thickness token (e.g. `1px`, `2px`).
    pub thickness: ResponsiveValue<String>,
    /// Inset token controlling leading/trailing padding.
    pub inset: ResponsiveValue<String>,
}

impl DividerTokens {
    /// Build a horizontal divider with uniform tokens.
    pub fn horizontal(thickness: impl Into<String>) -> Self {
        let thickness = thickness.into();
        Self {
            orientation: ResponsiveValue::from(DividerOrientation::Horizontal),
            thickness: ResponsiveValue::from(thickness),
            inset: ResponsiveValue::from(String::new()),
        }
    }
}

/// Divider state shared across adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DividerState {
    tokens: DividerTokens,
    breakpoints: BreakpointConfig,
    role: DividerRole,
}

impl DividerState {
    /// Create a new divider state.
    pub fn new(tokens: DividerTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: DividerRole::default(),
        }
    }

    /// Override the ARIA role.
    pub fn with_role(mut self, role: DividerRole) -> Self {
        self.role = role;
        self
    }

    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> DividerEvaluation<'_> {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> DividerEvaluation<'_> {
        DividerEvaluation {
            breakpoint,
            orientation: *self.tokens.orientation.value_for(breakpoint),
            thickness: self.tokens.thickness.value_for(breakpoint),
            inset: self.tokens.inset.value_for(breakpoint),
            role: self.role,
        }
    }

    #[inline]
    pub fn attributes(&self) -> DividerAttributes<'_> {
        DividerAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Evaluated divider metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DividerEvaluation<'a> {
    pub breakpoint: Breakpoint,
    pub orientation: DividerOrientation,
    pub thickness: &'a String,
    pub inset: &'a String,
    pub role: DividerRole,
}

/// Attribute builder for divider surfaces.
#[derive(Debug, Clone)]
pub struct DividerAttributes<'a> {
    state: &'a DividerState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> DividerAttributes<'a> {
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
    pub fn data_orientation(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let breakpoint = self.state.breakpoints.active_at(viewport_width);
        let orientation = self.state.tokens.orientation.value_for(breakpoint).as_str();
        ("data-orientation", orientation)
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
