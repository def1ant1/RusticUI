use mui_system::{
    r#box::{build_box_style, BoxStyleInputs},
    responsive::Responsive,
    Theme,
};

#[test]
fn box_resolves_all_responsive_groups() {
    let theme = Theme::default();
    let margin = Responsive {
        xs: "2px".into(),
        sm: Some("4px".into()),
        md: Some("8px".into()),
        lg: Some("16px".into()),
        xl: None,
    };
    let padding = Responsive {
        xs: "1rem".into(),
        sm: Some("1.5rem".into()),
        md: Some("2rem".into()),
        lg: None,
        xl: Some("3rem".into()),
    };
    let font_size = Responsive {
        xs: "12px".into(),
        sm: None,
        md: Some("16px".into()),
        lg: Some("18px".into()),
        xl: Some("20px".into()),
    };
    let font_weight = Responsive {
        xs: "400".into(),
        sm: None,
        md: Some("500".into()),
        lg: Some("600".into()),
        xl: None,
    };
    let line_height = Responsive {
        xs: "20px".into(),
        sm: None,
        md: Some("24px".into()),
        lg: Some("28px".into()),
        xl: None,
    };
    let letter_spacing = Responsive {
        xs: "0px".into(),
        sm: Some("0.5px".into()),
        md: Some("1px".into()),
        lg: None,
        xl: Some("1.5px".into()),
    };
    let color = Responsive {
        xs: "#111111".into(),
        sm: None,
        md: Some("#222222".into()),
        lg: Some("#333333".into()),
        xl: None,
    };
    let background = Responsive {
        xs: "#ffffff".into(),
        sm: None,
        md: Some("#f0f0f0".into()),
        lg: Some("#e0e0e0".into()),
        xl: Some("#d0d0d0".into()),
    };
    let width = Responsive {
        xs: "100%".into(),
        sm: None,
        md: Some("75%".into()),
        lg: Some("50%".into()),
        xl: None,
    };
    let height = Responsive {
        xs: "auto".into(),
        sm: None,
        md: Some("320px".into()),
        lg: Some("400px".into()),
        xl: None,
    };
    let min_width = Responsive {
        xs: "240px".into(),
        sm: None,
        md: Some("320px".into()),
        lg: Some("360px".into()),
        xl: None,
    };
    let max_width = Responsive {
        xs: "480px".into(),
        sm: None,
        md: Some("640px".into()),
        lg: Some("720px".into()),
        xl: None,
    };
    let min_height = Responsive {
        xs: "120px".into(),
        sm: None,
        md: Some("160px".into()),
        lg: Some("180px".into()),
        xl: None,
    };
    let max_height = Responsive {
        xs: "260px".into(),
        sm: None,
        md: Some("320px".into()),
        lg: Some("360px".into()),
        xl: None,
    };
    let position = Responsive {
        xs: "relative".into(),
        sm: None,
        md: Some("relative".into()),
        lg: Some("absolute".into()),
        xl: None,
    };
    let offsets = Responsive {
        xs: "0".into(),
        sm: None,
        md: Some("4px".into()),
        lg: Some("8px".into()),
        xl: None,
    };

    let base = build_box_style(
        500,
        &theme.breakpoints,
        BoxStyleInputs {
            margin: Some(&margin),
            padding: Some(&padding),
            font_size: Some(&font_size),
            font_weight: Some(&font_weight),
            line_height: Some(&line_height),
            letter_spacing: Some(&letter_spacing),
            color: Some(&color),
            background_color: Some(&background),
            width: Some(&width),
            height: Some(&height),
            min_width: Some(&min_width),
            max_width: Some(&max_width),
            min_height: Some(&min_height),
            max_height: Some(&max_height),
            position: Some(&position),
            top: Some(&offsets),
            right: Some(&offsets),
            bottom: Some(&offsets),
            left: Some(&offsets),
            display: Some("flex"),
            align_items: Some("center"),
            justify_content: Some("flex-start"),
            sx: "border-radius:4px;",
        },
    );
    assert!(base.contains("margin:2px;"));
    assert!(base.contains("padding:1rem;"));
    assert!(base.contains("font-size:12px;"));
    assert!(base.contains("font-weight:400;"));
    assert!(base.contains("line-height:20px;"));
    assert!(base.contains("letter-spacing:0px;"));
    assert!(base.contains("color:#111111;"));
    assert!(base.contains("background-color:#ffffff;"));
    assert!(base.contains("width:100%;"));
    assert!(base.contains("height:auto;"));
    assert!(base.contains("min-width:240px;"));
    assert!(base.contains("max-width:480px;"));
    assert!(base.contains("min-height:120px;"));
    assert!(base.contains("max-height:260px;"));
    assert!(base.contains("position:relative;"));
    assert!(base.contains("top:0;"));
    assert!(base.contains("display:flex;"));
    assert!(base.ends_with("border-radius:4px;"));

    let large = build_box_style(
        1400,
        &theme.breakpoints,
        BoxStyleInputs {
            margin: Some(&margin),
            padding: Some(&padding),
            font_size: Some(&font_size),
            font_weight: Some(&font_weight),
            line_height: Some(&line_height),
            letter_spacing: Some(&letter_spacing),
            color: Some(&color),
            background_color: Some(&background),
            width: Some(&width),
            height: Some(&height),
            min_width: Some(&min_width),
            max_width: Some(&max_width),
            min_height: Some(&min_height),
            max_height: Some(&max_height),
            position: Some(&position),
            top: Some(&offsets),
            right: Some(&offsets),
            bottom: Some(&offsets),
            left: Some(&offsets),
            display: Some("flex"),
            align_items: Some("center"),
            justify_content: Some("flex-start"),
            sx: "border-radius:4px;",
        },
    );
    assert!(large.contains("margin:16px;"));
    assert!(large.contains("padding:2rem;"));
    assert!(large.contains("font-size:18px;"));
    assert!(large.contains("font-weight:600;"));
    assert!(large.contains("line-height:28px;"));
    assert!(large.contains("letter-spacing:1px;"));
    assert!(large.contains("color:#333333;"));
    assert!(large.contains("background-color:#e0e0e0;"));
    assert!(large.contains("width:50%;"));
    assert!(large.contains("height:400px;"));
    assert!(large.contains("min-width:360px;"));
    assert!(large.contains("max-width:720px;"));
    assert!(large.contains("min-height:180px;"));
    assert!(large.contains("max-height:360px;"));
    assert!(large.contains("position:absolute;"));
    assert!(large.contains("top:8px;"));
}
