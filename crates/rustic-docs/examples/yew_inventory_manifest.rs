//! Prints the generated documentation inventory as JSON for Yew oriented smoke tests.

#[cfg(feature = "yew-docs")]
fn main() {
    let manifest: Vec<_> = rustic_docs::content::docs_inventory()
        .iter()
        .map(|entry| {
            serde_json::json!({
                "source": entry.source_path,
                "route": entry.route_hint,
                "category": format!("{:?}", entry.category),
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "entries": manifest,
        }))
        .expect("serialize manifest"),
    );
}

#[cfg(not(feature = "yew-docs"))]
fn main() {
    eprintln!("Enable the `yew-docs` feature to run this example");
}
