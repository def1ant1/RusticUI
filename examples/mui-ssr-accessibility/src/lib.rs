use mui_shared::{
    layout::{self, AppShell, Framework},
    routes::{RouteDescriptor, ABOUT, HOME},
    theme::{material_example_theme, MaterialExampleTheme},
};
use rustic_ui_material::{AppBar, AppBarColor, AppBarSize};
use rustic_ui_styled_engine::StyledEngineProvider;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew::ServerRenderer;

/// Properties consumed by the SSR Yew tree.
#[derive(Properties, Clone, PartialEq)]
pub struct AppProps {
    /// Shared shell instance reused across SSR and CSR renderers.
    pub shell: AppShell<'static>,
    /// Material theme blueprint ensuring the same tokens flow into the CSR pass.
    pub theme: MaterialExampleTheme,
}

/// Header + navigation fragment rendered via Yew during SSR.
///
/// The component intentionally mirrors the CSR layout from `examples/mui-yew`.
/// All automation identifiers (`data-rustic-*`) are sourced from
/// [`mui_shared`]'s deterministic builders so the DOM matches exactly once the
/// client hydrates and transitions its reducer from
/// `HydrationPhase::Server` to `HydrationPhase::Client`.
#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let automation = props.shell.automation();
    let header_attr = AttrValue::from(automation.child("header").value());
    let nav_attr = AttrValue::from(automation.child("navigation").value());

    let routes: [&RouteDescriptor; 2] = [&HOME, &ABOUT];
    let nav_items = routes.into_iter().map(|descriptor| {
        let link_attr = automation
            .child("navigation")
            .child(descriptor.automation_base)
            .value();
        html! {
            <li data-rustic-app-navigation={link_attr}>
                <a href={descriptor.path}>{ nav_label(descriptor) }</a>
            </li>
        }
    });

    html! {
        <StyledEngineProvider theme={props.theme.system_theme.clone()}>
            <header data-rustic-app-header={header_attr}>
                <AppBar
                    title="Material UI SSR"
                    aria_label="Primary navigation"
                    color={AppBarColor::Primary}
                    size={AppBarSize::Medium}
                />
                <nav data-rustic-app-navigation={nav_attr} aria-label="Primary">
                    <ul>
                        { for nav_items }
                    </ul>
                </nav>
            </header>
        </StyledEngineProvider>
    }
}

/// Renders the SSR document and returns the resulting HTML string.
///
/// The helper composes [`AppShell::render_ssr_document`] with the header
/// fragment emitted by [`App`].  The resulting markup includes the same
/// `data-rustic-*` automation hooks as the CSR entry points so Playwright and
/// Cypress suites can assert hydration parity without bespoke selectors.
pub async fn render_document() -> String {
    let theme = material_example_theme();
    let shell = AppShell::for_route(&HOME);
    let framework_automation = layout::automation_for_framework(&HOME, Framework::Yew);
    // The hydration marker is reused by CSR reducers (see `ModeState` in the Yew example)
    // to transition from `HydrationPhase::Server` once the DOM mounts.  Recomputing the
    // exact attribute here keeps those state transitions deterministic.
    let (_, hydration_value) = framework_automation.attribute("hydration-root");
    let shell_value = shell.automation().child("shell").value();

    let header_markup = ServerRenderer::<App>::with_props(AppProps {
        shell: shell.clone(),
        theme: theme.clone(),
    })
    .render()
    .await;

    let actions_markup = compose_actions(&shell);

    shell.render_ssr_document(
        |content| {
            format!(
                r#"<div id=\"app\" data-rustic-app-shell=\"{shell_value}\" data-rustic-app-hydration-root=\"{hydration_value}\">{header}{content}{actions}</div>"#,
                shell_value = shell_value,
                hydration_value = hydration_value,
                header = header_markup,
                content = content,
                actions = actions_markup,
            )
        },
        &theme,
    )
}

fn nav_label(descriptor: &RouteDescriptor) -> &'static str {
    match descriptor.path {
        "/" => "Home",
        _ => "About",
    }
}

fn compose_actions(shell: &AppShell<'_>) -> String {
    let automation = shell.automation();
    let mut actions = String::new();
    let primary = shell.primary_action();
    let secondary = shell.secondary_action();

    if primary.is_some() || secondary.is_some() {
        let actions_attr = automation.child("actions").value();
        actions.push_str(&format!(
            "<div data-rustic-app-actions=\"{value}\">",
            value = actions_attr
        ));

        if let Some(action) = primary {
            let attr = automation
                .child("actions")
                .child(action.automation_role)
                .value();
            actions.push_str(&format!(
                "<a class=\"cta primary\" data-rustic-app-action=\"{attr}\" href=\"{href}\">{label}</a>",
                attr = attr,
                href = html_escape::encode_double_quoted_attribute(action.href),
                label = html_escape::encode_text(action.label)
            ));
        }

        if let Some(action) = secondary {
            let attr = automation
                .child("actions")
                .child(action.automation_role)
                .value();
            actions.push_str(&format!(
                "<a class=\"cta secondary\" data-rustic-app-action=\"{attr}\" href=\"{href}\">{label}</a>",
                attr = attr,
                href = html_escape::encode_double_quoted_attribute(action.href),
                label = html_escape::encode_text(action.label)
            ));
        }

        actions.push_str("</div>");
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ssr_document_contains_expected_markers() {
        let html = render_document().await;
        assert!(
            html.contains("data-rustic-app-shell=\"app-home-shell\""),
            "missing shell marker: {html}"
        );
        assert!(
            html.contains(
                "data-rustic-app-hydration-root=\"app-home-framework-yew-hydration-root\""
            ),
            "missing hydration marker: {html}"
        );
        assert!(
            html.contains("data-rustic-app-actions=\"app-home-actions\""),
            "missing action container marker: {html}"
        );
    }
}
