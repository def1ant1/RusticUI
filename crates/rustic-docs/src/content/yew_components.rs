//! Yew adapters mirroring the Leptos inventory components.

use yew::prelude::*;

use crate::theme::palette_for_scheme;
use rustic_ui_system::theme::ColorScheme;

use super::inventory::{docs_inventory, InventoryCategory};

fn category_label(category: InventoryCategory) -> &'static str {
    match category {
        InventoryCategory::Component => "Component demo",
        InventoryCategory::Page => "Page layout",
        InventoryCategory::Data => "Data fixture",
    }
}

fn join(values: &[&'static str]) -> String {
    values.iter().copied().collect::<Vec<_>>().join(", ")
}

/// Properties accepted by the Yew inventory board.
#[derive(Properties, PartialEq)]
pub struct InventoryBoardProps {
    /// Optional category filter.
    #[prop_or_default]
    pub filter: Option<InventoryCategory>,
}

/// Render the docs inventory using Material-inspired styling.
#[function_component(InventoryBoard)]
pub fn inventory_board(props: &InventoryBoardProps) -> Html {
    let entries = docs_inventory();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|entry| props.filter.map_or(true, |cat| entry.category == cat))
        .collect();
    let summary = format!("Planning {} of {} assets", filtered.len(), entries.len());
    let palette = palette_for_scheme(ColorScheme::Light);
    let card_style = format!(
        "background:{};color:{};padding:1rem;border-radius:0.75rem;box-shadow:0 6px 18px rgba(15,23,42,0.15);",
        palette.surface,
        palette.text_primary
    );

    html! {
        <section aria-label="RusticUI documentation inventory" style="display: grid; gap: 1rem;">
            <p style="margin: 0; font-weight: 600;">{summary.clone()}</p>
            {for filtered.into_iter().map(|entry| {
                let frameworks = entry
                    .frameworks
                    .iter()
                    .map(|plan| format!("{} · {} ({})", plan.framework, plan.component, plan.module_path))
                    .collect::<Vec<_>>()
                    .join(" | ");
                let primitives = join(entry.recommended_primitives);
                html! {
                    <article style={card_style.clone()}>
                        <header style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <h3 style="margin: 0;">{entry.source_path}</h3>
                            <span style="font-size: 0.875rem; color: rgba(0,0,0,0.65);">{category_label(entry.category)}</span>
                        </header>
                        <div style="display: grid; gap: 0.25rem; margin-top: 0.5rem;">
                            <p style="margin: 0;">{entry.notes}</p>
                            <p style="margin: 0; font-size: 0.875rem;">
                                <strong>{"Framework plan:"}</strong>{" "}{frameworks.clone()}
                            </p>
                            <p style="margin: 0; font-size: 0.875rem;">
                                <strong>{"Primitives:"}</strong>{" "}{primitives.clone()}
                            </p>
                            <p style="margin: 0; font-size: 0.875rem;">
                                <strong>{"Route:"}</strong>{" "}{entry.route_hint}
                            </p>
                        </div>
                    </article>
                }
            })}
        </section>
    }
}
