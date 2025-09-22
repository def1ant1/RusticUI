use yew::prelude::*;

use crate::helpers::{resolve_aspect_ratio_styles, AspectRatioStyles};
use crate::joy_props;

joy_props!(AspectRatioProps {
    /// Desired width to height ratio (e.g. 16.0 / 9.0).
    ratio: f32,
    /// Child element rendered inside the constrained box.
    children: Children,
});

/// Maintains a consistent width/height ratio for its child.
///
/// # Design tokens
/// This primitive does not consume palette tokens directly. It relies on
/// [`resolve_aspect_ratio_styles`](crate::helpers::resolve_aspect_ratio_styles) to emit the
/// padding-top hack and is therefore safe to use across all framework adapters without additional
/// styling.
///
/// # Headless state contract
/// Aspect ratios are stateless; no headless state machine is required. Each adapter simply renders
/// the deterministic layout scaffolding supplied by the helper.
#[function_component(AspectRatio)]
pub fn aspect_ratio(props: &AspectRatioProps) -> Html {
    let styles: AspectRatioStyles = resolve_aspect_ratio_styles(props.ratio);
    html! {
        <div style={styles.outer}>
            <div style={styles.inner}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
