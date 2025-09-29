//! Responsive Stack state powering vertical/horizontal layout primitives.
//!
//! The Stack component orchestrates spacing between children and optionally
//! switches orientation per breakpoint.  Mirroring the approach used by the grid
//! and container modules we keep the bookkeeping centralised and heavily
//! documented so adapters remain declarative and predictable.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Orientation of the stack at a given breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    /// Children are laid out vertically.
    Vertical,
    /// Children are laid out horizontally.
    Horizontal,
}

impl StackDirection {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Default ARIA role for stacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackRole {
    /// Treat the stack as a semantic list of items.
    List,
    /// Anonymous grouping behaviour.
    Group,
    /// Presentation role when the stack is purely structural.
    Presentation,
}

impl StackRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Group => "group",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for StackRole {
    fn default() -> Self {
        Self::Group
    }
}

/// Responsive tokens controlling the stack layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackTokens {
    /// Responsive direction token (vertical vs. horizontal).
    pub direction: ResponsiveValue<StackDirection>,
    /// Gap token applied between children.
    pub gap: ResponsiveValue<String>,
    /// Optional divider token that adapters can map to border or pseudo elements.
    pub divider: ResponsiveValue<Option<String>>,
}

impl StackTokens {
    /// Build a stack that defaults to vertical orientation with a uniform gap.
    pub fn vertical(gap: impl Into<String>) -> Self {
        Self {
            direction: ResponsiveValue::from(StackDirection::Vertical),
            gap: ResponsiveValue::from(gap.into()),
            divider: ResponsiveValue::from(None),
        }
    }
}

/// Stack state shared across adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackState {
    tokens: StackTokens,
    breakpoints: BreakpointConfig,
    role: StackRole,
}

impl StackState {
    /// Create a new stack state.
    pub fn new(tokens: StackTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: StackRole::default(),
        }
    }

    /// Override the ARIA role.
    pub fn with_role(mut self, role: StackRole) -> Self {
        self.role = role;
        self
    }

    /// Returns the configured breakpoint configuration.
    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    /// Evaluate tokens for a concrete viewport width.
    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> StackEvaluation<'_> {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    /// Evaluate tokens for a specific breakpoint.
    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> StackEvaluation<'_> {
        StackEvaluation {
            breakpoint,
            direction: *self.tokens.direction.value_for(breakpoint),
            gap: self.tokens.gap.value_for(breakpoint),
            divider: self.tokens.divider.value_for(breakpoint),
            role: self.role,
        }
    }

    /// Returns an attribute builder mirroring other primitives.
    #[inline]
    pub fn attributes(&self) -> StackAttributes<'_> {
        StackAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Evaluated stack metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackEvaluation<'a> {
    pub breakpoint: Breakpoint,
    pub direction: StackDirection,
    pub gap: &'a String,
    pub divider: &'a Option<String>,
    pub role: StackRole,
}

/// Attribute builder for the stack root element.
#[derive(Debug, Clone)]
pub struct StackAttributes<'a> {
    state: &'a StackState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> StackAttributes<'a> {
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
    pub fn data_direction(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let breakpoint = self.state.breakpoints.active_at(viewport_width);
        let direction = self.state.tokens.direction.value_for(breakpoint).as_str();
        ("data-direction", direction)
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
