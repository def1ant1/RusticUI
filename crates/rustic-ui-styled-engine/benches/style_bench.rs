use criterion::{criterion_group, criterion_main, Criterion};
use mui_styled_engine::{css, Style};

fn rust_style() {
    Style::new(css!("color: red;")).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rust_style", |b| b.iter(rust_style));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
