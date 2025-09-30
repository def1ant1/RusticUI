//! Dioxus shell for the shared InputBase blueprint.
//!
//! The example renders the SSR markup produced by `forms-input-base-shared`
//! alongside automation documentation so teams can diff the output against the
//! other framework adapters.  Dioxus can hydrate the HTML fragment via
//! `dangerous_inner_html` while still benefiting from the shared analytics
//! namespace and status identifiers.

use dioxus::prelude::*;
use forms_input_base_shared::{
    automation_value, InputBaseBlueprint, CONTROLLED_ANALYTICS_ID, CONTROLLED_STATUS_ID,
    HYDRATION_NOTE, PLACEHOLDER, UNCONTROLLED_ANALYTICS_ID, UNCONTROLLED_STATUS_ID,
};
use rustic_ui_system::theme::Theme;

pub fn app(cx: Scope) -> Element {
    let blueprint = InputBaseBlueprint::new();
    let theme: Theme = blueprint.enterprise_theme();
    let palette = theme.palette.active().clone();
    let automation_attributes = blueprint.automation_attributes();
    let automation_hint = automation_value(["example", "dioxus"]);

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

    cx.render(rsx! {
        section { style: "{container_style}", "data-rustic-input-base-example": "{automation_hint}",
            article { style: "{panel_style}",
                header {
                    h1 { style: "margin:0;font-size:2rem;", "RusticUI InputBase – Dioxus" }
                    p { style: "margin:8px 0 0;max-width:72ch;", "The SSR fragment below matches the interactive adapters so QA selectors stay reusable." }
                    p { style: "margin:12px 0 0;font-size:0.95rem;color:#cbd5f5;", "{HYDRATION_NOTE}" }
                }
                section { style: "display:grid;grid-template-columns:repeat(auto-fit, minmax(280px, 1fr));gap:24px;",
                    div { style: "display:flex;flex-direction:column;gap:12px;",
                        h2 { style: "margin:0;", "Controlled snapshot" }
                        p { style: "margin:0;font-size:0.95rem;color:#cbd5f5;", "Automation id: {CONTROLLED_ANALYTICS_ID}" }
                        div {
                            class: "input-base-controlled",
                            dangerous_inner_html: "{controlled_html}",
                        }
                    }
                    div { style: "display:flex;flex-direction:column;gap:12px;",
                        h2 { style: "margin:0;", "Uncontrolled snapshot" }
                        p { style: "margin:0;font-size:0.95rem;color:#cbd5f5;", "Automation id: {UNCONTROLLED_ANALYTICS_ID}" }
                        div {
                            class: "input-base-uncontrolled",
                            dangerous_inner_html: "{uncontrolled_html}",
                        }
                    }
                }
                section { style: "display:grid;grid-template-columns:repeat(auto-fit, minmax(260px, 1fr));gap:24px;",
                    div {
                        h3 { style: "margin-top:0;", "Automation attributes" }
                        ul {
                            {automation_attributes.iter().map(|attr| rsx! { li { code { "{attr}" } } }).collect::<Vec<_>>()} 
                        }
                    }
                    div {
                        h3 { style: "margin-top:0;", "Hydration guidance" }
                        p { style: "margin:0;font-size:0.95rem;color:#cbd5f5;", "Placeholder preserved across renderers: {PLACEHOLDER}" }
                    }
                }
            }
        }
    })
}

#[cfg(feature = "csr")]
pub fn main() {
    dioxus_web::launch(app);
}
