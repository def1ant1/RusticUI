use mui_styled_engine::css_with_theme;
use yew::prelude::*;

#[function_component(Test)]
fn test() -> Html {
    // The macro still injects `use_theme` even if the theme isn't referenced.
    let _style = css_with_theme!(r#"color: red;"#);
    html! {}
}

fn main() {}
