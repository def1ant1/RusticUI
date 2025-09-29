//! Material renderer for [`SpeedDialState`](rustic_ui_headless::speed_dial::SpeedDialState).
//!
//! The adapter centralises styling, automation markers, and analytics hooks so
//! React, Yew, Leptos, Dioxus, and Sycamore render identical markup.

use rustic_ui_headless::speed_dial::{
    SpeedDialListAttributes, SpeedDialState, SpeedDialTriggerAttributes,
};
use rustic_ui_styled_engine::{css_with_theme, Style};
use rustic_ui_utils::attributes_to_html;

use crate::style_helpers;

/// Describes the trigger button rendered for the speed dial.
#[derive(Debug, Clone)]
pub struct SpeedDialTriggerDescriptor<'a> {
    /// Optional DOM id applied to the trigger button.
    pub id: Option<&'a str>,
    /// Optional analytics identifier overriding the state configuration.
    pub analytics_tag: Option<&'a str>,
    /// Inner HTML rendered inside the trigger.
    pub content: &'a str,
}

/// Describes an action button rendered inside the floating list.
#[derive(Debug, Clone)]
pub struct SpeedDialActionDescriptor<'a> {
    /// Zero-based action index.
    pub index: usize,
    /// Optional DOM id applied to the action button.
    pub id: Option<&'a str>,
    /// Optional analytics identifier overriding the state configuration.
    pub analytics_tag: Option<&'a str>,
    /// Optional accessible label overriding the default.
    pub aria_label: Option<&'a str>,
    /// Inner HTML rendered inside the action.
    pub content: &'a str,
}

/// Adapter props shared across framework integrations.
#[derive(Debug, Clone)]
pub struct SpeedDialAdapterProps<'a> {
    /// Headless state describing open/highlight semantics and analytics hooks.
    pub state: &'a SpeedDialState,
    /// Trigger attribute builder returned by [`SpeedDialState::trigger_attributes`].
    pub trigger_attributes: SpeedDialTriggerAttributes<'a>,
    /// List attribute builder returned by [`SpeedDialState::list_attributes`].
    pub list_attributes: SpeedDialListAttributes<'a>,
    /// Trigger descriptor.
    pub trigger: SpeedDialTriggerDescriptor<'a>,
    /// Action descriptors rendered inside the dial.
    pub actions: &'a [SpeedDialActionDescriptor<'a>],
    /// Optional event channel surfaced via `data-on-activate` when actions fire.
    pub on_action_event: Option<&'a str>,
    /// Optional event channel surfaced via `data-on-toggle` when the dial opens/closes.
    pub on_toggle_event: Option<&'a str>,
}

/// Rendered speed dial output.
#[derive(Debug, Clone)]
pub struct SpeedDialRenderOutput {
    container_attributes: Vec<(String, String)>,
    trigger_attributes: Vec<(String, String)>,
    list_attributes: Vec<(String, String)>,
    actions: Vec<SpeedDialRenderedAction>,
    trigger_content: String,
}

impl SpeedDialRenderOutput {
    /// Attributes applied to the outer container.
    pub fn container_attributes(&self) -> &[(String, String)] {
        &self.container_attributes
    }

    /// Attributes applied to the trigger button.
    pub fn trigger_attributes(&self) -> &[(String, String)] {
        &self.trigger_attributes
    }

    /// Attributes applied to the action list.
    pub fn list_attributes(&self) -> &[(String, String)] {
        &self.list_attributes
    }

    /// Rendered actions.
    pub fn actions(&self) -> &[SpeedDialRenderedAction] {
        &self.actions
    }

    /// Inner HTML rendered inside the trigger.
    pub fn trigger_content(&self) -> &str {
        &self.trigger_content
    }
}

/// Rendered action metadata.
#[derive(Debug, Clone)]
pub struct SpeedDialRenderedAction {
    container_attributes: Vec<(String, String)>,
    button_attributes: Vec<(String, String)>,
    content: String,
}

impl SpeedDialRenderedAction {
    /// Attributes applied to the `<li>` wrapper.
    pub fn container_attributes(&self) -> &[(String, String)] {
        &self.container_attributes
    }

    /// Attributes applied to the `<button>`.
    pub fn button_attributes(&self) -> &[(String, String)] {
        &self.button_attributes
    }

    /// Inner HTML rendered inside the action.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Render the speed dial into attribute maps.
#[must_use]
pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> SpeedDialRenderOutput {
    let container_attributes = build_container_attributes(props.on_toggle_event);
    let trigger_attributes =
        build_trigger_attributes(props.state, props.trigger_attributes, &props.trigger);
    let list_attributes = build_list_attributes(props.list_attributes, props.on_action_event);
    let actions = props
        .actions
        .iter()
        .map(|descriptor| build_action(props.state, descriptor))
        .collect();

    SpeedDialRenderOutput {
        container_attributes,
        trigger_attributes,
        list_attributes,
        actions,
        trigger_content: props.trigger.content.to_string(),
    }
}

/// Render the speed dial into HTML markup.
#[must_use]
pub fn render_speed_dial_html(props: SpeedDialAdapterProps<'_>) -> String {
    let rendered = render_speed_dial(props);
    let trigger_html = format!(
        "<button {attrs}>{content}</button>",
        attrs = attributes_to_html(rendered.trigger_attributes()),
        content = rendered.trigger_content()
    );
    let actions_html = rendered
        .actions()
        .iter()
        .map(|action| {
            let button_html = format!(
                "<button {attrs}>{content}</button>",
                attrs = attributes_to_html(action.button_attributes()),
                content = action.content()
            );
            format!(
                "<li {attrs}>{button}</li>",
                attrs = attributes_to_html(action.container_attributes()),
                button = button_html
            )
        })
        .collect::<String>();
    let list_html = format!(
        "<ul {attrs}>{items}</ul>",
        attrs = attributes_to_html(rendered.list_attributes()),
        items = actions_html
    );

    format!(
        "<div {attrs}>{trigger}{list}</div>",
        attrs = attributes_to_html(rendered.container_attributes()),
        trigger = trigger_html,
        list = list_html
    )
}

fn build_container_attributes(on_toggle_event: Option<&str>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(4);
    pairs.push((
        "data-component".into(),
        style_helpers::component_marker("speed-dial"),
    ));
    pairs.push((
        style_helpers::automation_data_attr("speed-dial", ["root"]),
        "true".into(),
    ));
    if let Some(event) = on_toggle_event {
        pairs.push(("data-on-toggle".into(), event.into()));
    }
    style_helpers::themed_attributes(speed_dial_container_style(), pairs)
}

fn build_trigger_attributes(
    state: &SpeedDialState,
    attrs: SpeedDialTriggerAttributes<'_>,
    descriptor: &SpeedDialTriggerDescriptor<'_>,
) -> Vec<(String, String)> {
    let mut builder = attrs;
    if let Some(id) = descriptor.id {
        builder = builder.id(id);
    }
    if let Some(tag) = descriptor.analytics_tag {
        builder = builder.analytics_tag(tag);
    }
    let pairs: Vec<(String, String)> = builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let mut attributes = style_helpers::themed_attributes(speed_dial_trigger_style(), pairs);
    attributes.push((
        "data-component".into(),
        style_helpers::component_marker("speed-dial-trigger"),
    ));
    attributes.push((
        "data-state".into(),
        if state.is_open() { "open" } else { "closed" }.into(),
    ));
    attributes
}

fn build_list_attributes(
    attrs: SpeedDialListAttributes<'_>,
    on_action_event: Option<&str>,
) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = attrs
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    if let Some(event) = on_action_event {
        pairs.push(("data-on-activate".into(), event.into()));
    }
    style_helpers::themed_attributes(speed_dial_list_style(), pairs)
}

fn build_action(
    state: &SpeedDialState,
    descriptor: &SpeedDialActionDescriptor<'_>,
) -> SpeedDialRenderedAction {
    let mut container_pairs: Vec<(String, String)> = Vec::with_capacity(6);
    container_pairs.push((
        style_helpers::automation_data_attr("speed-dial", ["item", &descriptor.index.to_string()]),
        "true".into(),
    ));
    container_pairs.push((
        "data-component".into(),
        style_helpers::component_marker("speed-dial-item"),
    ));
    let container_attributes =
        style_helpers::themed_attributes(speed_dial_item_style(), container_pairs);

    let mut builder = state.action_attributes(descriptor.index);
    if let Some(id) = descriptor.id {
        builder = builder.id(id);
    }
    if let Some(tag) = descriptor.analytics_tag {
        builder = builder.analytics_tag(tag);
    }
    if let Some(label) = descriptor.aria_label {
        builder = builder.aria_label(label);
    }
    let pairs: Vec<(String, String)> = builder
        .as_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    let mut button_attributes =
        style_helpers::themed_attributes(speed_dial_action_button_style(), pairs);
    button_attributes.push((
        "data-component".into(),
        style_helpers::component_marker("speed-dial-action"),
    ));

    SpeedDialRenderedAction {
        container_attributes,
        button_attributes,
        content: descriptor.content.to_string(),
    }
}

fn speed_dial_container_style() -> Style {
    css_with_theme!(
        r#"
        position: relative;
        display: inline-flex;
        flex-direction: column;
        align-items: center;
        gap: ${gap};
    "#,
        gap = theme.spacing(1)
    )
}

fn speed_dial_trigger_style() -> Style {
    css_with_theme!(
        r#"
        border: none;
        border-radius: 999px;
        width: 56px;
        height: 56px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background: ${background};
        color: ${color};
        box-shadow: ${shadow};
        cursor: pointer;
        transition: transform 150ms ease, box-shadow 150ms ease;
        &[data-state="open"] {
            transform: rotate(45deg);
        }
    "#,
        background = theme.palette.primary.clone(),
        color = theme.palette.background_paper.clone(),
        shadow = theme.joy.shadow.surface.clone()
    )
}

fn speed_dial_list_style() -> Style {
    css_with_theme!(
        r#"
        list-style: none;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column-reverse;
        gap: ${gap};
        position: absolute;
        bottom: 64px;
        right: 0;
    "#,
        gap = theme.spacing(1)
    )
}

fn speed_dial_item_style() -> Style {
    css_with_theme!(
        r#"
        display: inline-flex;
        align-items: center;
        justify-content: flex-end;
    "#
    )
}

fn speed_dial_action_button_style() -> Style {
    css_with_theme!(
        r#"
        border: none;
        border-radius: 50%;
        width: 40px;
        height: 40px;
        background: ${background};
        color: ${color};
        display: inline-flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: transform 120ms ease, box-shadow 120ms ease;
        &:hover {
            transform: translateY(-2px);
            box-shadow: ${shadow};
        }
        &[data-disabled="true"] {
            cursor: default;
            opacity: 0.5;
            box-shadow: none;
        }
    "#,
        background = theme.palette.secondary.clone(),
        color = theme.palette.background_paper.clone(),
        shadow = theme.joy.shadow.surface.clone()
    )
}

/// React adapter producing deterministic HTML.
pub mod react {
    use super::*;

    /// Render the speed dial for React SSR pipelines.
    pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> String {
        super::render_speed_dial_html(props)
    }
}

/// Yew adapter mirroring the React implementation.
pub mod yew {
    use super::*;

    /// Render the speed dial into HTML markup.
    pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> String {
        super::render_speed_dial_html(props)
    }
}

/// Leptos adapter delegating to the shared renderer.
pub mod leptos {
    use super::*;

    /// Render the speed dial into HTML markup.
    pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> String {
        super::render_speed_dial_html(props)
    }
}

/// Sycamore adapter reusing the shared HTML renderer.
pub mod sycamore {
    use super::*;

    /// Render the speed dial into HTML markup.
    pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> String {
        super::render_speed_dial_html(props)
    }
}

/// Dioxus adapter mirroring every other integration.
pub mod dioxus {
    use super::*;

    /// Render the speed dial into HTML markup.
    pub fn render_speed_dial(props: SpeedDialAdapterProps<'_>) -> String {
        super::render_speed_dial_html(props)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustic_ui_headless::ControlStrategy;

    fn build_state() -> SpeedDialState {
        SpeedDialState::new(
            3,
            false,
            ControlStrategy::Uncontrolled,
            ControlStrategy::Uncontrolled,
        )
    }

    fn build_props<'a>(state: &'a SpeedDialState) -> SpeedDialAdapterProps<'a> {
        SpeedDialAdapterProps {
            state,
            trigger_attributes: state.trigger_attributes(),
            list_attributes: state.list_attributes(),
            trigger: SpeedDialTriggerDescriptor {
                id: Some("fab"),
                analytics_tag: Some("primary"),
                content: "<span>+</span>",
            },
            actions: &[
                SpeedDialActionDescriptor {
                    index: 0,
                    id: Some("action-0"),
                    analytics_tag: Some("create"),
                    aria_label: Some("Create document"),
                    content: "<span>D</span>",
                },
                SpeedDialActionDescriptor {
                    index: 1,
                    id: Some("action-1"),
                    analytics_tag: Some("upload"),
                    aria_label: Some("Upload"),
                    content: "<span>U</span>",
                },
                SpeedDialActionDescriptor {
                    index: 2,
                    id: Some("action-2"),
                    analytics_tag: Some("share"),
                    aria_label: Some("Share"),
                    content: "<span>S</span>",
                },
            ],
            on_action_event: Some("speed-dial-action"),
            on_toggle_event: Some("speed-dial-toggle"),
        }
    }

    #[test]
    fn container_attributes_include_toggle_channel() {
        let state = build_state();
        let rendered = render_speed_dial(build_props(&state));
        assert!(rendered
            .container_attributes()
            .iter()
            .any(|(k, v)| k == "data-on-toggle" && v == "speed-dial-toggle"));
    }

    #[test]
    fn html_renderer_emits_trigger_and_actions() {
        let state = build_state();
        let html = render_speed_dial_html(build_props(&state));
        assert!(html.contains("<button"));
        assert!(html.matches("<li").count() >= 3);
    }
}
