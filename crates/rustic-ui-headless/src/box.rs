//! Responsive state for the headless `Box` primitive.
//!
//! The `Box` component is intentionally minimal – it simply wires design tokens
//! to DOM attributes without imposing structure.  Nevertheless enterprise
//! adapters still need deterministic rules for which spacing/background tokens
//! should apply at a given breakpoint and which ARIA role to expose.  This
//! module centralises that logic and mirrors the commentary-heavy
//! documentation style used throughout the crate so future contributors can
//! audit design trade-offs without spelunking through downstream adapters.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Enumerates the default roles supported by the Box primitive.
///
/// The default mirrors the pattern used by Material and Joy where a generic
/// container maps to a `group` role.  We expose presentation and region
/// variants so enterprise teams can tighten semantics when the box represents a
/// dedicated subsection of an application shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxRole {
    /// Default grouping semantics.  Works for anonymous layout wrappers.
    Group,
    /// Expose the element as a landmark region so screen readers can jump to it.
    Region,
    /// Presentation role for boxes that purely exist for styling/layout.
    Presentation,
}

impl BoxRole {
    /// Returns the ARIA role string that adapters should apply.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Group => "group",
            Self::Region => "region",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for BoxRole {
    fn default() -> Self {
        Self::Group
    }
}

/// Declarative responsive tokens that describe the Box surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxTokens {
    /// Padding token applied to the Box surface.
    pub padding: ResponsiveValue<String>,
    /// Margin token applied to the Box surface.
    pub margin: ResponsiveValue<String>,
    /// Background token – adapters can translate this into classes or inline styles.
    pub background: ResponsiveValue<String>,
}

impl BoxTokens {
    /// Helper constructing a token set with the same value for all breakpoints.
    pub fn uniform(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            padding: ResponsiveValue::from(value.clone()),
            margin: ResponsiveValue::from(String::new()),
            background: ResponsiveValue::from(String::new()),
        }
    }
}

/// Runtime Box state shared across framework adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxState {
    tokens: BoxTokens,
    breakpoints: BreakpointConfig,
    role: BoxRole,
}

impl BoxState {
    /// Create a new Box state using the supplied tokens and breakpoint config.
    pub fn new(tokens: BoxTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: BoxRole::default(),
        }
    }

    /// Override the default role while keeping fluent builder ergonomics.
    pub fn with_role(mut self, role: BoxRole) -> Self {
        self.role = role;
        self
    }

    /// Returns the configured breakpoint map.
    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    /// Evaluate the responsive tokens for a concrete viewport width.
    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> BoxEvaluation<'_> {
        let active = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(active)
    }

    /// Evaluate the responsive tokens for a known breakpoint.
    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> BoxEvaluation<'_> {
        BoxEvaluation {
            breakpoint,
            padding: self.tokens.padding.value_for(breakpoint),
            margin: self.tokens.margin.value_for(breakpoint),
            background: self.tokens.background.value_for(breakpoint),
            role: self.role,
        }
    }

    /// Returns a builder exposing ergonomic attribute helpers for adapters.
    #[inline]
    pub fn attributes(&self) -> BoxAttributes<'_> {
        BoxAttributes {
            state: self,
            id: None,
            class: None,
        }
    }
}

/// Result of evaluating the Box responsive tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxEvaluation<'a> {
    /// Breakpoint resolved for the current viewport.
    pub breakpoint: Breakpoint,
    /// Active padding token.
    pub padding: &'a String,
    /// Active margin token.
    pub margin: &'a String,
    /// Active background token.
    pub background: &'a String,
    /// Active role metadata.
    pub role: BoxRole,
}

/// Ergonomic attribute builder mirroring the approach used by other state machines.
#[derive(Debug, Clone)]
pub struct BoxAttributes<'a> {
    state: &'a BoxState,
    id: Option<&'a str>,
    class: Option<&'a str>,
}

impl<'a> BoxAttributes<'a> {
    /// Attach an id attribute.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Attach a class attribute used for styling tokens.
    #[inline]
    pub fn class(mut self, value: &'a str) -> Self {
        self.class = Some(value);
        self
    }

    /// Returns the computed role attribute tuple.
    #[inline]
    pub fn role(&self) -> (&'static str, &'static str) {
        ("role", self.state.role.as_str())
    }

    /// Returns a tuple describing the active breakpoint token for automation.
    #[inline]
    pub fn data_breakpoint(&self, viewport_width: u32) -> (&'static str, &'static str) {
        let breakpoint = self.state.breakpoints.active_at(viewport_width);
        ("data-breakpoint", breakpoint.as_token())
    }

    /// Returns the id tuple when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the class tuple when configured.
    #[inline]
    pub fn class_attr(&self) -> Option<(&'static str, &str)> {
        self.class.map(|value| ("class", value))
    }
}
