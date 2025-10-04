//! Selection control state machines and telemetry adapters exported to React via WebAssembly.
//!
//! This crate intentionally contains exhaustive inline documentation so that engineers adopting
//! the pattern in enterprise environments understand every moving part.  The code is compiled to
//! `wasm32-unknown-unknown` through `wasm-pack` and surfaced to React components through the
//! bindings emitted by `wasm-bindgen`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use chrono::Utc;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Identifier used for analytics when a control mounts and renders for the first time.
static MOUNT_ACTION: Lazy<String> = Lazy::new(|| "mount".to_owned());
/// Identifier used when a control is toggled through direct user interaction (click / tap / key).
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
static USER_ACTION: Lazy<String> = Lazy::new(|| "user".to_owned());
/// Identifier used for programmatic updates, typically triggered by controlled React props.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
static PROGRAMMATIC_ACTION: Lazy<String> = Lazy::new(|| "programmatic".to_owned());

/// Enumeration of the control kind so analytics pipelines can bucket events consistently.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ControlKind {
    Checkbox,
    Switch,
    Radio,
}

/// Source descriptor captures *why* a state mutation happened which is crucial for automation
/// and regression tooling.  Automation engines can assert that a programmatic update immediately
/// follows the user intent they simulated.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TelemetrySource {
    /// Represents direct user intent (click, tap, keyboard navigation).
    User,
    /// Represents updates flowing from React props or other orchestrated automation.
    Programmatic,
    /// Represents the component wiring itself together while mounting/hydrating.
    Lifecycle,
}

/// Exhaustive telemetry payload emitted for every state transition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryEvent {
    /// Monotonic sequence number to guarantee ordering even when timestamps are identical.
    pub sequence: u64,
    /// The selection control type that generated the event.
    pub control_kind: ControlKind,
    /// Developer supplied identifier that is stable across renders and environments.
    pub control_id: String,
    /// High level action descriptor ("mount", "user", "programmatic").
    pub action: String,
    /// The resulting checked state after the action completed.
    pub checked: bool,
    /// Timestamp captured as milliseconds from epoch.
    pub timestamp_ms: i64,
    /// Whether the transition came from lifecycle wiring or real user intent.
    pub source: TelemetrySource,
    /// Indicates whether the control is operating in controlled (React drives state) mode.
    pub controlled: bool,
}

/// Internal telemetry delegate handle that is shared between the exported wasm API and the host
/// side tests.  The handle owns the buffer of events and the optional JavaScript callback.
#[derive(Clone)]
struct TelemetryDelegateHandle {
    events: Rc<RefCell<Vec<TelemetryEvent>>>,
    #[cfg(target_arch = "wasm32")]
    // The JavaScript callback only exists for wasm builds; `js-sys` intentionally omits
    // `Function` definitions for native targets so we hide the field behind a cfg gate.
    callback: Rc<RefCell<Option<js_sys::Function>>>,
    sequence: Rc<Cell<u64>>,
}

impl TelemetryDelegateHandle {
    fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::with_capacity(32))),
            #[cfg(target_arch = "wasm32")]
            // For wasm we eagerly seed the callback holder so hooks can bind immediately after
            // construction while native unit tests keep the leaner host-only handle.
            callback: Rc::new(RefCell::new(None)),
            sequence: Rc::new(Cell::new(0)),
        }
    }

    fn next_sequence(&self) -> u64 {
        let next = self.sequence.get() + 1;
        self.sequence.set(next);
        next
    }

    fn record(&self, mut event: TelemetryEvent) {
        if event.sequence == 0 {
            event.sequence = self.next_sequence();
        }
        self.events.borrow_mut().push(event.clone());
        #[cfg(target_arch = "wasm32")]
        if let Some(cb) = self.callback.borrow().as_ref() {
            // The callback receives the serialized telemetry payload so that React applications can
            // pipe it into analytics or automation streams without extra glue code.
            if let Ok(payload) = JsValue::from_serde(&event) {
                let _ = cb.call1(&JsValue::NULL, &payload);
            }
        }
    }

    fn drain(&self) -> Vec<TelemetryEvent> {
        let mut borrowed = self.events.borrow_mut();
        let mut drained = Vec::new();
        std::mem::swap(&mut *borrowed, &mut drained);
        drained
    }
}

/// Delegate exposed to JavaScript consumers.  The delegate can stream telemetry events to a
/// callback (when running in wasm) or allow Rust tests to read the buffer directly.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct TelemetryDelegate {
    handle: TelemetryDelegateHandle,
}

impl TelemetryDelegate {
    fn new_internal() -> Self {
        Self {
            handle: TelemetryDelegateHandle::new(),
        }
    }

    fn clone_internal(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn drain_host(&self) -> Vec<TelemetryEvent> {
        self.handle.drain()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TelemetryDelegate {
    /// Creates a new delegate.  In React applications the same delegate can be shared across
    /// multiple controls to drive a unified analytics stream.
    #[wasm_bindgen(constructor)]
    pub fn new() -> TelemetryDelegate {
        TelemetryDelegate::new_internal()
    }

    /// Binds a JavaScript callback that receives each telemetry event as soon as it is generated.
    #[wasm_bindgen(js_name = bind)]
    pub fn bind_callback(&self, callback: &js_sys::Function) {
        *self.handle.callback.borrow_mut() = Some(callback.clone());
    }

    /// Flushes the telemetry buffer into JavaScript as a JSON array, returning it for further
    /// processing (e.g. to assert ordering in automated tests).
    pub fn drain(&self) -> JsValue {
        let drained = self.handle.drain();
        JsValue::from_serde(&drained).expect("telemetry to serialize")
    }

    /// Clones the delegate so multiple React hooks can share the same telemetry sink.
    pub fn clone_handle(&self) -> TelemetryDelegate {
        self.clone_internal()
    }
}

/// Internal state machine used by all binary selection controls.
struct BinaryControlMachine {
    id: String,
    kind: ControlKind,
    checked: bool,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    controlled: bool,
    delegate: TelemetryDelegateHandle,
}

impl BinaryControlMachine {
    fn new(
        id: String,
        kind: ControlKind,
        initial_checked: bool,
        controlled: bool,
        delegate: TelemetryDelegateHandle,
    ) -> Self {
        let mut machine = Self {
            id,
            kind,
            checked: initial_checked,
            controlled,
            delegate,
        };
        // Emit an initial lifecycle event so telemetry clients know when hydration happened.
        machine.emit_event(&MOUNT_ACTION, TelemetrySource::Lifecycle);
        machine
    }

    fn emit_event(&mut self, action: &str, source: TelemetrySource) {
        let event = TelemetryEvent {
            sequence: 0,
            control_kind: self.kind.clone(),
            control_id: self.id.clone(),
            action: action.to_owned(),
            checked: self.checked,
            timestamp_ms: Utc::now().timestamp_millis(),
            source,
            controlled: self.controlled,
        };
        self.delegate.record(event);
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn toggle_user(&mut self) -> bool {
        self.checked = !self.checked;
        self.emit_event(&USER_ACTION, TelemetrySource::User);
        self.checked
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn set_checked(&mut self, checked: bool, source: TelemetrySource) -> bool {
        self.checked = checked;
        self.emit_event(&PROGRAMMATIC_ACTION, source);
        self.checked
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn current(&self) -> bool {
        self.checked
    }
}

/// Handle returned to React bindings representing a "hook" like API.  The name mirrors React's
/// hook naming, but it works in any JavaScript environment.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct ControlHook {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    machine: Rc<RefCell<BinaryControlMachine>>,
    delegate: TelemetryDelegate,
}

impl ControlHook {
    fn new(id: String, kind: ControlKind, initial_checked: bool, controlled: bool) -> Self {
        let delegate = TelemetryDelegate::new_internal();
        let machine = BinaryControlMachine::new(
            id,
            kind,
            initial_checked,
            controlled,
            delegate.handle.clone(),
        );
        Self {
            machine: Rc::new(RefCell::new(machine)),
            delegate,
        }
    }

    fn from_delegate(
        id: String,
        kind: ControlKind,
        initial_checked: bool,
        controlled: bool,
        delegate: TelemetryDelegate,
    ) -> Self {
        let machine = BinaryControlMachine::new(
            id,
            kind,
            initial_checked,
            controlled,
            delegate.handle.clone(),
        );
        Self {
            machine: Rc::new(RefCell::new(machine)),
            delegate,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn delegate_host(&self) -> TelemetryDelegate {
        self.delegate.clone_internal()
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn toggle_internal(&self) -> bool {
        self.machine.borrow_mut().toggle_user()
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn set_checked_internal(&self, checked: bool, source: TelemetrySource) -> bool {
        self.machine.borrow_mut().set_checked(checked, source)
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn current_internal(&self) -> bool {
        self.machine.borrow().current()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ControlHook {
    /// Returns the latest `checked` state.  React controlled components can call this during
    /// render while uncontrolled components can invoke it from effects.
    pub fn checked(&self) -> bool {
        self.current_internal()
    }

    /// Toggles the control as if the user interacted with it.
    #[wasm_bindgen(js_name = userToggle)]
    pub fn user_toggle(&self) -> bool {
        self.toggle_internal()
    }

    /// Applies a programmatic update (e.g. React controlled prop change).
    #[wasm_bindgen(js_name = setChecked)]
    pub fn set_checked(&self, checked: bool) -> bool {
        self.set_checked_internal(checked, TelemetrySource::Programmatic)
    }

    /// Exposes the telemetry delegate so JavaScript can bind callbacks.
    pub fn delegate(&self) -> TelemetryDelegate {
        self.delegate.clone_internal()
    }
}

/// Shared factory powering the exported hook helpers.
fn build_hook(
    control_id: String,
    kind: ControlKind,
    initial: bool,
    controlled: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    match delegate {
        Some(delegate) => {
            ControlHook::from_delegate(control_id, kind, initial, controlled, delegate)
        }
        None => ControlHook::new(control_id, kind, initial, controlled),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_checkbox_controlled(
    control_id: String,
    checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(control_id, ControlKind::Checkbox, checked, true, delegate)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_checkbox_uncontrolled(
    control_id: String,
    default_checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(
        control_id,
        ControlKind::Checkbox,
        default_checked,
        false,
        delegate,
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_switch_controlled(
    control_id: String,
    checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(control_id, ControlKind::Switch, checked, true, delegate)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_switch_uncontrolled(
    control_id: String,
    default_checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(
        control_id,
        ControlKind::Switch,
        default_checked,
        false,
        delegate,
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_radio_controlled(
    control_id: String,
    checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(control_id, ControlKind::Radio, checked, true, delegate)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn use_radio_uncontrolled(
    control_id: String,
    default_checked: bool,
    delegate: Option<TelemetryDelegate>,
) -> ControlHook {
    build_hook(
        control_id,
        ControlKind::Radio,
        default_checked,
        false,
        delegate,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_events_are_emitted_in_order() {
        let hook = build_hook(
            "checkbox-1".to_string(),
            ControlKind::Checkbox,
            false,
            true,
            None,
        );
        assert!(!hook.current_internal());
        hook.set_checked_internal(true, TelemetrySource::Programmatic);
        hook.toggle_internal();
        hook.set_checked_internal(false, TelemetrySource::Programmatic);
        let events = hook.delegate_host().drain_host();
        let sequences: Vec<u64> = events.iter().map(|event| event.sequence).collect();
        let mut sorted = sequences.clone();
        sorted.sort();
        assert_eq!(
            sequences, sorted,
            "Telemetry events should have monotonically increasing sequence numbers"
        );
        assert_eq!(events.first().unwrap().action, *MOUNT_ACTION);
        assert_eq!(events.last().unwrap().checked, false);
        assert!(events.iter().all(|event| event.controlled));
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn wasm_toggle_updates_state() {
        let hook = use_switch_uncontrolled("switch-1".to_owned(), false, None);
        assert_eq!(hook.checked(), false);
        assert_eq!(hook.user_toggle(), true);
        assert_eq!(hook.checked(), true);
        let drained: Vec<TelemetryEvent> = hook
            .delegate()
            .drain()
            .into_serde()
            .expect("telemetry to deserialize");
        assert!(drained
            .iter()
            .any(|event| event.source == TelemetrySource::User));
        assert!(drained.iter().any(|event| event.controlled == false));
    }
}
