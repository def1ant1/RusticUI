use serde_json::json;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;

use mui_shared::{
    automation::AutomationIdBuilder,
    layout::{self, AppShell, Framework},
    routes::{RouteDescriptor, ABOUT, HOME},
    theme::{material_example_theme, ColorSchemeAvailability},
};
use rustic_ui_system::{
    theme::ColorScheme, Box, Stack, ThemeProvider, Typography, TypographyVariant,
};

/// Typed routes shared across CSR and SSR entry points.
#[derive(Clone, Routable, PartialEq, Eq, Debug)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl AppRoute {
    /// Returns the descriptor supplied by the shared integration crate.
    fn descriptor(&self) -> &'static RouteDescriptor {
        match self {
            Self::Home | Self::NotFound => &HOME,
            Self::About => &ABOUT,
        }
    }

    /// Navigation label rendered inside the top level shell.
    fn nav_label(&self) -> &'static str {
        match self {
            Self::Home | Self::NotFound => "Home",
            Self::About => "About",
        }
    }
}

/// Hydration lifecycle for the mode switch state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HydrationPhase {
    /// Server rendered markup prior to any browser APIs being available.
    Server,
    /// Client side pass where we can safely read `window` and register handlers.
    Client,
}

/// Developer facing representation of the user's colour preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModePreference {
    /// Mirrors the archived "System" option – use the OS/browser preference.
    System,
    /// Explicit user choice overriding the detected system preference.
    Explicit(ColorScheme),
}

impl ModePreference {
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
            // The server cannot observe client preferences so we fall back to the
            // archival "system" choice which resolves to light mode until
            // hydration promotes the state to `HydrationPhase::Client`.
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
                // Hydration is a one-way transition. Once the browser provides
                // the system preference we mark the state as client controlled
                // and retain both the OS preference and any explicit override.
                let mut next = self.clone();
                next.phase = HydrationPhase::Client;
                next.system_scheme = system_scheme;
                next
            }
            ModeAction::Select(preference) => {
                // Selecting an explicit mode never destroys the recorded system
                // preference. Enterprises often flip back to "system" in test
                // automation and expect the prior OS reading to be retained.
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

/// Actions that drive the mode switch state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModeAction {
    /// Fired once the client resolves `matchMedia` and the DOM hydrates.
    Hydrated { system_scheme: ColorScheme },
    /// Fired when the user chooses an explicit option from the select element.
    Select(ModePreference),
}

/// Root application component wiring the router, theme provider, and automation.
#[function_component(App)]
fn app() -> Html {
    let theme_blueprint = use_memo((), |_| material_example_theme());
    let mode_state = use_state(ModeState::default);

    let mut active_theme = (*theme_blueprint).system_theme.clone();
    let snapshot = (*mode_state).clone();
    active_theme.palette.initial_color_scheme = snapshot.effective_scheme();

    let dispatch = {
        let mode_state = mode_state.clone();
        Callback::from(move |action: ModeAction| {
            let current = (*mode_state).clone();
            let next = current.transition(action);
            if next != current {
                mode_state.set(next);
            }
        })
    };

    let availability = (*theme_blueprint).color_schemes.clone();

    html! {
        <ThemeProvider theme={active_theme}>
            <BrowserRouter>
                <Switch<AppRoute> render={{
                    let mode_state = mode_state.clone();
                    let dispatch = dispatch.clone();
                    move |route| {
                        let state_snapshot = (*mode_state).clone();
                        render_route(route, state_snapshot, dispatch.clone(), availability.clone())
                    }
                }} />
            </BrowserRouter>
        </ThemeProvider>
    }
}

fn render_route(
    route: AppRoute,
    state: ModeState,
    dispatch: Callback<ModeAction>,
    availability: ColorSchemeAvailability,
) -> Html {
    let descriptor = route.descriptor();
    let shell = AppShell::for_route(descriptor);
    let automation = shell.automation();
    let framework_builder = layout::automation_for_framework(descriptor, Framework::Yew);

    let shell_attr = AttrValue::from(automation.child("shell").value());
    let hydration_attr = AttrValue::from(framework_builder.child("hydration-root").value());
    let header_attr = AttrValue::from(automation.child("header").value());
    let nav_attr = AttrValue::from(automation.child("navigation").value());
    let main_attr = AttrValue::from(automation.child("main").value());
    let body_attr = AttrValue::from(automation.child("body").value());
    let headline_attr = AttrValue::from(automation.child("headline").value());
    let actions_attr = AttrValue::from(automation.child("actions").value());
    let pro_tip_attr = AttrValue::from(automation.child("pro-tip").value());

    let nav_items = [AppRoute::Home, AppRoute::About].into_iter().map(|target| {
        let descriptor = target.descriptor();
        let link_attr_value = automation
            .child("navigation")
            .child(descriptor.automation_base)
            .value();
        let is_active = route == target;
        html! {
            <li data-rustic-app-navigation={link_attr_value}>
                <Link<AppRoute> to={target.clone()} classes={classes!("nav-link", is_active.then_some("active"))}>
                    { target.nav_label() }
                </Link<AppRoute>>
            </li>
        }
    });

    let primary_action = shell.primary_action().map(|action| {
        let attr = automation
            .child("actions")
            .child(action.automation_role)
            .value();
        html! {
            <a class="cta primary" data-rustic-app-action={attr} href={action.href}>
                { action.label }
            </a>
        }
    });
    let secondary_action = shell.secondary_action().map(|action| {
        let attr = automation
            .child("actions")
            .child(action.automation_role)
            .value();
        html! {
            <a class="cta secondary" data-rustic-app-action={attr} href={action.href}>
                { action.label }
            </a>
        }
    });

    let pro_tip = shell.pro_tip();
    let mode_switch_automation = automation.child("mode-switch");

    html! {
        <div id="app" data-rustic-app-shell={shell_attr} data-rustic-app-hydration-root={hydration_attr}>
            <Box sx={Some(json!({
                "minHeight": "100vh",
                "display": "flex",
                "flexDirection": "column",
                "gap": "24px",
                "padding": "24px",
            }))}>
                <header data-rustic-app-header={header_attr.clone()}>
                    <nav data-rustic-app-navigation={nav_attr.clone()}>
                        <ul class="nav-list">
                            { for nav_items }
                        </ul>
                    </nav>
                    <ModeSwitch
                        automation={mode_switch_automation}
                        availability={availability.clone()}
                        state={state}
                        dispatch={dispatch.clone()}
                    />
                </header>
                <main data-rustic-app-main={main_attr.clone()}>
                    <Stack spacing={Some(rustic_ui_system::responsive::Responsive::constant("24px".to_string()))}>
                        <Typography variant={Some(TypographyVariant::H1)}>
                            <span data-rustic-app-headline={headline_attr}>{ shell.headline() }</span>
                        </Typography>
                        <Typography>
                            <span data-rustic-app-body={body_attr}>{ shell.body_copy() }</span>
                        </Typography>
                        <div class="cta-container" data-rustic-app-actions={actions_attr}>
                            { primary_action }
                            { secondary_action }
                        </div>
                    </Stack>
                    <footer class="pro-tip" data-rustic-app-pro-tip={pro_tip_attr}>
                        <strong>{ pro_tip.lead_in }</strong>
                        { " " }
                        <a href={pro_tip.link_href}>{ pro_tip.link_label }</a>
                        { format!(" {}", pro_tip.tail_text) }
                    </footer>
                </main>
            </Box>
        </div>
    }
}

#[derive(Properties, Clone)]
struct ModeSwitchProps {
    pub automation: AutomationIdBuilder,
    pub availability: ColorSchemeAvailability,
    pub state: ModeState,
    pub dispatch: Callback<ModeAction>,
}

impl PartialEq for ModeSwitchProps {
    fn eq(&self, other: &Self) -> bool {
        self.automation == other.automation
            && self.availability == other.availability
            && self.state == other.state
    }
}

/// Mode switcher ported from the archival React demo with explicit SSR notes.
///
/// The reducer state forms a two phase machine:
///
/// * `HydrationPhase::Server` renders static markup. Events are inert and the
///   selected option defaults to `ModePreference::System` so SSR stays
///   deterministic.
/// * Once hydrated the effect dispatches `ModeAction::Hydrated`, transitioning
///   the state to `HydrationPhase::Client` and capturing the browser preference
///   via `matchMedia`. From that point the select behaves like the archived
///   client component and dispatches `ModeAction::Select` for automation to
///   observe.
#[function_component(ModeSwitch)]
fn mode_switch(props: &ModeSwitchProps) -> Html {
    let container_attr = AttrValue::from(props.automation.value());
    let select_id = format!("mode-select-{}", props.automation.value());
    let label_id = format!("{select_id}-label");
    let selected = AttrValue::from(props.state.preference.select_value());

    {
        let dispatch = props.dispatch.clone();
        use_effect_with(props.state.phase, move |phase| {
            if *phase != HydrationPhase::Client {
                let system_scheme = detect_system_preference();
                dispatch.emit(ModeAction::Hydrated { system_scheme });
            }
            || {}
        });
    }

    let on_change = {
        let dispatch = props.dispatch.clone();
        Callback::from(move |event: Event| {
            if let Some(preference) = selection_from_event(&event) {
                dispatch.emit(ModeAction::Select(preference));
            }
        })
    };

    html! {
        <div class="mode-switch" data-rustic-app-mode-switch={container_attr}>
            <label for={select_id.clone()} id={label_id.clone()}>{ "Theme" }</label>
            <select id={select_id} aria-labelledby={label_id} value={selected} onchange={on_change}>
                <option value="system">{ "System" }</option>
                {props.availability.light.then_some(html! { <option value="light">{ "Light" }</option> })}
                {props.availability.dark.then_some(html! { <option value="dark">{ "Dark" }</option> })}
            </select>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn selection_from_event(event: &Event) -> Option<ModePreference> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlSelectElement;

    event
        .target()
        .and_then(|node| node.dyn_into::<HtmlSelectElement>().ok())
        .map(|select| match select.value().as_str() {
            "system" => ModePreference::System,
            "dark" => ModePreference::Explicit(ColorScheme::Dark),
            _ => ModePreference::Explicit(ColorScheme::Light),
        })
}

#[cfg(not(target_arch = "wasm32"))]
fn selection_from_event(_event: &Event) -> Option<ModePreference> {
    None
}

fn detect_system_preference() -> ColorScheme {
    #[cfg(target_arch = "wasm32")]
    {
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
    {
        // Non-wasm (SSR/tests) environments cannot inspect browser APIs. The
        // deterministic light fallback keeps automation stable while the client
        // hydration pass upgrades to the real system preference.
        ColorScheme::Light
    }
}

#[cfg(all(feature = "csr", not(feature = "ssr"), target_arch = "wasm32"))]
fn main() {
    // Hydrate existing SSR markup when present, otherwise render from scratch.
    yew::Renderer::<App>::new().hydrate();
}

#[cfg(all(feature = "csr", not(feature = "ssr"), not(target_arch = "wasm32")))]
fn main() {
    // Non-wasm targets (tests, CLI invocations) skip the hydration call.
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use yew::ServerRenderer;

    // Server side rendering composes the shared HTML skeleton with the actual
    // Yew markup. The shared shell generates deterministic automation ids so
    // Playwright/Cypress suites can locate nodes pre-hydration.
    let theme = material_example_theme();
    let shell = AppShell::for_route(&HOME);
    let app_markup = ServerRenderer::<App>::new().render().await;
    let document = shell.render_ssr_document(|_| app_markup, &theme);
    println!("{document}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_map_to_descriptors() {
        assert_eq!(AppRoute::Home.descriptor().path, HOME.path);
        assert_eq!(AppRoute::About.descriptor().path, ABOUT.path);
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
}
