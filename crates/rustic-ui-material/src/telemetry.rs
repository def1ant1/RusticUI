#![allow(dead_code)]
//! Shared telemetry helpers used across adapter modules.
//!
//! The utilities in this module keep instrumentation consistent across
//! frameworks. Adapters feed their render closures through
//! [`instrument_render`] which automatically enters the provided
//! [`tracing::Span`], executes success callbacks, and reports panics to
//! opt-in error hooks. The indirection eliminates repetitive plumbing in
//! each framework integration while giving enterprise applications a single
//! struct they can configure to bolt analytics, tracing, and error
//! collection into RusticUI widgets.
//!
//! In addition to the render lifecycle, the hooks surfaced here coordinate
//! enterprise observability pipelines. Central platforms wire
//! [`TelemetryHooks`] into selection controls and form inputs so analytics
//! beacons, focus transitions, state mutations, backend commit
//! acknowledgements, and panic handling all flow through a single
//! thread-safe delegate. Doing so keeps CI/CD and SecOps dashboards aligned
//! without forcing individual adapters to duplicate boilerplate each time a
//! new sink (for example, a data lake or compliance archive) comes online.

use std::collections::BTreeMap;
use std::fmt;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;

use tracing::{field, span, Level, Span};

/// Context describing the component instance currently being rendered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryDescriptorMetadata {
    /// Human readable label surfaced by the descriptor.
    pub label: String,
    /// Attribute snapshot captured at render time to aid debugging.
    pub attributes: BTreeMap<String, String>,
}

/// Context describing the component instance currently being rendered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryContext {
    /// Fully-qualified component identifier (e.g. module path).
    pub component: &'static str,
    /// Optional analytics identifier reported alongside telemetry events.
    pub analytics_id: Option<String>,
    /// Optional automation identifier associated with the rendered element.
    pub automation_id: Option<String>,
    /// Snapshot of descriptor metadata captured when entering the span.
    pub descriptor: Option<TelemetryDescriptorMetadata>,
}

impl TelemetryContext {
    /// Construct a new telemetry context for the supplied component.
    #[must_use]
    pub const fn new(component: &'static str) -> Self {
        Self {
            component,
            analytics_id: None,
            automation_id: None,
            descriptor: None,
        }
    }

    /// Attach an analytics identifier to the context.
    #[must_use]
    pub fn with_analytics(mut self, analytics_id: Option<String>) -> Self {
        self.analytics_id = analytics_id;
        self
    }

    /// Attach an automation identifier to the context.
    #[must_use]
    pub fn with_automation(mut self, automation_id: Option<String>) -> Self {
        self.automation_id = automation_id;
        self
    }

    /// Attach descriptor metadata to the context so downstream telemetry spans
    /// can reason about the rendered attributes without re-computing them.
    #[must_use]
    pub fn with_descriptor_metadata<I, K, V>(
        mut self,
        label: impl Into<String>,
        attributes: I,
    ) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut map = BTreeMap::new();
        for (key, value) in attributes {
            map.insert(key.into(), value.into());
        }
        self.descriptor = Some(TelemetryDescriptorMetadata {
            label: label.into(),
            attributes: map,
        });
        self
    }
}

/// Error payload reported to telemetry hooks when rendering panics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TelemetryError {
    /// Human readable description of the failure.
    pub message: String,
}

impl TelemetryError {
    /// Construct a telemetry error with the provided message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

fn span_id(span: &Span) -> Option<tracing::Id> {
    span.id()
}

fn span_option_eq(lhs: &Option<Span>, rhs: &Option<Span>) -> bool {
    match (lhs, rhs) {
        (Some(a), Some(b)) => span_id(a) == span_id(b),
        (None, None) => true,
        _ => false,
    }
}

fn callback_eq<T: ?Sized>(lhs: &Option<Arc<T>>, rhs: &Option<Arc<T>>) -> bool {
    match (lhs, rhs) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Normalised analytics payload broadcast to hooks so enterprise platforms can
/// emit a consistent schema regardless of which widget originated the event.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryAnalyticsPayload {
    /// Logical channel or stream name for the analytics beacon.
    pub channel: String,
    /// Arbitrary key/value metadata captured alongside the emission.
    pub properties: BTreeMap<String, String>,
}

/// Focus transition payload shared between adapters and observability sinks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryFocusPayload {
    /// Whether focus is currently active for the instrumented widget.
    pub focused: bool,
    /// Arbitrary key/value metadata mirroring component state when focus
    /// changed. This keeps downstream automation resilient to schema growth.
    pub properties: BTreeMap<String, String>,
}

/// State mutation payload surfaced whenever selection controls change value.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryStateChangePayload {
    /// Identifier describing the previous logical state.
    pub previous: String,
    /// Identifier describing the next logical state requested by the user.
    pub next: String,
    /// Arbitrary key/value metadata emitted with the transition.
    pub properties: BTreeMap<String, String>,
}

/// Commit acknowledgement payload invoked when a backend confirms persistence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryCommitPayload {
    /// Optional correlation identifier for distributed tracing.
    pub correlation_id: Option<String>,
    /// Arbitrary key/value metadata captured with the acknowledgement.
    pub properties: BTreeMap<String, String>,
}

/// Alias for analytics callbacks so adapters can downcast and share behaviour.
pub type TelemetryAnalyticsCallback =
    dyn Fn(TelemetryContext, TelemetryAnalyticsPayload) + Send + Sync + 'static;
/// Alias for focus transition callbacks.
pub type TelemetryFocusCallback =
    dyn Fn(TelemetryContext, TelemetryFocusPayload) + Send + Sync + 'static;
/// Alias for state change callbacks.
pub type TelemetryStateChangeCallback =
    dyn Fn(TelemetryContext, TelemetryStateChangePayload) + Send + Sync + 'static;
/// Alias for commit acknowledgement callbacks.
pub type TelemetryCommitCallback =
    dyn Fn(TelemetryContext, TelemetryCommitPayload) + Send + Sync + 'static;
/// Alias for error callbacks.
pub type TelemetryErrorCallback = dyn Fn(TelemetryContext, TelemetryError) + Send + Sync + 'static;

/// Configurable hooks invoked around adapter renders.
#[derive(Clone, Default)]
pub struct TelemetryHooks {
    /// Optional analytics identifier automatically applied when the adapter
    /// options do not specify one.
    pub analytics_id: Option<String>,
    /// Optional automation identifier automatically applied when adapter
    /// options do not specify one.
    pub automation_id: Option<String>,
    /// Optional tracing span entered for the duration of the render.
    pub span: Option<Span>,
    /// Callback executed when rendering succeeds.
    pub on_render: Option<Arc<dyn Fn(TelemetryContext) + Send + Sync + 'static>>,
    /// Callback executed whenever an analytics payload is emitted by the
    /// widget. Useful for batching events before forwarding to external sinks.
    pub on_analytics: Option<Arc<TelemetryAnalyticsCallback>>,
    /// Callback executed when focus transitions occur on the widget.
    pub on_focus_transition: Option<Arc<TelemetryFocusCallback>>,
    /// Callback executed when the widget transitions between logical states.
    pub on_state_change: Option<Arc<TelemetryStateChangeCallback>>,
    /// Callback executed when a backend acknowledges the requested mutation.
    pub on_commit_ack: Option<Arc<TelemetryCommitCallback>>,
    /// Callback executed when rendering panics.
    pub on_error: Option<Arc<TelemetryErrorCallback>>,
}

impl PartialEq for TelemetryHooks {
    fn eq(&self, other: &Self) -> bool {
        self.analytics_id == other.analytics_id
            && self.automation_id == other.automation_id
            && span_option_eq(&self.span, &other.span)
            && callback_eq(&self.on_render, &other.on_render)
            && callback_eq(&self.on_analytics, &other.on_analytics)
            && callback_eq(&self.on_focus_transition, &other.on_focus_transition)
            && callback_eq(&self.on_state_change, &other.on_state_change)
            && callback_eq(&self.on_commit_ack, &other.on_commit_ack)
            && callback_eq(&self.on_error, &other.on_error)
    }
}

impl fmt::Debug for TelemetryHooks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TelemetryHooks")
            .field("analytics_id", &self.analytics_id)
            .field("automation_id", &self.automation_id)
            .field("span", &self.span.as_ref().map(|_| "configured"))
            .field("on_render", &self.on_render.as_ref().map(|_| "callback"))
            .field(
                "on_analytics",
                &self.on_analytics.as_ref().map(|_| "callback"),
            )
            .field(
                "on_focus_transition",
                &self.on_focus_transition.as_ref().map(|_| "callback"),
            )
            .field(
                "on_state_change",
                &self.on_state_change.as_ref().map(|_| "callback"),
            )
            .field(
                "on_commit_ack",
                &self.on_commit_ack.as_ref().map(|_| "callback"),
            )
            .field("on_error", &self.on_error.as_ref().map(|_| "callback"))
            .finish()
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_string()
    } else {
        String::from("render panic")
    }
}

/// Execute the provided closure within the configured tracing span and
/// telemetry hooks.
#[must_use]
pub fn instrument_render<F, T>(hooks: &TelemetryHooks, context: TelemetryContext, render: F) -> T
where
    F: FnOnce() -> T,
{
    let analytics = context
        .analytics_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("n/a");
    let automation = context
        .automation_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("n/a");
    let descriptor_label = context
        .descriptor
        .as_ref()
        .map(|descriptor| descriptor.label.as_str())
        .unwrap_or("n/a");
    let descriptor_attribute_count = context
        .descriptor
        .as_ref()
        .map(|descriptor| descriptor.attributes.len())
        .unwrap_or_default();
    let span = hooks.span.clone().unwrap_or_else(|| {
        span!(
            Level::INFO,
            "rustic_ui_component",
            component = context.component,
            analytics_id = field::display(analytics),
            automation_id = field::display(automation),
            descriptor_label = field::display(descriptor_label),
            descriptor_attributes = descriptor_attribute_count
        )
    });
    let _guard = span.enter();
    match panic::catch_unwind(AssertUnwindSafe(render)) {
        Ok(output) => {
            if let Some(on_render) = &hooks.on_render {
                on_render(context.clone());
            }
            output
        }
        Err(payload) => {
            let message = panic_message(&*payload);
            let error = TelemetryError::new(message.clone());
            tracing::error!(
                component = context.component,
                analytics_id = analytics,
                automation_id = automation,
                descriptor_label = descriptor_label,
                descriptor_attributes = descriptor_attribute_count,
                message = %message,
                "adapter render panic"
            );
            if let Some(on_error) = &hooks.on_error {
                on_error(context.clone(), error);
            }
            panic::resume_unwind(payload);
        }
    }
}
