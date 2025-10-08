//! Performance smoke tests for the shared transition state machine.
//!
//! The benches double as executable documentation: they highlight the
//! steady-state lifecycle we expect overlay components to follow and provide a
//! regression signal when additional bookkeeping sneaks into the hot path.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rustic_ui_headless::transition::{TransitionPhase, TransitionState};

/// Exercises the full enter/exit lifecycle and records how much work is
/// required to advance through the phases. Overlay adapters call this sequence
/// on every frame; keeping it lean prevents unnecessary layout thrashing in the
/// web renderers that wrap the headless core.
fn bench_transition_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("headless/transition/lifecycle");
    group.throughput(Throughput::Elements(1));

    group.bench_function("enter_visible_exit", |b| {
        b.iter(|| {
            let mut state = TransitionState::new(Some("modal".into()));
            assert!(state.begin_enter());
            assert_eq!(state.snapshot().phase(), TransitionPhase::Entering);
            assert!(state.mark_visible());
            assert_eq!(state.snapshot().phase(), TransitionPhase::Visible);
            assert!(state.begin_exit());
            assert_eq!(state.snapshot().phase(), TransitionPhase::Exiting);
            assert!(state.complete());
            assert_eq!(state.snapshot().phase(), TransitionPhase::Completed);
            state.reset();
            black_box(state);
        });
    });

    group.finish();
}

/// Benchmarks the cost of driving multiple transitions in lockstep as seen in
/// menus that orchestrate nested overlays (for example popovers rendering
/// tooltips and confirmation sheets simultaneously).
///
/// Each iteration reuses the same transition pool to focus the measurement on
/// state-machine bookkeeping rather than allocation churn. The benchmark walks
/// through a representative frame sequence: starting enter animations,
/// acknowledging the visible phase, beginning exit animations, and finally
/// completing the lifecycle before resetting back to `Idle`.
fn bench_transition_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("headless/transition/pool");
    group.throughput(Throughput::Elements(1));

    group.bench_function("overlay_cluster", |b| {
        let mut pool: Vec<TransitionState> = (0..8)
            .map(|idx| TransitionState::new(Some(format!("overlay-{idx}"))))
            .collect();

        b.iter(|| {
            for state in &mut pool {
                assert!(state.begin_enter());
            }
            for state in &mut pool {
                assert!(state.mark_visible());
            }
            for state in &mut pool {
                assert!(state.begin_exit());
            }
            for state in &mut pool {
                assert!(state.complete());
                state.reset();
                black_box(state);
            }
        });
    });

    group.finish();
}

criterion_group!(
    transition_benches,
    bench_transition_lifecycle,
    bench_transition_pool
);
criterion_main!(transition_benches);
