//! Throttle utility.
//!
//! Unlike [`debounce`](crate::debounce), throttling ensures a function executes
//! at most once within a given interval. Excess calls are dropped which keeps
//! hot paths lean and predictable.
//!
//! # Examples
//! Throttling a counter using an explicit unit type. Passing `()` keeps the
//! closure generic while allowing easy integration with event handlers that
//! pass richer arguments:
//! ```
//! use mui_utils::throttle;
//! use std::time::Duration;
//!
//! let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
//! let c = counter.clone();
//! let mut throttled = throttle(move |_| {
//!     *c.lock().unwrap() += 1;
//! }, Duration::from_millis(50));
//!
//! throttled(());
//! throttled(()); // ignored
//! std::thread::sleep(Duration::from_millis(60));
//! throttled(()); // runs again
//! assert_eq!(*counter.lock().unwrap(), 2);
//! ```
//!
//! Supplying a zero interval disables throttling and every invocation is
//! forwarded:
//! ```
//! use mui_utils::throttle;
//! use std::sync::{Arc, Mutex};
//! use std::time::Duration;
//!
//! let called = Arc::new(Mutex::new(0));
//! let c = called.clone();
//! let mut t = throttle(move |_| *c.lock().unwrap() += 1, Duration::ZERO);
//! t(());
//! t(());
//! assert_eq!(*called.lock().unwrap(), 2);
//! ```

use std::time::Duration;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
use wasm_bindgen::prelude::*;

/// Create a throttled version of `func`.
///
/// Calls to the returned closure that occur more often than `interval` will be
/// ignored. The implementation relies on the most efficient timing primitives
/// available for the current target. If `interval` is zero the original
/// function is returned unchanged.
pub fn throttle<T, F>(func: F, interval: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + 'static,
{
    throttle_impl(func, interval)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
fn throttle_impl<T, F>(mut func: F, interval: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + 'static,
{
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    let last = Arc::new(Mutex::new(None::<Instant>));
    move |arg: T| {
        if interval.is_zero() {
            func(arg);
            return;
        }
        let now = Instant::now();
        let mut last_lock = last.lock().unwrap();
        if last_lock.is_none_or(|l| now.duration_since(l) >= interval) {
            *last_lock = Some(now);
            func(arg);
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn throttle_impl<T, F>(mut func: F, interval: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + 'static,
{
    use std::cell::Cell;
    use std::rc::Rc;

    let last = Rc::new(Cell::new(0f64));
    let ms = interval.as_millis() as f64;
    move |arg: T| {
        if interval.is_zero() {
            func(arg);
            return;
        }
        let now = js_sys::Date::now();
        let prev = last.get();
        if now - prev >= ms {
            last.set(now);
            func(arg);
        }
    }
}
