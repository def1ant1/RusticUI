use proptest::prelude::*;

// Pure algorithm used for property based testing: given a list of event
// times (monotonic, milliseconds) return the times when the debounced
// function would actually execute. Each call schedules execution `wait`
// milliseconds after the last input in the burst.
fn simulated(times: &[u64], wait: u64) -> Vec<u64> {
    if times.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut last = times[0];
    for &t in &times[1..] {
        if t - last >= wait {
            out.push(last + wait);
            last = t;
        } else {
            last = t;
        }
    }
    out.push(last + wait);
    out
}

proptest! {
    #[test]
    fn debounced_events_are_spaced(wait in 1u64..100u64, mut times in proptest::collection::vec(0u64..1000u64, 1..20)) {
        times.sort_unstable();
        let out = simulated(&times, wait);
        for w in out.windows(2) {
            prop_assert!(w[1] - w[0] >= wait);
        }
    }
}
