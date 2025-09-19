//! Minimal Leptos example wiring RusticUI data display renderers into a CSR app.

use leptos::*;
use mui_headless::list::{ListState, SelectionMode};
use mui_material::list::{self, ListDensity, ListItem, ListProps, ListTypography};
use mui_material::table::{self, TableColumn, TableProps, TableRow};
use mui_styled_engine::{Theme, ThemeProvider};

#[component]
fn App() -> impl IntoView {
    let list_props = ListProps::new(vec![
        ListItem::new("Billing").with_secondary("Invoices ready"),
        ListItem::new("Compliance").with_meta("Up to date"),
    ])
    .with_density(ListDensity::Compact)
    .with_primary_typography(ListTypography::Body1)
    .with_selection_mode(SelectionMode::Single)
    .with_automation_id("cookbook-list");
    let list_state = ListState::uncontrolled(list_props.items.len(), &[], SelectionMode::Single);
    let list_markup = list::leptos::render(&list_props, &list_state);

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
    let table_markup = table::leptos::render(&table_props, &table_state);

    view! {
        <ThemeProvider theme=Theme::default()>
            <div class="demo-panel" inner_html=list_markup></div>
            <div class="demo-panel" inner_html=table_markup></div>
        </ThemeProvider>
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> leptos::Result<()> {
    leptos::ssr::render_to_stream(|cx| view! { cx, <App /> }).await;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    leptos::mount_to_body(|| view! { <App /> });
}
