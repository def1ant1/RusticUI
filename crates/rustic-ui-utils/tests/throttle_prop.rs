use proptest::prelude::*;

// Pure helper for property based tests. Given monotonically increasing
// event times it returns the subset that would trigger execution when
// throttled by `interval`.
fn simulated(times: &[u64], interval: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut last = None;
    for &t in times {
        if last.map_or(true, |l| t - l >= interval) {
            out.push(t);
            last = Some(t);
        }
    }
    out
}

proptest! {
    #[test]
    fn throttled_events_are_spaced(interval in 1u64..100u64, mut times in proptest::collection::vec(0u64..1000u64, 1..20)) {
        times.sort_unstable();
        let out = simulated(&times, interval);
        for w in out.windows(2) {
            prop_assert!(w[1] - w[0] >= interval);
        }
    }
}
