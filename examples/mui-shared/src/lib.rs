//! Shared Material UI example primitives for RusticUI multi-framework demos.
//!
//! # Overview
//!
//! The archived Next.js reference implementation shipped with a small amount
//! of global state: a dual light/dark theme configuration, a home/about route
//! pair, and a reusable shell composed of a container, a hero heading, and the
//! "ProTip" helper.  Each active RusticUI example (Leptos, Yew, Dioxus and
//! Sycamore) previously reimplemented those details which made consistency
//! checks, accessibility audits, and automation hooks difficult to maintain.
//!
//! This crate centralises that knowledge into strongly typed primitives that
//! can be reused across server-side rendered (SSR) and client-side rendered
//! (CSR) entry points.  Everything is deterministic, serialisable, and heavily
//! documented so enterprise consumers can wire the primitives into existing
//! deployment pipelines with confidence.
//!
//! * [`theme`] exposes a serialisable representation of the Material theme and
//!   companion helpers for bridging to [`rustic_ui_system::theme::Theme`].
//! * [`routes`] provides typed descriptors for the home (`/`) and about
//!   (`/about`) pages including automation identifiers and localisation friendly
//!   metadata.
//! * [`layout`] supplies hydration-safe layout copy (headings, buttons, and the
//!   ProTip block) with ergonomic automation hooks for Playwright/Cypress
//!   suites.
//! * [`automation`] implements deterministic `data-rustic-*` attribute builders
//!   inspired by the historical select menu helpers but generalised for the
//!   Material demos.
//!
//! # SSR bootstrapping
//!
//! The [`layout::AppShell`] type encapsulates everything required to build an
//! SSR friendly HTML shell.  Pair it with [`theme::material_example_theme`] to
//! produce stable automation identifiers and pre-hydration markup:
//!
//! ```no_run
//! use mui_shared::{layout::AppShell, routes::HOME, theme::material_example_theme};
//!
//! # fn render_html() -> String {
//! let theme = material_example_theme();
//! let shell = AppShell::for_route(&HOME);
//! let ssr_html = shell.render_ssr_document(|content| {
//!     format!("<div id=\"app\">{content}</div>")
//! }, &theme);
//! # ssr_html
//! # }
//! ```
//!
//! The helper returns a complete HTML string with `<main>` content annotated by
//! the automation IDs exposed via [`automation::AutomationIdBuilder`].  Because
//! every identifier is deterministic and based purely on route metadata, the
//! resulting DOM stays stable between server and client renders, ensuring zero
//! hydration warnings.
//!
//! # CSR hydration
//!
//! Client-side entry points typically mount into `#app`.  The [`layout::AppShell`]
//! struct surfaces the same copy and automation metadata so each framework can
//! hydrate using its idiomatic primitives while reusing the same identifiers:
//!
//! ```ignore
//! // Framework specific pseudo-code (Leptos shown) illustrating hydration.
//! use leptos::*;
//! use mui_shared::{layout::AppShell, routes::HOME, theme::material_example_theme};
//!
//! fn client_mount() {
//!     let shell = AppShell::for_route(&HOME);
//!     let theme = material_example_theme();
//!     leptos::mount_to_body(move || {
//!         let headline = shell.headline();
//!         view! { <h1 data-rustic-app-headline={shell.automation().value()}> {headline} </h1> }
//!     });
//! }
//! ```
//!
//! The snippet intentionally focuses on automation usage instead of framework
//! specifics.  `shell.automation()` can be reused across Yew, Dioxus, and
//! Sycamore to drive consistent `data-rustic-*` attributes.
//!
//! # Automation hooks
//!
//! [`automation::AutomationIdBuilder`] mirrors the ergonomics of the historical
//! select menu shared crate.  It sanitises arbitrary user-provided segments into
//! predictable kebab-case identifiers and formats `data-rustic-*` attributes
//! using the component namespace (here `app`).  Extensive unit tests cover the
//! sanitisation rules so QA teams can rely on these markers for synthetic test
//! orchestration.
//!
//! # Cross-framework usage examples
//!
//! The integration tests located under `examples/mui-shared/tests` demonstrate
//! how each framework consumes [`AppShell`], [`HOME`], and [`ABOUT`].  The tests
//! intentionally avoid real rendering to keep CI fast while still guaranteeing
//! API stability for automation tooling.

#[cfg(feature = "theme")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "theme")]
use std::borrow::Cow;

#[cfg(feature = "theme")]
use rustic_ui_system::theme::{ColorScheme, PaletteScheme, Theme};

/// Utilities for deterministic automation identifiers.
#[cfg(feature = "automation")]
pub mod automation {
    use super::sanitise_segment;

    /// Namespace used for all automation data attributes emitted by this crate.
    const COMPONENT_PREFIX: &str = "app";

    /// Builder that assembles deterministic automation identifiers.
    ///
    /// The builder retains all segments as owned strings to avoid lifetime
    /// headaches and guarantees that every identifier is safe to embed inside
    /// HTML attributes.  Identifiers always start with the component prefix
    /// (`"app"`) followed by the caller supplied base id and any child
    /// segments.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AutomationIdBuilder {
        segments: Vec<String>,
    }

    impl AutomationIdBuilder {
        /// Creates a new [`AutomationIdBuilder`] using the provided base id.
        ///
        /// Empty or whitespace-only identifiers fall back to `"app"` so that
        /// invalid consumer input never leaks into the DOM.  Segments are
        /// sanitised immediately to ensure stable ordering during SSR/CSR
        /// passes.
        pub fn new(base_id: impl AsRef<str>) -> Self {
            let mut segments = Vec::with_capacity(4);
            segments.push(COMPONENT_PREFIX.to_string());
            let base = sanitise_segment(base_id.as_ref());
            if !base.is_empty() {
                segments.push(base);
            }
            Self { segments }
        }

        /// Appends a child segment and returns a new builder.
        pub fn child(&self, segment: impl AsRef<str>) -> Self {
            let mut next = self.clone();
            let sanitised = sanitise_segment(segment.as_ref());
            if !sanitised.is_empty() {
                next.segments.push(sanitised);
            }
            next
        }

        /// Returns the final automation identifier value (e.g. `"app-home"`).
        pub fn value(&self) -> String {
            self.segments.join("-")
        }

        /// Formats a `data-rustic-*` attribute pair for embedding in markup.
        ///
        /// The attribute name is derived from the component namespace and the
        /// provided role.  For example, calling `attribute("headline")`
        /// returns `("data-rustic-app-headline", "app-home-headline")` when
        /// invoked on the home route's builder.
        pub fn attribute(&self, role: impl AsRef<str>) -> (String, String) {
            let sanitised_role = sanitise_segment(role.as_ref());
            let attr_name = if sanitised_role.is_empty() {
                "data-rustic-app".to_string()
            } else {
                format!("data-rustic-{COMPONENT_PREFIX}-{sanitised_role}")
            };
            let attr_value = self.child(role).value();
            (attr_name, attr_value)
        }

        /// Convenience helper that renders the full `key="value"` pair.
        pub fn attribute_pair(&self, role: impl AsRef<str>) -> String {
            let (key, value) = self.attribute(role);
            format!("{key}=\"{value}\"")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::AutomationIdBuilder;

        #[test]
        fn builder_sanitises_segments() {
            let id = AutomationIdBuilder::new(" Home 🚀 ");
            assert_eq!(id.value(), "app-home");
        }

        #[test]
        fn attribute_formats_key_and_value() {
            let id = AutomationIdBuilder::new("home");
            let (key, value) = id.attribute("Headline Section");
            assert_eq!(key, "data-rustic-app-headline-section");
            assert_eq!(value, "app-home-headline-section");
        }

        #[test]
        fn empty_segments_collapse() {
            let id = AutomationIdBuilder::new("   ");
            assert_eq!(id.value(), "app");
            let (key, value) = id.attribute("");
            assert_eq!(key, "data-rustic-app");
            assert_eq!(value, "app");
        }
    }
}

/// Route descriptors for the demo.
#[cfg(feature = "routes")]
pub mod routes {
    use super::automation::AutomationIdBuilder;

    /// Static descriptor for the home route (`/`).
    pub static HOME: RouteDescriptor = RouteDescriptor {
        path: "/",
        title: "Material UI - RusticUI",
        headline: "Material UI - Multi-framework RusticUI demo",
        body_copy: "Experience the shared layout rendered by Leptos, Yew, Dioxus, and Sycamore without duplicating logic.",
        primary_action: Some(RouteAction {
            label: "Visit the about page",
            href: "/about",
            automation_role: "primary-action",
        }),
        secondary_action: None,
        pro_tip: ProTipCopy {
            lead_in: "Pro tip: See more",
            link_href: "https://mui.com/material-ui/getting-started/templates/",
            link_label: "templates",
            tail_text: "in the Material UI documentation.",
        },
        automation_base: "home",
    };

    /// Static descriptor for the about route (`/about`).
    pub static ABOUT: RouteDescriptor = RouteDescriptor {
        path: "/about",
        title: "About - RusticUI",
        headline: "Material UI - About this RusticUI demo",
        body_copy: "This page demonstrates deterministic SSR and CSR hydration across frameworks.",
        primary_action: Some(RouteAction {
            label: "Return to home",
            href: "/",
            automation_role: "primary-action",
        }),
        secondary_action: Some(RouteAction {
            label: "Explore RusticUI on GitHub",
            href: "https://github.com/RusticUI/rusticui",
            automation_role: "secondary-action",
        }),
        pro_tip: ProTipCopy {
            lead_in: "Need more patterns?",
            link_href: "https://mui.com/store/#popular-items",
            link_label: "Browse production-grade templates",
            tail_text: "curated by the MUI team.",
        },
        automation_base: "about",
    };

    /// Returns a deterministic automation builder for the descriptor.
    pub fn automation(route: &RouteDescriptor) -> AutomationIdBuilder {
        AutomationIdBuilder::new(route.automation_base)
    }

    /// Strongly typed shell actions.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RouteAction {
        /// Visible label rendered inside the CTA button/link.
        pub label: &'static str,
        /// Target href used across frameworks.
        pub href: &'static str,
        /// Logical role used when generating automation identifiers.
        pub automation_role: &'static str,
    }

    /// Copy block for the ProTip component.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ProTipCopy {
        pub lead_in: &'static str,
        pub link_href: &'static str,
        pub link_label: &'static str,
        pub tail_text: &'static str,
    }

    /// Shared descriptor for a route.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RouteDescriptor {
        pub path: &'static str,
        pub title: &'static str,
        pub headline: &'static str,
        pub body_copy: &'static str,
        pub primary_action: Option<RouteAction>,
        pub secondary_action: Option<RouteAction>,
        pub pro_tip: ProTipCopy,
        pub automation_base: &'static str,
    }
}

/// Layout primitives that mirror the archived Container + ProTip shell.
#[cfg(feature = "layout")]
pub mod layout {
    use super::{automation::AutomationIdBuilder, routes, theme::MaterialExampleTheme};
    use routes::RouteDescriptor;

    /// Hydration-safe layout definition reused across frameworks.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AppShell<'a> {
        route: &'a RouteDescriptor,
        max_width: &'static str,
    }

    impl<'a> AppShell<'a> {
        /// Creates a new [`AppShell`] for the given route descriptor.
        pub fn for_route(route: &'a RouteDescriptor) -> Self {
            Self {
                route,
                max_width: "lg",
            }
        }

        /// Returns the automation builder attached to this route.
        pub fn automation(&self) -> AutomationIdBuilder {
            routes::automation(self.route)
        }

        /// Headline copy reused across SSR and CSR renders.
        pub fn headline(&self) -> &'static str {
            self.route.headline
        }

        /// Supporting body copy for the shell.
        pub fn body_copy(&self) -> &'static str {
            self.route.body_copy
        }

        /// Primary call-to-action for the shell (if present).
        pub fn primary_action(&self) -> Option<routes::RouteAction> {
            self.route.primary_action.clone()
        }

        /// Secondary call-to-action for the shell (if present).
        pub fn secondary_action(&self) -> Option<routes::RouteAction> {
            self.route.secondary_action.clone()
        }

        /// Returns the ProTip copy block.
        pub fn pro_tip(&self) -> routes::ProTipCopy {
            self.route.pro_tip.clone()
        }

        /// Returns the container max width token (mirrors Material `Container`).
        pub fn max_width(&self) -> &'static str {
            self.max_width
        }

        /// Renders a minimal SSR document containing the layout skeleton.
        pub fn render_ssr_document<F>(&self, body: F, theme: &MaterialExampleTheme) -> String
        where
            F: FnOnce(String) -> String,
        {
            let automation = self.automation();
            let (headline_key, headline_value) = automation.attribute("headline");
            let (body_key, body_value) = automation.attribute("body");
            let (shell_key, shell_value) = automation.attribute("shell");
            let pro_tip = self.pro_tip();
            let content = format!(
                "<div {headline_key}=\"{headline_value}\"><h1>{}</h1><p {body_key}=\"{body_value}\">{}</p></div>",
                html_escape::encode_text(self.headline()),
                html_escape::encode_text(self.body_copy())
            );
            let inner = body(content);
            let (main_key, main_value) = automation.attribute("main");
            format!(
                "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"/><title>{}</title></head><body data-theme=\"{}\" {}=\"{}\"><main {main_key}=\"{}\">{}<footer data-pro-tip-lead=\"{}\"><a href=\"{}\">{}</a> {}</footer></main></body></html>",
                html_escape::encode_text(self.route.title),
                theme.active_color_scheme().as_str(),
                shell_key,
                shell_value,
                main_value,
                inner,
                html_escape::encode_text(pro_tip.lead_in),
                html_escape::encode_double_quoted_attribute(pro_tip.link_href),
                html_escape::encode_text(pro_tip.link_label),
                html_escape::encode_text(pro_tip.tail_text)
            )
        }
    }

    /// Framework identifier used inside documentation examples and automation ids.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Framework {
        Leptos,
        Yew,
        Dioxus,
        Sycamore,
    }

    impl Framework {
        /// Returns a kebab-case identifier for automation usage.
        pub fn id(&self) -> &'static str {
            match self {
                Framework::Leptos => "leptos",
                Framework::Yew => "yew",
                Framework::Dioxus => "dioxus",
                Framework::Sycamore => "sycamore",
            }
        }
    }

    /// Returns an automation builder scoped to a specific framework.
    pub fn automation_for_framework(
        route: &RouteDescriptor,
        framework: Framework,
    ) -> AutomationIdBuilder {
        routes::automation(route).child(format!("framework-{}", framework.id()))
    }

    /// Generates a deterministic hydration marker for a given framework.
    pub fn hydration_marker(route: &RouteDescriptor, framework: Framework) -> String {
        let builder = automation_for_framework(route, framework);
        let (key, value) = builder.attribute("hydration-root");
        format!("{key}=\"{value}\"")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::routes::{ABOUT, HOME};

        #[test]
        fn automation_extends_with_framework() {
            let marker = hydration_marker(&HOME, Framework::Leptos);
            assert_eq!(
                marker,
                "data-rustic-app-hydration-root=\"app-home-framework-leptos-hydration-root\""
            );
        }

        #[test]
        fn ssr_document_encodes_html() {
            let theme = crate::theme::material_example_theme();
            let shell = AppShell::for_route(&ABOUT);
            let html = shell.render_ssr_document(|content| content, &theme);
            assert!(html.contains("Material UI - About this RusticUI demo"));
            assert!(html.contains("data-pro-tip-lead=\"Need more patterns?\""));
        }
    }
}

/// Theme utilities bridging the archived TypeScript configuration with Rust.
#[cfg(feature = "theme")]
pub mod theme {
    use super::*;

    /// Serialisable representation of the dual light/dark availability flag.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct ColorSchemeAvailability {
        pub light: bool,
        pub dark: bool,
    }

    impl Default for ColorSchemeAvailability {
        fn default() -> Self {
            Self {
                light: true,
                dark: true,
            }
        }
    }

    /// CSS variable configuration mirroring `cssVariables.colorSchemeSelector`.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct CssVariablesConfig {
        pub color_scheme_selector: Cow<'static, str>,
    }

    impl Default for CssVariablesConfig {
        fn default() -> Self {
            Self {
                color_scheme_selector: Cow::Borrowed("class"),
            }
        }
    }

    /// Typography overrides derived from the Roboto Next.js font helper.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct TypographyConfig {
        pub font_family: Cow<'static, str>,
    }

    impl Default for TypographyConfig {
        fn default() -> Self {
            Self {
                font_family: Cow::Borrowed("'Roboto', 'Helvetica', 'Arial', sans-serif"),
            }
        }
    }

    /// MuiAlert severity overrides.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct AlertSeverityOverrides {
        pub info_background: Cow<'static, str>,
    }

    impl Default for AlertSeverityOverrides {
        fn default() -> Self {
            Self {
                info_background: Cow::Borrowed("#60a5fa"),
            }
        }
    }

    /// Collection of component overrides for the demo.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct ComponentOverrides {
        pub alert: AlertSeverityOverrides,
    }

    impl Default for ComponentOverrides {
        fn default() -> Self {
            Self {
                alert: AlertSeverityOverrides::default(),
            }
        }
    }

    /// Aggregate theme structure.
    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct MaterialExampleTheme {
        pub color_schemes: ColorSchemeAvailability,
        pub css_variables: CssVariablesConfig,
        pub typography: TypographyConfig,
        pub components: ComponentOverrides,
        pub system_theme: Theme,
    }

    impl MaterialExampleTheme {
        /// Returns the active [`ColorScheme`] derived from the system theme.
        pub fn active_color_scheme(&self) -> ColorScheme {
            self.system_theme.palette.initial_color_scheme
        }

        /// Returns the palette scheme for a given color mode.
        pub fn palette_scheme(&self, scheme: ColorScheme) -> &PaletteScheme {
            self.system_theme.palette.scheme(scheme)
        }

        /// Exposes a mutable reference to the inner system theme for advanced overrides.
        pub fn system_theme_mut(&mut self) -> &mut Theme {
            &mut self.system_theme
        }
    }

    /// Constructs the material example theme, aligning with the archived TS config.
    pub fn material_example_theme() -> MaterialExampleTheme {
        let mut system_theme = Theme::default();
        // Mirror the archived typography override.
        system_theme.typography.font_family =
            "'Roboto', 'Helvetica', 'Arial', sans-serif".to_string();

        // Provide a slightly richer dark palette to match the alert background tone.
        let mut dark = system_theme.palette.dark.clone();
        dark.info = "#60a5fa".to_string();
        system_theme.palette.dark = dark;
        system_theme.palette.initial_color_scheme = ColorScheme::Light;

        MaterialExampleTheme {
            color_schemes: ColorSchemeAvailability::default(),
            css_variables: CssVariablesConfig::default(),
            typography: TypographyConfig::default(),
            components: ComponentOverrides::default(),
            system_theme,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn theme_overrides_match_archive() {
            let theme = material_example_theme();
            assert_eq!(theme.css_variables.color_scheme_selector, "class");
            assert_eq!(
                theme.typography.font_family,
                "'Roboto', 'Helvetica', 'Arial', sans-serif"
            );
            assert_eq!(theme.components.alert.info_background, "#60a5fa");
        }
    }
}

#[cfg(feature = "automation")]
fn sanitise_segment(input: &str) -> String {
    let mut output = String::new();
    let mut prev_dash = false;
    for ch in input.chars() {
        let mapped = match ch {
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' => Some(ch),
            '-' | '_' | ' ' | ':' | '.' | '/' => None,
            _ => None,
        };
        if let Some(valid) = mapped {
            output.push(valid);
            prev_dash = false;
        } else if !prev_dash {
            output.push('-');
            prev_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

#[cfg(all(test, feature = "automation"))]
mod tests {
    use super::automation::AutomationIdBuilder;

    #[test]
    fn sanitise_handles_unicode() {
        let builder = AutomationIdBuilder::new("Über 🚀 Beta");
        assert_eq!(builder.value(), "app-ber-beta");
    }
}
