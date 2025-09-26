//! Headless slider state machine shared by Joy UI components.
//!
//! The implementation prioritises predictability for enterprise dashboards.  The
//! state machine clamps values, snaps to configured steps, and exposes helpers
//! for keyboard/page interaction so automated tests can assert exact transitions
//! without simulating DOM measurements.  Rendering adapters only need to apply
//! the returned [`SliderChange`] data and copy the documented ARIA attributes.

use crate::aria;

/// Orientation of the slider track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Horizontal sliders grow left-to-right.
    Horizontal,
    /// Vertical sliders grow bottom-to-top.
    Vertical,
}

impl SliderOrientation {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Declarative configuration consumed by [`SliderState`].
#[derive(Debug, Clone)]
pub struct SliderConfig {
    /// Minimum logical value.
    pub min: f64,
    /// Maximum logical value.
    pub max: f64,
    /// Increment applied for keyboard nudges.
    pub step: f64,
    /// Increment applied for PageUp/PageDown style movements.
    pub page: f64,
    /// Initial value used when constructing the slider.
    pub default_value: f64,
    /// Whether the slider starts disabled.
    pub disabled: bool,
    /// Orientation of the slider track.
    pub orientation: SliderOrientation,
}

impl SliderConfig {
    /// Enterprise defaults matching Joy’s UX guidelines.
    pub fn enterprise_defaults(min: f64, max: f64) -> Self {
        let range = (max - min).abs().max(1.0);
        Self {
            min,
            max,
            step: range / 100.0,
            page: range / 10.0,
            default_value: min,
            disabled: false,
            orientation: SliderOrientation::Horizontal,
        }
    }
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self::enterprise_defaults(0.0, 100.0)
    }
}

/// Change metadata returned by [`SliderState`] APIs.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SliderChange {
    /// The new value if it changed.
    pub value: Option<f64>,
}

impl SliderChange {
    fn value(value: f64) -> Self {
        Self { value: Some(value) }
    }
}

/// Slider state machine.
#[derive(Debug, Clone)]
pub struct SliderState {
    config: SliderConfig,
    value: f64,
    dragging: bool,
}

impl SliderState {
    /// Construct a new slider.
    pub fn new(config: SliderConfig) -> Self {
        let mut state = Self {
            value: config.default_value,
            config,
            dragging: false,
        };
        state.value = state.clamp_and_snap(state.value);
        state
    }

    /// Returns the current logical value.
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the current value as a percentage between 0 and 100.
    pub fn percent(&self) -> f64 {
        let denom = (self.config.max - self.config.min).abs();
        if denom == 0.0 {
            return 0.0;
        }
        ((self.value - self.config.min) / denom).clamp(0.0, 1.0) * 100.0
    }

    /// Returns whether the slider is currently disabled.
    #[inline]
    pub fn is_disabled(&self) -> bool {
        self.config.disabled
    }

    /// Update the disabled flag.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.config.disabled = disabled;
    }

    /// Returns whether a pointer drag is in progress.
    #[inline]
    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    /// Mark the beginning of a drag gesture.
    pub fn begin_drag(&mut self) {
        if !self.config.disabled {
            self.dragging = true;
        }
    }

    /// Mark the end of a drag gesture.
    pub fn end_drag(&mut self) {
        self.dragging = false;
    }

    /// Directly set the slider value.
    pub fn set_value(&mut self, value: f64) -> SliderChange {
        if self.config.disabled {
            return SliderChange::default();
        }
        let snapped = self.clamp_and_snap(value);
        if (snapped - self.value).abs() < f64::EPSILON {
            return SliderChange::default();
        }
        self.value = snapped;
        SliderChange::value(self.value)
    }

    /// Increment the slider using the configured step.
    pub fn increment(&mut self) -> SliderChange {
        let step = self.config.step.abs().max(f64::EPSILON);
        self.set_value(self.value + step)
    }

    /// Decrement the slider using the configured step.
    pub fn decrement(&mut self) -> SliderChange {
        let step = self.config.step.abs().max(f64::EPSILON);
        self.set_value(self.value - step)
    }

    /// Increment the slider using the configured page size.
    pub fn page_increment(&mut self) -> SliderChange {
        let page = self.config.page.abs().max(self.config.step.abs());
        self.set_value(self.value + page)
    }

    /// Decrement the slider using the configured page size.
    pub fn page_decrement(&mut self) -> SliderChange {
        let page = self.config.page.abs().max(self.config.step.abs());
        self.set_value(self.value - page)
    }

    /// Build the ARIA/data attributes for the thumb element.
    pub fn thumb_accessibility_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::with_capacity(6);
        attrs.push(("role", "slider".into()));
        attrs.push(("aria-valuemin", self.config.min.to_string()));
        attrs.push(("aria-valuemax", self.config.max.to_string()));
        attrs.push(("aria-valuenow", self.value.to_string()));
        attrs.push(("aria-orientation", self.config.orientation.as_str().into()));
        aria::extend_disabled_attributes(&mut attrs, self.config.disabled);
        attrs
    }

    fn clamp_and_snap(&self, value: f64) -> f64 {
        let mut clamped = value.clamp(self.config.min, self.config.max);
        let step = self.config.step.abs();
        if step > 0.0 {
            let offset = clamped - self.config.min;
            let steps = (offset / step).round();
            clamped = self.config.min + steps * step;
        }
        clamped = clamped.clamp(self.config.min, self.config.max);
        if clamped.is_finite() {
            clamped
        } else {
            self.config.min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapping_aligns_to_step() {
        let mut slider = SliderState::new(SliderConfig {
            min: 0.0,
            max: 10.0,
            step: 2.0,
            page: 4.0,
            default_value: 0.0,
            disabled: false,
            orientation: SliderOrientation::Horizontal,
        });
        let change = slider.set_value(3.3);
        assert_eq!(change.value, Some(4.0));
        assert_eq!(slider.value(), 4.0);
    }

    #[test]
    fn percent_returns_expected_range() {
        let slider = SliderState::new(SliderConfig {
            min: 0.0,
            max: 5.0,
            step: 0.5,
            page: 2.0,
            default_value: 2.5,
            disabled: false,
            orientation: SliderOrientation::Horizontal,
        });
        assert!((slider.percent() - 50.0).abs() < 0.01);
    }

    #[test]
    fn disabled_slider_ignores_updates() {
        let mut slider = SliderState::new(SliderConfig {
            min: 0.0,
            max: 10.0,
            step: 1.0,
            page: 3.0,
            default_value: 5.0,
            disabled: true,
            orientation: SliderOrientation::Horizontal,
        });
        let change = slider.increment();
        assert_eq!(change.value, None);
        assert_eq!(slider.value(), 5.0);
    }
}
