#![allow(dead_code)]
//! Material automation harness for `{{component_pascal}}`.
//!
//! Generated via `cargo xtask new-component` so teams get a documented starting
//! point that already captures automation identifiers, telemetry hooks, and the
//! docs stub location. The module intentionally favours narrative comments over
//! blank templates; the goal is to explain the intended architecture while
//! leaving room for framework-specific adapters.

/// Stable automation identifier prefix mirrored across adapters.
///
/// Updating this value requires synchronising the React/TypeScript adapters and
/// docs frontmatter generated alongside this file.
pub const {{component_shouty_snake}}_AUTOMATION_ROOT: &str = "{{automation_id}}";

/// Blueprint structure describing the Material surface under construction.
///
/// Replace the placeholder fields with the concrete props required by the
/// component (for example density, orientation, or data providers). Keeping a
/// strongly typed builder makes it straightforward to share the automation
/// contract with JavaScript adapters.
#[derive(Debug, Clone)]
pub struct {{component_pascal}}MaterialBlueprint {
    /// Deterministic automation identifier propagated into `data-*` hooks and
    /// SSR snapshots. This defaults to `{{automation_id}}` so hydration and
    /// analytics pipelines all reference a single source of truth.
    pub automation_root: String,
    /// TODO: extend with state specific configuration (slots, overrides, etc.).
    pub notes: Vec<&'static str>,
}

impl Default for {{component_pascal}}MaterialBlueprint {
    fn default() -> Self {
        Self {
            automation_root: {{component_shouty_snake}}_AUTOMATION_ROOT.to_string(),
            notes: vec![
                "Connect rustic_ui_headless state machine once available.",
                "Surface framework props for Yew, Leptos, Dioxus, Sycamore, and React adapters.",
                "Document telemetry hooks inside docs/src/pages/system/components/{{component_kebab}}.mdx.",
            ],
        }
    }
}

impl {{component_pascal}}MaterialBlueprint {
    /// Helper showing where to attach the headless state machine and theming
    /// adapters. Replace the `todo!()` call once the implementation is ready.
    pub fn build(self) {
        todo!("Implement Material adapters for {{component_pascal}} and update the docs stub");
    }
}
