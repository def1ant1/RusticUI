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
//! use rustic_ui_utils::debounce;
//! use std::time::Duration;
//!
//! let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
//! let c = counter.clone();
//! let mut debounced = debounce(move |_| {
//!     *c.lock().unwrap() += 1;
//! }, Duration::from_millis(50));
//!
//! debounced(());
//! debounced(());
//! std::thread::sleep(Duration::from_millis(60));
//! assert_eq!(*counter.lock().unwrap(), 1);
//! ```
//!
//! Supplying a zero delay forwards calls immediately:
//! ```
//! use rustic_ui_utils::debounce;
//! use std::sync::{Arc, Mutex};
//! use std::time::Duration;
//!
//! let called = Arc::new(Mutex::new(0));
//! let c = called.clone();
//! let mut d = debounce(move |_| *c.lock().unwrap() += 1, Duration::ZERO);
//! d(());
//! d(());
//! assert_eq!(*called.lock().unwrap(), 2);
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
pub fn debounce<T, F>(func: F, delay: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + 'static,
    T: Clone + 'static,
{
    debounce_impl(func, delay)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
pub fn debounce<T, F>(func: F, delay: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + Send + 'static,
    T: Send + Clone + 'static,
{
    debounce_impl(func, delay)
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
fn debounce_impl<T, F>(func: F, delay: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + Send + 'static,
    T: Send + Clone + 'static,
{
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    };
    use std::thread;

    let func = Arc::new(Mutex::new(func));
    let pending = Arc::new(Mutex::new(None::<Arc<AtomicBool>>));
    let latest = Arc::new(Mutex::new(None::<T>));

    move |arg: T| {
        if delay.is_zero() {
            (func.lock().unwrap())(arg);
            return;
        }
        if let Some(flag) = pending.lock().unwrap().take() {
            flag.store(true, Ordering::SeqCst);
        }
        *latest.lock().unwrap() = Some(arg.clone());
        let func = func.clone();
        let flag = Arc::new(AtomicBool::new(false));
        let latest_clone = latest.clone();
        *pending.lock().unwrap() = Some(flag.clone());
        thread::spawn(move || {
            thread::sleep(delay);
            if !flag.load(Ordering::SeqCst) {
                if let Some(val) = latest_clone.lock().unwrap().take() {
                    (func.lock().unwrap())(val);
                }
            }
        });
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn debounce_impl<T, F>(mut func: F, delay: Duration) -> impl FnMut(T) + 'static
where
    F: FnMut(T) + 'static,
    T: Clone + 'static,
{
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    let func = Rc::new(RefCell::new(func));
    let handle = Rc::new(Cell::new(None));
    let latest = Rc::new(RefCell::new(None::<T>));
    let ms = delay.as_millis() as i32;

    move |arg: T| {
        if delay.is_zero() {
            (func.borrow_mut())(arg);
            return;
        }
        let window = web_sys::window().expect("window available");
        if let Some(id) = handle.get() {
            window.clear_timeout_with_handle(id);
        }
        *latest.borrow_mut() = Some(arg.clone());
        let func = func.clone();
        let latest_clone = latest.clone();
        let handle_clone = handle.clone();
        let closure = Closure::once_into_js(move || {
            if let Some(v) = latest_clone.borrow_mut().take() {
                (func.borrow_mut())(v);
            }
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
