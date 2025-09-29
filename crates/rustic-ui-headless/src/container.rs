//! Responsive state for the layout `Container` primitive.
//!
//! Containers typically enforce max-widths and horizontal gutters that change at
//! each breakpoint.  Centralising the evaluation logic keeps SSR/CSR adapters in
//! sync and allows us to document the trade-offs around token resolution so
//! enterprise teams can reason about future extensions such as density modes or
//! design-token driven spacing overrides.

use crate::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};

/// Tokens describing how the container should behave across breakpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerTokens {
    /// Max-width token applied at each breakpoint.
    pub max_width: ResponsiveValue<String>,
    /// Horizontal padding token applied to keep content aligned.
    pub padding_inline: ResponsiveValue<String>,
}

impl ContainerTokens {
    /// Construct a container token set using the same padding across breakpoints.
    pub fn with_fixed_padding(
        max_width: ResponsiveValue<String>,
        padding: impl Into<String>,
    ) -> Self {
        Self {
            max_width,
            padding_inline: ResponsiveValue::from(padding.into()),
        }
    }
}

/// Default ARIA roles for containers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRole {
    /// Anonymous grouping role used for layout wrappers.
    Group,
    /// Presentation role when the container is purely structural.
    Presentation,
}

impl ContainerRole {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Group => "group",
            Self::Presentation => "presentation",
        }
    }
}

impl Default for ContainerRole {
    fn default() -> Self {
        Self::Group
    }
}

/// Container state shared across adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerState {
    tokens: ContainerTokens,
    breakpoints: BreakpointConfig,
    role: ContainerRole,
    fixed: bool,
}

impl ContainerState {
    /// Create a new container state with tokens and breakpoint configuration.
    pub fn new(tokens: ContainerTokens, breakpoints: BreakpointConfig) -> Self {
        Self {
            tokens,
            breakpoints,
            role: ContainerRole::default(),
            fixed: false,
        }
    }

    /// Mark the container as fixed width (matching Material's `containerFixed`).
    pub fn fixed(mut self, fixed: bool) -> Self {
        self.fixed = fixed;
        self
    }

    /// Override the ARIA role used by the container.
    pub fn with_role(mut self, role: ContainerRole) -> Self {
        self.role = role;
        self
    }

    /// Returns the configured breakpoint map.
    #[inline]
    pub fn breakpoints(&self) -> &BreakpointConfig {
        &self.breakpoints
    }

    /// Resolve the tokens for a specific viewport width.
    #[inline]
    pub fn evaluate(&self, viewport_width: u32) -> ContainerEvaluation<'_> {
        let breakpoint = self.breakpoints.active_at(viewport_width);
        self.evaluate_for(breakpoint)
    }

    /// Resolve the tokens for a concrete breakpoint.
    #[inline]
    pub fn evaluate_for(&self, breakpoint: Breakpoint) -> ContainerEvaluation<'_> {
        ContainerEvaluation {
            breakpoint,
            max_width: self.tokens.max_width.value_for(breakpoint),
            padding_inline: self.tokens.padding_inline.value_for(breakpoint),
            role: self.role,
            fixed: self.fixed,
        }
    }

    /// Returns a builder for ergonomic attribute access.
    #[inline]
    pub fn attributes(&self) -> ContainerAttributes<'_> {
        ContainerAttributes {
            state: self,
            id: None,
            class: None,
            data_density: None,
        }
    }
}

/// Evaluated container tokens and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerEvaluation<'a> {
    pub breakpoint: Breakpoint,
    pub max_width: &'a String,
    pub padding_inline: &'a String,
    pub role: ContainerRole,
    pub fixed: bool,
}

/// Attribute builder used by framework adapters.
#[derive(Debug, Clone)]
pub struct ContainerAttributes<'a> {
    state: &'a ContainerState,
    id: Option<&'a str>,
    class: Option<&'a str>,
    data_density: Option<&'a str>,
}

impl<'a> ContainerAttributes<'a> {
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

    /// Attach a density hint (compact, comfortable, etc.).  The state machine
    /// stays agnostic but exposes the hook to minimise duplicated wiring.
    #[inline]
    pub fn data_density(mut self, value: &'a str) -> Self {
        self.data_density = Some(value);
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
    pub fn fixed(&self) -> Option<(&'static str, &'static str)> {
        if self.state.fixed {
            Some(("data-fixed", "true"))
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

    #[inline]
    pub fn density_attr(&self) -> Option<(&'static str, &str)> {
        self.data_density.map(|value| ("data-density", value))
    }
}
