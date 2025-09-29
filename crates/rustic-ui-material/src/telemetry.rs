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

use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;

use tracing::{field, span, Level, Span};

/// Context describing the component instance currently being rendered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelemetryContext {
    /// Fully-qualified component identifier (e.g. module path).
    pub component: &'static str,
    /// Optional analytics identifier reported alongside telemetry events.
    pub analytics_id: Option<String>,
    /// Optional automation identifier associated with the rendered element.
    pub automation_id: Option<String>,
}

impl TelemetryContext {
    /// Construct a new telemetry context for the supplied component.
    #[must_use]
    pub const fn new(component: &'static str) -> Self {
        Self {
            component,
            analytics_id: None,
            automation_id: None,
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
    /// Callback executed when rendering panics.
    pub on_error: Option<Arc<dyn Fn(TelemetryContext, TelemetryError) + Send + Sync + 'static>>,
}

impl PartialEq for TelemetryHooks {
    fn eq(&self, other: &Self) -> bool {
        self.analytics_id == other.analytics_id
            && self.automation_id == other.automation_id
            && span_option_eq(&self.span, &other.span)
            && callback_eq(&self.on_render, &other.on_render)
            && callback_eq(&self.on_error, &other.on_error)
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
    let span = hooks.span.clone().unwrap_or_else(|| {
        span!(
            Level::INFO,
            "rustic_ui_component",
            component = context.component,
            analytics_id = field::display(analytics),
            automation_id = field::display(automation)
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
