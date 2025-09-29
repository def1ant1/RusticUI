//! Headless typography contracts powering Material renderers.
//!
//! The state exposes deterministic mappings between design-system variants and
//! semantic HTML tags while centralising ARIA authoring for enterprise
//! automation.

/// Material typography variants understood by the headless layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Subtitle1,
    Subtitle2,
    Body1,
    Body2,
    Button,
    Caption,
    Overline,
}

impl TypographyVariant {
    /// Returns a stable string identifier for analytics/automation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Subtitle1 => "subtitle1",
            Self::Subtitle2 => "subtitle2",
            Self::Body1 => "body1",
            Self::Body2 => "body2",
            Self::Button => "button",
            Self::Caption => "caption",
            Self::Overline => "overline",
        }
    }

    /// Returns the default HTML tag associated with the variant.
    pub fn default_tag(&self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Subtitle1 | Self::Subtitle2 | Self::Body1 | Self::Body2 => "p",
            Self::Button | Self::Overline | Self::Caption => "span",
        }
    }
}

/// Headless typography state capturing the variant and custom semantic tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypographyState {
    variant: TypographyVariant,
    component: Option<String>,
    id: Option<String>,
}

impl TypographyState {
    /// Creates a new state for the provided variant.
    pub fn new(variant: TypographyVariant) -> Self {
        Self {
            variant,
            component: None,
            id: None,
        }
    }

    /// Overrides the semantic tag rendered by adapters.
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Assigns a DOM id for linking headings.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Returns the typography variant.
    pub fn variant(&self) -> TypographyVariant {
        self.variant
    }

    /// Returns the semantic HTML tag adapters should use.
    pub fn tag(&self) -> &str {
        self.component
            .as_deref()
            .unwrap_or_else(|| self.variant.default_tag())
    }

    /// Returns ARIA attributes for the typography element.
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(1);
        if let Some(id) = &self.id {
            attrs.push(("id", id.clone()));
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tag_matches_variant() {
        let state = TypographyState::new(TypographyVariant::H3);
        assert_eq!(state.tag(), "h3");
    }

    #[test]
    fn custom_component_overrides_default_tag() {
        let state = TypographyState::new(TypographyVariant::Body1).with_component("div");
        assert_eq!(state.tag(), "div");
    }

    #[test]
    fn id_is_reflected_in_accessibility_attributes() {
        let state = TypographyState::new(TypographyVariant::H4).with_id("title");
        let attrs = state.accessibility_attributes();
        assert_eq!(attrs[0], ("id", "title".to_string()));
    }
}
