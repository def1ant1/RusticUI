//! Headless app bar state shared across renderer adapters.
//!
//! The state object centralises accessibility affordances, telemetry hooks and
//! automation identifiers for the high-level navigation banner. Renderer
//! adapters in `rustic-ui-material` and future design systems consume the state
//! to produce framework specific markup without duplicating ARIA attributes or
//! analytics plumbing.  The builder-style API keeps call sites ergonomic while
//! allowing enterprise adopters to opt into deterministic identifiers for their
//! QA suites.
//!
//! ## Accessibility
//!
//! * The generated HTML attributes always include `role="banner"` so assistive
//!   technologies identify the region as the primary application header.
//! * `aria-label` defaults to the title but can be overridden explicitly. This
//!   mirrors the Material guidance which prefers a descriptive label even when
//!   visible text is present.
//! * SVG focused helpers mirror the HTML attributes by producing an
//!   `aria-labelledby` relationship suitable for inline logos or navigation
//!   icons.  Keeping the logic inside the state guarantees that hydration and
//!   SSR output stay aligned across frameworks.
//!
//! ## Telemetry and automation
//!
//! The state stores optional analytics identifiers and exposes attribute
//! builders that render them as `data-*` markers. These hooks allow centralised
//! telemetry platforms to observe impressions (`data-analytics-view-id`) and
//! interactions (`data-analytics-interaction-id`) without manual plumbing in
//! each framework.  The automation id can be wired into existing selectors via
//! `data-automation-id`.
//!
//! ## Concurrency contract
//!
//! `AppBarState` is `Clone + Send + Sync` so renderers can memoize or share the
//! state across tasks (for example, server-side streaming or concurrent Yew
//! rendering) without additional synchronisation primitives. All builder methods
//! consume `self` and return the updated state to avoid interior mutability while
//! still enabling fluent configuration.

/// Colour palette applied to the navigation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppBarColor {
    /// Use the design system's primary tone.
    Primary,
    /// Use the secondary tone, typically reserved for alternate flows.
    Secondary,
}

impl AppBarColor {
    /// Returns the canonical automation-friendly label for the colour.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// Size variants that influence the overall bar height and density.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppBarSize {
    /// Compact density for dashboard chrome or embedded scenarios.
    Small,
    /// Default density recommended by the Material specification.
    Medium,
    /// Tall density for marketing heavy surfaces.
    Large,
}

impl AppBarSize {
    /// Returns the canonical automation label for the size variant.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Optional analytics identifiers applied to rendered attributes.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AppBarAnalytics {
    view_id: Option<String>,
    interaction_id: Option<String>,
}

impl AppBarAnalytics {
    /// Attach an identifier representing the banner impression event.
    #[must_use]
    pub fn with_view_id(mut self, id: impl Into<String>) -> Self {
        self.view_id = Some(id.into());
        self
    }

    /// Attach an identifier representing click/tap interactions.
    #[must_use]
    pub fn with_interaction_id(mut self, id: impl Into<String>) -> Self {
        self.interaction_id = Some(id.into());
        self
    }

    fn has_values(&self) -> bool {
        self.view_id.is_some() || self.interaction_id.is_some()
    }

    fn iter(&self) -> impl Iterator<Item = (&'static str, &String)> {
        self.view_id
            .iter()
            .map(|id| ("data-analytics-view-id", id))
            .chain(
                self.interaction_id
                    .iter()
                    .map(|id| ("data-analytics-interaction-id", id)),
            )
    }

    /// Returns the configured view identifier (if any).
    #[must_use]
    pub fn view_id(&self) -> Option<&str> {
        self.view_id.as_deref()
    }

    /// Returns the configured interaction identifier (if any).
    #[must_use]
    pub fn interaction_id(&self) -> Option<&str> {
        self.interaction_id.as_deref()
    }
}

/// Headless representation of the application bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppBarState {
    title: String,
    aria_label: String,
    color: AppBarColor,
    size: AppBarSize,
    automation_id: Option<String>,
    analytics: AppBarAnalytics,
    svg_title_id: Option<String>,
}

impl AppBarState {
    /// Construct a new app bar state with the provided title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            aria_label: title.clone(),
            title,
            color: AppBarColor::Primary,
            size: AppBarSize::Medium,
            automation_id: None,
            analytics: AppBarAnalytics::default(),
            svg_title_id: None,
        }
    }

    /// Override the accessible label announced by assistive technology.
    #[must_use]
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    /// Change the colour palette variant applied by renderers.
    #[must_use]
    pub fn with_color(mut self, color: AppBarColor) -> Self {
        self.color = color;
        self
    }

    /// Change the size variant consumed by renderers.
    #[must_use]
    pub fn with_size(mut self, size: AppBarSize) -> Self {
        self.size = size;
        self
    }

    /// Attach an automation identifier used by QA tooling.
    #[must_use]
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Merge analytics identifiers into the state.
    #[must_use]
    pub fn with_analytics(mut self, analytics: AppBarAnalytics) -> Self {
        self.analytics = analytics;
        self
    }

    /// Provide an explicit SVG `<title>` id.
    ///
    /// Inline SVG logos frequently require deterministic ids so `<svg>` elements
    /// can reference the title via `aria-labelledby`. The state stores the id and
    /// reuses it when [`svg_attributes`] is invoked.
    #[must_use]
    pub fn with_svg_title_id(mut self, id: impl Into<String>) -> Self {
        self.svg_title_id = Some(id.into());
        self
    }

    /// Returns the configured title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the accessible label applied to the header region.
    #[must_use]
    pub fn aria_label(&self) -> &str {
        &self.aria_label
    }

    /// Returns the palette variant.
    #[must_use]
    pub fn color(&self) -> AppBarColor {
        self.color
    }

    /// Returns the size variant.
    #[must_use]
    pub fn size(&self) -> AppBarSize {
        self.size
    }

    /// Returns the optional automation identifier.
    #[must_use]
    pub fn automation_id(&self) -> Option<&str> {
        self.automation_id.as_deref()
    }

    /// Returns analytics identifiers (if any).
    #[must_use]
    pub fn analytics(&self) -> &AppBarAnalytics {
        &self.analytics
    }

    /// Returns the SVG title id (if configured).
    #[must_use]
    pub fn svg_title_id(&self) -> Option<&str> {
        self.svg_title_id.as_deref()
    }

    /// Build attribute pairs for HTML renderers.
    ///
    /// The output always includes the semantic banner role and the configured
    /// accessible label. Automation and analytics identifiers are appended when
    /// present.
    #[must_use]
    pub fn html_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(4);
        attrs.push(("role", "banner".to_string()));
        attrs.push(("aria-label", self.aria_label.clone()));

        if let Some(id) = &self.automation_id {
            attrs.push(("data-automation-id", id.clone()));
        }

        if self.analytics.has_values() {
            for (key, value) in self.analytics.iter() {
                attrs.push((key, value.clone()));
            }
        }

        attrs
    }

    /// Build attribute pairs for inline SVG logos associated with the app bar.
    ///
    /// When a dedicated `<svg>` is used for branding it should mirror the
    /// accessible labelling of the surrounding header. The helper returns a
    /// tuple of attributes referencing the configured title or aria label so the
    /// logo participates in the same automation and accessibility flows.
    #[must_use]
    pub fn svg_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(3);
        attrs.push(("role", "img".to_string()));

        if let Some(id) = &self.svg_title_id {
            attrs.push(("aria-labelledby", id.clone()));
        } else {
            attrs.push(("aria-label", self.aria_label.clone()));
        }

        if let Some(id) = &self.automation_id {
            attrs.push(("data-automation-id", id.clone()));
        }

        attrs
    }

    /// Returns attributes for the SVG `<title>` node referenced by
    /// [`svg_attributes`].
    #[must_use]
    pub fn svg_title_attributes(&self) -> Option<Vec<(&'static str, String)>> {
        self.svg_title_id
            .as_ref()
            .map(|id| vec![("id", id.clone())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_attributes_include_analytics() {
        let analytics = AppBarAnalytics::default()
            .with_view_id("view.nav")
            .with_interaction_id("click.nav");
        let state = AppBarState::new("Dashboard")
            .with_aria_label("Main navigation")
            .with_automation_id("app-bar.global")
            .with_analytics(analytics);
        let attrs = state.html_attributes();
        assert!(attrs.iter().any(|(k, v)| k == &"role" && v == "banner"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-analytics-view-id" && v == "view.nav"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-analytics-interaction-id" && v == "click.nav"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-automation-id" && v == "app-bar.global"));
    }

    #[test]
    fn svg_attributes_fall_back_to_label() {
        let state = AppBarState::new("Dashboard").with_aria_label("Global navigation");
        let attrs = state.svg_attributes();
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"aria-label" && v == "Global navigation"));
    }

    #[test]
    fn svg_title_attributes_are_optional() {
        let state = AppBarState::new("Dashboard")
            .with_svg_title_id("app-bar-logo-title")
            .with_automation_id("nav");
        let svg_attrs = state.svg_attributes();
        assert!(svg_attrs
            .iter()
            .any(|(k, v)| k == &"aria-labelledby" && v == "app-bar-logo-title"));
        let title_attrs = state.svg_title_attributes().expect("title attrs");
        assert!(title_attrs
            .iter()
            .any(|(k, v)| k == &"id" && v == "app-bar-logo-title"));
    }
}
