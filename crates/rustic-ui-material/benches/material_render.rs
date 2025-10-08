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
use rustic_ui_headless::drawer::{DrawerAnchor, DrawerState, DrawerVariant};
use rustic_ui_headless::ControlStrategy;
use rustic_ui_material::button::{self, ButtonProps};
use rustic_ui_material::drawer;
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

/// Benchmarks the drawer surface renderer which stitches together the headless
/// accessibility attributes, theming metadata and HTML payload for modal/persistent
/// drawers.
///
/// Two scenarios are covered:
///
/// * `modal_surface_open` mirrors a top-layer navigation drawer with focus
///   trapping enabled. It exercises attribute assembly (`data-open`,
///   `aria-modal`, automation ids) alongside the style conversion path that
///   maps system spacing tokens into inline CSS.
/// * `persistent_surface_closed` captures the steady-state footprint of a
///   persistent drawer that remains part of the document flow. This ensures the
///   renderer stays lean even when most automation flags are disabled.
fn bench_drawer_surface_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("material/drawer/render_surface_html");
    group.throughput(Throughput::Elements(1));

    group.bench_function("modal_surface_open", |b| {
        let state = DrawerState::new(
            true,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Modal,
            DrawerAnchor::Start,
        );
        let attrs = state
            .surface_attributes()
            .id("primary-navigation")
            .labelled_by("nav-title")
            .described_by("nav-description");

        b.iter(|| {
            black_box(drawer::render_drawer_surface_html(
                &state,
                attrs.clone(),
                "<nav role=\"navigation\">...</nav>",
            ));
        });
    });

    group.bench_function("persistent_surface_closed", |b| {
        let state = DrawerState::new(
            false,
            ControlStrategy::Uncontrolled,
            DrawerVariant::Persistent,
            DrawerAnchor::End,
        );
        let attrs = state.surface_attributes();

        b.iter(|| {
            black_box(drawer::render_drawer_surface_html(
                &state,
                attrs.clone(),
                "<aside>Reports</aside>",
            ));
        });
    });

    group.finish();
}

criterion_group!(
    material_renderers,
    bench_button_render,
    bench_drawer_surface_render
);
criterion_main!(material_renderers);
