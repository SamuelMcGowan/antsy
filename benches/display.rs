use antsy::lazy_format_args;
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
    use antsy::{styled, Color};

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

fn benchmark_yansi() {
    use yansi::Paint;

    let styled = "Hello".bold().red();
    black_box_fmt(Paint::new(styled));
}

fn benchmark_antsy_error_message() {
    use antsy::{styled, Color};

    let styled = lazy_format_args!(
        "{}: {}",
        styled!("Error").bold().fg(Color::red()),
        styled!("something went wrong").italic()
    );
    black_box_fmt(styled)
}

fn benchmark_colored_error_message() {
    use colored::Colorize;

    let styled = lazy_format_args!(
        "{}: {}",
        "Error".bold().red(),
        "something went wrong".italic()
    );

    black_box_fmt(styled);
}

fn benchmark_owo_colors_error_message() {
    use owo_colors::OwoColorize;
    use owo_colors::Style;

    let styled = lazy_format_args!(
        "{}: {}",
        "Error".style(Style::new().bold().fg::<owo_colors::colors::Red>()),
        "something went wrong".style(Style::new().italic())
    );
    black_box_fmt(styled);
}

fn benchmark_yansi_error_message() {
    use yansi::Paint;

    let styled = lazy_format_args!(
        "{}: {}",
        "Error".bold().red(),
        "something went wrong".italic()
    );
    black_box_fmt(Paint::new(styled));
}

fn benchmark(c: &mut Criterion) {
    colored::control::set_override(true);
    antsy::set_style_mode(antsy::StyleMode::enable());

    c.bench_function("antsy", |b| b.iter(benchmark_antsy));
    c.bench_function("colored", |b| b.iter(benchmark_colored));
    c.bench_function("owo-colors", |b| b.iter(benchmark_owo_colors));
    c.bench_function("yansi", |b| b.iter(benchmark_yansi));
}

fn benchmark_error_message(c: &mut Criterion) {
    colored::control::set_override(true);
    antsy::set_style_mode(antsy::StyleMode::enable());

    c.bench_function("err_antsy", |b| b.iter(benchmark_antsy_error_message));
    c.bench_function("err_colored", |b| b.iter(benchmark_colored_error_message));
    c.bench_function("err_owo-colors", |b| {
        b.iter(benchmark_owo_colors_error_message)
    });
    c.bench_function("err_yansi", |b| b.iter(benchmark_yansi_error_message));
}

criterion_group!(benches, benchmark, benchmark_error_message);
criterion_main!(benches);
