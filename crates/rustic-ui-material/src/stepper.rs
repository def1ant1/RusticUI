//! Renderer for the Material stepper built on top of [`StepperState`](rustic_ui_headless::stepper::StepperState).
//!
//! Enterprises rely on the stepper to guide users through complex workflows
//! such as onboarding, checkout, or provisioning flows.  The headless state
//! machine already captures control semantics (linear vs non-linear traversal,
//! disabled islands, completion metadata).  This module translates that state
//! into deterministic theme-aware classes, ARIA attributes, and automation
//! selectors so adapters across React, Yew, Leptos, Dioxus, and Sycamore can
//! remain wafer thin.
//!
//! # Architecture notes
//!
//! * **Single renderer:** all frameworks call [`render_stepper`] which emits the
//!   themed classes, automation IDs, and ARIA wiring.  This removes repetitive
//!   string concatenation from adapters and guarantees parity between SSR and
//!   hydration.
//! * **Automation-first:** selectors follow the `rustic-*` convention defined in
//!   [`style_helpers`](crate::style_helpers).  Centralising the automation ID
//!   formatting means QA suites can trust the DOM contract regardless of which
//!   framework renders the widget.
//! * **Controlled vs uncontrolled flows:** [`StepperAdapterProps`] stores the
//!   headless [`StepperState`] by reference and documents how controlled hooks or
//!   uncontrolled constructors feed into the same renderer.  Adapters simply pass
//!   the props through, making parity tests straightforward.

use rustic_ui_headless::stepper::{StepStatus, StepperState};
use rustic_ui_styled_engine::{css_with_theme, Style};

use crate::style_helpers::{
    automation_data_attr, automation_id, component_marker, themed_class, EMPTY_SEGMENTS,
};

/// Aggregate render output consumed by framework adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepperRenderOutput {
    /// Scoped class applied to the outer `<ol>` wrapper.  Every framework shares
    /// this class so enterprise CSS overrides can target the workflow container
    /// without needing per-adapter knowledge.
    pub root_class: String,
    /// DOM attributes for the root element excluding the class.  This includes
    /// ARIA wiring, automation IDs, and bookkeeping flags such as
    /// `data-step-count`.
    pub root_attributes: Vec<(String, String)>,
    /// Metadata for each step trigger.
    pub steps: Vec<StepRenderStep>,
}

impl StepperRenderOutput {
    /// Helper returning the total step count.
    #[inline]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Metadata describing a single step trigger/label pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepRenderStep {
    /// Zero-based index of the step.
    pub index: usize,
    /// Semantic status used by adapters to drive iconography.
    pub status: StepStatus,
    /// Base container class applied to the `<li>` element.
    pub container_class: String,
    /// Classes applied to the clickable control (usually a `<button>`).  The
    /// vector contains both the shared base styling and the status-specific
    /// accent class so adapters can opt into `class="base status"` composition
    /// without manual concatenation.
    pub trigger_classes: Vec<String>,
    /// Classes applied to the textual label associated with the trigger.
    pub label_classes: Vec<String>,
    /// Attribute pairs for the trigger element.  ARIA metadata comes from the
    /// headless state machine while automation identifiers and status markers
    /// are appended by the renderer.
    pub trigger_attributes: Vec<(String, String)>,
    /// Automation ID unique to this step.
    pub automation_id: String,
    /// Automation data attribute used by QA suites to target the trigger.
    pub automation_data_attribute: String,
}

/// Adapter props shared across frameworks.
#[derive(Clone, Copy, Debug)]
pub struct StepperAdapterProps<'a> {
    /// Headless state describing the workflow.
    pub state: &'a StepperState,
    /// Optional caller supplied identifier appended to automation selectors.
    pub automation_id: Option<&'a str>,
    /// Optional ARIA label describing the workflow intent.
    pub aria_label: Option<&'a str>,
}

impl<'a> StepperAdapterProps<'a> {
    /// Construct props targeting the provided state.  Controlled integrations
    /// keep the state in a hook/signal while uncontrolled flows instantiate it
    /// per render.  Both scenarios forward the borrowed state to this renderer.
    #[inline]
    pub fn new(state: &'a StepperState) -> Self {
        Self {
            state,
            automation_id: None,
            aria_label: None,
        }
    }

    /// Attach a caller provided automation identifier so multiple steppers can
    /// co-exist on a page without selector collisions.
    #[inline]
    pub fn with_automation_id(mut self, automation_id: &'a str) -> Self {
        self.automation_id = Some(automation_id);
        self
    }

    /// Provide an accessible label for assistive technologies.
    #[inline]
    pub fn with_aria_label(mut self, aria_label: &'a str) -> Self {
        self.aria_label = Some(aria_label);
        self
    }
}

/// Render the Material stepper into theme-aware classes and automation hooks.
#[must_use]
pub fn render_stepper(props: &StepperAdapterProps<'_>) -> StepperRenderOutput {
    let root_class = themed_class(stepper_root_style());
    let root_id = automation_id("stepper", props.automation_id, EMPTY_SEGMENTS);
    let root_data_attr = automation_data_attr("stepper", ["root"]);

    let mut root_attributes = Vec::with_capacity(6);
    root_attributes.push(("id".into(), root_id.clone()));
    root_attributes.push((root_data_attr.clone(), String::from("container")));
    root_attributes.push(("data-component".into(), component_marker("stepper")));
    root_attributes.push((
        "data-step-count".into(),
        props.state.step_count().to_string(),
    ));
    root_attributes.push(("role".into(), String::from("group")));
    root_attributes.push((
        "aria-roledescription".into(),
        String::from("progress steps"),
    ));

    if let Some(label) = props.aria_label {
        root_attributes.push(("aria-label".into(), label.to_string()));
    }

    let mut active_dom_id = None;

    let container_class = themed_class(step_container_style());
    let trigger_base_class = themed_class(step_trigger_base_style());
    let label_base_class = themed_class(step_label_base_style());

    let mut steps = Vec::with_capacity(props.state.step_count());
    for index in 0..props.state.step_count() {
        let status = props.state.step_status(index);
        let trigger_status_class = themed_class(step_trigger_status_style(status));
        let label_status_class = themed_class(step_label_status_style(status));

        let step_segment = format!("step-{index}");
        let automation_id = automation_id("stepper", props.automation_id, [step_segment.clone()]);
        let automation_data_attribute =
            automation_data_attr("stepper", ["step", step_segment.as_str()]);

        if matches!(status, StepStatus::Active) {
            active_dom_id = Some(automation_id.clone());
        }

        let mut trigger_attributes = props
            .state
            .step_button_attributes(index)
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect::<Vec<_>>();
        trigger_attributes.push(("id".into(), automation_id.clone()));
        trigger_attributes.push((automation_data_attribute.clone(), String::from("trigger")));
        trigger_attributes.push(("data-status".into(), step_status_token(status)));

        steps.push(StepRenderStep {
            index,
            status,
            container_class: container_class.clone(),
            trigger_classes: vec![trigger_base_class.clone(), trigger_status_class],
            label_classes: vec![label_base_class.clone(), label_status_class],
            trigger_attributes,
            automation_id,
            automation_data_attribute,
        });
    }

    if let Some(active_id) = active_dom_id {
        root_attributes.push(("aria-activedescendant".into(), active_id));
    }

    StepperRenderOutput {
        root_class,
        root_attributes,
        steps,
    }
}

fn step_status_token(status: StepStatus) -> String {
    match status {
        StepStatus::Pending => String::from("pending"),
        StepStatus::Active => String::from("active"),
        StepStatus::Completed => String::from("completed"),
        StepStatus::Disabled => String::from("disabled"),
    }
}

fn stepper_root_style() -> Style {
    css_with_theme! {
        r#"
        display: flex;
        flex-direction: column;
        gap: ${gap};
        padding: ${padding};
        list-style: none;
        counter-reset: rustic-step;
        "#,
        gap = format!("{}px", theme.spacing(2)),
        padding = format!("{}px", theme.spacing(1)),
    }
}

fn step_container_style() -> Style {
    css_with_theme! {
        r#"
        display: flex;
        align-items: flex-start;
        gap: ${gap};
        position: relative;
        padding-inline-start: ${padding};
        "#,
        gap = format!("{}px", theme.spacing(1)),
        padding = format!("{}px", theme.spacing(1)),
    }
}

fn step_trigger_base_style() -> Style {
    css_with_theme! {
        r#"
        appearance: none;
        border: none;
        background: transparent;
        padding: ${padding_y} ${padding_x};
        text-align: left;
        font: inherit;
        cursor: pointer;
        color: ${text};
        display: inline-flex;
        align-items: center;
        gap: ${gap};
        "#,
        padding_y = format!("{}px", theme.spacing(1) / 2),
        padding_x = format!("{}px", theme.spacing(1)),
        gap = format!("{}px", theme.spacing(1)),
        text = theme.palette.text_primary.clone(),
    }
}

fn step_trigger_status_style(status: StepStatus) -> Style {
    match status {
        StepStatus::Pending => css_with_theme! {
            r#"
            opacity: 0.64;
            "#
        },
        StepStatus::Active => css_with_theme! {
            r#"
            font-weight: ${weight};
            color: ${color};
            "#,
            weight = theme.typography.font_weight_medium.to_string(),
            color = theme.palette.primary.clone(),
        },
        StepStatus::Completed => css_with_theme! {
            r#"
            color: ${color};
            text-decoration: none;
            "#,
            color = theme.palette.success.clone(),
        },
        StepStatus::Disabled => css_with_theme! {
            r#"
            color: ${color};
            cursor: not-allowed;
            opacity: 0.48;
            "#,
            color = theme.palette.text_secondary.clone(),
        },
    }
}

fn step_label_base_style() -> Style {
    css_with_theme! {
        r#"
        font-size: ${size};
        line-height: 1.4;
        margin: 0;
        color: ${color};
        "#,
        size = format!("{:.3}rem", theme.typography.body2),
        color = theme.palette.text_secondary.clone(),
    }
}

fn step_label_status_style(status: StepStatus) -> Style {
    match status {
        StepStatus::Pending => css_with_theme! { r#""# },
        StepStatus::Active => css_with_theme! {
            r#"
            color: ${color};
            "#,
            color = theme.palette.primary.clone(),
        },
        StepStatus::Completed => css_with_theme! {
            r#"
            color: ${color};
            "#,
            color = theme.palette.success.clone(),
        },
        StepStatus::Disabled => css_with_theme! {
            r#"
            color: ${color};
            "#,
            color = theme.palette.text_secondary.clone(),
        },
    }
}

/// Internal helper used by adapters to avoid duplicate rendering code.
#[cfg_attr(
    not(any(
        feature = "react",
        feature = "yew",
        feature = "leptos",
        feature = "dioxus",
        feature = "sycamore"
    )),
    allow(dead_code)
)]
fn render_stepper_with_props(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
    render_stepper(&props)
}

/// React adapter bridging to the shared renderer.
#[cfg(feature = "react")]
pub mod react {
    use super::*;

    /// Controlled React hooks can persist a [`StepperState`] across renders
    /// while uncontrolled flows rebuild the state each render.  Both scenarios
    /// pass the resulting [`StepperAdapterProps`] to this helper so SSR and
    /// client renders stay byte-for-byte identical.
    pub fn render(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
        super::render_stepper_with_props(props)
    }
}

/// Yew adapter forwarding to the canonical renderer.
#[cfg(feature = "yew")]
pub mod yew {
    use super::*;

    /// Yew components feed their headless [`StepperState`] (usually stored in a
    /// `UseStateHandle`) into this helper.  Uncontrolled demos can construct the
    /// state inline and pass the borrowed reference just as easily.
    pub fn render(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
        super::render_stepper_with_props(props)
    }
}

/// Leptos adapter mirroring the React/Yew contracts.
#[cfg(feature = "leptos")]
pub mod leptos {
    use super::*;

    /// Whether the `StepperState` lives in a reactive signal or a plain Rust
    /// variable, delegating to the shared renderer keeps SSR snapshots and
    /// client hydration aligned without duplicating automation logic.
    pub fn render(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
        super::render_stepper_with_props(props)
    }
}

/// Dioxus adapter following the same controlled/uncontrolled story.
#[cfg(feature = "dioxus")]
pub mod dioxus {
    use super::*;

    /// Dioxus renderers call into this helper during virtual DOM diffing so the
    /// automation selectors and ARIA wiring mirror SSR output precisely.
    pub fn render(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
        super::render_stepper_with_props(props)
    }
}

/// Sycamore adapter keeping parity with the other frameworks.
#[cfg(feature = "sycamore")]
pub mod sycamore {
    use super::*;

    /// Sycamore signals or plain values both project into [`StepperAdapterProps`]
    /// making it trivial to keep SSR baselines, hydration, and client renders in
    /// lockstep.
    pub fn render(props: StepperAdapterProps<'_>) -> StepperRenderOutput {
        super::render_stepper_with_props(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::stepper::{StepperChange, StepperConfig};

    #[test]
    fn root_attributes_include_automation_and_aria_metadata() {
        let state = StepperState::new(StepperConfig::enterprise_defaults(3));
        let props = StepperAdapterProps::new(&state).with_aria_label("Checkout flow");
        let render = render_stepper(&props);

        let class_present = !render.root_class.is_empty();
        assert!(class_present, "root class should be generated");

        let mut attr_map = std::collections::BTreeMap::new();
        for (key, value) in &render.root_attributes {
            attr_map.insert(key.as_str(), value.as_str());
        }

        assert_eq!(attr_map.get("aria-label"), Some(&"Checkout flow"));
        assert_eq!(attr_map.get("role"), Some(&"group"));
        assert!(attr_map.contains_key("data-step-count"));
        assert!(attr_map.contains_key("aria-activedescendant"));
    }

    #[test]
    fn step_triggers_include_status_and_automation_ids() {
        let mut state = StepperState::new(StepperConfig::enterprise_defaults(2));
        state.complete_active();
        let props = StepperAdapterProps::new(&state).with_automation_id("provisioning");
        let render = render_stepper(&props);

        assert_eq!(render.step_count(), 2);
        let first = &render.steps[0];
        assert_eq!(first.status, StepStatus::Completed);
        assert!(first
            .automation_id
            .starts_with("rustic-stepper-provisioning"));
        assert!(first
            .trigger_attributes
            .iter()
            .any(|(k, _)| k.starts_with("data-rustic-stepper-step")));
        assert!(first
            .trigger_attributes
            .iter()
            .any(|(k, v)| k == "data-status" && v == "completed"));
    }

    #[test]
    fn adapter_parity_matches_direct_renderer() {
        let mut state = StepperState::new(StepperConfig::enterprise_defaults(3));
        state.complete_active();
        let base = render_stepper(&StepperAdapterProps::new(&state));
        let via_adapter = super::render_stepper_with_props(StepperAdapterProps::new(&state));
        assert_eq!(base, via_adapter);
    }

    #[test]
    fn linear_progression_updates_active_dom_reference() {
        let mut state = StepperState::new(StepperConfig::enterprise_defaults(3));
        let initial = {
            let props = StepperAdapterProps::new(&state);
            render_stepper(&props)
        };
        let first_active = initial
            .root_attributes
            .iter()
            .find(|(k, _)| k == "aria-activedescendant")
            .map(|(_, v)| v.clone())
            .expect("active descendant present");

        let StepperChange { active, .. } = state.complete_active();
        assert_eq!(active, Some(1));
        let second = {
            let props = StepperAdapterProps::new(&state);
            render_stepper(&props)
        };
        let second_active = second
            .root_attributes
            .iter()
            .find(|(k, _)| k == "aria-activedescendant")
            .map(|(_, v)| v.clone())
            .expect("active descendant present");

        assert_ne!(first_active, second_active);
    }
}
