//! Cross-framework helpers for rendering themed text input controls complete
//! with scoped class names, generated CSS and ARIA metadata.
//!
//! The adapters exposed here produce an `<input type="text">` element for
//! Leptos, Dioxus and Sycamore while sharing the same styling pipeline.  Each
//! variant resolves palette colours, typography and focus treatments from the
//! active [`Theme`] before wiring the values into a scoped stylesheet generated
//! by [`css_with_theme!`](rustic_ui_styled_engine_macros::css_with_theme). Centralising
//! the behaviour avoids subtle regressions between frameworks and keeps
//! enterprise teams from hand-maintaining near-identical CSS snippets.
//!
//! ## Theme-driven styling
//! * **Colour** – Defaults to [`Theme::palette.text_primary`] so input text
//!   inherits the brand's body copy tone. Consumers can override the value via
//!   [`ThemedProps::color`] to reflect contextual states such as success or
//!   warning flows.
//! * **Spacing** – Falls back to `theme.spacing(1)` giving the control generous
//!   padding that still mirrors Material defaults. An optional
//!   [`ThemedProps::padding`] string allows full customisation while the helper
//!   continues to look after border radii, typography and focus feedback.
//! * **Variants** – [`Variant::Outlined`] swaps in a subtle border using the
//!   secondary text colour whereas [`Variant::Plain`] keeps the background clean
//!   for use inside already-contained layouts. A deterministic BEM modifier is
//!   appended to the scoped class so downstream automation can target
//!   `rustic_ui_themed_input__outlined`/`--plain` without string concatenation.
//! * **Overrides** – [`ThemedProps::style_overrides`] accepts raw CSS which is
//!   spliced directly into the generated stylesheet. This makes it trivial to
//!   adjust corner cases (for example, forcing uppercase text) without giving up
//!   the theme-synchronised defaults.
//!
//! ## Debounce-friendly metadata
//! When [`ThemedProps::debounce_ms`] is provided the adapters emit a
//! `data-debounce-ms` attribute. Downstream frameworks frequently pair the
//! rendered markup with [`rustic_ui_utils::debounce`] to delay expensive network or
//! state updates; surfacing the chosen debounce window in the DOM keeps this
//! behaviour declarative and easy to introspect during QA walkthroughs.
//!
//! ## Accessibility
//! Assistive technologies rely on `aria-label` metadata to describe the purpose
//! of text inputs.  The helpers automatically merge [`ThemedProps::aria_label`]
//! into the rendered attributes via [`rustic_ui_utils::collect_attributes`], ensuring
//! hydration friendly ordering while keeping the implementation identical across
//! frameworks.  Additional attributes such as `placeholder` and
//! `data-debounce-ms` are merged using [`rustic_ui_utils::extend_attributes`], making
//! it easy to audit which metadata ships with the control in server rendered
//! output.

#[cfg(all(not(feature = "leptos"), any(feature = "dioxus", feature = "sycamore")))]
use crate::theme_provider::use_theme;
#[cfg(feature = "leptos")]
use crate::theme_provider::use_theme_leptos as use_theme;
use rustic_ui_utils::{attributes_to_html, collect_attributes, extend_attributes};

/// Available visual variants for the themed element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// Minimal styling with no border.
    #[default]
    Plain,
    /// Outlined style often used for emphasis.
    Outlined,
}

impl Variant {
    /// Returns the modifier portion used in BEM style class names.
    fn modifier(self) -> &'static str {
        match self {
            Variant::Plain => "plain",
            Variant::Outlined => "outlined",
        }
    }
}

/// Properties shared by all adapter implementations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThemedProps {
    /// Input value rendered in the `value` attribute.
    pub value: String,
    /// Optional placeholder hint shown when the input is empty.
    pub placeholder: Option<String>,
    /// Optional text colour. Defaults to the theme's primary text colour.
    pub color: Option<String>,
    /// Optional padding value applied to the element.
    pub padding: Option<String>,
    /// Style variant determining the generated class name.
    pub variant: Variant,
    /// Human readable label exposed via `aria-label`.
    pub aria_label: Option<String>,
    /// Additional CSS appended to the generated stylesheet.
    pub style_overrides: Option<String>,
    /// Optional debounce window surfaced through `data-debounce-ms`.
    pub debounce_ms: Option<u64>,
}

/// Scoped CSS class prefix used by every adapter.  Centralising the constant
/// avoids subtle typos when new integrations are added in the future.
const BASE_CLASS: &str = "rustic_ui_themed_input";

/// Convenience type holding precomputed visual tokens.  The helpers below share
/// this struct so that colour, padding and border calculations remain consistent
/// regardless of which adapter triggered the work.
#[derive(Clone, Debug)]
struct VisualTokens {
    text_color: String,
    padding: String,
    background: String,
    border: String,
    border_radius: String,
    font_family: String,
    font_size: String,
    placeholder_color: String,
    focus_border: String,
    focus_shadow: String,
}

/// Scoped style artefact produced by [`css_with_theme!`].
///
/// The struct records both the generated class name and the CSS string so SSR
/// adapters can inline the stylesheet while client side frameworks simply reuse
/// the class. Having a dedicated type keeps future automation (for example
/// generating documentation snippets) straightforward.
#[derive(Clone, Debug)]
struct ScopedStyle {
    class: String,
    stylesheet: String,
}

/// Resolves theme driven styling tokens, applying sensible defaults where
/// callers omitted a value.
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn resolve_visual_tokens(props: &ThemedProps) -> VisualTokens {
    let theme = use_theme();
    let palette = theme.palette.active();
    // Default to the primary body colour so the control mirrors Material text
    // fields when no overrides are provided.
    let text_color = props
        .color
        .clone()
        .unwrap_or_else(|| palette.text_primary.clone());
    // Provide a predictable padding default based on the spacing scale so the
    // layout feels harmonious even without explicit configuration.
    let padding = props
        .padding
        .clone()
        .unwrap_or_else(|| format!("{}px", theme.spacing(1)));
    let (background, border) = match props.variant {
        Variant::Plain => (
            palette.background_paper.clone(),
            String::from("1px solid transparent"),
        ),
        Variant::Outlined => (
            palette.background_default.clone(),
            format!("1px solid {}", palette.text_secondary.clone()),
        ),
    };
    let border_radius = format!("{}px", theme.joy.radius);
    let font_family = theme.typography.font_family.clone();
    let font_size = format!("{}px", theme.typography.font_size);
    let placeholder_color = palette.text_secondary.clone();
    let focus_color = theme.joy.focus_color_from_palette(palette);
    let focus_border = focus_color.clone();
    let focus_shadow = theme.joy.focus_shadow_for_color(&focus_color);
    VisualTokens {
        text_color,
        padding,
        background,
        border,
        border_radius,
        font_family,
        font_size,
        placeholder_color,
        focus_border,
        focus_shadow,
    }
}

/// Builds a deterministic class list using a BEM style modifier.
fn deterministic_class(variant: Variant) -> String {
    format!("{BASE_CLASS} {BASE_CLASS}--{}", variant.modifier())
}

/// Generates a scoped CSS class and stylesheet using the active [`Theme`].
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn scoped_style(tokens: &VisualTokens, overrides: Option<&str>) -> ScopedStyle {
    use rustic_ui_styled_engine_macros::css_with_theme;

    // Drive every declaration from theme tokens so updates to palette/spacing
    // cascade through the component without touching presentation code.
    let style = css_with_theme!(
        r#"
            color: ${text_color};
            padding: ${padding};
            background-color: ${background};
            border: ${border};
            border-radius: ${border_radius};
            font-family: ${font_family};
            font-size: ${font_size};
            line-height: 1.5;
            width: 100%;
            box-sizing: border-box;
            transition: border-color 120ms ease, box-shadow 120ms ease;
            &::placeholder {
                color: ${placeholder_color};
                opacity: 1;
            }
            &:focus {
                outline: none;
                border-color: ${focus_border};
                box-shadow: ${focus_shadow};
            }
        "#,
        text_color = tokens.text_color.clone(),
        padding = tokens.padding.clone(),
        background = tokens.background.clone(),
        border = tokens.border.clone(),
        border_radius = tokens.border_radius.clone(),
        font_family = tokens.font_family.clone(),
        font_size = tokens.font_size.clone(),
        placeholder_color = tokens.placeholder_color.clone(),
        focus_border = tokens.focus_border.clone(),
        focus_shadow = tokens.focus_shadow.clone()
    );

    let mut stylesheet = style.get_style_str().to_string();
    let class = style.get_class_name().to_string();
    // Immediately unregister the temporary handle so the style registry remains
    // free of duplicates when multiple adapters render concurrently.
    style.unregister();

    if let Some(extra) = overrides.and_then(|ov| {
        let trimmed = ov.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    }) {
        stylesheet.push_str(&format!("\n.{class} {{{extra}}}"));
    }

    ScopedStyle { class, stylesheet }
}

/// Resolves the deterministic BEM class and scoped theme class in one go.
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "sycamore"))]
fn themed_classes(props: &ThemedProps) -> (String, ScopedStyle) {
    let tokens = resolve_visual_tokens(props);
    let scoped = scoped_style(&tokens, props.style_overrides.as_deref());
    let classes = format!("{} {}", deterministic_class(props.variant), scoped.class);
    (classes, scoped)
}

/// Collects HTML attributes shared across adapters.
///
/// The helper funnels the caller supplied class list through
/// [`rustic_ui_utils::collect_attributes`] before layering optional ARIA metadata on
/// top. Returning a `Vec` keeps the structure ergonomic for
/// [`attributes_to_html`], SSR renderers and potential future automation.
fn attribute_pairs(props: &ThemedProps, classes: String) -> Vec<(String, String)> {
    let mut attrs = collect_attributes(Some(classes), core::iter::empty::<(String, String)>());
    extend_attributes(
        &mut attrs,
        [
            (String::from("type"), String::from("text")),
            (String::from("value"), props.value.clone()),
        ],
    );
    if let Some(placeholder) = &props.placeholder {
        extend_attributes(
            &mut attrs,
            [(String::from("placeholder"), placeholder.clone())],
        );
    }
    if let Some(label) = &props.aria_label {
        extend_attributes(&mut attrs, [(String::from("aria-label"), label.clone())]);
    }
    if let Some(debounce) = props.debounce_ms {
        extend_attributes(
            &mut attrs,
            [(String::from("data-debounce-ms"), debounce.to_string())],
        );
    }
    attrs
}

/// Renders the final `<input>` markup, optionally prefixing an inline
/// `<style>` tag for SSR scenarios.
fn render_input(props: &ThemedProps, classes: String, stylesheet: Option<String>) -> String {
    let attrs = attribute_pairs(props, classes);
    let attr_string = attributes_to_html(&attrs);
    // Compose the markup manually so adapters can run in headless test
    // environments without pulling a virtual DOM dependency.
    let markup = format!("<input {attrs} />", attrs = attr_string);
    if let Some(css) = stylesheet {
        format!("<style>{}</style>{}", css, markup)
    } else {
        markup
    }
}

#[cfg(feature = "leptos")]
pub mod leptos {
    //! Leptos adapter that renders a themed `<input>` while exercising the
    //! shared styling helpers.
    //!
    //! The adapter leans on [`css_with_theme!`](rustic_ui_styled_engine::css_with_theme)
    //! to derive palette driven spacing, typography and focus outlines ensuring
    //! the scoped class aligns with the active [`Theme`](crate::theme::Theme).
    //! By delegating attribute assembly to [`attribute_pairs`](super::attribute_pairs)
    //! we guarantee ARIA labels, placeholders and debounce metadata surface in a
    //! predictable order across frameworks – a massive win when validating
    //! accessibility in large enterprises.
    use super::*;

    /// Render a themed `<input>` with ARIA metadata using Leptos.
    pub fn render(props: &ThemedProps) -> String {
        let (classes, scoped) = themed_classes(props);
        // Feed the generated stylesheet back into the markup so SSR output and
        // client side hydration share the exact same CSS payload. The helpers
        // emit an `<input>` so front-end integrations can hydrate directly onto
        // the rendered string.
        render_input(props, classes, Some(scoped.stylesheet))
    }
}

#[cfg(feature = "dioxus")]
pub mod dioxus {
    //! Dioxus adapter that renders the themed input markup as a plain string.
    //!
    //! Styling is pulled from [`resolve_visual_tokens`](super::resolve_visual_tokens)
    //! which guarantees parity with the Leptos and Sycamore implementations.
    //! The generated CSS class is merged with `aria-label` metadata and optional
    //! debounce data so server rendered strings and virtual DOM components share
    //! the same accessibility contract – a vital property for pre-production QA.
    use super::*;

    /// Render a themed `<input>` with ARIA metadata using Dioxus.
    pub fn render(props: &ThemedProps) -> String {
        // Share the same scoped stylesheet as the other adapters so string based
        // renderers remain perfectly in sync with client side components.
        let (classes, scoped) = themed_classes(props);
        render_input(props, classes, Some(scoped.stylesheet))
    }
}

#[cfg(feature = "sycamore")]
pub mod sycamore {
    //! Sycamore adapter that outputs a semantic text input string.
    //!
    //! The implementation mirrors the Dioxus adapter so future tweaks to token
    //! resolution or ARIA defaults automatically cascade across both Virtual DOM
    //! ecosystems. This keeps enterprise teams from writing bespoke wrappers in
    //! each project and instead centralises the behaviour here.
    use super::*;

    /// Render a themed `<input>` with ARIA metadata using Sycamore.
    pub fn render(props: &ThemedProps) -> String {
        let (classes, scoped) = themed_classes(props);
        render_input(props, classes, Some(scoped.stylesheet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_props() -> ThemedProps {
        ThemedProps {
            value: String::from("hello"),
            placeholder: Some(String::from("Type here")),
            ..Default::default()
        }
    }

    #[test]
    fn deterministic_class_uses_bem_modifier() {
        assert_eq!(
            deterministic_class(Variant::Plain),
            format!("{BASE_CLASS} {BASE_CLASS}--plain")
        );
        assert_eq!(
            deterministic_class(Variant::Outlined),
            format!("{BASE_CLASS} {BASE_CLASS}--outlined")
        );
    }

    #[test]
    fn renders_input_markup_with_value_and_placeholder() {
        let html = render_input(&base_props(), String::from("mui"), None);
        assert!(html.starts_with("<input"));
        assert!(html.contains("class=\"mui"));
        assert!(html.contains("value=\"hello\""));
        assert!(html.contains("placeholder=\"Type here\""));
    }

    #[test]
    fn debounce_attribute_is_rendered() {
        let mut props = base_props();
        props.debounce_ms = Some(300);
        let html = render_input(&props, String::from("mui"), None);
        assert!(html.contains("data-debounce-ms=\"300\""));
    }

    #[test]
    fn aria_label_is_propagated() {
        let mut props = base_props();
        props.aria_label = Some(String::from("Search input"));
        let html = render_input(&props, String::from("mui"), None);
        assert!(html.contains("aria-label=\"Search input\""));
    }
}
