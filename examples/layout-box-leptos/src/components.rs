use crate::blueprint::{panel_blueprint, PanelBlueprint};
use leptos::*;
use rustic_ui_system::theme_provider::CssBaseline;
use rustic_ui_system::{
    responsive::Responsive, theme::Theme, Box as MuiBox, Container, Stack, ThemeProvider,
    Typography, TypographyVariant,
};
use serde_json::json;

/// Shared hydration root id referenced by CSR and SSR entrypoints.
pub const HYDRATION_CONTAINER_ID: &str = "layout-box-root";

/// Hydration lifecycle indicator surfaced to observability tooling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HydrationPhase {
    /// Server rendered markup before any browser APIs are invoked.
    Server,
    /// Client phase after Leptos attaches event handlers.
    Client,
}

/// Properties accepted by [`LayoutBoxApp`].
#[component]
pub fn LayoutBoxApp(#[prop(optional)] theme: Option<Theme>) -> impl IntoView {
    let theme = theme.unwrap_or_else(Theme::default);
    let hydration = create_rw_signal(HydrationPhase::Server);
    register_hydration_transition(hydration);

    let panel_cards = {
        let theme = theme.clone();
        panel_blueprint()
            .into_iter()
            .map(|panel| {
                let theme = theme.clone();
                view! { <PanelCard panel theme /> }
            })
            .collect_view()
    };
    let breakpoint_label = active_breakpoint_label(&theme).to_string();
    let container_widths = Responsive {
        xs: "100%".into(),
        sm: Some("760px".into()),
        md: Some("960px".into()),
        lg: Some("1040px".into()),
        xl: Some("1120px".into()),
    };
    let stack_spacing = Responsive {
        xs: "24px".into(),
        sm: Some("28px".into()),
        md: Some("32px".into()),
        lg: Some("36px".into()),
        xl: Some("40px".into()),
    };

    view! {
        <ThemeProvider theme=theme.clone()>
            <CssBaseline />
            <div
                data-rustic-layout-box-shell="layout-box-shell"
                data-rustic-layout-box-phase=move || format!("{:?}", hydration.get())
                data-rustic-layout-box-breakpoint=breakpoint_label.clone()
            >
                <Container max_width=container_widths>
                    <Stack spacing=stack_spacing>
                        <header>
                            <Typography variant=TypographyVariant::H2>
                                {"Responsive content panels"}
                            </Typography>
                            <Typography>
                                {"Box props come from a shared blueprint so SSR output, hydration, and runtime signals all describe the same layout."}
                            </Typography>
                        </header>
                        <section style="display:flex;flex-direction:column;gap:20px;">
                            {panel_cards.clone()}
                        </section>
                    </Stack>
                </Container>
            </div>
        </ThemeProvider>
    }
}

#[component]
fn PanelCard(panel: PanelBlueprint, theme: Theme) -> impl IntoView {
    let automation_attr = format!("layout-box-panel-{}", panel.id);
    let theme_for_metrics = theme.clone();
    let panel_for_metrics = panel.clone();
    let metrics = move || {
        let width = rustic_ui_system::responsive::viewport_width();
        let padding = panel_for_metrics.resolved_padding(width, &theme_for_metrics);
        let max_width = panel_for_metrics.resolved_max_width(width, &theme_for_metrics);
        format!("Padding {padding} — max width {max_width} at this breakpoint.")
    };

    view! {
        <div data-rustic-layout-box-panel=automation_attr>
            <MuiBox
                p=panel.padding.clone()
                max_width=panel.max_width.clone()
                display="flex"
                justify_content="space-between"
                align_items="flex-start"
                sx=json!({
                    "flexDirection": "column",
                    "gap": "16px",
                    "background": "radial-gradient(circle at top left, rgba(49,130,206,0.18), rgba(15,23,42,0.6))",
                    "paddingInline": "clamp(16px, 2vw, 28px)",
                    "borderRadius": "20px",
                    "boxShadow": "0 20px 60px rgba(15,23,42,0.28)",
                })
            >
                <Stack spacing=Responsive::from("12px".to_string())>
                    <Typography variant=TypographyVariant::H2 sx="font-size:1.3rem;font-weight:600;">
                        {panel.title}
                    </Typography>
                    <Typography>
                        {panel.summary}
                    </Typography>
                    <Typography sx="font-size:0.85rem;color:rgba(15,23,42,0.78);">
                        {metrics}
                    </Typography>
                </Stack>
                <footer>
                    <Typography sx="text-transform:uppercase;font-size:0.75rem;letter-spacing:0.08em;">
                        {"Hydration safe metrics displayed above"}
                    </Typography>
                </footer>
            </MuiBox>
        </div>
    }
}

/// Registers a one-shot transition that flips the hydration phase once the
/// component mounts in the browser.
fn register_hydration_transition(handle: RwSignal<HydrationPhase>) {
    #[cfg(target_arch = "wasm32")]
    {
        on_mount(move || {
            handle.set(HydrationPhase::Client);
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = handle;
    }
}

/// Returns the active breakpoint label using the shared theme configuration.
fn active_breakpoint_label(theme: &Theme) -> &'static str {
    let width = rustic_ui_system::responsive::viewport_width();
    if width >= theme.breakpoints.xl {
        "xl"
    } else if width >= theme.breakpoints.lg {
        "lg"
    } else if width >= theme.breakpoints.md {
        "md"
    } else if width >= theme.breakpoints.sm {
        "sm"
    } else {
        "xs"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_breakpoint_defaults_to_xs() {
        let theme = Theme::default();
        assert_eq!(active_breakpoint_label(&theme), "xs");
    }
}
