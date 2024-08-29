use antsy::{styled, Color};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::{self, Write};
use std::hint::black_box;

// Avoid measuring allocations by implementing a formatter that doesn't do anything.
fn black_box_fmt(value: impl fmt::Display) {
    struct BlackBoxFormatter;

    impl fmt::Write for BlackBoxFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            black_box(s);
            Ok(())
        }
    }

    let mut f = BlackBoxFormatter;
    let _ = write!(f, "{value}");
}

fn benchmark_antsy() {
    let styled = styled!("Hello").bold().fg(Color::red());
    black_box_fmt(styled);
}

fn benchmark_colored() {
    use colored::Colorize;

    let styled = "Hello".bold().red();
    black_box_fmt(styled);
}

fn benchmark_owo_colors() {
    use owo_colors::OwoColorize;
    use owo_colors::Style;

    let styled = "Hello".style(Style::new().bold().fg::<owo_colors::colors::Red>());
    black_box_fmt(styled);
}

fn benchmark(c: &mut Criterion) {
    colored::control::set_override(true);
    antsy::set_style_mode(antsy::StyleMode::enable());

    c.bench_function("antsy", |b| b.iter(benchmark_antsy));
    c.bench_function("colored", |b| b.iter(benchmark_colored));
    c.bench_function("owo-colors", |b| b.iter(benchmark_owo_colors));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
