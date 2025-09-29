//! Headless representation of the Material `Paper` surface.
//!
//! The state object tracks semantic configuration such as the chosen surface
//! variant (elevated, outlined or plain), corner rounding and whether the
//! surface participates in a grouped disclosure widget.  Renderers translate
//! these settings into concrete CSS classes, elevation tokens and ARIA
//! attributes without duplicating business logic.  Enterprise teams can rely on
//! the deterministic [`PaperState::tokens`] contract to script automated visual
//! regression checks across frameworks.

/// Enumerates the supported Paper variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperVariant {
    /// Elevated surfaces receive a drop shadow derived from the Material
    /// elevation ramp.
    Elevated,
    /// Outlined surfaces remove the drop shadow in favour of a stroked border.
    Outlined,
    /// Plain surfaces inherit the parent background while still exposing shape
    /// tokens for rounded corners.
    Plain,
}

impl PaperVariant {
    /// Returns the stable variant identifier used by renderers and QA tooling.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Elevated => "elevated",
            Self::Outlined => "outlined",
            Self::Plain => "plain",
        }
    }
}

/// Tracks surface configuration independent from any view layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaperState {
    variant: PaperVariant,
    elevation: u8,
    square: bool,
    labelled_by: Option<String>,
}

impl PaperState {
    /// Builds a new [`PaperState`] using the provided variant.
    pub fn new(variant: PaperVariant) -> Self {
        Self {
            variant,
            elevation: 1,
            square: false,
            labelled_by: None,
        }
    }

    /// Configures the elevation level used by the renderer.
    ///
    /// Values above 24 clamp to 24 to match the Material guidelines. Returning
    /// `self` enables builder style ergonomics.
    pub fn with_elevation(mut self, level: u8) -> Self {
        self.elevation = level.min(24);
        self
    }

    /// Configures whether the surface should render with rounded corners.
    pub fn with_square(mut self, square: bool) -> Self {
        self.square = square;
        self
    }

    /// Associates the surface with an external label for accessibility.
    pub fn with_labelled_by(mut self, labelled_by: impl Into<String>) -> Self {
        self.labelled_by = Some(labelled_by.into());
        self
    }

    /// Returns the configured variant.
    pub fn variant(&self) -> PaperVariant {
        self.variant
    }

    /// Returns the resolved elevation level (0-24).
    pub fn elevation(&self) -> u8 {
        self.elevation
    }

    /// Returns whether the surface should suppress corner rounding.
    pub fn square(&self) -> bool {
        self.square
    }

    /// Returns the ARIA attributes required by accessibility adapters.
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(2);
        if let Some(label) = &self.labelled_by {
            attrs.push(("aria-labelledby", label.clone()));
        }
        attrs
    }

    /// Returns deterministic tokens that renderers translate into CSS classes
    /// and data attributes.
    pub fn tokens(&self) -> Vec<(&'static str, String)> {
        vec![
            ("variant", self.variant.as_str().to_string()),
            ("elevation", self.elevation.to_string()),
            ("square", self.square.to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elevation_clamps_to_material_range() {
        let state = PaperState::new(PaperVariant::Elevated).with_elevation(42);
        assert_eq!(state.elevation(), 24);
    }

    #[test]
    fn tokens_include_variant_and_square_flag() {
        let state = PaperState::new(PaperVariant::Outlined).with_square(true);
        let tokens = state.tokens();
        assert!(tokens
            .iter()
            .any(|(k, v)| *k == "variant" && v == "outlined"));
        assert!(tokens.iter().any(|(k, v)| *k == "square" && v == "true"));
    }

    #[test]
    fn labelled_surfaces_emit_aria_relationship() {
        let state = PaperState::new(PaperVariant::Plain).with_labelled_by("accordion-summary");
        let attrs = state.accessibility_attributes();
        assert_eq!(
            attrs[0],
            ("aria-labelledby", "accordion-summary".to_string())
        );
    }
}
