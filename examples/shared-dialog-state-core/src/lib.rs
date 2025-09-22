//! Shared dialog, popover, and text field state machine orchestration used by
//! the cross-framework automation blueprints.  The helpers in this crate keep
//! the deterministic [`mui_headless`] state machines front-and-center so every
//! adapter (Yew, Leptos, Dioxus, Sycamore) can render identical markup, emit the
//! same automation hooks, and perform validation with matching semantics without
//! copy/pasting lifecycle code.
//!
//! The goal is to minimise manual wiring for enterprise teams.  Server rendered
//! HTML, client hydration, and pre-production automation pipelines all flow
//! through the same state containers.  Consumers typically clone the
//! [`SharedOverlayState`] into a UI specific signal/`use_state` handle and call
//! the intent helpers when user events fire.

use std::time::Duration;

use mui_headless::dialog::{DialogPhase, DialogState, DialogTransition};
use mui_headless::popover::{AnchorGeometry, CollisionOutcome, PopoverPlacement, PopoverState};
use mui_headless::text_field::TextFieldState;

/// ASCII anchor/floating surface illustration rendered in each example README
/// to explain how the shared state tracks geometry between SSR and hydration.
pub const ANCHOR_DIAGRAM: &str = r#"
┌─────────────────────────┐
│ Trigger button (A11y ID │
│ shared-popover-anchor)  │
└────────────┬────────────┘
             │ anchor geometry captured
             ▼
      ╔════════════════════╗
      ║   Popover surface  ║
      ║ data-preferred=bottom
      ║ data-resolved=top  ║
      ╚════════════════════╝
"#;

/// Default analytics identifier for the dialog surface.
pub const DIALOG_SURFACE_ANALYTICS_ID: &str = "shared-dialog-surface";
/// Default analytics identifier for the popover surface.
pub const POPOVER_SURFACE_ANALYTICS_ID: &str = "shared-popover-surface";
/// Element identifier shared by every framework for the anchor button.
pub const POPOVER_ANCHOR_ID: &str = "shared-popover-anchor";
/// Identifier used by validation status elements in the examples.
pub const TEXT_FIELD_STATUS_ID: &str = "shared-text-field-status";
/// Analytics identifier emitted by the text field automation hooks.
pub const TEXT_FIELD_ANALYTICS_ID: &str = "shared-text-field-automation";
/// Heading identifier linked via `aria-labelledby` for the dialog surface.
pub const DIALOG_TITLE_ID: &str = "shared-dialog-title";
/// Description identifier linked via `aria-describedby` for the dialog surface.
pub const DIALOG_DESCRIPTION_ID: &str = "shared-dialog-description";

/// Snapshot summarising the state machines for dashboards, logging, or UI state
/// renderers.  Every example requests a snapshot on each render to surface
/// parity between SSR and CSR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedOverlaySnapshot {
    /// Lifecycle phase of the dialog surface.
    pub dialog_phase: DialogPhase,
    /// Whether the focus trap is currently engaged.
    pub dialog_focus_trap: bool,
    /// Last transition intent emitted by the dialog state.
    pub dialog_transition: Option<DialogTransition>,
    /// Whether the popover is visible.
    pub popover_open: bool,
    /// Preferred popover placement configured by the design system.
    pub popover_preferred: PopoverPlacement,
    /// Placement resolved after collision detection.
    pub popover_resolved: PopoverPlacement,
    /// Outcome of the most recent collision run.
    pub popover_collision_outcome: CollisionOutcome,
    /// Anchor identifier mirrored across frameworks for analytics hooks.
    pub popover_anchor_id: Option<String>,
    /// Value currently stored in the text field.
    pub text_field_value: String,
    /// Whether the text field diverges from the initial value.
    pub text_field_dirty: bool,
    /// Whether the text field has been visited (blurred/committed).
    pub text_field_visited: bool,
    /// Validation errors currently applied to the text field.
    pub text_field_errors: Vec<String>,
}

impl SharedOverlaySnapshot {
    /// Convenience helper returning `true` when validation errors exist.
    #[inline]
    pub fn text_field_has_errors(&self) -> bool {
        !self.text_field_errors.is_empty()
    }
}

/// Minimal log structure collected after each intent helper executes.  The
/// examples push these entries into framework specific signals so developer
/// consoles and QA dashboards can confirm identical lifecycles across targets.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LifecycleLog {
    /// Human readable notes describing lifecycle events.
    pub entries: Vec<String>,
}

impl LifecycleLog {
    /// Record a lifecycle message.
    pub fn record(&mut self, line: impl Into<String>) {
        self.entries.push(line.into());
    }

    /// Merge another log into the current collection.
    pub fn extend(&mut self, other: LifecycleLog) {
        self.entries.extend(other.entries);
    }
}

/// Aggregated dialog/popover/text field state used by the automation-first
/// blueprints.  The struct derives [`Clone`] so framework hooks can copy the
/// current snapshot, mutate it, and push the new state back into the reactive
/// system without dealing with reference cycles.
#[derive(Debug, Clone)]
pub struct SharedOverlayState {
    dialog: DialogState,
    popover: PopoverState,
    text_field: TextFieldState,
}

impl SharedOverlayState {
    /// Construct the canonical state configuration shared by every example.
    /// Dialogs and popovers are controlled so the caller remains the source of
    /// truth for visibility, mirroring real applications where SSR snapshots and
    /// client hydration need to line up exactly.
    pub fn enterprise_defaults() -> Self {
        let mut dialog = DialogState::controlled();
        dialog.set_modal(true);
        dialog.set_escape_closes(true);

        let mut popover = PopoverState::controlled(PopoverPlacement::Bottom);
        popover.set_anchor_metadata(
            Some(POPOVER_ANCHOR_ID),
            Some(AnchorGeometry {
                x: 320.0,
                y: 640.0,
                width: 240.0,
                height: 48.0,
            }),
        );

        let text_field = TextFieldState::controlled(
            "Automation ready company",
            Some(Duration::from_millis(250)),
        );

        Self {
            dialog,
            popover,
            text_field,
        }
    }

    /// Returns the current snapshot for analytics or read-only rendering.
    pub fn snapshot(&self) -> SharedOverlaySnapshot {
        SharedOverlaySnapshot {
            dialog_phase: self.dialog.phase(),
            dialog_focus_trap: self.dialog.focus_trap_engaged(),
            dialog_transition: self.dialog.last_transition(),
            popover_open: self.popover.is_open(),
            popover_preferred: self.popover.preferred_placement(),
            popover_resolved: self.popover.resolved_placement(),
            popover_collision_outcome: self.popover.last_outcome(),
            popover_anchor_id: self.popover.anchor_id().map(str::to_string),
            text_field_value: self.text_field.value().to_string(),
            text_field_dirty: self.text_field.dirty(),
            text_field_visited: self.text_field.visited(),
            text_field_errors: self.text_field.errors().to_vec(),
        }
    }

    /// Immutable accessors used by framework adapters to pull attribute builders.
    pub fn dialog(&self) -> &DialogState {
        &self.dialog
    }

    /// Mutable access to the dialog state.  Typically reserved for unit tests.
    pub fn dialog_mut(&mut self) -> &mut DialogState {
        &mut self.dialog
    }

    /// Immutable accessors used by framework adapters to pull attribute builders.
    pub fn popover(&self) -> &PopoverState {
        &self.popover
    }

    /// Mutable access to the popover state.  Typically reserved for unit tests.
    pub fn popover_mut(&mut self) -> &mut PopoverState {
        &mut self.popover
    }

    /// Immutable accessors used by framework adapters to pull attribute builders.
    pub fn text_field(&self) -> &TextFieldState {
        &self.text_field
    }

    /// Mutable access to the text field state.  Typically reserved for unit tests.
    pub fn text_field_mut(&mut self) -> &mut TextFieldState {
        &mut self.text_field
    }

    /// Request the dialog to open and synchronise the visible state.
    pub fn request_dialog_open(mut self) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        let mut desired = false;
        self.dialog.open(|next| {
            desired = next;
            log.record("dialog requested open");
        });
        self.dialog.sync_open(desired);
        self.dialog.finish_open();
        log.record(format!(
            "dialog phase -> {} (focus trap engaged: {})",
            self.dialog.phase().as_str(),
            self.dialog.focus_trap_engaged()
        ));
        (self, log)
    }

    /// Request the dialog to close and synchronise the visible state.
    pub fn request_dialog_close(mut self) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        let mut desired = true;
        self.dialog.close(|next| {
            desired = next;
            log.record("dialog requested close");
        });
        if !desired {
            self.dialog.sync_open(false);
            self.dialog.finish_close();
        }
        log.record("dialog phase -> closed (focus trap released)");
        (self, log)
    }

    /// Toggle the popover visibility and perform a lightweight collision
    /// resolution that mirrors the SSR logic used in the blueprints.
    pub fn toggle_popover(mut self) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        let mut desired = None;
        self.popover.toggle(|next| {
            desired = Some(next);
            log.record(format!(
                "popover requested {}",
                if next { "open" } else { "close" }
            ));
        });
        if let Some(open) = desired {
            self.popover.sync_open(open);
            if open {
                self.popover.resolve_with(|geometry, preferred| {
                    if geometry.y + geometry.height > 640.0 {
                        PopoverPlacement::Top
                    } else {
                        preferred
                    }
                });
                log.record(format!(
                    "popover resolved placement -> {} ({:?})",
                    self.popover.resolved_placement().as_str(),
                    self.popover.last_outcome()
                ));
            }
        }
        (self, log)
    }

    /// Update the stored anchor geometry.  The examples call this when viewport
    /// resizes occur so SSR snapshots and hydrated layouts remain aligned.
    pub fn update_anchor_geometry(mut self, geometry: AnchorGeometry) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        self.popover
            .set_anchor_metadata(Some(POPOVER_ANCHOR_ID), Some(geometry));
        log.record(format!(
            "anchor geometry updated -> x:{:.1} y:{:.1} w:{:.1} h:{:.1}",
            geometry.x, geometry.y, geometry.width, geometry.height
        ));
        (self, log)
    }

    /// Apply a change to the text field.  Controlled state is synchronised so the
    /// caller remains the source of truth for the value.
    pub fn change_text(mut self, next: impl Into<String>) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        let mut latest = String::new();
        self.text_field.change(next, |snapshot| {
            latest = snapshot.value.to_string();
            log.record(format!(
                "text change -> '{}' (dirty: {}, debounce: {:?})",
                snapshot.value,
                snapshot.dirty,
                snapshot.debounce.map(|d| d.as_millis())
            ));
        });
        self.text_field.sync_value(latest.clone());
        log.record(format!("text value synchronised -> '{}'", latest));
        (self, log)
    }

    /// Commit the text field (blur/enter) and run validation.  Errors are stored
    /// inside the state machine so every framework renders identical status copy.
    pub fn commit_text(mut self) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        let validation = self.recompute_validation();
        self.text_field.commit(|snapshot| {
            log.record(format!(
                "text commit -> '{}' (visited before: {}, errors present: {})",
                snapshot.value, snapshot.previously_visited, snapshot.has_errors
            ));
        });
        if let Some(message) = validation {
            log.record(format!("validation -> {}", message));
        } else {
            log.record("validation -> clear".to_string());
        }
        (self, log)
    }

    /// Reset the text field to its initial value, clearing validation errors.
    pub fn reset_text(mut self) -> (Self, LifecycleLog) {
        let mut log = LifecycleLog::default();
        self.text_field.reset(|snapshot| {
            log.record(format!(
                "text reset -> '{}' (cleared errors: {})",
                snapshot.value, snapshot.cleared_errors
            ));
        });
        (self, log)
    }

    fn recompute_validation(&mut self) -> Option<String> {
        let value = self.text_field.value().trim();
        let mut errors = Vec::new();
        if value.is_empty() {
            errors.push("Company name is required.".to_string());
        }
        if value.len() < 3 {
            errors.push("Company name must be at least 3 characters.".to_string());
        }
        if value.chars().all(|c| c.is_ascii_alphabetic()) {
            // Accept purely alphabetic strings; automation users often paste
            // identifiers containing spaces and digits.
        } else if value.chars().any(|c| c.is_ascii_punctuation()) {
            errors.push("Remove punctuation before submitting.".to_string());
        }
        if errors.is_empty() {
            self.text_field.set_errors(Vec::new());
            None
        } else {
            let joined = errors.join(" ");
            self.text_field.set_errors(errors);
            Some(joined)
        }
    }
}

impl Default for SharedOverlayState {
    fn default() -> Self {
        Self::enterprise_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_open_and_close_log_transitions() {
        let state = SharedOverlayState::enterprise_defaults();
        let (state, log) = state.request_dialog_open();
        assert!(state.dialog().is_open());
        assert!(log.entries.iter().any(|line| line.contains("dialog phase")));

        let (_, log_close) = state.request_dialog_close();
        assert!(log_close
            .entries
            .iter()
            .any(|line| line.contains("dialog phase -> closed")));
    }

    #[test]
    fn popover_toggle_updates_snapshot() {
        let state = SharedOverlayState::enterprise_defaults();
        let (state, _) = state.toggle_popover();
        let snapshot = state.snapshot();
        assert!(snapshot.popover_open);
        assert_eq!(
            snapshot.popover_anchor_id.as_deref(),
            Some(POPOVER_ANCHOR_ID)
        );
    }

    #[test]
    fn text_validation_marks_errors() {
        let state = SharedOverlayState::enterprise_defaults();
        let (state, _) = state.change_text("x");
        let (state, _) = state.commit_text();
        let snapshot = state.snapshot();
        assert!(snapshot.text_field_has_errors());
        assert!(snapshot
            .text_field_errors
            .iter()
            .any(|msg| msg.contains("Company name")));
    }
}
