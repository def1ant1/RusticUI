//! Throttle utility.
//!
//! Unlike [`debounce`](crate::debounce), throttling ensures a function executes
//! at most once within a given interval. Excess calls are dropped which keeps
//! hot paths lean and predictable.
//!
//! # Examples
//! ```
//! use mui_utils::throttle;
//! use std::time::Duration;
//!
//! let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
//! let c = counter.clone();
//! let mut throttled = throttle(move || {
//!     *c.lock().unwrap() += 1;
//! }, Duration::from_millis(50));
//!
//! throttled();
//! throttled(); // ignored
//! std::thread::sleep(Duration::from_millis(60));
//! throttled(); // runs again
//! assert_eq!(*counter.lock().unwrap(), 2);
//! ```

use std::time::Duration;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
use wasm_bindgen::prelude::*;

/// Create a throttled version of `func`.
///
/// Calls to the returned closure that occur more often than `interval` will be
/// ignored. The implementation relies on the most efficient timing primitives
/// available for the current target.
pub fn throttle<F>(func: F, interval: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + 'static,
{
    throttle_impl(func, interval)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
fn throttle_impl<F>(mut func: F, interval: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + 'static,
{
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    let last = Arc::new(Mutex::new(None::<Instant>));
    move || {
        let now = Instant::now();
        let mut last_lock = last.lock().unwrap();
        if last_lock.map_or(true, |l| now.duration_since(l) >= interval) {
            *last_lock = Some(now);
            func();
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn throttle_impl<F>(mut func: F, interval: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + 'static,
{
    use std::cell::Cell;
    use std::rc::Rc;

    let last = Rc::new(Cell::new(0f64));
    let ms = interval.as_millis() as f64;
    move || {
        let now = js_sys::Date::now();
        let prev = last.get();
        if now - prev >= ms {
            last.set(now);
            func();
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    // Pure helper for property based tests. Given monotonically increasing
    // event times it returns the subset that would trigger execution when
    // throttled by `interval`.
    pub(crate) fn simulated(times: &[u64], interval: u64) -> Vec<u64> {
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
}
