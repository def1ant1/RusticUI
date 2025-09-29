//! Responsive Grid state mirroring Material's layout primitives.
//!
//! The goal is to keep downstream adapters declarative: they simply forward
//! viewport information (or let the server pick a default) and receive the
//! computed track counts and spacing tokens.  This prevents divergent logic
//! between React/Yew/Dioxus adapters while documenting the rationale behind the
//! defaults so large organisations can audit future changes.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Orientation aware role selection for grids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridRole {
    /// Treat the element as a generic grid (`role="grid"`).
    InteractiveGrid,
    /// Expose the element as a purely structural grid (`role="presentation"`).
    Presentation,
}

impl GridRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InteractiveGrid => "grid",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for GridRole {
    fn default() -> Self {
        Self::Presentation
    }
}

/// Responsive tokens controlling track counts and gaps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridTokens {
    /// Number of columns at each breakpoint.  Using `u16` keeps serialization small.
    pub columns: ResponsiveValue<u16>,
    /// Horizontal gap token.
    pub column_gap: ResponsiveValue<String>,
    /// Vertical gap token.
    pub row_gap: ResponsiveValue<String>,
}

impl GridTokens {
    /// Construct a grid token set with uniform gaps but responsive column counts.
    pub fn with_uniform_gaps(columns: ResponsiveValue<u16>, gap: impl Into<String>) -> Self {
        let gap = gap.into();
        Self {
            columns,
            column_gap: ResponsiveValue::from(gap.clone()),
            row_gap: ResponsiveValue::from(gap),
        }
    }
}

/// Grid state consumed by framework adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridState {
    tokens: GridTokens,
    breakpoints: BreakpointConfig,
    role: GridRole,
    dense: bool,
}

impl GridState {
    /// Instantiate the grid state machine.
    pub fn new(tokens: GridTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: GridRole::default(),
            dense: false,
        }
    }

    /// Switch to an interactive grid role so screen readers expect focusable children.
    pub fn interactive(mut self) -> Self {
        self.role = GridRole::InteractiveGrid;
        self
    }

    /// Enable CSS grid auto-placement density hints.
    pub fn dense(mut self, dense: bool) -> Self {
        self.dense = dense;
        self
    }

    /// Returns the configured breakpoint configuration.
    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    /// Evaluate tokens for a concrete viewport width.
    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> GridEvaluation<'_> {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    /// Evaluate tokens for the specified breakpoint.
    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> GridEvaluation<'_> {
        GridEvaluation {
            breakpoint,
            columns: *self.tokens.columns.value_for(breakpoint),
            column_gap: self.tokens.column_gap.value_for(breakpoint),
            row_gap: self.tokens.row_gap.value_for(breakpoint),
            role: self.role,
            dense: self.dense,
        }
    }

    /// Return an attribute builder for adapters.
    #[inline]
    pub fn attributes(&self) -> GridAttributes<'_> {
        GridAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Fully evaluated grid metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridEvaluation<'a> {
    pub breakpoint: Breakpoint,
    pub columns: u16,
    pub column_gap: &'a String,
    pub row_gap: &'a String,
    pub role: GridRole,
    pub dense: bool,
}

/// Attribute builder for grid surfaces.
#[derive(Debug, Clone)]
pub struct GridAttributes<'a> {
    state: &'a GridState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> GridAttributes<'a> {
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
    pub fn data_breakpoint(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let breakpoint = self.state.breakpoints.active_at(viewport_width);
        ("data-breakpoint", breakpoint.as_token())
    }

    #[inline]
    pub fn data_dense(&self) -> Option<(&'static str, &'static str)> {
        if self.state.dense {
            Some(("data-grid-density", "dense"))
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
