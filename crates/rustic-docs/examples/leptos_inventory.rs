//! Smoke example rendering the inventory board into an SSR snapshot.

#[cfg(feature = "ssr")]
fn main() {
    let html = leptos::ssr::render_to_string(|| {
        leptos::view! {
            <rustic_docs::theme::DocsThemeShell>
                <rustic_docs::content::leptos_components::InventoryBoard />
            </rustic_docs::theme::DocsThemeShell>
        }
    });
    println!("{html}");
}

#[cfg(not(feature = "ssr"))]
fn main() {
    eprintln!("Enable the `ssr` feature to run this example");
}
