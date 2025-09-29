//! Shared responsive layout primitives.
//!
//! Layout driven components rely on similar responsive bookkeeping regardless of
//! whether they render a `Box`, a `Grid` or a complex image list.  Centralising
//! the breakpoint evaluation logic keeps adapters light-weight and guards
//! against divergent behaviour between frameworks.  The helpers in this module
//! embrace Material Design's breakpoint vocabulary while still being
//! customisable so enterprise teams can plug in organisation specific
//! thresholds.

use std::collections::BTreeMap;

/// Canonical breakpoints used across Material and Joy layout primitives.
///
/// The ordering matches the progression from the base layout up to the
/// ultra-wide `xxl` screens.  Deriving `Ord` allows us to reuse ordered maps
/// without additional boilerplate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Breakpoint {
    /// Base breakpoint covering the entire responsive range.
    Base,
    /// Small screens (`sm`) typically around 600px.
    Sm,
    /// Medium screens (`md`) such as tablets and small laptops.
    Md,
    /// Large screens (`lg`) used by desktop layouts.
    Lg,
    /// Extra large screens (`xl`) for wide displays.
    Xl,
    /// Extra-extra large screens (`xxl`) for command centres and dashboards.
    Xxl,
}

impl Breakpoint {
    /// Returns the canonical token name for CSS/automation mappings.
    #[inline]
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "xxl",
        }
    }
}

/// Declarative breakpoint thresholds evaluated by responsive components.
///
/// The configuration stores the minimum viewport width (in CSS pixels) required
/// for a breakpoint to become active.  The implementation intentionally keeps
/// the structure immutable to encourage sharing the same configuration across
/// multiple layout primitives.  This mirrors how design systems treat
/// breakpoints: they are organisational constants rather than per-component
/// tweaks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakpointConfig {
    thresholds: BTreeMap<Breakpoint, u32>,
}

impl BreakpointConfig {
    /// Construct a configuration from explicit breakpoint thresholds.
    ///
    /// The `Base` breakpoint is automatically populated with `0` to ensure the
    /// state machines always have a fallback value.  Additional breakpoints can
    /// be supplied using [`with_threshold`].
    pub fn new() -> Self {
        let mut thresholds = BTreeMap::new();
        thresholds.insert(Breakpoint::Base, 0);
        Self { thresholds }
    }

    /// Convenience constructor matching Material Design's recommended values.
    ///
    /// Enterprise deploys frequently tweak these, but providing a sensible
    /// default keeps examples and tests ergonomic while maintaining backwards
    /// compatibility with existing adapters.
    pub fn material() -> Self {
        Self::new()
            .with_threshold(Breakpoint::Sm, 600)
            .with_threshold(Breakpoint::Md, 900)
            .with_threshold(Breakpoint::Lg, 1200)
            .with_threshold(Breakpoint::Xl, 1536)
            .with_threshold(Breakpoint::Xxl, 1800)
    }

    /// Returns the minimum viewport width registered for a breakpoint when
    /// present.
    #[inline]
    pub fn threshold(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.thresholds.get(&breakpoint).copied()
    }

    /// Register or override the minimum width required for `breakpoint`.
    ///
    /// The API returns `self` so state builders can inline configuration without
    /// cloning temporary structures.
    pub fn with_threshold(mut self, breakpoint: Breakpoint, min_width: u32) -> Self {
        self.thresholds.insert(breakpoint, min_width);
        self
    }

    /// Returns an iterator over the configured breakpoints in ascending order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Breakpoint, u32)> + '_ {
        self.thresholds.iter().map(|(bp, width)| (*bp, *width))
    }

    /// Determine which breakpoint is active for the supplied viewport width.
    ///
    /// The method walks the ordered thresholds and returns the highest
    /// breakpoint with `min_width <= viewport_width`.  Retaining the iteration in
    /// Rust keeps the logic deterministic while remaining easy to port to other
    /// languages when adapters are implemented in TypeScript or Kotlin.
    pub fn active_at(&self, viewport_width: u32) -> Breakpoint {
        let mut active = Breakpoint::Base;
        for (breakpoint, min_width) in self.thresholds.iter() {
            if viewport_width >= *min_width && *breakpoint >= active {
                active = *breakpoint;
            }
        }
        active
    }
}

impl Default for BreakpointConfig {
    fn default() -> Self {
        Self::material()
    }
}

/// Responsive value with overrides per breakpoint.
///
/// The structure intentionally stores `T` by value so tokens can represent
/// strings, numbers or richer data structures.  Most state machines cache token
/// references, therefore the evaluation helpers return borrowed references to
/// avoid cloning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponsiveValue<T> {
    base: T,
    overrides: BTreeMap<Breakpoint, T>,
}

impl<T> ResponsiveValue<T> {
    /// Create a new responsive value with a base token.
    pub fn new(base: T) -> Self {
        Self {
            base,
            overrides: BTreeMap::new(),
        }
    }

    /// Register an override for `breakpoint`.
    pub fn with_override(mut self, breakpoint: Breakpoint, value: T) -> Self {
        self.overrides.insert(breakpoint, value);
        self
    }

    /// Returns the token that should apply for the active breakpoint.
    pub fn value_for(&self, active: Breakpoint) -> &T {
        self.overrides
            .iter()
            .filter(|(breakpoint, _)| **breakpoint <= active)
            .map(|(_, value)| value)
            .last()
            .unwrap_or(&self.base)
    }

    /// Convenience helper that accepts a viewport width and a configuration.
    pub fn resolve_with(&self, viewport_width: u32, config: &BreakpointConfig) -> &T {
        let active = config.active_at(viewport_width);
        self.value_for(active)
    }
}

impl<T> From<T> for ResponsiveValue<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
