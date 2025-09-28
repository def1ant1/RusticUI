use std::rc::Rc;

use mui_shared::{
    automation::AutomationIdBuilder,
    layout::{self, AppShell, Framework},
    routes::{RouteDescriptor, ABOUT, HOME},
    theme::{material_example_theme, ColorSchemeAvailability, MaterialExampleTheme},
};
use rustic_ui_system::theme::ColorScheme;
use sycamore::prelude::*;
use sycamore_router::{Route, StaticRouter};

#[cfg(target_arch = "wasm32")]
use sycamore_router::{HistoryIntegration, Router};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlInputElement, HtmlSelectElement};

/// Typed route identifiers shared between the router and automation harnesses.
#[derive(Route, Clone, Copy, Debug, PartialEq, Eq)]
enum AppRoute {
    /// Landing page that mirrors the archived Next.js hero section.
    #[to("/")]
    Home,
    /// Secondary information page summarising the example goals.
    #[to("/about")]
    About,
    /// Fallback used when the router encounters an unknown path.
    #[not_found]
    NotFound,
}

impl Default for AppRoute {
    fn default() -> Self {
        Self::NotFound
    }
}

impl AppRoute {
    /// Returns the shared descriptor supplied by `mui-shared`.
    fn descriptor(&self) -> &'static RouteDescriptor {
        match self {
            Self::Home | Self::NotFound => &HOME,
            Self::About => &ABOUT,
        }
    }

    /// Navigation label rendered in the shared header.
    fn nav_label(&self) -> &'static str {
        match self {
            Self::Home | Self::NotFound => "Home",
            Self::About => "About",
        }
    }
}

/// Hydration lifecycle used by the theme mode switch.
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
    /// Mirrors the archival "System" option – use the OS/browser preference.
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

    /// Parses a select value emitted from DOM events.
    fn from_value(value: &str) -> Option<Self> {
        match value {
            "system" => Some(Self::System),
            "light" => Some(Self::Explicit(ColorScheme::Light)),
            "dark" => Some(Self::Explicit(ColorScheme::Dark)),
            _ => None,
        }
    }
}

/// Reducer-friendly container tracking hydration phase and effective theme mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    fn transition(self, action: ModeAction) -> Self {
        match action {
            ModeAction::Hydrated { system_scheme } => {
                let mut next = self;
                next.phase = HydrationPhase::Client;
                next.system_scheme = system_scheme;
                next
            }
            ModeAction::Select(preference) => {
                let mut next = self;
                next.preference = preference;
                next
            }
        }
    }

    /// Effective colour scheme applied to the shared theme provider.
    fn effective_scheme(self) -> ColorScheme {
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

/// Creates a dispatch handle that upgrades the `ModeState` signal when actions occur.
fn create_mode_dispatch(state: Signal<ModeState>) -> ModeDispatch {
    Rc::new(move |action| {
        state.update(|current| {
            let next = (*current).transition(action);
            if next != *current {
                *current = next;
            }
        });
    })
}

fn shell_for_route(route: AppRoute) -> AppShell<'static> {
    AppShell::for_route(route.descriptor())
}

/// Root application component wiring the router, theme provider, and automation IDs.
///
/// * The Material theme blueprint and layout copy come from `mui-shared` so the
///   Sycamore surface remains in lock-step with the Yew/Dioxus adapters.
/// * SSR renders inert markup using `HydrationPhase::Server`; once hydrated the
///   mode switch promotes itself to `HydrationPhase::Client` and reacts to
///   `matchMedia` changes without introducing DOM diffs.
/// * Automation identifiers are generated exclusively through
///   `mui_shared::automation::AutomationIdBuilder` helpers to keep Playwright and
///   Cypress selectors stable across frameworks and rendering modes.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[component]
fn AppContainer<G: Html>(cx: Scope, route_signal: ReadSignal<AppRoute>) -> View<G> {
    // Material theme blueprint shared with the other framework adapters. We keep
    // it inside an `Rc` so nested closures can borrow from it without cloning
    // heavy structures on every render.
    let theme_blueprint = Rc::new(material_example_theme());

    // Mode machine state shared between the select element and the theme-aware
    // automation attributes.
    let mode_state = *create_signal(cx, ModeState::default());
    let dispatch = create_mode_dispatch(mode_state);

    let availability: ColorSchemeAvailability = theme_blueprint.color_schemes.clone();
    let effective_scheme = create_memo(cx, move || mode_state.get().effective_scheme());

    let current_route = *create_signal(cx, route_signal.get());
    create_effect(cx, move || {
        current_route.set(route_signal.get());
    });

    let shell_state = *create_signal(cx, shell_for_route(route_signal.get()));
    create_effect(cx, move || {
        shell_state.set(shell_for_route(current_route.get()));
    });

    let automation_memo = create_memo(cx, move || shell_state.with(|shell| shell.automation()));
    let nav_targets = [AppRoute::Home, AppRoute::About];

    view! { cx,
        div(
            id="app",
            data-rustic-app-shell=move || automation_memo.get_clone().child("shell").value(),
            data-rustic-app-hydration-root=move || current_route.with(|route| {
                layout::automation_for_framework(route.descriptor(), Framework::Sycamore)
                    .child("hydration-root")
                    .value()
            }),
            data-color-scheme=move || effective_scheme.get().as_str().to_string(),
        ) {
            div(class="app-surface") {
                header(
                    data-rustic-app-header=move || {
                        automation_memo.get_clone().child("header").value()
                    },
                ) {
                    nav(
                        data-rustic-app-navigation=move || {
                            automation_memo.get_clone().child("navigation").value()
                        },
                    ) {
                        ul(class="nav-list") {
                            (View::new_fragment(
                                nav_targets
                                    .iter()
                                    .map(|target| {
                                        let descriptor = target.descriptor();
                                        view! { cx,
                                            li(
                                                data-rustic-app-navigation=move || {
                                                    automation_memo
                                                        .get_clone()
                                                        .child("navigation")
                                                        .child(descriptor.automation_base)
                                                        .value()
                                                },
                                            ) {
                                                a(
                                                    href=descriptor.path,
                                                    class=move || {
                                                        if current_route.get() == *target {
                                                            "nav-link active".to_string()
                                                        } else {
                                                            "nav-link".to_string()
                                                        }
                                                    },
                                                ) { (target.nav_label()) }
                                            }
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                            ))
                        }
                    }
                    ModeSwitch(
                        automation=automation_memo.get_clone().child("mode-switch"),
                        availability=availability.clone(),
                        state=mode_state,
                        dispatch=dispatch.clone(),
                    )
                }
                main(
                    data-rustic-app-main=move || {
                        automation_memo.get_clone().child("main").value()
                    },
                ) {
                    section(class="hero") {
                        h1(
                            data-rustic-app-headline=move || {
                                automation_memo.get_clone().child("headline").value()
                            },
                        ) {
                            (move || shell_state.with(|shell| shell.headline().to_string()))
                        }
                        p(
                            data-rustic-app-body=move || {
                                automation_memo.get_clone().child("body").value()
                            },
                        ) {
                            (move || shell_state.with(|shell| shell.body_copy().to_string()))
                        }
                        div(
                            class="cta-container",
                            data-rustic-app-actions=move || {
                                automation_memo.get_clone().child("actions").value()
                            },
                        ) {
                            {shell_state.with(|shell| {
                                let automation = automation_memo.get_clone();
                                if let Some(action) = shell.primary_action().cloned() {
                                    let attr = automation
                                        .child("actions")
                                        .child(action.automation_role)
                                        .value();
                                    view! { cx,
                                        a(
                                            class="cta primary",
                                            href=action.href,
                                            data-rustic-app-action=attr,
                                        ) { (action.label) }
                                    }
                                } else {
                                    View::empty()
                                }
                            })}
                            {shell_state.with(|shell| {
                                let automation = automation_memo.get_clone();
                                if let Some(action) = shell.secondary_action().cloned() {
                                    let attr = automation
                                        .child("actions")
                                        .child(action.automation_role)
                                        .value();
                                    view! { cx,
                                        a(
                                            class="cta secondary",
                                            href=action.href,
                                            data-rustic-app-action=attr,
                                        ) { (action.label) }
                                    }
                                } else {
                                    View::empty()
                                }
                            })}
                        }
                    }
                    ShowcaseArea(
                        automation=automation_memo.get_clone().child("showcases"),
                        blueprint=theme_blueprint.clone(),
                    )
                    {shell_state.with(|shell| {
                        let automation = automation_memo.get_clone();
                        let pro_tip_attr = automation.child("pro-tip").value();
                        let pro_tip = shell.pro_tip();
                        view! { cx,
                            footer(class="pro-tip", data-rustic-app-pro-tip=pro_tip_attr) {
                                strong { (pro_tip.lead_in) }
                                (" ")
                                a(href=pro_tip.link_href) { (pro_tip.link_label) }
                                (format!(" {}", pro_tip.tail_text))
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Mode switcher ported from the archival React demo with explicit SSR notes.
///
/// The reducer state forms a two phase machine:
/// * `HydrationPhase::Server` renders static markup. Events are inert and the
///   selected option defaults to `ModePreference::System` so SSR stays deterministic.
/// * Once hydrated the effect dispatches `ModeAction::Hydrated`, transitioning to
///   `HydrationPhase::Client` and capturing the browser preference via `matchMedia`.
///   From that point the select behaves like the archived client component and
///   dispatches `ModeAction::Select` for automation to observe.
#[component(inline_props)]
fn ModeSwitch<G: Html>(
    cx: Scope,
    automation: AutomationIdBuilder,
    availability: ColorSchemeAvailability,
    state: Signal<ModeState>,
    dispatch: ModeDispatch,
) -> View<G> {
    let container_attr = automation.value();
    let select_id = format!("mode-select-{}", container_attr);
    let label_id = format!("{select_id}-label");

    #[cfg(target_arch = "wasm32")]
    {
        let dispatch = dispatch.clone();
        create_effect(cx, move || {
            let snapshot = state.get();
            if snapshot.phase != HydrationPhase::Client {
                let system_scheme = detect_system_preference();
                dispatch(ModeAction::Hydrated { system_scheme });
            }
        });
    }

    let on_change = move |event: Event| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                if let Some(preference) = ModePreference::from_value(&target.value()) {
                    dispatch(ModeAction::Select(preference));
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = event;
        }
    };

    view! { cx,
        div(class="mode-switch", data-rustic-app-mode-switch=container_attr) {
            label(for=select_id.clone(), id=label_id.clone()) { "Theme" }
            select(
                id=select_id,
                aria-labelledby=label_id,
                value=move || state.get().preference.select_value().to_string(),
                on:change=on_change,
            ) {
                option(value="system") { "System" }
                {(if availability.light {
                    view! { cx, option(value="light") { "Light" } }
                } else {
                    View::empty()
                })}
                {(if availability.dark {
                    view! { cx, option(value="dark") { "Dark" } }
                } else {
                    View::empty()
                })}
            }
        }
    }
}

/// Small showcase grid mirroring the alert/slider/popover highlights from the shared demo.
///
/// Each widget documents its hydration strategy inline so QA and performance
/// teams can reason about SSR vs CSR behaviour when scripting automation flows.
#[component(inline_props)]
fn ShowcaseArea<G: Html>(
    cx: Scope,
    automation: AutomationIdBuilder,
    blueprint: Rc<MaterialExampleTheme>,
) -> View<G> {
    let alert_attr = automation.child("alert").value();
    let slider_attr = automation.child("slider").value();
    let slider_value_attr = automation.child("slider").child("value").value();
    let popover_attr = automation.child("popover").value();
    let popover_surface_attr = automation.child("popover").child("surface").value();

    let alert_background = blueprint
        .components
        .alert
        .info_background
        .to_string();

    let slider_value = *create_signal(cx, 42.0f64);
    let on_slider_input = move |event: Event| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                if let Ok(parsed) = target.value().parse::<f64>() {
                    slider_value.set(parsed);
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = event;
        }
    };

    let popover_open = *create_signal(cx, false);
    let toggle_popover = move |_| {
        popover_open.update(|open| *open = !*open);
    };

    view! { cx,
        section(class="showcases", data-rustic-app-showcases=automation.value()) {
            article(class="showcase-card alert", data-rustic-app-showcase-alert=alert_attr.clone()) {
                h2 { "Alert customisation" }
                p {
                    "The shared theme injects a deterministic info background colour ("
                    (alert_background.clone())
                    ") so automation can assert visual parity across frameworks."
                }
                div(
                    class="alert-demo",
                    role="alert",
                    style=format!("background:{};color:#0f172a;padding:12px;border-radius:8px;", alert_background),
                ) {
                    strong { "Heads up" }
                    ": deployment pipelines share alert theming via `mui-shared`."
                }
            }

            article(class="showcase-card slider", data-rustic-app-showcase-slider=slider_attr.clone()) {
                h2 { "Deterministic slider" }
                p {
                    "Slider telemetry feeds into enterprise analytics. The signal based handler "
                    "keeps SSR output stable while hydrating client events."
                }
                label(class="slider-label", for="showcase-capacity") { "Deployment capacity" }
                input(
                    id="showcase-capacity",
                    class="slider-input",
                    type="range",
                    min="0",
                    max="100",
                    step="1",
                    value=move || slider_value.get().to_string(),
                    on:input=on_slider_input,
                    data-rustic-app-showcase-slider-control=slider_attr.clone(),
                    aria-valuemin="0",
                    aria-valuemax="100",
                    aria-valuenow=move || slider_value.get().round().to_string(),
                    aria-labelledby="showcase-capacity",
                )
                output(data-rustic-app-showcase-slider-value=slider_value_attr.clone()) {
                    (move || format!("{:.0}% allocated", slider_value.get()))
                }
            }

            article(class="showcase-card popover", data-rustic-app-showcase-popover=popover_attr.clone()) {
                h2 { "Popover orchestration" }
                p {
                    "The popover surface toggles deterministically so SSR and CSR agree on "
                    "collision metadata during analytics captures."
                }
                button(
                    class="popover-trigger",
                    data-rustic-app-showcase-popover-trigger=
                        automation.child("popover").child("trigger").value(),
                    on:click=toggle_popover,
                    aria-expanded=move || popover_open.get().to_string(),
                    aria-controls="popover-surface",
                ) {
                    (move || if popover_open.get() { "Close popover" } else { "Open popover" })
                }
                {(if popover_open.get() {
                    view! { cx,
                        div(
                            id="popover-surface",
                            role="dialog",
                            class="popover-surface",
                            data-rustic-app-showcase-popover-surface=popover_surface_attr.clone(),
                        ) {
                            p { "Popover is open. Collision logic mirrors SSR output." }
                        }
                    }
                } else {
                    View::empty()
                })}
            }
        }
    }
}

/// CSR entry point wiring the browser router to the shared container.
#[cfg(target_arch = "wasm32")]
#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=move |route: ReadSignal<AppRoute>| view! { AppContainer(route_signal=route) },
        )
    }
}

/// SSR helper that renders a specific route once without accessing browser APIs.
#[cfg(feature = "ssr")]
#[component(inline_props)]
fn StaticApp<G: Html>(cx: Scope, route: AppRoute) -> View<G> {
    view! { cx,
        StaticRouter(
            route=route,
            view=move |route_signal: ReadSignal<AppRoute>| view! { AppContainer(route_signal=route_signal) },
        )
    }
}

/// Extracts the select value from the change event in CSR mode.
#[cfg(target_arch = "wasm32")]
fn detect_system_preference() -> ColorScheme {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(media)) = window.match_media("(prefers-color-scheme: dark)") {
            if media.matches() {
                return ColorScheme::Dark;
            }
        }
    }

    ColorScheme::Light
}

/// Non-wasm targets (SSR/tests) cannot access browser APIs, so we return light mode.
#[cfg(not(target_arch = "wasm32"))]
fn detect_system_preference() -> ColorScheme {
    ColorScheme::Light
}

#[cfg(feature = "csr")]
fn main() {
    // Hydrate existing SSR markup when present, otherwise render from scratch. The
    // router automatically registers history listeners and intercepts anchor
    // clicks, so no additional bootstrapping is required here.
    sycamore::render(|cx| view! { cx, App {} });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Server-side rendering entry point returning a static HTML string. In a real
    // deployment the route would be driven by the HTTP request path; we default
    // to the home descriptor here to keep the example deterministic.
    let shell = AppShell::for_route(&HOME);
    let theme = material_example_theme();
    let app_markup = sycamore::render_to_string(|cx| view! { cx, StaticApp(route=AppRoute::Home) });
    let document = shell.render_ssr_document(|_| app_markup.clone(), &theme);
    println!("{document}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_map_to_descriptors() {
        assert_eq!(AppRoute::Home.descriptor().path, HOME.path);
        assert_eq!(AppRoute::About.descriptor().path, ABOUT.path);
        assert_eq!(AppRoute::NotFound.descriptor().path, HOME.path);
    }

    #[test]
    fn automation_ids_are_deterministic() {
        let shell = AppShell::for_route(&HOME);
        let automation = shell.automation();
        assert_eq!(automation.value(), "app-home");
        assert_eq!(
            automation.child("navigation").child("home").value(),
            "app-home-navigation-home"
        );
        assert_eq!(
            layout::automation_for_framework(&HOME, Framework::Sycamore)
                .child("hydration-root")
                .value(),
            "app-home-framework-sycamore-hydration-root"
        );
    }

    #[test]
    fn mode_state_transitions_preserve_system_scheme() {
        let base = ModeState::default();
        let dark_override = base.transition(ModeAction::Select(ModePreference::Explicit(
            ColorScheme::Dark,
        )));
        assert_eq!(dark_override.effective_scheme(), ColorScheme::Dark);
        let hydrated = dark_override.transition(ModeAction::Hydrated {
            system_scheme: ColorScheme::Light,
        });
        assert_eq!(hydrated.phase, HydrationPhase::Client);
        // Hydration should not clobber the explicit override.
        assert_eq!(hydrated.effective_scheme(), ColorScheme::Dark);
    }

    #[test]
    fn mode_preference_parsing() {
        assert_eq!(ModePreference::from_value("system"), Some(ModePreference::System));
        assert_eq!(
            ModePreference::from_value("light"),
            Some(ModePreference::Explicit(ColorScheme::Light))
        );
        assert_eq!(ModePreference::from_value("dark"), Some(ModePreference::Explicit(ColorScheme::Dark)));
        assert_eq!(ModePreference::from_value("unknown"), None);
    }

    #[test]
    fn detect_system_preference_defaults_to_light_on_host() {
        assert_eq!(detect_system_preference(), ColorScheme::Light);
    }
}
