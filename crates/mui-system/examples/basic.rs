//! Minimal usage example demonstrating theming and layout helpers.
//!
//! Run with `cargo run --example basic --features yew` targeting `wasm32`.

use mui_system::{style_props, Box, Grid, Theme, ThemeProvider};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let theme = Theme::default();
    html! {
        <ThemeProvider theme={theme}>
            <Box style={style_props!{ padding: "16px" }}>
                <Grid span={6} columns={12} style={style_props!{ background_color: "#eee" }}>
                    {"Responsive grid item"}
                </Grid>
            </Box>
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
