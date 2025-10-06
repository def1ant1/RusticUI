#![allow(dead_code)]
//! Headless state machine scaffold for `{{component_pascal}}`.
//!
//! Emitted by `cargo xtask new-component` to keep automation identifiers and
//! instrumentation notes centralised. Extend this file with the real state
//! transitions, events, and derived telemetry once product requirements land.

/// Canonical automation identifier used by both the headless and Material surfaces.
pub const {{component_shouty_snake}}_AUTOMATION_ROOT: &str = "{{automation_id}}";

/// Skeleton state container describing the minimum automation requirements.
#[derive(Debug, Clone)]
pub struct {{component_pascal}}State {
    /// Deterministic automation prefix replicated by adapters and documentation.
    pub automation_root: String,
    /// Inline notes captured during scaffolding so future maintainers understand
    /// the intended control flow and observability expectations.
    pub notes: Vec<&'static str>,
}

impl Default for {{component_pascal}}State {
    fn default() -> Self {
        Self {
            automation_root: {{component_shouty_snake}}_AUTOMATION_ROOT.to_string(),
            notes: vec![
                "Model focus/keyboard interactions mirroring existing headless components.",
                "Expose telemetry delegates that emit structured events before user callbacks fire.",
                "Document automation selectors inside docs/src/pages/system/components/{{component_kebab}}.mdx.",
            ],
        }
    }
}

impl {{component_pascal}}State {
    /// Placeholder builder that should eventually configure slot maps, event
    /// handlers, and SSR behaviour. Keeping the function body as `todo!()` lets
    /// the generator compile while flagging the missing implementation.
    pub fn build(self) {
        todo!("Design the headless state machine for {{component_pascal}} and thread automation IDs through each transition");
    }
}
