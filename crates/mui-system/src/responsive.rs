use crate::theme::Breakpoints;
use serde::{Deserialize, Serialize};

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
    /// Resolves the appropriate value for a given viewport width.
    pub fn resolve(&self, width: u32, bp: &Breakpoints) -> T {
        if width >= bp.xl {
            self.xl
                .as_ref()
                .or(self.lg.as_ref())
                .or(self.md.as_ref())
                .or(self.sm.as_ref())
                .unwrap_or(&self.xs)
                .clone()
        } else if width >= bp.lg {
            self.lg
                .as_ref()
                .or(self.md.as_ref())
                .or(self.sm.as_ref())
                .unwrap_or(&self.xs)
                .clone()
        } else if width >= bp.md {
            self.md
                .as_ref()
                .or(self.sm.as_ref())
                .unwrap_or(&self.xs)
                .clone()
        } else if width >= bp.sm {
            self.sm.as_ref().unwrap_or(&self.xs).clone()
        } else {
            self.xs.clone()
        }
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
