//! Sycamore blueprint rendering the shared InputBase markup.
//!
//! Sycamore currently consumes the Material InputBase renderer through SSR HTML
//! rather than direct DOM event bindings.  We still expose both controlled and
//! uncontrolled snapshots alongside automation documentation so QA teams can use
//! the same selectors as the interactive adapters.

use forms_input_base_shared::{
    automation_value, InputBaseBlueprint, CONTROLLED_ANALYTICS_ID, CONTROLLED_STATUS_ID,
    HYDRATION_NOTE, PLACEHOLDER, UNCONTROLLED_ANALYTICS_ID, UNCONTROLLED_STATUS_ID,
};
use rustic_ui_system::theme::Theme;
use sycamore::prelude::*;

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let blueprint = InputBaseBlueprint::new();
    let theme: Theme = blueprint.enterprise_theme();
    let palette = theme.palette.active().clone();
    let automation_attributes = blueprint.automation_attributes();
    let automation_hint = automation_value(["example", "sycamore"]);
    let placeholder = PLACEHOLDER;

    let uncontrolled_html = blueprint.render_markup(
        &blueprint.uncontrolled_state(),
        UNCONTROLLED_ANALYTICS_ID,
        UNCONTROLLED_STATUS_ID,
    );
    let controlled_html = blueprint.render_markup(
        &blueprint.controlled_state(),
        CONTROLLED_ANALYTICS_ID,
        CONTROLLED_STATUS_ID,
    );

    let container_style = format!(
        "min-height:100vh;padding:48px;box-sizing:border-box;background:{};display:flex;justify-content:center;align-items:flex-start;",
        palette.background_default
    );
    let panel_style = format!(
        "width:100%;max-width:920px;background:{};color:{};padding:40px;border-radius:16px;box-shadow:0 24px 64px rgba(15,23,42,0.45);display:flex;flex-direction:column;gap:24px;",
        palette.background_paper, palette.text_primary
    );

    view! { cx,
        section(style = container_style, data-rustic-input-base-example = automation_hint) {
            article(style = panel_style) {
                header {
                    h1(style = "margin:0;font-size:2rem;") { "RusticUI InputBase – Sycamore" }
                    p(style = "margin:8px 0 0;max-width:72ch;") {
                        "Sycamore consumes the shared SSR markup so automation selectors stay aligned with other renderers."
                    }
                    p(style = "margin:12px 0 0;font-size:0.95rem;color:#cbd5f5;") { HYDRATION_NOTE }
                }
                section(style = "display:grid;grid-template-columns:repeat(auto-fit, minmax(280px, 1fr));gap:24px;") {
                    div(style = "display:flex;flex-direction:column;gap:12px;") {
                        h2(style = "margin:0;") { "Controlled snapshot" }
                        p(style = "margin:0;font-size:0.95rem;color:#cbd5f5;") {
                            format!("Automation id: {}", CONTROLLED_ANALYTICS_ID)
                        }
                        pre(style = "background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;") {
                            {controlled_html.clone()}
                        }
                    }
                    div(style = "display:flex;flex-direction:column;gap:12px;") {
                        h2(style = "margin:0;") { "Uncontrolled snapshot" }
                        p(style = "margin:0;font-size:0.95rem;color:#cbd5f5;") {
                            format!("Automation id: {}", UNCONTROLLED_ANALYTICS_ID)
                        }
                        pre(style = "background:#020617;padding:12px;border-radius:8px;white-space:pre-wrap;") {
                            {uncontrolled_html.clone()}
                        }
                    }
                }
                section(style = "display:grid;grid-template-columns:repeat(auto-fit, minmax(260px, 1fr));gap:24px;") {
                    div {
                        h3(style = "margin-top:0;") { "Automation attributes" }
                        ul {
                            {(automation_attributes.into_iter().map(|attr| view! { cx,
                                li { code { (attr) } }
                            })).collect::<Vec<_>>>()}
                        }
                    }
                    div {
                        h3(style = "margin-top:0;") { "Hydration guidance" }
                        p(style = "margin:0;font-size:0.95rem;color:#cbd5f5;") {
                            format!("Mirror the InputState snapshot and keep the placeholder (`{placeholder}`) consistent across SSR and hydration runs.")
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    sycamore::render(|cx| view! { cx, App {} });
}
