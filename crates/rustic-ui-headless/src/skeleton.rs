//! Skeleton loading placeholder state machine.

/// Animation strategy for skeletons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkeletonAnimation {
    /// No animation.
    None,
    /// Pulsing animation.
    Pulse,
    /// Wave animation traversing across the placeholder.
    Wave,
}

impl SkeletonAnimation {
    const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pulse => "pulse",
            Self::Wave => "wave",
        }
    }
}

/// State for a skeleton placeholder.
#[derive(Debug, Clone)]
pub struct SkeletonState {
    animation: SkeletonAnimation,
    width: Option<String>,
    height: Option<String>,
    automation_id: Option<String>,
}

impl SkeletonState {
    /// Create a new skeleton state.
    pub fn new(animation: SkeletonAnimation) -> Self {
        Self {
            animation,
            width: None,
            height: None,
            automation_id: None,
        }
    }

    /// Configure the size tokens.
    pub fn with_size(mut self, width: Option<String>, height: Option<String>) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Attach an automation identifier.
    pub fn with_automation_id(mut self, id: impl Into<String>) -> Self {
        self.automation_id = Some(id.into());
        self
    }

    /// Returns the animation mode.
    pub fn animation(&self) -> SkeletonAnimation {
        self.animation
    }

    /// Build ARIA/data attributes for the placeholder.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(5);
        attrs.push(("aria-hidden", "true".into()));
        attrs.push(("data-skeleton-animation", self.animation.as_str().into()));
        if let Some(width) = &self.width {
            attrs.push(("data-skeleton-width", width.clone()));
        }
        if let Some(height) = &self.height {
            attrs.push(("data-skeleton-height", height.clone()));
        }
        if let Some(id) = &self.automation_id {
            attrs.push(("data-automation-id", id.clone()));
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_attributes_include_dimensions() {
        let state = SkeletonState::new(SkeletonAnimation::Wave)
            .with_size(Some("120px".into()), Some("32px".into()))
            .with_automation_id("skeleton.hero");
        let attrs = state.aria_attributes();
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-skeleton-width" && v == "120px"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-skeleton-height" && v == "32px"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == &"data-automation-id" && v == "skeleton.hero"));
    }
}
