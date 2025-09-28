//! Enterprise ready Material UI showcase rendered with Dioxus.
//!
//! This example mirrors the Leptos/Yew integrations by consuming the shared
//! [`mui_shared`] crate for all theme, routing, and automation metadata. The
//! goal is to keep SSR output identical between frameworks so hydration remains
//! deterministic for CI and large scale automation harnesses.

use std::rc::Rc;

use dioxus::events::{FormEvent, MouseEvent};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use mui_shared::{
    automation::AutomationIdBuilder,
    layout::{self, AppShell, Framework},
    routes::{RouteDescriptor, ABOUT, HOME},
    theme::{material_example_theme, ColorSchemeAvailability, MaterialExampleTheme},
};
use rustic_ui_system::theme::ColorScheme;

/// Typed routes shared between the Dioxus router and automation harnesses.
#[derive(Clone, Debug, PartialEq, Eq, Routable)]
enum AppRoute {
    /// Landing page that mirrors the archived Next.js hero section.
    #[route("/")]
    Home {},
    /// Secondary information page summarising the example goals.
    #[route("/about")]
    About {},
    /// Fallback used when the router encounters an unknown path.
    #[route("/:.._segments")]
    NotFound { _segments: Vec<String> },
}

impl AppRoute {
    /// Returns the shared descriptor supplied by `mui-shared`.
    fn descriptor(&self) -> &'static RouteDescriptor {
        match self {
            Self::Home {} | Self::NotFound { .. } => &HOME,
            Self::About {} => &ABOUT,
        }
    }

    /// Navigation label rendered in the shared header.
    fn nav_label(&self) -> &'static str {
        match self {
            Self::Home {} | Self::NotFound { .. } => "Home",
            Self::About {} => "About",
        }
    }

    /// Automation namespace used to highlight the active navigation item.
    fn nav_key(&self) -> &'static str {
        self.descriptor().automation_base
    }
}

/// Hydration lifecycle used by the theme mode switch state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HydrationPhase {
    /// Server rendered markup prior to any browser APIs being available.
    Server,
    /// Client side pass after hydration completes and DOM APIs are safe to use.
    Client,
}

/// Developer facing representation of the user's colour preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModePreference {
    /// Mirrors the archived "System" option – use the OS/browser preference.
    System,
    /// Explicit user override forcing a specific colour scheme.
    Explicit(ColorScheme),
}

impl ModePreference {
    /// Value attribute used by the `<select>` element.
    fn select_value(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Explicit(ColorScheme::Light) => "light",
            Self::Explicit(ColorScheme::Dark) => "dark",
        }
    }
}

/// Reducer-friendly container tracking hydration phase and effective theme mode.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ModeState {
    phase: HydrationPhase,
    preference: ModePreference,
    system_scheme: ColorScheme,
}

impl Default for ModeState {
    fn default() -> Self {
        Self {
            phase: HydrationPhase::Server,
            // The server cannot observe the client's preference so we start with the
            // archival "system" option. This resolves to light mode until hydration
            // promotes the machine to `HydrationPhase::Client`.
            preference: ModePreference::System,
            system_scheme: ColorScheme::Light,
        }
    }
}

impl ModeState {
    /// Applies a state transition in response to UI interactions or hydration.
    fn transition(&self, action: ModeAction) -> Self {
        match action {
            ModeAction::Hydrated { system_scheme } => {
                let mut next = self.clone();
                next.phase = HydrationPhase::Client;
                next.system_scheme = system_scheme;
                next
            }
            ModeAction::Select(preference) => {
                let mut next = self.clone();
                next.preference = preference;
                next
            }
        }
    }

    /// Effective colour scheme applied to automation and theme data attributes.
    fn effective_scheme(&self) -> ColorScheme {
        match self.preference {
            ModePreference::System => self.system_scheme,
            ModePreference::Explicit(mode) => mode,
        }
    }
}

/// Actions emitted by the mode switch state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModeAction {
    /// Hydration completed and the browser preference has been captured.
    Hydrated { system_scheme: ColorScheme },
    /// User selected an explicit option from the `<select>` widget.
    Select(ModePreference),
}

/// Shared handle used to dispatch `ModeAction`s from deeply nested components.
type ModeDispatch = Rc<dyn Fn(ModeAction)>;

/// Context wrapper supplying the shared `ModeDispatch` handle.
#[derive(Clone)]
struct ModeController {
    dispatch: ModeDispatch,
}

/// Root application component wiring the router, shared theme blueprint, and
/// automation identifiers provided by `mui-shared`.
#[component]
fn App() -> Element {
    // Provide the shared Material theme blueprint and mode state so every route
    // can tap into deterministic SSR data and hydration aware state.
    use_shared_state_provider(cx, material_example_theme);
    use_shared_state_provider(cx, ModeState::default);

    let mode_state = use_shared_state::<ModeState>(cx).expect("mode state context");
    let controller = {
        let mode_state = mode_state.clone();
        ModeController {
            dispatch: Rc::new(move |action| {
                mode_state.with_mut(|state| {
                    let current = state.clone();
                    let next = current.transition(action);
                    if next != current {
                        *state = next;
                    }
                });
            }),
        }
    };
    use_context_provider(cx, || controller);

    cx.render(rsx! {
        Router::<AppRoute> {}
    })
}

/// Renders the shared shell for the provided `AppRoute`.
fn render_route(cx: Scope, route: AppRoute) -> Element {
    let theme_blueprint = use_shared_state::<MaterialExampleTheme>(cx)
        .expect("theme blueprint should be provided");
    let mode_state = use_shared_state::<ModeState>(cx).expect("mode state should exist");
    let controller = use_context::<ModeController>(cx).expect("mode controller should exist");

    let descriptor = route.descriptor();
    let shell = AppShell::for_route(descriptor);
    let automation = shell.automation();
    let framework_builder = layout::automation_for_framework(descriptor, Framework::Dioxus);

    let shell_attr = automation.child("shell").value();
    let hydration_attr = framework_builder.child("hydration-root").value();
    let header_attr = automation.child("header").value();
    let nav_attr = automation.child("navigation").value();
    let main_attr = automation.child("main").value();
    let body_attr = automation.child("body").value();
    let headline_attr = automation.child("headline").value();
    let actions_attr = automation.child("actions").value();
    let pro_tip_attr = automation.child("pro-tip").value();
    let showcases_automation = automation.child("showcases");
    let showcases_attr = showcases_automation.value();
    let slider_attr = showcases_automation.child("slider").value();
    let slider_value_attr = showcases_automation.child("slider").child("value").value();
    let popover_attr = showcases_automation.child("popover").value();
    let popover_surface_attr = showcases_automation.child("popover").child("surface").value();
    let popover_trigger_attr = showcases_automation.child("popover").child("trigger").value();
    let mode_switch_automation = automation.child("mode-switch");

    let availability = theme_blueprint.read().color_schemes.clone();
    let alert_background = theme_blueprint
        .read()
        .components
        .alert
        .info_background
        .clone();
    let active_scheme = mode_state.read().effective_scheme();

    let slider_value = use_state(cx, || 42.0_f64);
    let on_slider_input = {
        let slider_value = slider_value.clone();
        move |event: FormEvent| {
            if let Ok(parsed) = event.value.parse::<f64>() {
                slider_value.set(parsed);
            }
        }
    };
    let slider_snapshot = slider_value.current();
    let slider_numeric = *slider_snapshot;
    let slider_input_value = format!("{slider_numeric:.0}");
    let slider_output = format!("{slider_numeric:.0}% allocated");

    let popover_open = use_state(cx, || false);
    let toggle_popover = {
        let popover_open = popover_open.clone();
        move |_: MouseEvent| {
            let current = *popover_open.current();
            popover_open.set(!current);
        }
    };
    let popover_is_open = *popover_open.current();
    let popover_expanded = if popover_is_open { "true" } else { "false" };
    let popover_label = if popover_is_open {
        "Close popover"
    } else {
        "Open popover"
    };

    let nav_items = [AppRoute::Home {}, AppRoute::About {}];

    cx.render(rsx! {
        div {
            id: "app",
            class: "app-shell",
            "data-rustic-app-shell": "{shell_attr}",
            "data-rustic-app-hydration-root": "{hydration_attr}",
            "data-rustic-app-theme": "{active_scheme.as_str()}",
            header {
                class: "app-header",
                "data-rustic-app-header": "{header_attr}",
                nav {
                    class: "app-nav",
                    "data-rustic-app-navigation": "{nav_attr}",
                    ul {
                        class: "nav-list",
                        nav_items.iter().map(|target| {
                            let descriptor = target.descriptor();
                            let link_attr = automation
                                .child("navigation")
                                .child(descriptor.automation_base)
                                .value();
                            let is_active = route.nav_key() == descriptor.automation_base;
                            rsx! {
                                li {
                                    class: "nav-item",
                                    "data-rustic-app-navigation": "{link_attr}",
                                    Link {
                                        to: target.clone(),
                                        class: if is_active { "nav-link active" } else { "nav-link" },
                                        target.nav_label()
                                    }
                                }
                            }
                        })
                    }
                }
                ModeSwitch {
                    automation: mode_switch_automation,
                    availability,
                }
            }
            main {
                class: "app-main",
                "data-rustic-app-main": "{main_attr}",
                section {
                    class: "hero",
                    h1 {
                        "data-rustic-app-headline": "{headline_attr}",
                        shell.headline()
                    }
                    p {
                        "data-rustic-app-body": "{body_attr}",
                        shell.body_copy()
                    }
                    div {
                        class: "cta-container",
                        "data-rustic-app-actions": "{actions_attr}",
                        shell.primary_action().map(|action| {
                            let attr = automation
                                .child("actions")
                                .child(action.automation_role)
                                .value();
                            rsx! {
                                a {
                                    class: "cta primary",
                                    "data-rustic-app-action": "{attr}",
                                    href: "{action.href}",
                                    action.label
                                }
                            }
                        })
                        shell.secondary_action().map(|action| {
                            let attr = automation
                                .child("actions")
                                .child(action.automation_role)
                                .value();
                            rsx! {
                                a {
                                    class: "cta secondary",
                                    "data-rustic-app-action": "{attr}",
                                    href: "{action.href}",
                                    action.label
                                }
                            }
                        })
                    }
                }
                section {
                    class: "showcases",
                    "data-rustic-app-showcases": "{showcases_attr}",
                    article {
                        class: "showcase-card alert",
                        "data-rustic-app-showcase-alert": "{automation.child("showcases").child("alert").value()}",
                        h2 { "Alert customisation" }
                        p {
                            "The shared theme injects a deterministic info background colour ("
                            "{alert_background}" ") so automation can assert visual parity across frameworks."
                        }
                        div {
                            class: "alert-demo",
                            role: "alert",
                            style: "background:{alert_background};color:#0f172a;padding:12px;border-radius:8px;",
                            strong { "Heads up" }
                            span { ": deployment pipelines share alert theming via `mui-shared`." }
                        }
                    }
                    article {
                        class: "showcase-card slider",
                        "data-rustic-app-showcase-slider": "{slider_attr}",
                        h2 { "Deterministic slider" }
                        p {
                            "Slider telemetry feeds into enterprise analytics. The signal based handler keeps"
                            " SSR output stable while hydrating client events."
                        }
                        label {
                            class: "slider-label",
                            r#for: "showcase-capacity",
                            "Deployment capacity"
                        }
                        input {
                            id: "showcase-capacity",
                            class: "slider-input",
                            r#type: "range",
                            min: "0",
                            max: "100",
                            step: "1",
                            value: "{slider_input_value}",
                            oninput: on_slider_input,
                            "data-rustic-app-showcase-slider-control": "{slider_attr}",
                            "aria-valuemin": "0",
                            "aria-valuemax": "100",
                            "aria-valuenow": "{slider_input_value}",
                            "aria-labelledby": "showcase-capacity",
                        }
                        output {
                            "data-rustic-app-showcase-slider-value": "{slider_value_attr}",
                            "{slider_output}"
                        }
                    }
                    article {
                        class: "showcase-card popover",
                        "data-rustic-app-showcase-popover": "{popover_attr}",
                        h2 { "Popover orchestration" }
                        p {
                            "The popover surface toggles deterministically so SSR and CSR agree on collision"
                            " metadata during analytics captures."
                        }
                        button {
                            class: "popover-trigger",
                            "data-rustic-app-showcase-popover-trigger": "{popover_trigger_attr}",
                            onclick: toggle_popover,
                            "aria-expanded": "{popover_expanded}",
                            "aria-controls": "popover-surface",
                            "{popover_label}"
                        }
                        if popover_is_open {
                            rsx! {
                                div {
                                    id: "popover-surface",
                                    role: "dialog",
                                    class: "popover-surface",
                                    "data-rustic-app-showcase-popover-surface": "{popover_surface_attr}",
                                    p { "Popover is open. Collision logic mirrors SSR output." }
                                }
                            }
                        }
                    }
                }
                footer {
                    class: "pro-tip",
                    "data-rustic-app-pro-tip": "{pro_tip_attr}",
                    let pro_tip = shell.pro_tip();
                    strong { pro_tip.lead_in }
                    span { " " }
                    a { href: "{pro_tip.link_href}", pro_tip.link_label }
                    span { format!(" {}", pro_tip.tail_text) }
                }
            }
        }
    })
}

#[component]
fn Home() -> Element {
    render_route(cx, AppRoute::Home {})
}

#[component]
fn About() -> Element {
    render_route(cx, AppRoute::About {})
}

#[component]
fn NotFound(_segments: Vec<String>) -> Element {
    render_route(cx, AppRoute::NotFound { _segments })
}

/// Mode switcher ported from the archival React demo with explicit SSR notes.
///
/// The reducer state forms a two phase machine:
/// * `HydrationPhase::Server` renders static markup. Events are inert and the
///   selected option defaults to `ModePreference::System` so SSR stays
///   deterministic.
/// * Once hydrated the effect dispatches `ModeAction::Hydrated`, transitioning to
///   `HydrationPhase::Client` and capturing the browser preference via
///   `matchMedia`. From that point the select behaves like the archived client
///   component and dispatches `ModeAction::Select` for automation to observe.
#[component]
fn ModeSwitch(automation: AutomationIdBuilder, availability: ColorSchemeAvailability) -> Element {
    let mode_state = use_shared_state::<ModeState>(cx).expect("mode state should exist");
    let controller = use_context::<ModeController>(cx).expect("mode controller should exist");
    let container_attr = automation.value();
    let select_id = format!("mode-select-{}", container_attr);
    let label_id = format!("{select_id}-label");

    #[cfg(target_arch = "wasm32")]
    {
        let dispatch = controller.dispatch.clone();
        use_on_create(cx, move || {
            let dispatch = dispatch.clone();
            async move {
                dispatch(ModeAction::Hydrated {
                    system_scheme: detect_system_preference(),
                });
            }
        });
    }

    let on_change = {
        let dispatch = controller.dispatch.clone();
        move |event: FormEvent| {
            let preference = match event.value.as_str() {
                "system" => ModePreference::System,
                "dark" => ModePreference::Explicit(ColorScheme::Dark),
                _ => ModePreference::Explicit(ColorScheme::Light),
            };
            dispatch(ModeAction::Select(preference));
        }
    };

    let snapshot = mode_state.read();

    cx.render(rsx! {
        div {
            class: "mode-switch",
            "data-rustic-app-mode-switch": "{container_attr}",
            label {
                r#for: "{select_id}",
                id: "{label_id}",
                "Theme"
            }
            select {
                id: "{select_id}",
                "aria-labelledby": "{label_id}",
                value: "{snapshot.preference.select_value()}",
                onchange: on_change,
                option { value: "system", "System" }
                if availability.light {
                    rsx! { option { value: "light", "Light" } }
                }
                if availability.dark {
                    rsx! { option { value: "dark", "Dark" } }
                }
            }
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn detect_system_preference() -> ColorScheme {
    use wasm_bindgen::JsCast;

    let window = web_sys::window();
    if let Some(window) = window {
        if let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)") {
            if query.matches() {
                return ColorScheme::Dark;
            }
        }
    }
    ColorScheme::Light
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_system_preference() -> ColorScheme {
    // Non-wasm (SSR/tests) environments cannot inspect browser APIs. The
    // deterministic light fallback keeps automation stable while the client
    // hydration pass upgrades to the real system preference.
    ColorScheme::Light
}

#[cfg(all(feature = "csr", not(feature = "ssr"), target_arch = "wasm32"))]
fn main() {
    // Hydrate existing SSR markup when present, otherwise render from scratch.
    dioxus_web::launch(App);
}

#[cfg(feature = "ssr")]
fn main() {
    // Server-side rendering entry that outputs a static HTML string.
    // Real applications would embed this within a web framework.
    println!("{}", dioxus_ssr::render_lazy(App));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_ids_include_framework() {
        let builder = layout::automation_for_framework(&HOME, Framework::Dioxus);
        assert_eq!(builder.value(), "app-home-framework-dioxus");
    }

    #[test]
    fn route_descriptor_mapping_is_stable() {
        assert_eq!(AppRoute::Home {}.descriptor().path, HOME.path);
        assert_eq!(AppRoute::About {}.descriptor().path, ABOUT.path);
        let fallback = AppRoute::NotFound { _segments: vec!["missing".into()] };
        assert_eq!(fallback.descriptor().path, HOME.path);
    }

    #[test]
    fn route_paths_match_expected_strings() {
        assert_eq!(AppRoute::Home {}.to_string(), "/");
        assert_eq!(AppRoute::About {}.to_string(), "/about");
        let fallback = AppRoute::NotFound { _segments: vec!["missing".into()] };
        assert!(fallback.to_string().starts_with('/'));
    }
}
