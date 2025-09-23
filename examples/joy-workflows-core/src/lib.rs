//! Shared Joy UI workflow state powering the cross-framework demos.
//!
//! The goal of this crate is to capture **all** business logic, analytics
//! identifiers, and design-time defaults in one place so every adapter (Yew,
//! Leptos, Dioxus, Sycamore) can focus purely on rendering.  Keeping the
//! workflow deterministic dramatically reduces the amount of manual wiring that
//! enterprise application teams need to perform when spinning up new front-ends
//! or CI smoke tests.
//!
//! Highlights:
//! * **Single source of truth** – the [`JoyWorkflowMachine`] struct orchestrates
//!   progress tracking, snackbar messaging, and lifecycle logging without any
//!   framework specific code.
//! * **Design token aware** – descriptors expose [`mui_joy::Color`] and
//!   [`mui_joy::Variant`] enums so renderers can map directly onto Joy surface
//!   helpers (`resolve_surface_tokens`) without duplicating constants.
//! * **Automation ready** – every piece of UI state carries analytics IDs and
//!   data attribute helpers so QA pipelines can assert parity across SSR and
//!   hydrated runs.

use mui_headless::stepper::StepStatus;
use mui_joy::{Color, Variant};
use mui_system::theme::Theme;

/// Maximum number of lifecycle entries retained by the machine.
const MAX_LOG_ENTRIES: usize = 32;

/// Identifier bundle used by automation and analytics tooling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkflowAutomationIds {
    /// Analytics hook applied to the container card.
    pub card_id: &'static str,
    /// Identifier for the status chip representing the target environment.
    pub environment_chip_id: &'static str,
    /// Identifier applied to the capacity slider thumb.
    pub capacity_slider_id: &'static str,
    /// Identifier emitted on snackbar surfaces.
    pub snackbar_id: &'static str,
}

/// Declarative description of the Joy workflow blueprint.  This structure is
/// intentionally verbose so each adapter can reference the metadata directly in
/// templates without hard-coding labels or forgetting automation hooks.
#[derive(Clone)]
pub struct JoyWorkflowBlueprint {
    /// Theme shared across every framework example so design tokens stay in sync.
    pub theme: Theme,
    /// Headline describing the release under orchestration.
    pub release_title: &'static str,
    /// Supporting copy explaining the automation objective.
    pub release_summary: &'static str,
    /// Chip describing the target environment (production, staging, etc.).
    pub environment: ChipDescriptor,
    /// Primary action that advances the workflow.
    pub approve_action: ActionDescriptor,
    /// Secondary action that rolls the workflow back.
    pub rollback_action: ActionDescriptor,
    /// Slider controlling the deployment capacity percentage.
    pub capacity: SliderDescriptor,
    /// Ordered set of release checklist items rendered inside a stepper.
    pub steps: Vec<StepDescriptor>,
    /// Metrics surfaced in the card body (SLOs, automation coverage, etc.).
    pub metrics: Vec<CardMetric>,
    /// Snackbar descriptor used when steps complete or values change.
    pub snackbar: SnackbarDescriptor,
    /// Analytics identifiers consumed by QA tooling.
    pub automation: WorkflowAutomationIds,
}

impl JoyWorkflowBlueprint {
    /// Returns the enterprise defaults used by all demos.
    pub fn enterprise_release() -> Self {
        Self {
            theme: Theme::default(),
            release_title: "RusticUI Joy deployment pipeline",
            release_summary: "This dashboard mirrors the production change management flow. Every interaction flows through the shared machine so SSR and hydration stay in lockstep across frameworks.",
            environment: ChipDescriptor {
                label: "Production window",
                description: "Change freeze ends in 2h",
                color: Color::Neutral,
                variant: Variant::Soft,
            },
            approve_action: ActionDescriptor {
                label: "Approve next gate",
                description: "Confirms the current checklist item and unlocks the subsequent step.",
                color: Color::Primary,
                variant: Variant::Solid,
                analytics_id: "joy-approve-gate",
                throttle_ms: Some(500),
            },
            rollback_action: ActionDescriptor {
                label: "Rollback gate",
                description: "Reverts to the previous checklist item so issues can be triaged without bypassing controls.",
                color: Color::Danger,
                variant: Variant::Outlined,
                analytics_id: "joy-rollback-gate",
                throttle_ms: Some(500),
            },
            capacity: SliderDescriptor {
                min: 50.0,
                max: 150.0,
                step: 5.0,
                default: 100.0,
                marks: vec![
                    SliderMark { value: 60.0, label: "Canary" },
                    SliderMark { value: 100.0, label: "Baseline" },
                    SliderMark { value: 140.0, label: "Burst" },
                ],
            },
            steps: vec![
                StepDescriptor {
                    title: "Artifact integrity",
                    detail: "SBOM + signature validation mirrors supply chain policies.",
                },
                StepDescriptor {
                    title: "Security review",
                    detail: "Static analysis gates must succeed before scheduling a window.",
                },
                StepDescriptor {
                    title: "Schedule deployment",
                    detail: "Capacity slider is locked at 100% once this stage completes.",
                },
                StepDescriptor {
                    title: "Launch + telemetry",
                    detail: "Open the blast radius gradually and stream metrics into the SRE bridge.",
                },
            ],
            metrics: vec![
                CardMetric {
                    label: "Automation coverage",
                    value: "98.4%",
                    detail: "Unit + integration suites automatically enforce Joy parity across adapters.",
                },
                CardMetric {
                    label: "Pending approvals",
                    value: "2",
                    detail: "Design + security teams must approve before production rollout.",
                },
                CardMetric {
                    label: "Last parity audit",
                    value: "4h ago",
                    detail: "CI inventory report refreshed via `cargo xtask joy-parity`.",
                },
            ],
            snackbar: SnackbarDescriptor {
                success_label: "Workflow updated",
                analytics_id: "joy-workflow-snackbar",
            },
            automation: WorkflowAutomationIds {
                card_id: "joy-workflow-card",
                environment_chip_id: "joy-workflow-environment",
                capacity_slider_id: "joy-workflow-capacity",
                snackbar_id: "joy-workflow-snackbar",
            },
        }
    }
}

/// Lightweight description of a metric rendered inside the Joy card.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardMetric {
    /// Metric label (for example "Automation coverage").
    pub label: &'static str,
    /// Primary value rendered with emphasis.
    pub value: &'static str,
    /// Supporting detail exposed as helper text.
    pub detail: &'static str,
}

/// Descriptor for action buttons bound to the workflow state.
#[derive(Clone, PartialEq)]
pub struct ActionDescriptor {
    /// Visible button label.
    pub label: &'static str,
    /// Supporting helper text shown below or beside the button.
    pub description: &'static str,
    /// Joy color token applied to the surface.
    pub color: Color,
    /// Joy variant controlling background/border styling.
    pub variant: Variant,
    /// Analytics hook assigned to the rendered element.
    pub analytics_id: &'static str,
    /// Optional throttle window to guard against double clicks.
    pub throttle_ms: Option<u64>,
}

/// Descriptor for the environment chip rendered within the card header.
#[derive(Clone, PartialEq)]
pub struct ChipDescriptor {
    /// Chip label.
    pub label: &'static str,
    /// Supporting helper text describing the environment state.
    pub description: &'static str,
    /// Joy color token applied to the chip.
    pub color: Color,
    /// Joy variant used for the chip surface.
    pub variant: Variant,
}

/// Descriptor for the capacity slider.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderDescriptor {
    /// Minimum logical value (percentage) allowed by the slider.
    pub min: f64,
    /// Maximum logical value (percentage) allowed by the slider.
    pub max: f64,
    /// Step granularity when nudging via keyboard controls.
    pub step: f64,
    /// Default value when the workflow boots.
    pub default: f64,
    /// Annotated marks rendered alongside the track.
    pub marks: Vec<SliderMark>,
}

impl SliderDescriptor {
    /// Clamp a raw value into the configured slider range.
    pub fn clamp(&self, value: f64) -> f64 {
        value.clamp(self.min, self.max)
    }

    /// Convert a logical value into a percentage (0-100) for progress bars.
    pub fn percentage(&self, value: f64) -> f64 {
        let range = (self.max - self.min).abs();
        if range <= f64::EPSILON {
            return 0.0;
        }
        ((value - self.min) / range).clamp(0.0, 1.0) * 100.0
    }
}

/// Annotated slider mark containing a helper label.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderMark {
    /// Logical value represented by the mark.
    pub value: f64,
    /// Human friendly label displayed alongside the tick.
    pub label: &'static str,
}

/// Descriptor for each release checklist entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepDescriptor {
    /// Title rendered inside the Joy stepper.
    pub title: &'static str,
    /// Supporting copy explaining the step purpose.
    pub detail: &'static str,
}

/// Snackbar configuration shared by all adapters.
#[derive(Clone, PartialEq, Eq)]
pub struct SnackbarDescriptor {
    /// Short label communicated alongside snackbar payloads.
    pub success_label: &'static str,
    /// Analytics identifier applied to snackbar surfaces.
    pub analytics_id: &'static str,
}

/// Severity classification for snackbar messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnackbarSeverity {
    /// Informational update (for example slider changes).
    Info,
    /// Successful step completion.
    Success,
    /// Warning when rolling back.
    Warning,
}

/// Payload delivered to renderers whenever a snackbar is shown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnackbarPayload {
    /// Severity classification (maps to color/variant decisions in renderers).
    pub severity: SnackbarSeverity,
    /// Message presented to the user.
    pub message: String,
}

/// Snapshot of the workflow emitted after every state transition.  The snapshot
/// is intentionally serialisable/clonable so adapters can store it directly in
/// framework signals or state hooks.
#[derive(Clone, Debug, PartialEq)]
pub struct JoyWorkflowSnapshot {
    /// Logical value of the capacity slider.
    pub capacity_value: f64,
    /// Capacity value expressed as a percentage of the configured range.
    pub capacity_percent: f64,
    /// Index of the currently active step (if any).
    pub active_step: Option<usize>,
    /// Human friendly name of the active step (when present).
    pub active_step_label: Option<&'static str>,
    /// Presentation status for each configured step.
    pub step_status: Vec<StepStatus>,
    /// Snackbar payload visible after the latest transition (if any).
    pub snackbar: Option<SnackbarPayload>,
    /// Lifecycle log retained for QA dashboards.
    pub lifecycle_log: Vec<String>,
    /// Whether every step has been completed.
    pub completed: bool,
}

/// Events recognised by the workflow machine.
#[derive(Clone, Debug, PartialEq)]
pub enum JoyWorkflowEvent {
    /// Advance to the next step (if available).
    Advance,
    /// Roll back to the previous step (if possible).
    Rollback,
    /// Update the deployment capacity slider.
    SetCapacity(f64),
    /// Clear the currently visible snackbar message.
    DismissSnackbar,
}

/// Deterministic workflow state machine consumed by every demo.
#[derive(Clone)]
pub struct JoyWorkflowMachine {
    blueprint: JoyWorkflowBlueprint,
    capacity_value: f64,
    completed_steps: usize,
    snackbar: Option<SnackbarPayload>,
    lifecycle_log: Vec<String>,
}

impl JoyWorkflowMachine {
    /// Construct the workflow using the enterprise blueprint.
    pub fn new() -> Self {
        let blueprint = JoyWorkflowBlueprint::enterprise_release();
        let mut machine = Self {
            capacity_value: blueprint.capacity.default,
            completed_steps: 0,
            snackbar: None,
            lifecycle_log: Vec::new(),
            blueprint,
        };
        machine.push_log("Workflow initialised using enterprise defaults.");
        machine
    }

    /// Access the shared blueprint. Renderers typically clone individual
    /// descriptors from this structure so templates remain declarative.
    pub fn blueprint(&self) -> &JoyWorkflowBlueprint {
        &self.blueprint
    }

    /// Dispatch a workflow event and return the resulting snapshot.
    pub fn apply(&mut self, event: JoyWorkflowEvent) -> JoyWorkflowSnapshot {
        match event {
            JoyWorkflowEvent::Advance => self.advance_step(),
            JoyWorkflowEvent::Rollback => self.rollback_step(),
            JoyWorkflowEvent::SetCapacity(value) => self.update_capacity(value),
            JoyWorkflowEvent::DismissSnackbar => {
                self.snackbar = None;
                self.snapshot()
            }
        }
    }

    /// Generate a read-only snapshot of the current state.
    pub fn snapshot(&self) -> JoyWorkflowSnapshot {
        let step_status = self
            .blueprint
            .steps
            .iter()
            .enumerate()
            .map(|(index, _)| {
                if index < self.completed_steps {
                    StepStatus::Completed
                } else if index == self.completed_steps {
                    StepStatus::Active
                } else {
                    StepStatus::Pending
                }
            })
            .collect::<Vec<_>>();

        let active_step = if self.completed_steps < self.blueprint.steps.len() {
            Some(self.completed_steps)
        } else {
            None
        };

        let active_step_label = active_step.map(|index| self.blueprint.steps[index].title);
        let capacity_percent = self.blueprint.capacity.percentage(self.capacity_value);

        JoyWorkflowSnapshot {
            capacity_value: self.capacity_value,
            capacity_percent,
            active_step,
            active_step_label,
            step_status,
            snackbar: self.snackbar.clone(),
            lifecycle_log: self.lifecycle_log.clone(),
            completed: self.completed_steps >= self.blueprint.steps.len(),
        }
    }

    /// Convenience accessor mirroring the internal capacity profile helper.
    pub fn capacity_profile(&self) -> &'static str {
        self.resolve_capacity_profile()
    }

    /// Append an entry to the lifecycle log while keeping the ring buffer small.
    fn push_log(&mut self, message: impl Into<String>) {
        self.lifecycle_log.push(message.into());
        if self.lifecycle_log.len() > MAX_LOG_ENTRIES {
            let excess = self.lifecycle_log.len() - MAX_LOG_ENTRIES;
            self.lifecycle_log.drain(0..excess);
        }
    }

    /// Update the snackbar payload with the provided severity + message.
    fn set_snackbar(&mut self, severity: SnackbarSeverity, message: String) {
        self.snackbar = Some(SnackbarPayload { severity, message });
    }

    fn advance_step(&mut self) -> JoyWorkflowSnapshot {
        if self.completed_steps < self.blueprint.steps.len() {
            let label = self.blueprint.steps[self.completed_steps].title;
            self.completed_steps += 1;
            self.push_log(format!("Completed step: {label}"));
            if self.completed_steps < self.blueprint.steps.len() {
                let next = self.blueprint.steps[self.completed_steps].title;
                self.set_snackbar(SnackbarSeverity::Success, format!("Advanced to '{next}'."));
            } else {
                self.set_snackbar(
                    SnackbarSeverity::Success,
                    "Release checklist complete. Ready for production push.".into(),
                );
            }
        } else {
            self.push_log("Advance requested but workflow already completed.");
        }
        self.snapshot()
    }

    fn rollback_step(&mut self) -> JoyWorkflowSnapshot {
        if self.completed_steps > 0 {
            self.completed_steps -= 1;
            let label = self.blueprint.steps[self.completed_steps].title;
            self.push_log(format!("Rolled back to step: {label}"));
            self.set_snackbar(
                SnackbarSeverity::Warning,
                format!("Returned to '{label}' for remediation."),
            );
        } else {
            self.push_log("Rollback ignored – already at the first step.");
            self.set_snackbar(
                SnackbarSeverity::Info,
                "Already reviewing the first checklist item.".into(),
            );
        }
        self.snapshot()
    }

    fn update_capacity(&mut self, raw: f64) -> JoyWorkflowSnapshot {
        let clamped = self.blueprint.capacity.clamp(raw);
        self.capacity_value =
            (clamped / self.blueprint.capacity.step).round() * self.blueprint.capacity.step;
        self.push_log(format!(
            "Capacity adjusted to {:.1}% of baseline.",
            self.capacity_value
        ));
        self.set_snackbar(
            SnackbarSeverity::Info,
            format!(
                "Capacity now {:.0}% ({} mode).",
                self.capacity_value,
                self.resolve_capacity_profile()
            ),
        );
        self.snapshot()
    }

    fn resolve_capacity_profile(&self) -> &'static str {
        let value = self.capacity_value;
        if value < 80.0 {
            "canary"
        } else if value < 110.0 {
            "baseline"
        } else {
            "burst"
        }
    }
}
