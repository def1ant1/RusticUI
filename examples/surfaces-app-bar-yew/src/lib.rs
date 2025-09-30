use yew::prelude::*;

use rustic_ui_material::{AppBar, AppBarColor, AppBarSize};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div data-example="surfaces-app-bar-yew">
            <AppBar
                title="Operations Console"
                aria_label="Primary navigation"
                color={AppBarColor::Primary}
                size={AppBarSize::Large}
                automation_id={Some("operations-console".into())}
                analytics_view_id={Some("nav.operations.view".into())}
                analytics_interaction_id={Some("nav.operations.click".into())}
                svg_title_id={Some("operations-logo".into())}
            />
        </div>
    }
}
