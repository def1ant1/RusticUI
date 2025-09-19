//! Minimal Yew example demonstrating the shared list and table renderers.
//!
//! The example keeps all domain data inline and focuses on wiring the headless
//! state machines into the Material themed render functions. Both widgets reuse
//! the HTML string returned by `mui_material` which guarantees parity between
//! SSR and CSR flows.

use mui_headless::list::{ListState, SelectionMode};
use mui_material::list::{self, ListDensity, ListItem, ListProps, ListTypography};
use mui_material::table::{self, TableColumn, TableProps, TableRow};
use mui_styled_engine::{Theme, ThemeProvider};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let list_props = ListProps::new(vec![
        ListItem::new("Billing").with_secondary("Invoices ready"),
        ListItem::new("Compliance").with_meta("Up to date"),
    ])
    .with_density(ListDensity::Compact)
    .with_primary_typography(ListTypography::Body1)
    .with_selection_mode(SelectionMode::Single)
    .with_automation_id("cookbook-list");
    let list_state = ListState::uncontrolled(list_props.items.len(), &[], SelectionMode::Single);
    let list_markup = list::yew::render(&list_props, &list_state);

    let table_props = TableProps::new(
        vec![
            TableColumn::new("Service"),
            TableColumn::new("Active Users").numeric(),
        ],
        vec![
            TableRow::new(vec!["Workflow".into(), "128".into()]),
            TableRow::new(vec!["Analytics".into(), "76".into()]),
        ],
    )
    .with_selection_mode(SelectionMode::Single)
    .with_automation_id("cookbook-table");
    let table_state = ListState::uncontrolled(table_props.rows.len(), &[], SelectionMode::Single);
    let table_markup = table::yew::render(&table_props, &table_state);

    html! {
        <ThemeProvider theme={Theme::default()}>
            <div class="demo-panel" inner_html={list_markup} />
            <div class="demo-panel" inner_html={table_markup} />
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
