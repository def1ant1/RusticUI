use rustic_ui_headless::circular_progress::{CircularProgressState, ProgressMode};
use rustic_ui_headless::linear_progress::{LinearProgressMode, LinearProgressState};
use rustic_ui_material::{
    render_circular_progress, render_linear_progress, CircularProgressAdapterProps,
    LinearProgressAdapterProps,
};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let circular = CircularProgressState::new(ProgressMode::Determinate { value: 0.42 });
    let linear = LinearProgressState::new(LinearProgressMode::Buffer {
        value: 0.5,
        buffer: 0.75,
    });
    println!(
        "Circular HTML:\n{}\nLinear HTML:\n{}",
        render_circular_progress(&CircularProgressAdapterProps::new(&circular)).html,
        render_linear_progress(&LinearProgressAdapterProps::new(&linear)).html,
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use leptos::*;

    mount_to_body(|| view! { <AutomationHarness/> })
}

#[cfg(target_arch = "wasm32")]
#[component]
fn AutomationHarness() -> impl leptos::IntoView {
    let circular = CircularProgressState::new(ProgressMode::Indeterminate);
    let linear = LinearProgressState::new(LinearProgressMode::Determinate { value: 0.33 });
    let circular_html =
        render_circular_progress(&CircularProgressAdapterProps::new(&circular)).html;
    let linear_html = render_linear_progress(&LinearProgressAdapterProps::new(&linear)).html;
    view! {
        <div class="automation-harness">
            <pre>{circular_html}</pre>
            <pre>{linear_html}</pre>
        </div>
    }
}
