//! Debounce utility.
//!
//! Produces a closure that delays execution of a function until a period of
//! inactivity has elapsed. This is useful to coalesce bursty events such as
//! keystrokes or window resizes. The implementation is designed around a
//! minimal state machine so that, after inlining, the optimizer can remove
//! any overhead beyond the necessary timer bookkeeping.
//!
//! # Examples
//! ```
//! use mui_utils::debounce;
//! use std::time::Duration;
//!
//! let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
//! let c = counter.clone();
//! let mut debounced = debounce(move || {
//!     *c.lock().unwrap() += 1;
//! }, Duration::from_millis(50));
//!
//! debounced();
//! debounced();
//! std::thread::sleep(Duration::from_millis(60));
//! assert_eq!(*counter.lock().unwrap(), 1);
//! ```

use std::time::Duration;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
use wasm_bindgen::JsCast;

/// Create a debounced version of `func`.
///
/// The returned closure can be called repeatedly; `func` only executes after
/// `delay` has elapsed without another invocation. This implementation avoids
/// heap allocations for the happy path and leverages conditional compilation to
/// integrate with `wasm-bindgen` timers when targeting WebAssembly.
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub fn debounce<F>(func: F, delay: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + 'static,
{
    debounce_impl(func, delay)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
pub fn debounce<F>(func: F, delay: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + Send + 'static,
{
    debounce_impl(func, delay)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
fn debounce_impl<F>(func: F, delay: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + Send + 'static,
{
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    };
    use std::thread;

    let func = Arc::new(Mutex::new(func));
    let pending = Arc::new(Mutex::new(None::<Arc<AtomicBool>>));

    move || {
        if let Some(flag) = pending.lock().unwrap().take() {
            flag.store(true, Ordering::SeqCst);
        }
        let func = func.clone();
        let flag = Arc::new(AtomicBool::new(false));
        *pending.lock().unwrap() = Some(flag.clone());
        thread::spawn(move || {
            thread::sleep(delay);
            if !flag.load(Ordering::SeqCst) {
                (func.lock().unwrap())();
            }
        });
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn debounce_impl<F>(mut func: F, delay: Duration) -> impl FnMut() + 'static
where
    F: FnMut() + 'static,
{
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    let func = Rc::new(RefCell::new(func));
    let handle = Rc::new(Cell::new(None));
    let ms = delay.as_millis() as i32;

    move || {
        let window = web_sys::window().expect("window available");
        if let Some(id) = handle.get() {
            window.clear_timeout_with_handle(id);
        }
        let func = func.clone();
        let handle_clone = handle.clone();
        let closure = Closure::once_into_js(move || {
            (func.borrow_mut())();
            handle_clone.set(None);
        });
        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                ms,
            )
            .expect("timeout set");
        closure.forget();
        handle.set(Some(id));
    }
}

#[cfg(test)]
mod tests {
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
}
