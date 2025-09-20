//! Tooltip orchestration state machine with deterministic timing.
//!
//! The implementation is intentionally verbose and heavily documented.  Enterprise
//! adopters frequently need to integrate with centralized automation platforms
//! where deterministic behaviour, rich instrumentation and ARIA compliant
//! attributes are mandatory.  The state machine therefore exposes explicit
//! events for hover/focus coordination, configurable show/hide timers and a
//! predictable polling API which adapters can call from `requestAnimationFrame`
//! or their rendering loop.  Framework bindings simply forward DOM events and
//! mirror the derived attributes which keeps the UI layer essentially
//! stateless.
//!
//! The Material wrappers described in
//! [`mui-material/README.md`](../mui-material/README.md#feedback-primitives-tooltip--chip)
//! reuse these primitives to emit SSR-ready HTML for Yew, Leptos, Dioxus, and
//! Sycamore adapters.  The `examples/feedback-tooltips` bootstrap demonstrates
//! how a single state machine instance can hydrate across frameworks while
//! preserving automation identifiers and portal metadata.

use crate::aria;
use crate::timing::{Clock, SystemClock, Timer};
use std::time::Duration;

/// Configuration describing how the tooltip reacts to interactions.
#[derive(Debug, Clone)]
pub struct TooltipConfig {
    /// Delay before the tooltip becomes visible after focus/hover.
    pub show_delay: Duration,
    /// Delay before the tooltip hides once the trigger is no longer active.
    pub hide_delay: Duration,
    /// Whether escape or programmatic dismiss calls should immediately hide.
    pub dismissible: bool,
    /// When `true` the tooltip remains visible while the pointer hovers the
    /// surface.  This mirrors the behaviour of interactive tooltips in MUI.
    pub interactive: bool,
}

impl TooltipConfig {
    /// Enterprise friendly defaults that mirror the behaviour of Material UI
    /// while giving enough breathing room for telemetry.
    pub fn enterprise_defaults() -> Self {
        Self {
            show_delay: Duration::from_millis(150),
            hide_delay: Duration::from_millis(100),
            dismissible: true,
            interactive: true,
        }
    }
}

impl Default for TooltipConfig {
    fn default() -> Self {
        Self::enterprise_defaults()
    }
}

/// Outcome of processing timer driven transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TooltipChange {
    /// `Some(true)` when visibility toggled on, `Some(false)` when hidden.
    pub visibility_changed: Option<bool>,
}

impl TooltipChange {
    fn merge(self, other: TooltipChange) -> TooltipChange {
        TooltipChange {
            visibility_changed: other.visibility_changed.or(self.visibility_changed),
        }
    }

    fn from_visibility(visible: bool) -> Self {
        Self {
            visibility_changed: Some(visible),
        }
    }
}

/// Tooltip state machine parameterised over a [`Clock`].
#[derive(Debug, Clone)]
pub struct TooltipState<C: Clock = SystemClock> {
    clock: C,
    config: TooltipConfig,
    visible: bool,
    anchor_focused: bool,
    anchor_hovered: bool,
    surface_hovered: bool,
    show_timer: Timer<C>,
    hide_timer: Timer<C>,
}

impl TooltipState<SystemClock> {
    /// Construct a tooltip using the real system clock.
    pub fn new(config: TooltipConfig) -> Self {
        Self::with_clock(SystemClock, config)
    }
}

impl<C: Clock> TooltipState<C> {
    /// Construct a tooltip using a custom clock (handy for tests and playback).
    pub fn with_clock(clock: C, config: TooltipConfig) -> Self {
        Self {
            clock,
            config,
            visible: false,
            anchor_focused: false,
            anchor_hovered: false,
            surface_hovered: false,
            show_timer: Timer::new(),
            hide_timer: Timer::new(),
        }
    }

    /// Returns whether the tooltip surface is currently visible.
    #[inline]
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Returns the configuration backing the tooltip.
    #[inline]
    pub fn config(&self) -> &TooltipConfig {
        &self.config
    }

    /// Event fired when the anchor element receives focus.
    pub fn focus_anchor(&mut self) -> TooltipChange {
        self.anchor_focused = true;
        self.queue_show()
    }

    /// Event fired when the anchor element loses focus.
    pub fn blur_anchor(&mut self) -> TooltipChange {
        self.anchor_focused = false;
        self.queue_hide()
    }

    /// Event fired when the pointer enters the anchor.
    pub fn pointer_enter_anchor(&mut self) -> TooltipChange {
        self.anchor_hovered = true;
        self.queue_show()
    }

    /// Event fired when the pointer leaves the anchor.
    pub fn pointer_leave_anchor(&mut self) -> TooltipChange {
        self.anchor_hovered = false;
        self.queue_hide()
    }

    /// Event fired when the pointer enters the tooltip surface.
    pub fn pointer_enter_tooltip(&mut self) -> TooltipChange {
        if self.config.interactive {
            self.surface_hovered = true;
            self.hide_timer.cancel();
        }
        self.resolve_timers()
    }

    /// Event fired when the pointer leaves the tooltip surface.
    pub fn pointer_leave_tooltip(&mut self) -> TooltipChange {
        if self.config.interactive {
            self.surface_hovered = false;
            return self.queue_hide();
        }
        TooltipChange::default()
    }

    /// Dismiss the tooltip due to escape key or automation command.
    pub fn dismiss(&mut self) -> TooltipChange {
        if !self.config.dismissible || !self.visible {
            return TooltipChange::default();
        }
        self.show_timer.cancel();
        self.hide_timer.cancel();
        self.visible = false;
        TooltipChange::from_visibility(false)
    }

    /// Poll for timer driven changes.  Frameworks call this from animation
    /// frames or async tasks to drive visibility transitions.
    pub fn poll(&mut self) -> TooltipChange {
        self.resolve_timers()
    }

    fn queue_show(&mut self) -> TooltipChange {
        self.hide_timer.cancel();
        if self.visible {
            self.show_timer.cancel();
            return TooltipChange::default();
        }
        if self.config.show_delay.is_zero() {
            self.visible = true;
            return TooltipChange::from_visibility(true);
        }
        self.show_timer
            .schedule(&self.clock, self.config.show_delay);
        self.resolve_timers()
    }

    fn queue_hide(&mut self) -> TooltipChange {
        if self.anchor_focused
            || self.anchor_hovered
            || (self.config.interactive && self.surface_hovered)
        {
            return TooltipChange::default();
        }
        self.show_timer.cancel();
        if !self.visible {
            self.hide_timer.cancel();
            return TooltipChange::default();
        }
        if self.config.hide_delay.is_zero() {
            self.visible = false;
            return TooltipChange::from_visibility(false);
        }
        self.hide_timer
            .schedule(&self.clock, self.config.hide_delay);
        self.resolve_timers()
    }

    fn resolve_timers(&mut self) -> TooltipChange {
        let mut change = TooltipChange::default();
        if self.show_timer.fire_if_due(&self.clock) && !self.visible {
            self.visible = true;
            change = change.merge(TooltipChange::from_visibility(true));
        }
        if self.hide_timer.fire_if_due(&self.clock) && self.visible {
            self.visible = false;
            change = change.merge(TooltipChange::from_visibility(false));
        }
        change
    }
}

/// Builder for trigger element attributes.
#[derive(Debug, Clone)]
pub struct TooltipTriggerAttributes<'a, C: Clock> {
    state: &'a TooltipState<C>,
    id: Option<&'a str>,
    described_by: Option<&'a str>,
    has_popup: Option<&'static str>,
}

impl<'a, C: Clock> TooltipTriggerAttributes<'a, C> {
    /// Construct a new attribute builder.
    pub fn new(state: &'a TooltipState<C>) -> Self {
        Self {
            state,
            id: None,
            described_by: None,
            has_popup: None,
        }
    }

    /// Provide an explicit ID for the trigger element.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Link the trigger to the tooltip using `aria-describedby`.
    #[inline]
    pub fn described_by(mut self, value: &'a str) -> Self {
        self.described_by = Some(value);
        self
    }

    /// Advertise a richer popup relationship such as `dialog` or `menu`.
    #[inline]
    pub fn has_popup(mut self, kind: &'static str) -> Self {
        self.has_popup = Some(kind);
        self
    }

    /// Returns the `id` attribute if configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-describedby` attribute.
    #[inline]
    pub fn describedby(&self) -> Option<(&'static str, &str)> {
        self.described_by.map(aria::aria_describedby)
    }

    /// Returns the `aria-haspopup` attribute.
    #[inline]
    pub fn haspopup(&self) -> Option<(&'static str, &'static str)> {
        self.has_popup.map(aria::aria_haspopup)
    }

    /// Returns the `aria-expanded` attribute reflecting visibility.
    #[inline]
    pub fn expanded(&self) -> (&'static str, &'static str) {
        aria::aria_expanded(self.state.visible())
    }
}

/// Builder for tooltip surface attributes.
#[derive(Debug, Clone)]
pub struct TooltipSurfaceAttributes<'a, C: Clock> {
    state: &'a TooltipState<C>,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
}

impl<'a, C: Clock> TooltipSurfaceAttributes<'a, C> {
    /// Construct a new builder referencing the tooltip state.
    pub fn new(state: &'a TooltipState<C>) -> Self {
        Self {
            state,
            id: None,
            labelled_by: None,
        }
    }

    /// Assign an ID so that triggers can reference this tooltip.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Provide an `aria-labelledby` relationship for automation.
    #[inline]
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Returns the `role="tooltip"` tuple.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_tooltip()
    }

    /// Returns the `id` attribute if configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-labelledby` attribute if configured.
    #[inline]
    pub fn labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Returns `aria-hidden` reflecting the surface visibility.
    #[inline]
    pub fn hidden(&self) -> (&'static str, &'static str) {
        aria::aria_hidden(!self.state.visible())
    }
}
