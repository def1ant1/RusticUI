use leptos::*;
use mui_material::select::SelectOption;
use select_menu_shared::{
    enterprise_theme, fetch_regions, props_from_options, render_select_markup, selection_summary,
    to_select_options, AUTOMATION_ID,
};
#[cfg(feature = "ssr")]
use select_menu_shared::ssr_shell;

/// Leptos implementation of the select menu demo. The component mirrors the
/// Yew variant but leans on `RwSignal` for state management and Leptos specific
/// event handlers.
#[component]
pub fn App() -> impl IntoView {
    let theme = enterprise_theme();
    let container_style = move || {
        format!(
            "min-height:100vh;display:flex;align-items:center;justify-content:center;background:{};padding:32px;",
            theme.palette.background_default
        )
    };
    let panel_style = move || {
        format!(
            "max-width:720px;display:flex;flex-direction:column;gap:16px;background:{};padding:24px;border-radius:{}px;box-shadow:0 18px 60px rgba(0,0,0,0.35);color:{};",
            theme.palette.background_paper,
            theme.joy.radius,
            theme.palette.text_primary
        )
    };
    let options = create_rw_signal::<Vec<SelectOption>>(Vec::new());
    let selected = create_rw_signal::<Option<usize>>(None);
    let open = create_rw_signal(false);

    #[cfg(feature = "csr")]
    {
        let options = options.clone();
        let selected = selected.clone();
        spawn_local(async move {
            let regions = fetch_regions().await;
            let opts = to_select_options(&regions);
            if !opts.is_empty() {
                selected.set(Some(0));
            }
            options.set(opts);
        });
    }

    let summary = {
        let options = options.clone();
        let selected = selected.clone();
        create_memo(move |_| {
            let snapshot = options.get();
            let props = props_from_options(
                "Primary replication region",
                AUTOMATION_ID,
                &snapshot,
            );
            selection_summary(&props, selected.get())
        })
    };

    let rendered_select = move || {
        if options.with(|opts| opts.is_empty()) {
            view! { <p data-automation="select-menu-loading">"Loading datacenters…"</p> }.into_view()
        } else {
            let snapshot = options.get();
            let props = props_from_options(
                "Primary replication region",
                AUTOMATION_ID,
                &snapshot,
            );
            let html = render_select_markup(&props, open.get(), selected.get());
            view! { <div inner_html={html}></div> }.into_view()
        }
    };

    view! {
        <div style={container_style} data-automation="select-menu-leptos-shell">
            <div style={panel_style}>
                <header>
                    <h1 style="margin:0;font-size:1.75rem;">{"RusticUI Select Menu — Leptos"}</h1>
                    <p style="margin:4px 0 0;max-width:48ch;">
                        {"Signals drive the controlled open/selected state so hydration and client interactions stay in sync."}
                    </p>
                </header>
                <section style="display:flex;gap:8px;flex-wrap:wrap;">
                    <button
                        type="button"
                        on:click=move |_| open.update(|value| *value = !*value)
                        data-automation="select-menu-toggle-open"
                    >
                        {move || if open.get() { "Close menu" } else { "Open menu" }}
                    </button>
                    <button
                        type="button"
                        on:click=move |_| {
                            let len = options.with(|opts| opts.len());
                            if len == 0 {
                                return;
                            }
                            let next = selected
                                .get()
                                .map(|current| (current + 1) % len)
                                .unwrap_or(0);
                            selected.set(Some(next));
                        }
                        data-automation="select-menu-cycle"
                    >
                        {"Cycle selection"}
                    </button>
                </section>
                <p
                    aria-live="polite"
                    data-automation="select-menu-selection"
                    style="margin:0;font-weight:500;"
                >
                    {move || summary.get()}
                </p>
                <section data-automation="select-menu-rendered">
                    {rendered_select}
                </section>
            </div>
        </div>
    }
}

#[cfg(feature = "csr")]
fn main() {
    // Mount directly into the document body for SPA development.
    leptos::mount_to_body(|| view! { <App/> });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let regions = fetch_regions().await;
    let options = to_select_options(&regions);
    let props = props_from_options("Primary replication region", AUTOMATION_ID, &options);
    let html = render_select_markup(&props, true, Some(0));
    let theme = enterprise_theme();
    println!("{}", ssr_shell(&html, &theme));
}
