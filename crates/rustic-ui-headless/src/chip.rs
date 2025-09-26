//! Chip state machine coordinating hover driven affordances and deletion.
//!
//! Similar to [`TooltipState`](crate::tooltip::TooltipState) the implementation
//! is richly documented so framework adapters and enterprise QA automation can
//! reason about every transition.  Chips are commonly used as filters and quick
//! actions inside complex dashboards which means we need deterministic logic for
//! when the trailing delete affordance appears, how long a pending deletion
//! lingers for animations, and how keyboard driven cancellation behaves.
//!
//! The Material layer consumes this state machine to render the automation-rich
//! chips documented in [`mui-material/README.md`](../rustic-ui-material/README.md#feedback-primitives-tooltip--chip)
//! and exercised in `examples/feedback-chips`.  Those demos pair dismissible and
//! read-only chips across Yew, Leptos, Dioxus, and Sycamore to validate that the
//! automation hooks exposed here survive SSR and hydration.

use crate::aria;
use crate::timing::{Clock, SystemClock, Timer};
use std::time::Duration;

/// Configuration describing how the chip behaves.
#[derive(Debug, Clone)]
pub struct ChipConfig {
    /// Delay before the trailing action fades in after hover/focus.
    pub show_delay: Duration,
    /// Delay before the trailing action hides once no longer hovered/focused.
    pub hide_delay: Duration,
    /// Grace period before a deletion is committed allowing exit animations.
    pub delete_delay: Duration,
    /// Whether the chip exposes a delete affordance at all.
    pub dismissible: bool,
    /// Whether the chip starts disabled.  Disabled chips ignore interactions.
    pub disabled: bool,
}

impl ChipConfig {
    /// Defaults optimised for enterprise dashboards.
    pub fn enterprise_defaults() -> Self {
        Self {
            show_delay: Duration::from_millis(120),
            hide_delay: Duration::from_millis(160),
            delete_delay: Duration::from_millis(200),
            dismissible: true,
            disabled: false,
        }
    }
}

impl Default for ChipConfig {
    fn default() -> Self {
        Self::enterprise_defaults()
    }
}

/// Aggregated change information emitted from state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChipChange {
    /// `Some(true)` when the trailing affordance becomes visible.
    pub controls_visible: Option<bool>,
    /// Set when the chip has been logically removed.
    pub deleted: bool,
    /// Set when a pending deletion was cancelled (e.g. escape key).
    pub deletion_cancelled: bool,
}

impl ChipChange {
    fn merge(mut self, other: ChipChange) -> ChipChange {
        if other.controls_visible.is_some() {
            self.controls_visible = other.controls_visible;
        }
        self.deleted |= other.deleted;
        self.deletion_cancelled |= other.deletion_cancelled;
        self
    }

    fn controls(visible: bool) -> Self {
        Self {
            controls_visible: Some(visible),
            ..Self::default()
        }
    }

    fn deleted() -> Self {
        Self {
            deleted: true,
            ..Self::default()
        }
    }

    fn cancelled() -> Self {
        Self {
            deletion_cancelled: true,
            ..Self::default()
        }
    }
}

/// Chip state machine built on top of the reusable [`Clock`] abstraction.
#[derive(Debug, Clone)]
pub struct ChipState<C: Clock = SystemClock> {
    clock: C,
    config: ChipConfig,
    controls_visible: bool,
    hovered: bool,
    focused: bool,
    deleting: bool,
    visible: bool,
    show_timer: Timer<C>,
    hide_timer: Timer<C>,
    delete_timer: Timer<C>,
}

impl ChipState<SystemClock> {
    /// Construct a chip using the system clock.
    pub fn new(config: ChipConfig) -> Self {
        Self::with_clock(SystemClock, config)
    }
}

impl<C: Clock> ChipState<C> {
    /// Construct a chip bound to a specific clock (mock clocks for tests).
    pub fn with_clock(clock: C, config: ChipConfig) -> Self {
        Self {
            clock,
            config,
            controls_visible: false,
            hovered: false,
            focused: false,
            deleting: false,
            visible: true,
            show_timer: Timer::new(),
            hide_timer: Timer::new(),
            delete_timer: Timer::new(),
        }
    }

    /// Returns whether the chip is currently visible (not yet deleted).
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns whether the trailing delete affordance is visible.
    #[inline]
    pub fn controls_visible(&self) -> bool {
        self.controls_visible
    }

    /// Returns whether a deletion is pending.
    #[inline]
    pub fn deletion_pending(&self) -> bool {
        self.deleting
    }

    /// Returns whether the chip is disabled.
    #[inline]
    pub fn disabled(&self) -> bool {
        self.config.disabled
    }

    /// Programmatically toggle the disabled flag.
    #[inline]
    pub fn set_disabled(&mut self, value: bool) {
        self.config.disabled = value;
    }

    /// Pointer entered the chip surface.
    pub fn pointer_enter(&mut self) -> ChipChange {
        if self.config.disabled || !self.visible {
            return ChipChange::default();
        }
        self.hovered = true;
        self.queue_show_controls()
    }

    /// Pointer left the chip surface.
    pub fn pointer_leave(&mut self) -> ChipChange {
        if self.config.disabled || !self.visible {
            return ChipChange::default();
        }
        self.hovered = false;
        self.queue_hide_controls()
    }

    /// Focus moved to the chip (keyboard navigation).
    pub fn focus(&mut self) -> ChipChange {
        if self.config.disabled || !self.visible {
            return ChipChange::default();
        }
        self.focused = true;
        self.queue_show_controls()
    }

    /// Focus moved away from the chip.
    pub fn blur(&mut self) -> ChipChange {
        if self.config.disabled || !self.visible {
            return ChipChange::default();
        }
        self.focused = false;
        self.queue_hide_controls()
    }

    /// Request deletion (triggered by trailing icon or keyboard Delete).
    pub fn request_delete(&mut self) -> ChipChange {
        if self.config.disabled || !self.visible || !self.config.dismissible {
            return ChipChange::default();
        }
        if self.deleting {
            return ChipChange::default();
        }
        self.deleting = true;
        self.hide_timer.cancel();
        if self.config.delete_delay.is_zero() {
            return self.commit_deletion();
        }
        self.delete_timer
            .schedule(&self.clock, self.config.delete_delay);
        self.resolve_timers()
    }

    /// Cancel a pending deletion (escape key or focus loss recovery).
    pub fn cancel_delete(&mut self) -> ChipChange {
        if !self.deleting {
            return ChipChange::default();
        }
        self.deleting = false;
        self.delete_timer.cancel();
        ChipChange::cancelled().merge(self.queue_hide_controls())
    }

    /// Escape key is treated as a delete cancellation followed by hide logic.
    pub fn escape(&mut self) -> ChipChange {
        let mut change = self.cancel_delete();
        change = change.merge(self.queue_hide_controls());
        change
    }

    /// Poll for timer driven transitions.
    pub fn poll(&mut self) -> ChipChange {
        self.resolve_timers()
    }

    fn queue_show_controls(&mut self) -> ChipChange {
        if !self.config.dismissible {
            return ChipChange::default();
        }
        self.hide_timer.cancel();
        if self.controls_visible {
            return ChipChange::default();
        }
        if self.config.show_delay.is_zero() {
            self.controls_visible = true;
            return ChipChange::controls(true);
        }
        self.show_timer
            .schedule(&self.clock, self.config.show_delay);
        self.resolve_timers()
    }

    fn queue_hide_controls(&mut self) -> ChipChange {
        if !self.config.dismissible {
            return ChipChange::default();
        }
        if self.focused || self.hovered || self.deleting {
            return ChipChange::default();
        }
        self.show_timer.cancel();
        if !self.controls_visible {
            self.hide_timer.cancel();
            return ChipChange::default();
        }
        if self.config.hide_delay.is_zero() {
            self.controls_visible = false;
            return ChipChange::controls(false);
        }
        self.hide_timer
            .schedule(&self.clock, self.config.hide_delay);
        self.resolve_timers()
    }

    fn commit_deletion(&mut self) -> ChipChange {
        self.delete_timer.cancel();
        self.deleting = false;
        if !self.visible {
            return ChipChange::deleted();
        }
        self.visible = false;
        self.controls_visible = false;
        self.show_timer.cancel();
        self.hide_timer.cancel();
        ChipChange::controls(false).merge(ChipChange::deleted())
    }

    fn resolve_timers(&mut self) -> ChipChange {
        let mut change = ChipChange::default();
        if self.show_timer.fire_if_due(&self.clock) && !self.controls_visible {
            self.controls_visible = true;
            change = change.merge(ChipChange::controls(true));
        }
        if self.hide_timer.fire_if_due(&self.clock) && self.controls_visible {
            self.controls_visible = false;
            change = change.merge(ChipChange::controls(false));
        }
        if self.delete_timer.fire_if_due(&self.clock) && self.deleting {
            change = change.merge(self.commit_deletion());
        }
        change
    }
}

/// Builder for ARIA attributes on the chip root element.
#[derive(Debug, Clone)]
pub struct ChipAttributes<'a, C: Clock> {
    state: &'a ChipState<C>,
    id: Option<&'a str>,
    labelled_by: Option<&'a str>,
    described_by: Option<&'a str>,
}

impl<'a, C: Clock> ChipAttributes<'a, C> {
    /// Construct a new builder for the chip root.
    pub fn new(state: &'a ChipState<C>) -> Self {
        Self {
            state,
            id: None,
            labelled_by: None,
            described_by: None,
        }
    }

    /// Supply an ID for linking the chip from analytics dashboards.
    #[inline]
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Attach an `aria-labelledby` attribute.
    #[inline]
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.labelled_by = Some(value);
        self
    }

    /// Attach an `aria-describedby` attribute for hint copy.
    #[inline]
    pub fn described_by(mut self, value: &'a str) -> Self {
        self.described_by = Some(value);
        self
    }

    /// Returns the `role="button"` tuple which is the recommended baseline.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_button()
    }

    /// Returns the `id` attribute when configured.
    #[inline]
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Returns the `aria-labelledby` attribute when configured.
    #[inline]
    pub fn labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Returns the `aria-describedby` attribute when configured.
    #[inline]
    pub fn describedby(&self) -> Option<(&'static str, &str)> {
        self.described_by.map(aria::aria_describedby)
    }

    /// Returns the `aria-disabled` attribute when the chip is disabled.
    #[inline]
    pub fn disabled(&self) -> Option<(&'static str, String)> {
        aria::aria_disabled(self.state.disabled())
    }

    /// Returns the `data-disabled` attribute when the chip is disabled.
    #[inline]
    pub fn data_disabled(&self) -> Option<(&'static str, String)> {
        aria::data_disabled(self.state.disabled())
    }

    /// Returns the `aria-hidden` attribute when the chip has been deleted.
    #[inline]
    pub fn hidden(&self) -> (&'static str, &'static str) {
        aria::aria_hidden(!self.state.is_visible())
    }
}

/// Builder for the delete button attributes.
#[derive(Debug, Clone)]
pub struct ChipDeleteAttributes<'a, C: Clock> {
    state: &'a ChipState<C>,
    label: Option<&'a str>,
}

impl<'a, C: Clock> ChipDeleteAttributes<'a, C> {
    /// Construct a new builder.
    pub fn new(state: &'a ChipState<C>) -> Self {
        Self { state, label: None }
    }

    /// Provide the `aria-label` describing the delete action.
    #[inline]
    pub fn label(mut self, value: &'a str) -> Self {
        self.label = Some(value);
        self
    }

    /// Returns the `role="button"` tuple.
    #[inline]
    pub fn role(&self) -> &'static str {
        aria::role_button()
    }

    /// Returns the `aria-hidden` attribute reflecting control visibility.
    #[inline]
    pub fn hidden(&self) -> (&'static str, &'static str) {
        aria::aria_hidden(!self.state.controls_visible())
    }

    /// Returns the `aria-label` tuple if provided.
    #[inline]
    pub fn aria_label(&self) -> Option<(&'static str, &str)> {
        self.label.map(|value| ("aria-label", value))
    }
}
