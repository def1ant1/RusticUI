#![deny(missing_docs)]
//! Headless pagination controller shared across framework adapters.
//!
//! The state machine centralises keyboard handling, ARIA wiring, and analytics
//! payload generation so renderers can remain deterministic.  Consumers describe
//! the number of pages, optionally expose first/last controls, and wire the
//! resulting attribute builders into their templates.

use crate::{
    aria,
    interaction::ControlKey,
    selection::{clamp_index, ControlStrategy},
};

/// Analytics payload emitted when the current page changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationAnalyticsEvent {
    /// Logical analytics channel surfaced on the rendered container.
    pub channel: String,
    /// Selected zero-based page index.
    pub page_index: usize,
    /// Optional page specific analytics tag.
    pub page_tag: Option<String>,
}

/// Outcome produced when handling keyboard input.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PaginationKeyboardOutcome {
    /// Item that should receive focus after the key press.
    pub focused: Option<PaginationItemKind>,
    /// Page that should be selected.
    pub selected_page: Option<usize>,
    /// Analytics payload describing the selection.
    pub analytics: Option<PaginationAnalyticsEvent>,
}

/// Enumerates the interactive elements rendered by the pagination component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaginationItemKind {
    /// Represents a numbered page button.
    Page(usize),
    /// Represents the "previous" control.
    Previous,
    /// Represents the "next" control.
    Next,
    /// Represents the "first" control.
    First,
    /// Represents the "last" control.
    Last,
}

impl PaginationItemKind {
    /// Returns a machine readable identifier used by automation hooks.
    pub fn as_data_value(self) -> &'static str {
        match self {
            Self::Page(_) => "page",
            Self::Previous => "previous",
            Self::Next => "next",
            Self::First => "first",
            Self::Last => "last",
        }
    }
}

/// Builder describing the `<nav>` wrapper attributes.
#[derive(Debug, Clone)]
pub struct PaginationRootAttributes<'a> {
    state: &'a PaginationState,
    id: Option<&'a str>,
    aria_label: Option<&'a str>,
    labelled_by: Option<&'a str>,
}

impl<'a> PaginationRootAttributes<'a> {
    /// Construct a new builder bound to the provided state.
    pub fn new(state: &'a PaginationState) -> Self {
        Self {
            state,
            id: None,
            aria_label: Some("Pagination"),
            labelled_by: None,
        }
    }

    /// Assign a DOM id to the navigation region.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Override the default `aria-label`.
    pub fn aria_label(mut self, value: &'a str) -> Self {
        self.aria_label = Some(value);
        self
    }

    /// Link the navigation region to an external heading or description.
    ///
    /// Providing an `aria-labelledby` target clears the default `aria-label`
    /// so the computed accessible name is driven exclusively by the supplied
    /// heading element.
    pub fn labelled_by(mut self, value: &'a str) -> Self {
        self.aria_label = None;
        self.labelled_by = Some(value);
        self
    }

    /// Returns the landmark role for the wrapper.
    pub fn role(&self) -> &'static str {
        aria::role_navigation()
    }

    /// Optional `id` attribute tuple.
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Optional `aria-label` tuple.
    pub fn aria_label_attr(&self) -> Option<(&'static str, &str)> {
        self.aria_label.map(|value| ("aria-label", value))
    }

    /// Optional `aria-labelledby` tuple when external labelling is configured.
    pub fn labelledby(&self) -> Option<(&'static str, &str)> {
        self.labelled_by.map(aria::aria_labelledby)
    }

    /// Optional analytics attribute.
    pub fn analytics_attribute(&self) -> Option<(&'static str, &str)> {
        self.state
            .analytics_channel
            .as_deref()
            .map(|value| ("data-rustic-analytics-channel", value))
    }
}

/// Builder describing the list wrapper attributes.
#[derive(Debug, Clone)]
pub struct PaginationListAttributes<'a> {
    state: &'a PaginationState,
    id: Option<&'a str>,
}

impl<'a> PaginationListAttributes<'a> {
    /// Construct a new builder for the list container.
    pub fn new(state: &'a PaginationState) -> Self {
        Self { state, id: None }
    }

    /// Assign a DOM id to the list element.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Returns the ARIA role for the list container.
    pub fn role(&self) -> &'static str {
        aria::role_list()
    }

    /// Optional `id` tuple.
    pub fn id_attr(&self) -> Option<(&'static str, &str)> {
        self.id.map(|value| ("id", value))
    }

    /// Optional analytics attribute mirrored from the root.
    pub fn analytics_attribute(&self) -> Option<(&'static str, &str)> {
        self.state
            .analytics_channel
            .as_deref()
            .map(|value| ("data-rustic-analytics-channel", value))
    }
}

/// Builder describing an interactive pagination control.
#[derive(Debug, Clone)]
pub struct PaginationItemAttributes<'a> {
    state: &'a PaginationState,
    kind: PaginationItemKind,
    id: Option<&'a str>,
    aria_label: Option<&'a str>,
}

impl<'a> PaginationItemAttributes<'a> {
    fn new(state: &'a PaginationState, kind: PaginationItemKind) -> Self {
        Self {
            state,
            kind,
            id: None,
            aria_label: None,
        }
    }

    /// Assign a DOM id to the interactive element.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Override the accessible label for non-page controls.
    pub fn aria_label(mut self, value: &'a str) -> Self {
        self.aria_label = Some(value);
        self
    }

    /// Collect ARIA/data attributes describing the element.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(10);
        pairs.push(("role", aria::role_button().to_string()));
        let focused = self.state.focused == Some(self.kind);
        pairs.push(("tabindex", if focused { "0" } else { "-1" }.to_string()));
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        let disabled = self.state.is_disabled(self.kind);
        if disabled {
            pairs.push(("aria-disabled", "true".into()));
            pairs.push(("data-disabled", "true".into()));
        }
        let label = self
            .aria_label
            .map(|value| value.to_string())
            .unwrap_or_else(|| match self.kind {
                PaginationItemKind::Previous => "Go to previous page".to_string(),
                PaginationItemKind::Next => "Go to next page".to_string(),
                PaginationItemKind::First => "Go to first page".to_string(),
                PaginationItemKind::Last => "Go to last page".to_string(),
                PaginationItemKind::Page(index) => {
                    if self.state.page_count == 0 {
                        "Page".to_string()
                    } else {
                        format!("Go to page {}", index + 1)
                    }
                }
            });
        pairs.push(("aria-label", label));
        pairs.push((
            "data-rustic-pagination-kind",
            self.kind.as_data_value().to_string(),
        ));
        if let PaginationItemKind::Page(index) = self.kind {
            pairs.push(("data-rustic-pagination-page", index.to_string()));
            if self.state.current == Some(index) {
                let (key, value) = aria::aria_current("page");
                pairs.push((key, value.to_string()));
            }
        }
        pairs
    }
}

/// Headless state machine managing pagination interactions.
#[derive(Debug, Clone)]
pub struct PaginationState {
    page_count: usize,
    current: Option<usize>,
    focused: Option<PaginationItemKind>,
    selection_mode: ControlStrategy,
    focus_mode: ControlStrategy,
    include_edge_controls: bool,
    analytics_channel: Option<String>,
    page_tags: Vec<Option<String>>,
}

impl PaginationState {
    /// Construct a new pagination controller.
    pub fn new(
        page_count: usize,
        current: Option<usize>,
        selection_mode: ControlStrategy,
        focus_mode: ControlStrategy,
    ) -> Self {
        let current = clamp_index(current, page_count);
        // Resolve a deterministic fallback so pagination always exposes a focus
        // target when pages exist. Using `then_some` keeps the logic allocation-free
        // while satisfying Clippy's strict `or_else` lint when both adapters build.
        let fallback_focus = (page_count > 0).then_some(PaginationItemKind::Page(0));
        let focused = current.map(PaginationItemKind::Page).or(fallback_focus);
        Self {
            page_count,
            current,
            focused,
            selection_mode,
            focus_mode,
            include_edge_controls: true,
            analytics_channel: None,
            page_tags: vec![None; page_count],
        }
    }

    /// Returns the root attribute builder.
    pub fn root_attributes(&self) -> PaginationRootAttributes<'_> {
        PaginationRootAttributes::new(self)
    }

    /// Returns the list attribute builder.
    pub fn list_attributes(&self) -> PaginationListAttributes<'_> {
        PaginationListAttributes::new(self)
    }

    /// Returns the item attribute builder for a specific kind.
    pub fn item_attributes(&self, kind: PaginationItemKind) -> PaginationItemAttributes<'_> {
        PaginationItemAttributes::new(self, kind)
    }

    /// Toggle whether "first"/"last" controls are rendered.
    pub fn set_include_edge_controls(&mut self, include: bool) {
        self.include_edge_controls = include;
    }

    /// Configure the analytics channel for telemetry pipelines.
    pub fn set_analytics_channel(&mut self, channel: Option<impl Into<String>>) {
        self.analytics_channel = channel.map(Into::into);
    }

    /// Configure the analytics tag for a specific page button.
    pub fn set_page_analytics_tag(&mut self, index: usize, tag: Option<impl Into<String>>) {
        if index >= self.page_count {
            return;
        }
        if let Some(slot) = self.page_tags.get_mut(index) {
            *slot = tag.map(Into::into);
        }
    }

    /// Update the number of available pages.
    pub fn set_page_count(&mut self, count: usize) {
        self.page_count = count;
        self.page_tags.resize(count, None);
        self.current = clamp_index(self.current, count);
        if let Some(PaginationItemKind::Page(index)) = self.focused {
            self.focused = clamp_index(Some(index), count).map(PaginationItemKind::Page);
        }
    }

    /// Synchronise the current page when controlled externally.
    pub fn sync_current(&mut self, index: Option<usize>) {
        if self.selection_mode.is_controlled() {
            self.current = clamp_index(index, self.page_count);
        }
    }

    /// Synchronise the focused item when roving tabindex is controlled.
    pub fn sync_focused(&mut self, kind: Option<PaginationItemKind>) {
        if self.focus_mode.is_controlled() {
            self.focused = kind;
        }
    }

    /// Returns whether the provided control is disabled.
    pub fn is_disabled(&self, kind: PaginationItemKind) -> bool {
        match kind {
            PaginationItemKind::Page(index) => index >= self.page_count,
            PaginationItemKind::Previous | PaginationItemKind::First => {
                self.page_count == 0 || self.current.unwrap_or(0) == 0
            }
            PaginationItemKind::Next | PaginationItemKind::Last => {
                if self.page_count == 0 {
                    true
                } else {
                    self.current.unwrap_or(0) >= self.page_count.saturating_sub(1)
                }
            }
        }
    }

    /// Process a keyboard event returning the resulting intent.
    pub fn on_key<F: FnMut(PaginationSelection)>(
        &mut self,
        key: ControlKey,
        mut on_select: F,
    ) -> PaginationKeyboardOutcome {
        let mut outcome = PaginationKeyboardOutcome::default();
        match key {
            ControlKey::Enter | ControlKey::Space => {
                if let Some(kind) = self.focused {
                    let result = self.activate(kind, &mut on_select);
                    outcome.selected_page = result.selected_page;
                    outcome.analytics = result.analytics;
                }
            }
            ControlKey::Home => {
                outcome.focused = self.apply_focus(self.first_focusable());
            }
            ControlKey::End => {
                outcome.focused = self.apply_focus(self.last_focusable());
            }
            _ if key.is_forward() => {
                outcome.focused = self.apply_focus(self.advance_focus(1));
            }
            _ if key.is_backward() => {
                outcome.focused = self.apply_focus(self.advance_focus(-1));
            }
            _ => {}
        }
        outcome
    }

    /// Imperatively activate a pagination control.
    pub fn activate<F: FnMut(PaginationSelection)>(
        &mut self,
        kind: PaginationItemKind,
        mut on_select: F,
    ) -> PaginationSelectionOutcome {
        self.activate_internal(kind, &mut on_select)
    }

    fn activate_internal<F: FnMut(PaginationSelection)>(
        &mut self,
        kind: PaginationItemKind,
        on_select: &mut F,
    ) -> PaginationSelectionOutcome {
        if self.is_disabled(kind) {
            return PaginationSelectionOutcome::default();
        }
        match kind {
            PaginationItemKind::Page(index) => self.apply_selection(index, on_select),
            PaginationItemKind::Previous => {
                let current = self.current.unwrap_or(0);
                if current > 0 {
                    self.apply_selection(current - 1, on_select)
                } else {
                    PaginationSelectionOutcome::default()
                }
            }
            PaginationItemKind::Next => {
                if let Some(current) = self.current {
                    if current + 1 < self.page_count {
                        self.apply_selection(current + 1, on_select)
                    } else {
                        PaginationSelectionOutcome::default()
                    }
                } else if self.page_count > 0 {
                    self.apply_selection(0, on_select)
                } else {
                    PaginationSelectionOutcome::default()
                }
            }
            PaginationItemKind::First => {
                if self.page_count > 0 {
                    self.apply_selection(0, on_select)
                } else {
                    PaginationSelectionOutcome::default()
                }
            }
            PaginationItemKind::Last => {
                if self.page_count > 0 {
                    self.apply_selection(self.page_count - 1, on_select)
                } else {
                    PaginationSelectionOutcome::default()
                }
            }
        }
    }

    fn apply_selection<F: FnMut(PaginationSelection)>(
        &mut self,
        index: usize,
        on_select: &mut F,
    ) -> PaginationSelectionOutcome {
        if index >= self.page_count {
            return PaginationSelectionOutcome::default();
        }
        if !self.selection_mode.is_controlled() {
            self.current = Some(index);
        }
        let analytics = self.analytics_payload(index);
        let selection = PaginationSelection {
            page_index: index,
            analytics: analytics.clone(),
        };
        on_select(selection);
        PaginationSelectionOutcome {
            selected_page: Some(index),
            analytics,
        }
    }

    fn analytics_payload(&self, index: usize) -> Option<PaginationAnalyticsEvent> {
        let channel = self.analytics_channel.as_ref()?;
        let tag = self
            .page_tags
            .get(index)
            .and_then(|value| value.as_ref().map(|value| value.to_string()));
        Some(PaginationAnalyticsEvent {
            channel: channel.clone(),
            page_index: index,
            page_tag: tag,
        })
    }

    fn apply_focus(&mut self, next: Option<PaginationItemKind>) -> Option<PaginationItemKind> {
        if !self.focus_mode.is_controlled() {
            self.focused = next;
        }
        next
    }

    fn first_focusable(&self) -> Option<PaginationItemKind> {
        self.focus_sequence()
            .into_iter()
            .find(|kind| !self.is_disabled(*kind))
    }

    fn last_focusable(&self) -> Option<PaginationItemKind> {
        self.focus_sequence()
            .into_iter()
            .rev()
            .find(|kind| !self.is_disabled(*kind))
    }

    fn advance_focus(&self, delta: isize) -> Option<PaginationItemKind> {
        let sequence = self.focus_sequence();
        if sequence.is_empty() {
            return None;
        }
        let current_index = self
            .focused
            .and_then(|focused| sequence.iter().position(|kind| *kind == focused))
            .unwrap_or(0);
        let mut next_index = current_index as isize + delta;
        let len = sequence.len() as isize;
        if next_index < 0 {
            next_index = len - 1;
        }
        next_index %= len;
        let mut attempts = 0;
        let mut candidate = next_index;
        while attempts < sequence.len() {
            let kind = sequence[candidate as usize];
            if !self.is_disabled(kind) {
                return Some(kind);
            }
            candidate = (candidate + delta).rem_euclid(len);
            attempts += 1;
        }
        None
    }

    fn focus_sequence(&self) -> Vec<PaginationItemKind> {
        let mut sequence = Vec::with_capacity(self.page_count + 4);
        if self.include_edge_controls {
            sequence.push(PaginationItemKind::First);
        }
        sequence.push(PaginationItemKind::Previous);
        for index in 0..self.page_count {
            sequence.push(PaginationItemKind::Page(index));
        }
        sequence.push(PaginationItemKind::Next);
        if self.include_edge_controls {
            sequence.push(PaginationItemKind::Last);
        }
        sequence
    }
}

/// Selection payload emitted when the current page changes.
#[derive(Debug, Clone)]
pub struct PaginationSelection {
    /// Selected zero-based page index.
    pub page_index: usize,
    /// Optional analytics payload describing the transition.
    pub analytics: Option<PaginationAnalyticsEvent>,
}

/// Result returned by [`PaginationState::activate`] and [`PaginationState::on_key`].
#[derive(Debug, Clone, Default)]
pub struct PaginationSelectionOutcome {
    /// The selected page index if a change occurred.
    pub selected_page: Option<usize>,
    /// Analytics payload describing the selection.
    pub analytics: Option<PaginationAnalyticsEvent>,
}
