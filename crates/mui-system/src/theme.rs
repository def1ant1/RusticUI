use serde::{Deserialize, Serialize};

/// Typed representation of the design system theme.
///
/// The struct mirrors the JS theme object but leverages Rust's strong
/// typing so invalid configurations are caught at compile time. `serde`
/// support enables seamless JSON (de)serialization for interop with
/// existing tooling and configuration files.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// Base spacing unit used by the `spacing` helper. Expressed in pixels
    /// to simplify calculations across platforms.
    pub spacing: u16,
    /// Responsive breakpoints measured in pixels.
    pub breakpoints: Breakpoints,
    /// Primary and secondary colors expressed as hex strings.
    pub palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            spacing: 8,
            breakpoints: Breakpoints::default(),
            palette: Palette::default(),
        }
    }
}

impl Theme {
    /// Calculates a spacing value by multiplying the base unit with the
    /// provided factor. This mirrors the JS `theme.spacing` utility.
    pub fn spacing(&self, factor: u16) -> u16 {
        self.spacing * factor
    }
}

/// Breakpoint definitions in ascending order. Consumers can extend this
/// struct if additional breakpoints are required.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Breakpoints {
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            sm: 600,
            md: 900,
            lg: 1200,
            xl: 1536,
        }
    }
}

/// Minimal color palette capturing primary and secondary accents.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    pub primary: String,
    pub secondary: String,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: "#1976d2".to_string(),
            secondary: "#dc004e".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_serializes_and_spacing_works() {
        let theme = Theme::default();
        // Verify spacing helper
        assert_eq!(theme.spacing(2), 16);

        // Round trip through JSON to ensure `serde` wiring is correct
        let json = serde_json::to_string(&theme).expect("serialize");
        let de: Theme = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(theme, de);
    }
}
