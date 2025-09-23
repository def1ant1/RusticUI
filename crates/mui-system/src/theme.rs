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
    pub joy: JoyTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            spacing: 8,
            breakpoints: Breakpoints::default(),
            palette: Palette::default(),
            typography: TypographyScheme::default(),
            joy: JoyTheme::default(),
        }
    }
}

impl Theme {
    /// Calculates a spacing value by multiplying the base unit with the
    /// provided factor. This mirrors the JS `theme.spacing` utility.
    pub fn spacing(&self, factor: u16) -> u16 {
        self.spacing * factor
    }

    /// Returns a [`Theme`] with Joy overrides applied on top of the canonical
    /// defaults. This keeps builder style ergonomics for automation pipelines
    /// that want to emit customised templates.
    pub fn with_joy_overrides<O>(overrides: O) -> Self
    where
        O: Into<JoyThemeOverrides>,
    {
        let mut theme = Self::default();
        theme.apply_joy_overrides(overrides);
        theme
    }

    /// Merges the supplied Joy overrides into the current theme instance.
    pub fn apply_joy_overrides<O>(&mut self, overrides: O)
    where
        O: Into<JoyThemeOverrides>,
    {
        self.joy.merge_overrides(overrides);
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
/// The metadata drives Joy component styling across frameworks, enables
/// automation tooling to emit comments/templates, and allows enterprise
/// override pipelines to tweak focus affordances without rewriting CSS.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyTheme {
    /// Default corner radius applied to Joy components.
    pub radius: u8,
    /// Focus specific configuration (thickness, palette hooks, formatting
    /// templates).
    pub focus: JoyFocus,
    /// Box-shadow presets aligned with Joy design guidance.
    pub shadow: JoyShadow,
    /// Metadata describing which palette keys the Joy workflow expects. The
    /// field doubles as inline documentation for automation pipelines.
    pub color_system: JoyColorSystemMetadata,
}

impl JoyTheme {
    /// Returns a builder used by automation scripts and tests.
    pub fn builder() -> JoyThemeBuilder {
        JoyThemeBuilder::default()
    }

    /// Applies the provided overrides to the current Joy token set.
    pub fn merge_overrides<O>(&mut self, overrides: O)
    where
        O: Into<JoyThemeOverrides>,
    {
        overrides.into().apply(self);
    }

    /// Convenience helper for building a Joy theme from overrides while
    /// starting from the canonical defaults.
    pub fn with_overrides<O>(overrides: O) -> Self
    where
        O: Into<JoyThemeOverrides>,
    {
        let mut base = Self::default();
        base.merge_overrides(overrides);
        base
    }

    /// Returns the outline declaration used by Joy components when a surface
    /// receives keyboard focus.
    pub fn focus_outline_for_color(&self, color: &str) -> String {
        self.focus.outline_for_color(color)
    }

    /// Returns the focus shadow (box-shadow) used by Joy components.
    pub fn focus_shadow_for_color(&self, color: &str) -> String {
        self.shadow
            .focus_ring_for_color(color, self.focus.thickness)
    }

    /// Resolves the palette colour referenced by the Joy focus configuration.
    pub fn focus_color_from_palette(&self, palette: &PaletteScheme) -> String {
        match self.focus.palette_reference.as_str() {
            "neutral" => palette.neutral.clone(),
            "danger" => palette.danger.clone(),
            "success" => palette.success.clone(),
            "warning" => palette.warning.clone(),
            "info" => palette.info.clone(),
            _ => palette.primary.clone(),
        }
    }

    /// Exposes automation-friendly comments so downstream tooling can embed the
    /// Joy metadata in generated configuration files without duplicating copy.
    pub fn automation_comments() -> Vec<&'static str> {
        vec![
            "radius – shared corner rounding applied to Joy surfaces.",
            "focus.thickness – pixel width of the focus outline + shadow.",
            "focus.palette_reference – palette key resolved for focus rings.",
            "focus.outline_template – string template used for outline CSS.",
            "shadow.focus_ring_template – format string for Joy focus shadows.",
            "shadow.surface – ambient elevation shadow used by Joy surfaces.",
            "color_system.palette_slots – expected palette keys for automation.",
            "color_system.automation_note – human readable explanation of the Joy palette contract.",
        ]
    }

    /// Serialises the Joy theme into a JSON object that doubles as a template
    /// for automation pipelines. The payload mirrors `serde_json::to_value` but
    /// stays stable across releases.
    pub fn json_template() -> serde_json::Value {
        let default = Self::default();
        serde_json::json!({
            "radius": default.radius,
            "focus": {
                "thickness": default.focus.thickness,
                "palette_reference": default.focus.palette_reference,
                "outline_template": default.focus.outline_template,
            },
            "shadow": {
                "focus_ring_template": default.shadow.focus_ring_template,
                "surface": default.shadow.surface,
            },
            "color_system": {
                "palette_slots": default.color_system.palette_slots,
                "automation_note": default.color_system.automation_note,
            }
        })
    }
}

impl Default for JoyTheme {
    fn default() -> Self {
        Self {
            radius: 4,
            focus: JoyFocus::default(),
            shadow: JoyShadow::default(),
            color_system: JoyColorSystemMetadata::default(),
        }
    }
}

/// Focus styling metadata used by Joy components.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyFocus {
    /// Thickness in pixels of the default focus ring used for accessibility.
    pub thickness: u8,
    /// Palette key resolved to produce the focus colour. Defaults to the Joy
    /// primary colour so focus affordances align with brand accents.
    pub palette_reference: String,
    /// Format string applied to the resolved colour. The template understands
    /// `{thickness}` and `{color}` placeholders.
    pub outline_template: String,
}

impl JoyFocus {
    /// Formats the outline declaration using the provided colour.
    pub fn outline_for_color(&self, color: &str) -> String {
        self.outline_template
            .replace("{thickness}", &self.thickness.to_string())
            .replace("{color}", color)
    }
}

impl Default for JoyFocus {
    fn default() -> Self {
        Self {
            thickness: 2,
            palette_reference: "primary".to_string(),
            outline_template: "{thickness}px solid {color}".to_string(),
        }
    }
}

/// Shadow presets surfaced to Joy components.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyShadow {
    /// Format string used to generate the focus shadow. The template understands
    /// `{thickness}` and `{color}` placeholders.
    pub focus_ring_template: String,
    /// Ambient elevation shadow applied to surfaces such as cards.
    pub surface: String,
}

impl JoyShadow {
    /// Formats the focus ring shadow using the provided colour and thickness.
    pub fn focus_ring_for_color(&self, color: &str, thickness: u8) -> String {
        self.focus_ring_template
            .replace("{thickness}", &thickness.to_string())
            .replace("{color}", color)
    }
}

impl Default for JoyShadow {
    fn default() -> Self {
        Self {
            focus_ring_template: "0 0 0 {thickness}px {color}".to_string(),
            surface: "0 8px 24px rgba(15, 23, 42, 0.18)".to_string(),
        }
    }
}

/// Captures the palette expectations for Joy automation pipelines.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyColorSystemMetadata {
    /// Palette keys (mirroring [`PaletteScheme`]) that Joy helpers depend on.
    pub palette_slots: Vec<String>,
    /// Human readable hint emitted into generated templates so downstream teams
    /// understand how Joy resolves colours.
    pub automation_note: String,
}

impl Default for JoyColorSystemMetadata {
    fn default() -> Self {
        Self {
            palette_slots: vec![
                "primary".to_string(),
                "neutral".to_string(),
                "danger".to_string(),
                "success".to_string(),
                "warning".to_string(),
                "info".to_string(),
            ],
            automation_note: "Joy helpers resolve palette colours via Theme::palette.active() and the slots declared above.".to_string(),
        }
    }
}

/// Partial overrides applied to [`JoyTheme`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyThemeOverrides {
    pub radius: Option<u8>,
    pub focus: Option<JoyFocusOverrides>,
    pub shadow: Option<JoyShadowOverrides>,
    pub color_system: Option<JoyColorSystemOverrides>,
}

impl JoyThemeOverrides {
    /// Applies the overrides to the provided [`JoyTheme`].
    pub fn apply(self, theme: &mut JoyTheme) {
        if let Some(radius) = self.radius {
            theme.radius = radius;
        }
        if let Some(focus) = self.focus {
            focus.apply(&mut theme.focus);
        }
        if let Some(shadow) = self.shadow {
            shadow.apply(&mut theme.shadow);
        }
        if let Some(color_system) = self.color_system {
            color_system.apply(&mut theme.color_system);
        }
    }
}

impl From<JoyThemeBuilder> for JoyThemeOverrides {
    fn from(builder: JoyThemeBuilder) -> Self {
        builder.overrides
    }
}

/// Builder style helper for constructing [`JoyThemeOverrides`].
#[derive(Clone, Debug, Default)]
pub struct JoyThemeBuilder {
    overrides: JoyThemeOverrides,
}

impl JoyThemeBuilder {
    /// Override the shared Joy radius.
    pub fn radius(mut self, radius: u8) -> Self {
        self.overrides.radius = Some(radius);
        self
    }

    /// Override the focus thickness (in pixels).
    pub fn focus_thickness(mut self, thickness: u8) -> Self {
        self.overrides
            .focus
            .get_or_insert_with(Default::default)
            .thickness = Some(thickness);
        self
    }

    /// Update the palette slot powering focus indicators.
    pub fn focus_palette_reference<S>(mut self, slot: S) -> Self
    where
        S: Into<String>,
    {
        self.overrides
            .focus
            .get_or_insert_with(Default::default)
            .palette_reference = Some(slot.into());
        self
    }

    /// Replace the focus outline formatting template.
    pub fn focus_outline_template<S>(mut self, template: S) -> Self
    where
        S: Into<String>,
    {
        self.overrides
            .focus
            .get_or_insert_with(Default::default)
            .outline_template = Some(template.into());
        self
    }

    /// Override the focus ring shadow template.
    pub fn shadow_focus_ring_template<S>(mut self, template: S) -> Self
    where
        S: Into<String>,
    {
        self.overrides
            .shadow
            .get_or_insert_with(Default::default)
            .focus_ring_template = Some(template.into());
        self
    }

    /// Override the Joy surface elevation shadow.
    pub fn shadow_surface<S>(mut self, surface: S) -> Self
    where
        S: Into<String>,
    {
        self.overrides
            .shadow
            .get_or_insert_with(Default::default)
            .surface = Some(surface.into());
        self
    }

    /// Replace the list of palette slots documented for automation.
    pub fn color_system_palette_slots<I, S>(mut self, slots: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.overrides
            .color_system
            .get_or_insert_with(Default::default)
            .palette_slots = Some(slots.into_iter().map(Into::into).collect());
        self
    }

    /// Provide a custom automation note for downstream tooling.
    pub fn color_system_note<S>(mut self, note: S) -> Self
    where
        S: Into<String>,
    {
        self.overrides
            .color_system
            .get_or_insert_with(Default::default)
            .automation_note = Some(note.into());
        self
    }

    /// Finalises the builder into a [`JoyThemeOverrides`] object.
    pub fn build(self) -> JoyThemeOverrides {
        self.into()
    }
}

/// Partial overrides for [`JoyFocus`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyFocusOverrides {
    pub thickness: Option<u8>,
    pub palette_reference: Option<String>,
    pub outline_template: Option<String>,
}

impl JoyFocusOverrides {
    fn apply(self, focus: &mut JoyFocus) {
        if let Some(thickness) = self.thickness {
            focus.thickness = thickness;
        }
        if let Some(reference) = self.palette_reference {
            focus.palette_reference = reference;
        }
        if let Some(template) = self.outline_template {
            focus.outline_template = template;
        }
    }
}

/// Partial overrides for [`JoyShadow`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyShadowOverrides {
    pub focus_ring_template: Option<String>,
    pub surface: Option<String>,
}

impl JoyShadowOverrides {
    fn apply(self, shadow: &mut JoyShadow) {
        if let Some(template) = self.focus_ring_template {
            shadow.focus_ring_template = template;
        }
        if let Some(surface) = self.surface {
            shadow.surface = surface;
        }
    }
}

/// Partial overrides for [`JoyColorSystemMetadata`].
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JoyColorSystemOverrides {
    pub palette_slots: Option<Vec<String>>,
    pub automation_note: Option<String>,
}

impl JoyColorSystemOverrides {
    fn apply(self, metadata: &mut JoyColorSystemMetadata) {
        if let Some(slots) = self.palette_slots {
            metadata.palette_slots = slots;
        }
        if let Some(note) = self.automation_note {
            metadata.automation_note = note;
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
        assert_eq!(theme.joy.focus.thickness, 2);
        assert_eq!(
            theme.joy.focus.outline_for_color("#ffffff"),
            "2px solid #ffffff"
        );
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

    #[test]
    fn palette_defaults_cover_light_and_dark_joy_colors() {
        let palette = Palette::default();
        for (scheme, tokens) in [
            (ColorScheme::Light, &palette.light),
            (ColorScheme::Dark, &palette.dark),
        ] {
            assert!(
                !tokens.success.is_empty(),
                "success token should exist for {:?}",
                scheme
            );
            assert!(
                !tokens.warning.is_empty(),
                "warning token should exist for {:?}",
                scheme
            );
            assert!(
                !tokens.info.is_empty(),
                "info token should exist for {:?}",
                scheme
            );
        }
    }

    #[test]
    fn joy_theme_builder_applies_overrides() {
        let overrides = JoyTheme::builder()
            .radius(8)
            .focus_thickness(4)
            .focus_palette_reference("neutral")
            .focus_outline_template("{thickness}px dotted {color}")
            .shadow_focus_ring_template("0 0 0 {thickness}px rgba(0,0,0,0.4)")
            .shadow_surface("0 2px 12px rgba(0,0,0,0.2)")
            .color_system_palette_slots(["primary", "neutral", "success"])
            .color_system_note("custom note")
            .build();

        let mut joy = JoyTheme::default();
        joy.merge_overrides(overrides.clone());

        assert_eq!(joy.radius, 8);
        assert_eq!(joy.focus.thickness, 4);
        assert_eq!(joy.focus.palette_reference, "neutral");
        assert_eq!(joy.focus.outline_template, "{thickness}px dotted {color}");
        assert_eq!(
            joy.shadow.focus_ring_template,
            "0 0 0 {thickness}px rgba(0,0,0,0.4)"
        );
        assert_eq!(joy.shadow.surface, "0 2px 12px rgba(0,0,0,0.2)");
        assert_eq!(
            joy.color_system.palette_slots,
            vec!["primary", "neutral", "success"]
        );
        assert_eq!(joy.color_system.automation_note, "custom note");

        let rebuilt = JoyTheme::with_overrides(overrides);
        assert_eq!(joy, rebuilt);
    }

    #[test]
    fn theme_with_joy_overrides_injects_updates() {
        let theme = Theme::with_joy_overrides(
            JoyTheme::builder()
                .focus_thickness(5)
                .color_system_note("automation")
                .build(),
        );
        assert_eq!(theme.joy.focus.thickness, 5);
        assert_eq!(theme.joy.color_system.automation_note, "automation");
    }

    #[test]
    fn joy_template_emits_commentary() {
        let comments = JoyTheme::automation_comments();
        assert!(comments.len() >= 6);
        assert!(comments.iter().any(|c| c.contains("radius")));

        let template = JoyTheme::json_template();
        assert_eq!(
            template
                .get("focus")
                .and_then(|focus| focus.get("thickness"))
                .and_then(|value| value.as_u64()),
            Some(2)
        );
    }
}
