use insta::assert_json_snapshot;
use rustic_ui_headless::layout::{Breakpoint, BreakpointConfig, ResponsiveValue};
use serde_json::json;

#[test]
fn box_breakpoint_evaluations_snapshot() {
    use rustic_ui_headless::r#box::{BoxRole, BoxState, BoxTokens};

    let tokens = BoxTokens {
        padding: ResponsiveValue::new(String::from("8px"))
            .with_override(Breakpoint::Md, String::from("16px"))
            .with_override(Breakpoint::Xl, String::from("32px")),
        margin: ResponsiveValue::new(String::from("0"))
            .with_override(Breakpoint::Lg, String::from("auto")),
        background: ResponsiveValue::from(String::from("var(--surface)")),
    };

    let state = BoxState::new(tokens, BreakpointConfig::material()).with_role(BoxRole::Region);
    let attrs = state
        .attributes()
        .id("analytics-shell")
        .class("rustic-box surface");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "padding": evaluation.padding,
                "margin": evaluation.margin,
                "background": evaluation.background,
                "role": evaluation.role.as_str(),
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "data_breakpoint@480": attrs.data_breakpoint(480).1,
            "data_breakpoint@1280": attrs.data_breakpoint(1280).1,
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_box_breakpoints", snapshot);
}

#[test]
fn container_breakpoint_evaluations_snapshot() {
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
    let attrs = state
        .attributes()
        .id("marketing-container")
        .class("rustic-container")
        .data_density("comfortable");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "max_width": evaluation.max_width,
                "padding_inline": evaluation.padding_inline,
                "fixed": evaluation.fixed,
                "role": evaluation.role.as_str(),
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "density": attrs.density_attr().map(|(_, value)| value.to_string()),
            "data_breakpoint@768": attrs.data_breakpoint(768).1,
            "data_breakpoint@1440": attrs.data_breakpoint(1440).1,
            "fixed": attrs.fixed().map(|(_, value)| value.to_string()),
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_container_breakpoints", snapshot);
}

#[test]
fn grid_breakpoint_evaluations_snapshot() {
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
    let attrs = state.attributes().id("dashboard-grid").class("rustic-grid");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "columns": evaluation.columns,
                "column_gap": evaluation.column_gap,
                "row_gap": evaluation.row_gap,
                "dense": evaluation.dense,
                "role": evaluation.role.as_str(),
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "data_breakpoint@600": attrs.data_breakpoint(600).1,
            "data_breakpoint@1920": attrs.data_breakpoint(1920).1,
            "dense": attrs.data_dense().map(|(_, value)| value.to_string()),
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_grid_breakpoints", snapshot);
}

#[test]
fn stack_breakpoint_evaluations_snapshot() {
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
    let attrs = state.attributes().id("action-stack").class("rustic-stack");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "direction": evaluation.direction.as_str(),
                "gap": evaluation.gap,
                "divider": evaluation.divider.clone(),
                "role": evaluation.role.as_str(),
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "data_direction@480": attrs.data_direction(480).1,
            "data_direction@1600": attrs.data_direction(1600).1,
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_stack_breakpoints", snapshot);
}

#[test]
fn image_list_breakpoint_evaluations_snapshot() {
    use rustic_ui_headless::image_list::{
        ImageListRole, ImageListState, ImageListTokens, ImageListVariant,
    };

    let tokens = ImageListTokens::uniform(
        ResponsiveValue::new(2)
            .with_override(Breakpoint::Sm, 3)
            .with_override(Breakpoint::Lg, 5),
        "12px",
        240,
    );

    let state = ImageListState::new(tokens, BreakpointConfig::material())
        .variant(ImageListVariant::Masonry)
        .with_role(ImageListRole::Presentation);
    let attrs = state.attributes().id("gallery").class("rustic-image-list");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "columns": evaluation.columns,
                "gap": evaluation.gap,
                "row_height": evaluation.row_height,
                "role": evaluation.role.as_str(),
                "variant": evaluation.variant.as_str(),
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "data_variant": attrs.data_variant().1,
            "data_breakpoint@720": attrs.data_breakpoint(720).1,
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_image_list_breakpoints", snapshot);
}

#[test]
fn hidden_breakpoint_evaluations_snapshot() {
    use rustic_ui_headless::hidden::{HiddenRole, HiddenState};

    let visibility = ResponsiveValue::new(false)
        .with_override(Breakpoint::Sm, true)
        .with_override(Breakpoint::Xl, false);

    let state = HiddenState::new(visibility, BreakpointConfig::material())
        .with_role(HiddenRole::Group)
        .inert(true);
    let attrs = state.attributes().id("inline-ad").class("rustic-hidden");

    let evaluations = state
        .breakpoints()
        .iter()
        .map(|(breakpoint, min_width)| {
            let evaluation = state.evaluate_for(breakpoint);
            json!({
                "breakpoint": breakpoint.as_token(),
                "min_width": min_width,
                "hidden": evaluation.hidden,
                "role": evaluation.role.as_str(),
                "inert": evaluation.inert,
            })
        })
        .collect::<Vec<_>>();

    let snapshot = json!({
        "attributes": {
            "role": attrs.role().1,
            "id": attrs.id_attr().map(|(_, value)| value.to_string()),
            "class": attrs.class_attr().map(|(_, value)| value.to_string()),
            "data_hidden@360": attrs.hidden(360).1,
            "data_hidden@1024": attrs.hidden(1024).1,
            "inert": attrs.inert().map(|(_, value)| value.to_string()),
        },
        "evaluations": evaluations,
    });

    assert_json_snapshot!("headless_hidden_breakpoints", snapshot);
}
