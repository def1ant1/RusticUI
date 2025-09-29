//! Lightweight backdrop state machine shared by modal surfaces.

/// Backdrop state toggling visibility and transition metadata.
#[derive(Debug, Clone)]
pub struct BackdropState {
    open: bool,
    animation_frame: u32,
}

impl BackdropState {
    /// Create a new backdrop.
    pub fn new(open: bool) -> Self {
        Self {
            open,
            animation_frame: 0,
        }
    }

    /// Returns whether the backdrop is open.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Toggle visibility.
    pub fn set_open(&mut self, open: bool) {
        if self.open != open {
            self.open = open;
            self.animation_frame = self.animation_frame.wrapping_add(1);
        }
    }

    /// Returns the animation frame counter which adapters can use to detect
    /// transition restarts.
    pub fn animation_frame(&self) -> u32 {
        self.animation_frame
    }

    /// Returns ARIA/data attributes for the backdrop element.
    pub fn aria_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(3);
        attrs.push(("data-open", self.open.to_string()));
        if !self.open {
            attrs.push(("aria-hidden", "true".into()));
        }
        attrs.push(("data-animation-frame", self.animation_frame.to_string()));
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_frame_increments() {
        let mut backdrop = BackdropState::new(false);
        let initial = backdrop.animation_frame();
        backdrop.set_open(true);
        assert!(backdrop.animation_frame() > initial);
    }
}
