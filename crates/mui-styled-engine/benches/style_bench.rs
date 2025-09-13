use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mui_styled_engine::{css_with_theme, Theme};

fn bench_generated_css(c: &mut Criterion) {
    let theme = Theme::default();
    c.bench_function("css_with_theme", |b| {
        b.iter(|| {
            let style = css_with_theme!(theme, r#"color: ${c};"#, c = theme.palette.primary.clone());
            black_box(style.get_style_str().len());
        })
    });

    c.bench_function("dynamic_format", |b| {
        b.iter(|| {
            let s = format!("color:{};", theme.palette.primary);
            black_box(s.len());
        })
    });
}

criterion_group!(benches, bench_generated_css);
criterion_main!(benches);

