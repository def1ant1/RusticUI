use insta::assert_json_snapshot;
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use serde_json::{json, to_value};

#[test]
fn material_box_renderer_snapshot() {
    use rustic_ui_headless::r#box::{BoxRole, BoxState, BoxTokens};

    let tokens = BoxTokens {
        padding: ResponsiveValue::new(String::from("8px"))
            .with_override(Breakpoint::Md, String::from("24px"))
            .with_override(Breakpoint::Xl, String::from("48px")),
        margin: ResponsiveValue::new(String::from("0"))
            .with_override(Breakpoint::Lg, String::from("auto")),
        background: ResponsiveValue::from(String::from("var(--surface-elevated)")),
    };

    let state = BoxState::new(tokens, BreakpointConfig::material()).with_role(BoxRole::Region);
    let render = rustic_ui_material::render_box(&state);
    let attrs = state
        .attributes()
        .id("layout-shell")
        .class("rustic-box surface");

    let mut attr_pairs = Vec::new();
    if let Some((key, value)) = attrs.id_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.class_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    attr_pairs.push((attrs.role().0, attrs.role().1.to_string()));

    let html = format!(
        "<div {attrs} style=\"{style}\"></div>",
        attrs = attr_pairs
            .into_iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(" "),
        style = render.inline_style()
    );

    let snapshot = json!({
        "html": html,
        "css_variables": to_value(render.css_variables()).expect("css variables serializable"),
        "inline_style": render.inline_style(),
        "breakpoints": {
            "viewport_480": attrs.data_breakpoint(480).1,
            "viewport_1366": attrs.data_breakpoint(1366).1,
        },
    });

    assert_json_snapshot!("material_box_renderer", snapshot);
}

#[test]
fn material_container_renderer_snapshot() {
    use rustic_ui_headless::container::{ContainerRole, ContainerState, ContainerTokens};

    let tokens = ContainerTokens {
        max_width: ResponsiveValue::new(String::from("540px"))
            .with_override(Breakpoint::Md, String::from("960px"))
            .with_override(Breakpoint::Xl, String::from("1280px")),
        padding_inline: ResponsiveValue::new(String::from("16px"))
            .with_override(Breakpoint::Lg, String::from("32px")),
    };

    let state = ContainerState::new(tokens, BreakpointConfig::material())
        .with_role(ContainerRole::Presentation)
        .fixed(true);
    let render = rustic_ui_material::render_container(&state);
    let attrs = state
        .attributes()
        .id("content-hub")
        .class("rustic-container")
        .data_density("comfortable");

    let mut attr_pairs = Vec::new();
    if let Some((key, value)) = attrs.id_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.class_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.density_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.fixed() {
        attr_pairs.push((key, value.to_string()));
    }
    attr_pairs.push((attrs.role().0, attrs.role().1.to_string()));
    let breakpoint_attr = attrs.data_breakpoint(1280);
    attr_pairs.push((breakpoint_attr.0, breakpoint_attr.1.to_string()));

    let html = format!(
        "<section {attrs} style=\"{style}\"></section>",
        attrs = attr_pairs
            .into_iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(" "),
        style = render.inline_style()
    );

    let snapshot = json!({
        "html": html,
        "css_variables": to_value(render.css_variables()).expect("css variables serializable"),
        "inline_style": render.inline_style(),
        "breakpoints": {
            "viewport_768": attrs.data_breakpoint(768).1,
            "viewport_1600": attrs.data_breakpoint(1600).1,
        },
    });

    assert_json_snapshot!("material_container_renderer", snapshot);
}

#[test]
fn material_grid_renderer_snapshot() {
    use rustic_ui_headless::grid::{GridState, GridTokens};

    let tokens = GridTokens {
        columns: ResponsiveValue::new(2)
            .with_override(Breakpoint::Sm, 4)
            .with_override(Breakpoint::Lg, 6),
        column_gap: ResponsiveValue::new(String::from("16px"))
            .with_override(Breakpoint::Md, String::from("24px")),
        row_gap: ResponsiveValue::from(String::from("32px")),
    };

    let state = GridState::new(tokens, BreakpointConfig::material())
        .interactive()
        .dense(true);
    let render = rustic_ui_material::render_grid(&state);
    let attrs = state.attributes().id("grid-shell").class("rustic-grid");

    let mut attr_pairs = Vec::new();
    if let Some((key, value)) = attrs.id_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.class_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.data_dense() {
        attr_pairs.push((key, value.to_string()));
    }
    attr_pairs.push((attrs.role().0, attrs.role().1.to_string()));
    let breakpoint_attr = attrs.data_breakpoint(1440);
    attr_pairs.push((breakpoint_attr.0, breakpoint_attr.1.to_string()));

    let html = format!(
        "<div {attrs} style=\"{style}\"></div>",
        attrs = attr_pairs
            .into_iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(" "),
        style = render.inline_style()
    );

    let snapshot = json!({
        "html": html,
        "css_variables": to_value(render.css_variables()).expect("css variables serializable"),
        "inline_style": render.inline_style(),
        "breakpoints": {
            "viewport_640": attrs.data_breakpoint(640).1,
            "viewport_1920": attrs.data_breakpoint(1920).1,
        },
    });

    assert_json_snapshot!("material_grid_renderer", snapshot);
}

#[test]
fn material_stack_renderer_snapshot() {
    use rustic_ui_headless::stack::{StackDirection, StackRole, StackState, StackTokens};

    let tokens = StackTokens {
        direction: ResponsiveValue::new(StackDirection::Vertical)
            .with_override(Breakpoint::Lg, StackDirection::Horizontal),
        gap: ResponsiveValue::new(String::from("12px"))
            .with_override(Breakpoint::Sm, String::from("16px")),
        divider: ResponsiveValue::new(Some(String::from("1px solid var(--border)")))
            .with_override(Breakpoint::Md, None),
    };

    let state = StackState::new(tokens, BreakpointConfig::material()).with_role(StackRole::List);
    let render = rustic_ui_material::render_stack(&state);
    let attrs = state.attributes().id("stack-shell").class("rustic-stack");

    let mut attr_pairs = Vec::new();
    if let Some((key, value)) = attrs.id_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    if let Some((key, value)) = attrs.class_attr() {
        attr_pairs.push((key, value.to_string()));
    }
    attr_pairs.push((attrs.role().0, attrs.role().1.to_string()));

    let html = format!(
        "<ul {attrs} style=\"{style}\"></ul>",
        attrs = attr_pairs
            .into_iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>()
            .join(" "),
        style = render.inline_style()
    );

    let snapshot = json!({
        "html": html,
        "css_variables": to_value(render.css_variables()).expect("css variables serializable"),
        "inline_style": render.inline_style(),
        "breakpoints": {
            "viewport_480": attrs.data_direction(480).1,
            "viewport_1600": attrs.data_direction(1600).1,
        },
    });

    assert_json_snapshot!("material_stack_renderer", snapshot);
}
