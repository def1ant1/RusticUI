#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
#[tokio::main]
async fn main() {
    use leptos::prelude::*;
    use leptos::ssr::render_to_string;
    use selection_controls_leptos::{ssr_snapshots, SelectionControlsDemo, TelemetryRecorder};

    let recorder = TelemetryRecorder::new();
    let render_recorder = recorder.clone();
    let html = render_to_string(move || {
        view! { <SelectionControlsDemo recorder=Some(render_recorder.clone()) /> }
    })
    .into_owned();

    println!("=== Hydration shell ===\n{html}\n");

    for (index, fragment) in ssr_snapshots().iter().enumerate() {
        println!("=== Descriptor fragment #{index} ===\n{fragment}\n");
    }

    println!(
        "=== Telemetry sequence ===\n{}",
        recorder
            .events()
            .iter()
            .map(|event| event.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn main() {
    use leptos::prelude::*;
    use selection_controls_leptos::SelectionControlsDemo;

    leptos::mount_to_body(|| view! { <SelectionControlsDemo/> });
}

#[cfg(all(
    not(feature = "ssr"),
    not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))
))]
fn main() {
    eprintln!(
        "selection-controls-leptos binary requires either `--features ssr` for server mode or `--features csr`/`--features hydrate` for wasm."
    );
}
