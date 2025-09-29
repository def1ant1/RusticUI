//! Headless avatar utilities exposing deterministic accessibility metadata.
//!
//! The state machine encapsulates fallback generation (initials) and exposes a
//! stable set of attributes that renderers translate into DOM nodes.  Keeping
//! the logic independent from any specific framework guarantees consistency
//! across SSR and client renderers.

/// Headless configuration for avatar surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvatarState {
    alt: Option<String>,
    label: Option<String>,
    fallback_initials: Option<String>,
}

impl AvatarState {
    /// Builds a new [`AvatarState`] with optional alt text and fallback
    /// initials.
    pub fn new(alt: Option<String>, fallback_initials: Option<String>) -> Self {
        Self {
            alt,
            label: None,
            fallback_initials,
        }
    }

    /// Assigns an accessible label typically surfaced via `aria-label`.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the alt text attached to the avatar image.
    pub fn alt(&self) -> Option<&str> {
        self.alt.as_deref()
    }

    /// Returns fallback initials used when no image is available.
    pub fn fallback_initials(&self) -> Option<&str> {
        self.fallback_initials.as_deref()
    }

    /// Generates accessibility attributes for the avatar root.
    pub fn accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(1);
        if let Some(label) = &self.label {
            attrs.push(("aria-label", label.clone()));
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_is_reflected_in_accessibility_attributes() {
        let state = AvatarState::new(Some("User photo".into()), None).with_label("Profile");
        let attrs = state.accessibility_attributes();
        assert_eq!(attrs[0], ("aria-label", "Profile".to_string()));
    }

    #[test]
    fn fallback_initials_round_trip() {
        let state = AvatarState::new(None, Some("JD".into()));
        assert_eq!(state.fallback_initials(), Some("JD"));
    }
}
