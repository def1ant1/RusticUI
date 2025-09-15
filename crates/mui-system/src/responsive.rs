use crate::theme::Breakpoints;
use serde::{Deserialize, Serialize};

/// Returns the current viewport width in pixels when executing in a browser
/// environment. When the code is evaluated in a headless test or a non WASM
/// target we simply return `0` so responsive props fall back to their base
/// (`xs`) values. Centralising this helper avoids each component having to
/// duplicate the same `web_sys::window` boilerplate and keeps breakpoints
/// consistent across frameworks.
pub fn viewport_width() -> u32 {
    #[cfg(any(
        feature = "yew",
        feature = "leptos",
        feature = "dioxus",
        feature = "sycamore",
    ))]
    {
        web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as u32
    }

    #[cfg(not(any(
        feature = "yew",
        feature = "leptos",
        feature = "dioxus",
        feature = "sycamore",
    )))]
    {
        0
    }
}

/// Helper structure representing values that change across breakpoints.
/// Missing values fall back to the next smallest defined one, mirroring
/// the cascading behavior of CSS media queries.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Responsive<T> {
    pub xs: T,
    pub sm: Option<T>,
    pub md: Option<T>,
    pub lg: Option<T>,
    pub xl: Option<T>,
}

impl<T: Clone> Responsive<T> {
    /// Convenience constructor that assigns the same value to all breakpoints.
    /// This keeps component props ergonomic when a value should stay constant
    /// yet still opts into the responsive type so downstream callers can opt
    /// into breakpoint specific overrides without changing the API surface.
    pub fn constant(value: T) -> Self {
        Self {
            xs: value,
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }

    /// Resolves the appropriate value for a given viewport width.
    pub fn resolve(&self, width: u32, bp: &Breakpoints) -> T {
        // Iterate from the largest breakpoint down to `xs`. The first matching
        // breakpoint that contains an explicit value wins while undefined
        // values cascade down to the next available entry, mirroring how CSS
        // media queries behave in the JavaScript implementation.
        let ordered = [
            (bp.xl, self.xl.as_ref()),
            (bp.lg, self.lg.as_ref()),
            (bp.md, self.md.as_ref()),
            (bp.sm, self.sm.as_ref()),
            (bp.xs, Some(&self.xs)),
        ];

        for (threshold, value) in ordered.into_iter() {
            if width >= threshold {
                if let Some(v) = value {
                    return v.clone();
                }
            }
        }

        // The array always contains `xs` so this branch only triggers if the
        // breakpoints are misconfigured (e.g. descending order). Falling back
        // to the base value keeps behaviour predictable even in that scenario.
        self.xs.clone()
    }
}

impl<T: Clone> From<T> for Responsive<T> {
    fn from(value: T) -> Self {
        Self::constant(value)
    }
}

/// Computes the percentage width a grid item should occupy given its span
/// and the total number of columns.
pub fn grid_span_to_percent(span: u16, columns: u16) -> f32 {
    (span as f32 / columns as f32) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn resolves_breakpoint_values() {
        let theme = Theme::default();
        let values = Responsive {
            xs: 1,
            sm: Some(2),
            md: Some(3),
            lg: None,
            xl: Some(5),
        };
        assert_eq!(values.resolve(500, &theme.breakpoints), 1);
        assert_eq!(values.resolve(700, &theme.breakpoints), 2);
        assert_eq!(values.resolve(1000, &theme.breakpoints), 3);
        assert_eq!(values.resolve(1300, &theme.breakpoints), 3); // fallback
        assert_eq!(values.resolve(1600, &theme.breakpoints), 5);
    }

    #[test]
    fn grid_span_calculates_percentage() {
        assert!((grid_span_to_percent(6, 12) - 50.0).abs() < f32::EPSILON);
    }
}
