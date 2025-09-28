use std::rc::Rc;

use leptos::ev::Event;
use leptos::leptos_dom::log;
use leptos::store_value;
use leptos::*;
use leptos_router::{Route, Router, Routes, A};
use mui_shared::{
    automation::AutomationIdBuilder,
    layout::{self, AppShell, Framework},
    routes::{RouteDescriptor, ABOUT, HOME},
    theme::{material_example_theme, ColorSchemeAvailability, MaterialExampleTheme},
};
use rustic_ui_system::theme::ColorScheme;

/// Typed route identifiers shared between the router and automation harnesses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppRoute {
    /// Landing page that mirrors the archived Next.js hero section.
    Home,
    /// Secondary information page summarising the example goals.
    About,
    /// Fallback used when the router encounters an unknown path.
    NotFound,
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
                // Hydration is a one-way transition. Once the browser provides the
                // system preference we mark the state as client controlled while
                // preserving any explicit override chosen before hydration.
                let mut next = self.clone();
                next.phase = HydrationPhase::Client;
                next.system_scheme = system_scheme;
                next
            }
            ModeAction::Select(preference) => {
                // Selecting an explicit mode never discards the recorded system
                // preference. Enterprise automation frequently flips back to
                // "system" and expects the prior OS reading to remain intact.
                let mut next = self.clone();
                next.preference = preference;
                next
            }
        }
    }

    /// Effective colour scheme applied to the shared theme provider.
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

/// Creates a dispatch handle that upgrades the `ModeState` signal when actions occur.
fn create_mode_dispatch(mode_state: RwSignal<ModeState>) -> ModeDispatch {
    Rc::new(move |action| {
        mode_state.update(|state| {
            let current = state.clone();
            let next = current.transition(action);
            if next != current {
                *state = next;
            }
        });
    })
}

/// Root application component wiring the router, theme provider, and automation IDs.
///
/// * The Material theme blueprint and layout copy come from `mui-shared` so the
///   Leptos surface remains in lock-step with the Yew/Dioxus adapters.
/// * SSR renders inert markup using `HydrationPhase::Server`; once hydrated the
///   mode switch promotes itself to `HydrationPhase::Client` and reacts to
///   `matchMedia` changes without introducing DOM diffs.
/// * Automation identifiers are generated exclusively through
///   `mui_shared::automation::AutomationIdBuilder` helpers to keep Playwright and
///   Cypress selectors stable across frameworks and rendering modes.
#[component]
fn App() -> impl IntoView {
    // Material theme blueprint shared with the other framework adapters. We keep
    // it inside a `StoredValue` so multiple closures can borrow from it without
    // cloning heavy structures on every render.
    let theme_blueprint = store_value(material_example_theme());

    // Mode machine state shared between the select element and the theme provider.
    let mode_state = create_rw_signal(ModeState::default());
    let dispatch = create_mode_dispatch(mode_state);

    #[cfg(feature = "csr")]
    {
        // Signal storing the active theme so downstream components can reactively re-render.
        let theme_signal = create_rw_signal(
            theme_blueprint.with_value(|blueprint| blueprint.system_theme.clone()),
        );
        provide_context(theme_signal.get());
        {
            let theme_signal = theme_signal;
            let theme_blueprint = theme_blueprint;
            let mode_state = mode_state;
            create_effect(move |_| {
                let scheme = mode_state.get().effective_scheme();
                let next_theme = theme_blueprint.with_value(|blueprint| {
                    let mut theme = blueprint.system_theme.clone();
                    theme.palette.initial_color_scheme = scheme;
                    theme
                });
                provide_context(next_theme.clone());
                theme_signal.set(next_theme);
            });
        }
    }

    let availability = theme_blueprint.with_value(|blueprint| blueprint.color_schemes.clone());

    let dispatch_home = dispatch.clone();
    let availability_home = availability.clone();
    let dispatch_root = dispatch.clone();
    let availability_root = availability.clone();
    let dispatch_about = dispatch.clone();
    let availability_about = availability.clone();
    let dispatch_not_found = dispatch.clone();
    let availability_not_found = availability;

    view! {
        <Router>
            <Routes>
                <Route
                    path=""
                    view=move || {
                        render_route(
                            AppRoute::Home,
                            mode_state,
                            dispatch_home.clone(),
                            availability_home.clone(),
                            theme_blueprint,
                        )
                    }
                />
                <Route
                    path="/"
                    view=move || {
                        render_route(
                            AppRoute::Home,
                            mode_state,
                            dispatch_root.clone(),
                            availability_root.clone(),
                            theme_blueprint,
                        )
                    }
                />
                <Route
                    path="/about"
                    view=move || {
                        render_route(
                            AppRoute::About,
                            mode_state,
                            dispatch_about.clone(),
                            availability_about.clone(),
                            theme_blueprint,
                        )
                    }
                />
                <Route
                    path="*"
                    view=move || {
                        render_route(
                            AppRoute::NotFound,
                            mode_state,
                            dispatch_not_found.clone(),
                            availability_not_found.clone(),
                            theme_blueprint,
                        )
                    }
                />
            </Routes>
        </Router>
    }
}

/// Renders a single routed page, composing the shared layout metadata with Leptos primitives.
fn render_route(
    route: AppRoute,
    mode_state: RwSignal<ModeState>,
    dispatch: ModeDispatch,
    availability: ColorSchemeAvailability,
    blueprint: StoredValue<MaterialExampleTheme>,
) -> View {
    let descriptor = route.descriptor();
    let shell = AppShell::for_route(descriptor);
    let automation = shell.automation();
    let framework_builder = layout::automation_for_framework(descriptor, Framework::Leptos);

    let shell_attr = automation.child("shell").value();
    let hydration_attr = framework_builder.child("hydration-root").value();
    let header_attr = automation.child("header").value();
    let nav_attr = automation.child("navigation").value();
    let main_attr = automation.child("main").value();
    let body_attr = automation.child("body").value();
    let headline_attr = automation.child("headline").value();
    let actions_attr = automation.child("actions").value();
    let pro_tip_attr = automation.child("pro-tip").value();
    let nav_targets = [AppRoute::Home, AppRoute::About];
    let nav_links = nav_targets.into_iter().map(|target| {
        let descriptor = target.descriptor();
        let link_attr = automation
            .child("navigation")
            .child(descriptor.automation_base)
            .value();
        let classes = if route == target {
            "nav-link active"
        } else {
            "nav-link"
        };
        view! {
            <li data-rustic-app-navigation={link_attr.clone()}>
                <A href=descriptor.path class=classes>{target.nav_label()}</A>
            </li>
        }
        .into_view()
    });

    let primary_action = shell.primary_action().map(|action| {
        let attr = automation
            .child("actions")
            .child(action.automation_role)
            .value();
        view! {
            <a class="cta primary" data-rustic-app-action={attr} href={action.href}>
                {action.label}
            </a>
        }
        .into_view()
    });

    let secondary_action = shell.secondary_action().map(|action| {
        let attr = automation
            .child("actions")
            .child(action.automation_role)
            .value();
        view! {
            <a class="cta secondary" data-rustic-app-action={attr} href={action.href}>
                {action.label}
            </a>
        }
        .into_view()
    });

    let pro_tip = shell.pro_tip();
    let mode_switch_automation = automation.child("mode-switch");

    let nav_list = View::from_iter(nav_links);

    view! {
        <div id="app" data-rustic-app-shell={shell_attr} data-rustic-app-hydration-root={hydration_attr}>
            <div class="app-surface">
                <header data-rustic-app-header={header_attr}>
                    <nav data-rustic-app-navigation={nav_attr}>
                        <ul class="nav-list">{nav_list}</ul>
                    </nav>
                    <ModeSwitch
                        automation={mode_switch_automation}
                        availability
                        state={mode_state}
                        dispatch={dispatch.clone()}
                    />
                </header>
                <main data-rustic-app-main={main_attr}>
                    <section class="hero">
                        <h1 data-rustic-app-headline={headline_attr}>{shell.headline()}</h1>
                        <p data-rustic-app-body={body_attr}>{shell.body_copy()}</p>
                        <div class="cta-container" data-rustic-app-actions={actions_attr}>
                            {primary_action}
                            {secondary_action}
                        </div>
                    </section>
                    <ShowcaseArea automation={automation.child("showcases")} blueprint={blueprint} />
                    <footer class="pro-tip" data-rustic-app-pro-tip={pro_tip_attr}>
                        <strong>{pro_tip.lead_in}</strong>
                        {" "}
                        <a href={pro_tip.link_href}>{pro_tip.link_label}</a>
                        {format!(" {}", pro_tip.tail_text)}
                    </footer>
                </main>
            </div>
        </div>
    }
    .into_view()
}

/// Small showcase grid mirroring the alert/slider/popover highlights from the shared demo.
#[component]
fn ShowcaseArea(
    automation: AutomationIdBuilder,
    blueprint: StoredValue<MaterialExampleTheme>,
) -> impl IntoView {
    let alert_attr = automation.child("alert").value();
    let slider_attr = automation.child("slider").value();
    let slider_value_attr = automation.child("slider").child("value").value();
    let popover_attr = automation.child("popover").value();
    let popover_surface_attr = automation.child("popover").child("surface").value();

    let alert_background =
        blueprint.with_value(|theme| theme.components.alert.info_background.to_string());

    let slider_value = create_rw_signal(42.0);
    let on_slider_input = {
        let slider_value = slider_value.clone();
        move |ev: Event| {
            if let Ok(parsed) = event_target_value(&ev).parse::<f64>() {
                slider_value.set(parsed);
            }
        }
    };

    let popover_open = create_rw_signal(false);
    let toggle_popover = {
        let popover_open = popover_open.clone();
        move |_| {
            popover_open.update(|open| *open = !*open);
        }
    };

    view! {
        <section class="showcases" data-rustic-app-showcases={automation.value()}>
            <article class="showcase-card alert" data-rustic-app-showcase-alert={alert_attr.clone()}>
                <h2>"Alert customisation"</h2>
                <p>
                    "The shared theme injects a deterministic info background colour ("
                    {alert_background.clone()}
                    ") so automation can assert visual parity across frameworks."
                </p>
                <div class="alert-demo" role="alert" style={format!("background:{};color:#0f172a;padding:12px;border-radius:8px;", alert_background)}>
                    <strong>"Heads up"</strong>
                    {": deployment pipelines share alert theming via `mui-shared`."}
                </div>
            </article>

            <article class="showcase-card slider" data-rustic-app-showcase-slider={slider_attr.clone()}>
                <h2>"Deterministic slider"</h2>
                <p>
                    "Slider telemetry feeds into enterprise analytics. The signal based handler"
                    " keeps SSR output stable while hydrating client events."
                </p>
                <label class="slider-label" for="showcase-capacity">"Deployment capacity"</label>
                <input
                    id="showcase-capacity"
                    class="slider-input"
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    value=move || slider_value.get().to_string()
                    on:input=on_slider_input
                    data-rustic-app-showcase-slider-control={slider_attr.clone()}
                    aria-valuemin="0"
                    aria-valuemax="100"
                    aria-valuenow=move || slider_value.get().round().to_string()
                    aria-labelledby="showcase-capacity"
                />
                <output data-rustic-app-showcase-slider-value={slider_value_attr.clone()}>
                    {move || format!("{:.0}% allocated", slider_value.get())}
                </output>
            </article>

            <article class="showcase-card popover" data-rustic-app-showcase-popover={popover_attr.clone()}>
                <h2>"Popover orchestration"</h2>
                <p>
                    "The popover surface toggles deterministically so SSR and CSR agree on"
                    " collision metadata during analytics captures."
                </p>
                <button
                    class="popover-trigger"
                    data-rustic-app-showcase-popover-trigger={automation.child("popover").child("trigger").value()}
                    on:click=toggle_popover
                    aria-expanded=move || popover_open.get().to_string()
                    aria-controls="popover-surface"
                >
                    {move || if popover_open.get() { "Close popover" } else { "Open popover" }}
                </button>
                <Show when=move || popover_open.get() fallback=|| view! { <></> }>
                    <div
                        id="popover-surface"
                        role="dialog"
                        class="popover-surface"
                        data-rustic-app-showcase-popover-surface={popover_surface_attr.clone()}
                    >
                        <p>"Popover is open. Collision logic mirrors SSR output."</p>
                    </div>
                </Show>
            </article>
        </section>
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
#[component]
fn ModeSwitch(
    automation: AutomationIdBuilder,
    availability: ColorSchemeAvailability,
    state: RwSignal<ModeState>,
    dispatch: ModeDispatch,
) -> impl IntoView {
    let container_attr = automation.value();
    let select_id = format!("mode-select-{}", container_attr);
    let label_id = format!("{select_id}-label");

    #[cfg(target_arch = "wasm32")]
    {
        let dispatch = dispatch.clone();
        create_effect(move |_| {
            let snapshot = state.get();
            if snapshot.phase != HydrationPhase::Client {
                let system_scheme = detect_system_preference();
                dispatch(ModeAction::Hydrated { system_scheme });
            }
        });
    }

    let on_change = {
        let dispatch = dispatch.clone();
        move |event: Event| {
            if let Some(preference) = selection_from_event(&event) {
                dispatch(ModeAction::Select(preference));
            }
        }
    };

    view! {
        <div class="mode-switch" data-rustic-app-mode-switch={container_attr}>
            <label for={select_id.clone()} id={label_id.clone()}>"Theme"</label>
            <select
                id={select_id}
                aria-labelledby={label_id}
                value=move || state.get().preference.select_value().to_string()
                on:change=on_change
            >
                <option value="system">"System"</option>
                {if availability.light {
                    view! { <option value="light">"Light"</option> }.into_view()
                } else {
                    ().into_view()
                }}
                {if availability.dark {
                    view! { <option value="dark">"Dark"</option> }.into_view()
                } else {
                    ().into_view()
                }}
            </select>
        </div>
    }
}

/// Extracts the select value from the change event.
fn selection_from_event(event: &Event) -> Option<ModePreference> {
    let value = event_target_value(event);
    match value.as_str() {
        "system" => Some(ModePreference::System),
        "dark" => Some(ModePreference::Explicit(ColorScheme::Dark)),
        "light" => Some(ModePreference::Explicit(ColorScheme::Light)),
        other => {
            log!("Unknown mode option '{other}', defaulting to light mode for determinism.");
            Some(ModePreference::Explicit(ColorScheme::Light))
        }
    }
}

/// Reads the browser preference using `matchMedia` on the client.
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
    // Hydrate existing SSR markup when present, otherwise render from scratch.
    leptos::mount_to_body(|| view! { <App /> });
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::ssr::render_to_string;

    let theme = material_example_theme();
    let shell = AppShell::for_route(&HOME);
    let app_markup = render_to_string(|| view! { <App /> }).into_owned();
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
            automation.child("navigation").value(),
            "app-home-navigation"
        );
        assert_eq!(
            automation.child("showcases").child("popover").value(),
            "app-home-showcases-popover"
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
    fn detect_system_preference_defaults_to_light_on_host() {
        assert_eq!(detect_system_preference(), ColorScheme::Light);
    }
}
