macro_rules! assert_css {
    ($value:expr => $( $func:ident => $prop:expr ),+ $(,)?) => {{
        $(
            let actual = mui_system::$func($value);
            let expected = format!("{}:{};", $prop, $value);
            assert_eq!(actual, expected, "helper {} should emit {}", stringify!($func), expected);
        )*
    }};
}

/// Macro generated helpers should faithfully emit `prop:value;` strings.
#[test]
fn style_helpers_cover_full_surface() {
    // Spacing ----------------------------------------------------------------
    assert_css!("8px" =>
        margin => "margin",
        padding => "padding",
        margin_top => "margin-top",
        margin_bottom => "margin-bottom",
        margin_left => "margin-left",
        margin_right => "margin-right",
        padding_top => "padding-top",
        padding_bottom => "padding-bottom",
        padding_left => "padding-left",
        padding_right => "padding-right",
    );
    assert_css!("1rem" =>
        gap => "gap",
        row_gap => "row-gap",
        column_gap => "column-gap",
    );

    // Layout ----------------------------------------------------------------
    assert_css!("flex" => display => "display");
    assert_css!("row" => flex_direction => "flex-direction");
    assert_css!("wrap" => flex_wrap => "flex-wrap");
    assert_css!("center" =>
        align_items => "align-items",
        align_content => "align-content",
        align_self => "align-self",
        justify_items => "justify-items",
        justify_self => "justify-self",
        place_items => "place-items",
        place_content => "place-content",
        place_self => "place-self",
    );
    assert_css!("space-between" => justify_content => "justify-content");
    assert_css!("1" =>
        flex_grow => "flex-grow",
        flex_shrink => "flex-shrink",
    );
    assert_css!("200px" => flex_basis => "flex-basis");
    assert_css!("2" => order => "order");
    assert_css!("dense" => grid_auto_flow => "grid-auto-flow");
    assert_css!("minmax(200px, 1fr)" =>
        grid_auto_columns => "grid-auto-columns",
        grid_auto_rows => "grid-auto-rows",
    );
    assert_css!("repeat(12, 1fr)" => grid_template_columns => "grid-template-columns");
    assert_css!("auto 1fr" => grid_template_rows => "grid-template-rows");
    assert_css!("\"hero hero\"" => grid_template_areas => "grid-template-areas");
    assert_css!("1 / span 2" =>
        grid_column => "grid-column",
        grid_row => "grid-row",
    );
    assert_css!("2" =>
        grid_column_start => "grid-column-start",
        grid_row_start => "grid-row-start",
    );
    assert_css!("span 3" =>
        grid_column_end => "grid-column-end",
        grid_row_end => "grid-row-end",
    );
    assert_css!("sales" => grid_area => "grid-area");

    // Typography -------------------------------------------------------------
    assert_css!("16px" =>
        font_size => "font-size",
        line_height => "line-height",
        letter_spacing => "letter-spacing",
    );
    assert_css!("600" => font_weight => "font-weight");

    // Sizing -----------------------------------------------------------------
    assert_css!("100%" =>
        width => "width",
        height => "height",
        min_width => "min-width",
        min_height => "min-height",
        max_width => "max-width",
        max_height => "max-height",
    );

    // Color & visual treatments ---------------------------------------------
    assert_css!("#123456" => color => "color");
    assert_css!("rgba(0,0,0,0.04)" => background_color => "background-color");
    assert_css!("8px" => border_radius => "border-radius");
    assert_css!("0 1px 2px rgba(0,0,0,0.2)" => box_shadow => "box-shadow");
    assert_css!("0.8" => opacity => "opacity");

    // Positioning ------------------------------------------------------------
    assert_css!("absolute" => position => "position");
    assert_css!("8px" =>
        top => "top",
        right => "right",
        bottom => "bottom",
        left => "left",
    );
    assert_css!("10" => z_index => "z-index");
    assert_css!("auto" => overflow => "overflow");
    assert_css!("scroll" =>
        overflow_x => "overflow-x",
        overflow_y => "overflow-y",
    );

    // Transforms -------------------------------------------------------------
    assert_css!("rotate(45deg)" => transform => "transform");
    assert_css!("50% 0" => transform_origin => "transform-origin");

    // Transitions ------------------------------------------------------------
    assert_css!("opacity 150ms ease" => transition => "transition");
    assert_css!("opacity" => transition_property => "transition-property");
    assert_css!("150ms" => transition_duration => "transition-duration");
    assert_css!("25ms" => transition_delay => "transition-delay");
    assert_css!("ease-in-out" => transition_timing_function => "transition-timing-function");

    // Animations -------------------------------------------------------------
    assert_css!("fade-in 500ms ease-in" => animation => "animation");
    assert_css!("fade-in" => animation_name => "animation-name");
    assert_css!("500ms" => animation_duration => "animation-duration");
    assert_css!("150ms" => animation_delay => "animation-delay");
    assert_css!("ease-in" => animation_timing_function => "animation-timing-function");
    assert_css!("infinite" => animation_iteration_count => "animation-iteration-count");
    assert_css!("alternate" => animation_direction => "animation-direction");
    assert_css!("forwards" => animation_fill_mode => "animation-fill-mode");
    assert_css!("paused" => animation_play_state => "animation-play-state");
}
