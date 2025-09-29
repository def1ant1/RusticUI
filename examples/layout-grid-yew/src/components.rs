use std::fmt::Write as _;

use crate::blueprint::{grid_columns, layout_blueprint, SectionBlueprint};
use rustic_ui_system::theme_provider::CssBaseline;
use rustic_ui_system::{
    responsive::Responsive, theme::Theme, Box, Container, Grid, Stack, ThemeProvider, Typography,
    TypographyVariant,
};
use serde_json::json;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

/// Stable DOM id shared between SSR and CSR entrypoints.
pub const HYDRATION_CONTAINER_ID: &str = "layout-grid-root";

/// Simple hydration phase marker exposed to monitoring dashboards. During SSR
/// the component renders `Server`; once hydration completes the value flips to
/// `Client` so observers can confirm that browser-only effects remain gated
/// until the DOM is safe to touch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HydrationPhase {
    /// Server rendered markup before hydration occurs.
    Server,
    /// Client phase after Yew attaches event handlers and effects can run.
    Client,
}

/// Top-level component rendering the responsive grid showcase.
#[derive(Properties, PartialEq, Clone)]
pub struct LayoutGridAppProps {
    /// Optional theme override. Tests can inject customised breakpoints without
    /// touching global state.
    #[prop_or_default]
    pub theme: Option<Theme>,
}

#[function_component(LayoutGridApp)]
pub fn layout_grid_app(props: &LayoutGridAppProps) -> Html {
    let theme = props.theme.clone().unwrap_or_else(Theme::default);
    let sections = layout_blueprint();
    let phase = use_state(|| HydrationPhase::Server);
    register_hydration_transition(&phase);

    let container_max_width = Responsive {
        xs: "100%".to_string(),
        sm: Some("100%".to_string()),
        md: Some("1200px".to_string()),
        lg: Some("1440px".to_string()),
        xl: Some("1600px".to_string()),
    };
    let stack_spacing = Responsive {
        xs: "16px".to_string(),
        sm: Some("20px".to_string()),
        md: Some("24px".to_string()),
        lg: Some("32px".to_string()),
        xl: Some("40px".to_string()),
    };
    let card_padding = Responsive {
        xs: "16px".to_string(),
        sm: Some("18px".to_string()),
        md: Some("20px".to_string()),
        lg: Some("24px".to_string()),
        xl: Some("28px".to_string()),
    };

    let breakpoint_label = active_breakpoint_label(&theme);
    let mut phase_text = String::new();
    let _ = write!(&mut phase_text, "{:?}", *phase);

    html! {
        <ThemeProvider theme={theme.clone()}>
            <CssBaseline />
            <div
                data-rustic-layout-grid-shell="layout-grid-shell"
                data-rustic-layout-grid-phase={phase_text.clone()}
                data-rustic-layout-grid-breakpoint={breakpoint_label}
            >
                <Container max_width={Some(container_max_width)}>
                    <Stack spacing={Some(stack_spacing.clone())}>
                        <header>
                            <Typography variant={Some(TypographyVariant::H2)}>
                                {"Responsive marketing grid"}
                            </Typography>
                            <Typography>
                                {"Each panel below resolves its span at runtime using RusticUI's Responsive helpers. "}
                                {"SSR renders deterministic markup and the hydration phase indicator flips once Yew executes in the browser."}
                            </Typography>
                        </header>
                        <div data-rustic-layout-grid-container="layout-grid-panels">
                            <Box
                                display={Some("flex".to_string())}
                                justify_content={Some("space-between".to_string())}
                                align_items={Some("stretch".to_string())}
                                sx={Some(json!({
                                    "flexWrap": "wrap",
                                    "gap": "24px"
                                }))}
                            >
                                { for sections.iter().map(|section| render_section(section, &theme, &card_padding)) }
                            </Box>
                        </div>
                    </Stack>
                </Container>
            </div>
        </ThemeProvider>
    }
}

fn render_section(section: &SectionBlueprint, theme: &Theme, padding: &Responsive<String>) -> Html {
    let automation = format!("layout-grid-section-{}", section.id);
    html! {
        <Grid
            key={section.id}
            span={Some(section.span.clone())}
            columns={section.columns.clone().or_else(|| Some(grid_columns()))}
            sx={Some(json!({
                "minWidth": "260px"
            }))}
        >
            <div data-rustic-layout-grid-item={AttrValue::from(automation)}>
                <Box
                    p={Some(padding.clone())}
                    display={Some("flex".to_string())}
                    justify_content={Some("space-between".to_string())}
                    align_items={Some("flex-start".to_string())}
                    sx={Some(json!({
                        "flexDirection": "column",
                        "background": "linear-gradient(180deg, rgba(49,130,206,0.12), rgba(56,189,248,0.18))",
                        "borderRadius": "18px",
                        "boxShadow": "0 14px 40px rgba(15,23,42,0.18)",
                        "minHeight": "220px"
                    }))}
                >
                    <Stack spacing={Some(Responsive::from("12px".to_string()))}>
                        <Typography variant={Some(TypographyVariant::H2)} sx="font-size:1.35rem;font-weight:600;">
                            {section.title}
                        </Typography>
                        <Typography>
                            {section.summary}
                        </Typography>
                        <Typography sx="font-size:0.85rem;color:rgba(15,23,42,0.78);">
                            {format_span_copy(section, theme)}
                        </Typography>
                    </Stack>
                    <footer>
                        <Typography sx="text-transform:uppercase;font-size:0.75rem;letter-spacing:0.08em;">
                            {"Track hydration & breakpoints via data attributes"}
                        </Typography>
                    </footer>
                </Box>
            </div>
        </Grid>
    }
}

/// Builds a small caption describing the resolved span for the current viewport.
fn format_span_copy(section: &SectionBlueprint, theme: &Theme) -> String {
    #[cfg(target_arch = "wasm32")]
    let width = rustic_ui_system::responsive::viewport_width();
    #[cfg(not(target_arch = "wasm32"))]
    let width = 0;
    let span = section.resolved_span(width, theme);
    let columns = section.resolved_columns(width, theme);
    format!("Spans {span} of {columns} columns at this breakpoint.")
}

/// Registers an effect that flips the hydration phase once the browser finishes
/// mounting the Yew component tree.
fn register_hydration_transition(state: &UseStateHandle<HydrationPhase>) {
    #[cfg(target_arch = "wasm32")]
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            state.set(HydrationPhase::Client);
            || ()
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = state;
    }
}

/// Returns the label of the currently active breakpoint.
fn active_breakpoint_label(theme: &Theme) -> &'static str {
    #[cfg(target_arch = "wasm32")]
    let width = rustic_ui_system::responsive::viewport_width();
    #[cfg(not(target_arch = "wasm32"))]
    let width = 0;
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
    fn breakpoint_labels_resolve_in_host_environment() {
        let theme = Theme::default();
        // Host tests see a viewport width of zero which should map to the base breakpoint.
        assert_eq!(active_breakpoint_label(&theme), "xs");
    }
}
