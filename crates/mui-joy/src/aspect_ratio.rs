use yew::prelude::*;

use crate::joy_props;

joy_props!(AspectRatioProps {
    /// Desired width to height ratio (e.g. 16.0 / 9.0).
    ratio: f32,
    /// Child element rendered inside the constrained box.
    children: Children,
});

/// Maintains a consistent width/height ratio for its child.
///
/// The component uses the classic padding-top hack so it works without any
/// JavaScript and keeps layout calculations on the GPU.
#[function_component(AspectRatio)]
pub fn aspect_ratio(props: &AspectRatioProps) -> Html {
    let padding = format!(
        "padding-top:{}%;position:relative;width:100%;",
        100.0 / props.ratio
    );
    html! {
        <div style={padding}>
            <div style="position:absolute;top:0;left:0;width:100%;height:100%;">
                { for props.children.iter() }
            </div>
        </div>
    }
}
