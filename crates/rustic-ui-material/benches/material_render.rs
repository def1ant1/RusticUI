//! Criterion-based micro benchmarks for the Material rendering helpers.
//!
//! The goal is not to micro-optimise individual instructions but to guard the
//! rendering contract shared across adapters. A failing benchmark provides an
//! immediate signal that a refactor regressed performance for one of the core
//! rendering paths (`ButtonState` -> themed attributes -> HTML string).
//! Each benchmark includes context about what we are measuring so engineers
//! landing large changes can reason about the deltas quickly without digging
//! through implementation details.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rustic_ui_headless::button::ButtonState;
use rustic_ui_material::button::{self, ButtonProps};
use rustic_ui_material::Theme;

/// Benchmarks the steady-state HTML renderer used by every Material adapter.
///
/// Rendering is intentionally exercised in two scenarios:
///
/// * `new_state` instantiates a fresh [`ButtonState`] per iteration. This mimics
///   initial renders where the adapter has to bootstrap state and styling.
/// * `reuse_state` reuses the same [`ButtonState`] handle. This mirrors rerenders
///   triggered by parent updates where the state machine is already hydrated.
fn bench_button_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("material/button/render_html");
    group.throughput(Throughput::Elements(1));

    group.bench_function("new_state", |b| {
        b.iter(|| {
            // Touch the theme so the styled engine warms caches before we
            // measure the real rendering code path.
            black_box(Theme::default());
            let state = ButtonState::new(false, None);
            let props = ButtonProps::new("Launch warp drive");
            black_box(button::yew::render(&props, &state));
        });
    });

    group.bench_function("reuse_state", |b| {
        let props = ButtonProps::new("Launch warp drive");
        let state = ButtonState::new(false, None);

        b.iter(|| {
            // Exercising the Leptos adapter keeps parity with other
            // frameworks while avoiding allocation differences.
            black_box(button::leptos::render(&props, &state));
        });
    });

    group.finish();
}

criterion_group!(material_renderers, bench_button_render);
criterion_main!(material_renderers);
