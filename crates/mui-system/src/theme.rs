use serde::{Deserialize, Serialize};

/// Enumerates the supported Material color schemes.
///
/// The default mirrors the upstream JavaScript implementation which starts in
/// light mode.  The enum serializes as lowercase strings so automation tooling
/// can share fixtures with the TypeScript ecosystem without translation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    /// High luminance surfaces with dark foreground content.
    Light,
    /// Darker backgrounds paired with lighter foreground content.
    Dark,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::Light
    }
}

impl ColorScheme {
    /// Returns the lowercase identifier used by `color-scheme` CSS declarations
    /// and HTML `data` attributes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Convenience helper used by toggling hooks.
    pub fn toggled(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// Typed representation of the design system theme.
///
/// The struct mirrors the JS theme object but leverages Rust's strong
/// typing so invalid configurations are caught at compile time. `serde`
/// support enables seamless JSON (de)serialization for interop with
/// existing tooling and configuration files.  We intentionally keep the
/// structure explicit (instead of opaque maps) so large enterprises can
/// audit every available contract and automate override generation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// Base spacing unit used by the `spacing` helper. Expressed in pixels
    /// to simplify calculations across platforms.
    pub spacing: u16,
    /// Responsive breakpoints measured in pixels.
    pub breakpoints: Breakpoints,
    /// Primary, secondary and extended palette colors expressed as hex strings.
    pub palette: Palette,
    /// Material typography ramp expressed in rems and point sizes.
    pub typography: TypographyScheme,
    /// Joy specific design tokens such as corner radius and focus outlines.
    pub joy: JoyTokens,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            spacing: 8,
            breakpoints: Breakpoints::default(),
            palette: Palette::default(),
            typography: TypographyScheme::default(),
            joy: JoyTokens::default(),
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
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            xs: 0,
            sm: 600,
            md: 900,
            lg: 1200,
            xl: 1536,
        }
    }
}

/// Minimal color palette capturing primary and secondary accents.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PaletteScheme {
    pub primary: String,
    pub secondary: String,
    /// Neutral color used by Joy components.
    pub neutral: String,
    /// Danger color used by Joy components.
    pub danger: String,
    /// Success color surfaced by Joy primitives for positive feedback states.
    #[serde(default = "default_palette_success")]
    pub success: String,
    /// Warning color mapped to Joy cautionary components.
    #[serde(default = "default_palette_warning")]
    pub warning: String,
    /// Informational accent rounding out the Joy palette.
    #[serde(default = "default_palette_info")]
    pub info: String,
    /// Background color used for the app shell.
    pub background_default: String,
    /// Background color for elevated surfaces like cards.
    pub background_paper: String,
    /// Primary body text color.
    pub text_primary: String,
    /// Secondary/disabled text color.
    pub text_secondary: String,
}

fn default_palette_success() -> String {
    "#2e7d32".to_string()
}

fn default_palette_warning() -> String {
    "#ed6c02".to_string()
}

fn default_palette_info() -> String {
    "#0288d1".to_string()
}

impl Default for PaletteScheme {
    fn default() -> Self {
        Self {
            primary: "#1976d2".to_string(),
            secondary: "#dc004e".to_string(),
            neutral: "#64748b".to_string(),
            danger: "#d32f2f".to_string(),
            success: default_palette_success(),
            warning: default_palette_warning(),
            info: default_palette_info(),
            background_default: "#fafafa".to_string(),
            background_paper: "#ffffff".to_string(),
            text_primary: "#1f2933".to_string(),
            text_secondary: "#52606d".to_string(),
        }
    }
}

/// Material color palette definitions for each supported color scheme.
///
/// The struct stores separate [`PaletteScheme`] instances for light and dark
/// operation so enterprise operators can vend both sets of tokens from a
/// single configuration file.  Framework adapters are expected to honour the
/// `initial_color_scheme` flag when emitting global styles or instantiating
/// providers and expose hooks/state that allow flipping the active scheme at
/// runtime without rebuilding the entire theme object.  This mirrors the
/// ergonomics of the JavaScript `createTheme` helper while remaining explicit
/// enough for large organisations to audit and automate.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    /// Light mode tokens align with `@mui/material` defaults.
    pub light: PaletteScheme,
    /// Dark mode tokens aligned to Material Design guidance.
    pub dark: PaletteScheme,
    /// Scheme that should be considered active when building CSS resets.
    #[serde(default)]
    pub initial_color_scheme: ColorScheme,
}

impl Palette {
    /// Returns the [`PaletteScheme`] declared as the initial/active scheme.
    pub fn active(&self) -> &PaletteScheme {
        match self.initial_color_scheme {
            ColorScheme::Light => &self.light,
            ColorScheme::Dark => &self.dark,
        }
    }

    /// Mutable variant of [`Palette::active`] used by helper utilities to
    /// update tokens in-place while maintaining the currently selected scheme.
    pub fn active_mut(&mut self) -> &mut PaletteScheme {
        match self.initial_color_scheme {
            ColorScheme::Light => &mut self.light,
            ColorScheme::Dark => &mut self.dark,
        }
    }

    /// Returns a reference to a specific [`ColorScheme`] regardless of the
    /// configured active mode.  This keeps automation pipelines explicit when
    /// generating per-scheme artefacts.
    pub fn scheme(&self, scheme: ColorScheme) -> &PaletteScheme {
        match scheme {
            ColorScheme::Light => &self.light,
            ColorScheme::Dark => &self.dark,
        }
    }

    /// Mutable accessor for [`Palette::scheme`].
    pub fn scheme_mut(&mut self, scheme: ColorScheme) -> &mut PaletteScheme {
        match scheme {
            ColorScheme::Light => &mut self.light,
            ColorScheme::Dark => &mut self.dark,
        }
    }
}

fn default_dark_palette() -> PaletteScheme {
    PaletteScheme {
        primary: "#90caf9".to_string(),
        secondary: "#f48fb1".to_string(),
        neutral: "#94a3b8".to_string(),
        danger: "#f44336".to_string(),
        success: "#66bb6a".to_string(),
        warning: "#ffb74d".to_string(),
        info: "#29b6f6".to_string(),
        background_default: "#121212".to_string(),
        background_paper: "#1e1e1e".to_string(),
        text_primary: "#ffffff".to_string(),
        text_secondary: "#cbd5f5".to_string(),
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            light: PaletteScheme::default(),
            dark: default_dark_palette(),
            initial_color_scheme: ColorScheme::Light,
        }
    }
}

/// Canonical Material Design typography ramp represented in Rust friendly types.
///
/// The font sizes are expressed in rems (matching CSS best practices) so that
/// scaling entire applications for accessibility scenarios remains as simple as
/// changing the document's root font size.  Consumers are encouraged to
/// deserialize this struct and pipe it into build pipelines or design tokens
/// engines.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TypographyScheme {
    /// Primary sans-serif stack.
    pub font_family: String,
    /// Matching monospace stack used by components like `<Code>` or `<Chip>`.
    pub font_family_monospace: String,
    /// Base font size used for body copy (in px).
    pub font_size: f32,
    /// Base browser font size (HTML element) used when translating rems to px.
    pub html_font_size: f32,
    /// Font weights capture corporate branding guidelines.
    pub font_weight_light: u16,
    pub font_weight_regular: u16,
    pub font_weight_medium: u16,
    pub font_weight_bold: u16,
    /// Representative rem sizes for each typography slot.
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
    pub h4: f32,
    pub h5: f32,
    pub h6: f32,
    pub subtitle1: f32,
    pub subtitle2: f32,
    pub body1: f32,
    pub body2: f32,
    pub button: f32,
    pub caption: f32,
    pub overline: f32,
    /// Default line height multiplier used across the ramp.
    pub line_height: f32,
    /// Letter spacing applied to uppercase text such as buttons.
    pub button_letter_spacing: f32,
}

impl Default for TypographyScheme {
    fn default() -> Self {
        Self {
            font_family: "Roboto, Helvetica, Arial, sans-serif".to_string(),
            font_family_monospace:
                "Roboto Mono, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"
                    .to_string(),
            font_size: 14.0,
            html_font_size: 16.0,
            font_weight_light: 300,
            font_weight_regular: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
            h1: 3.75,
            h2: 3.0,
            h3: 2.25,
            h4: 2.0,
            h5: 1.5,
            h6: 1.25,
            subtitle1: 1.0,
            subtitle2: 0.875,
            body1: 1.0,
            body2: 0.875,
            button: 0.875,
            caption: 0.75,
            overline: 0.75,
            line_height: 1.5,
            button_letter_spacing: 0.089,
        }
    }
}

/// Joy specific design tokens that do not exist in the core Material theme.
///
/// They capture stylistic elements unique to Joy such as rounded corners and
/// the thickness of focus indicators.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JoyTokens {
    /// Default corner radius applied to Joy components.
    pub radius: u8,
    /// Thickness in pixels of the default focus ring used for accessibility.
    pub focus_thickness: u8,
}

impl Default for JoyTokens {
    fn default() -> Self {
        Self {
            radius: 4,
            focus_thickness: 2,
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
        // Joy tokens available
        assert_eq!(theme.joy.radius, 4);
        assert_eq!(theme.joy.focus_thickness, 2);
        assert_eq!(theme.palette.light.neutral, "#64748b");
        assert_eq!(theme.palette.light.success, "#2e7d32");
        assert_eq!(theme.palette.light.warning, "#ed6c02");
        assert_eq!(theme.palette.light.info, "#0288d1");
        assert_eq!(theme.breakpoints.xs, 0);

        // Round trip through JSON to ensure `serde` wiring is correct
        let json = serde_json::to_string(&theme).expect("serialize");
        let de: Theme = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(theme, de);
    }
}
