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
    use antsy::styled;

    let styled = styled!("Hello").bold().red();
    black_box_fmt(styled);
}

fn _benchmark_colored() {
    use colored::Colorize;

    let styled = "Hello".bold().red();
    black_box_fmt(styled);
}

fn benchmark_owo_colors_generic() {
    use owo_colors::OwoColorize;

    black_box_fmt("Hello".bold().red());
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
    black_box_fmt(styled);
}

fn benchmark_antsy_error_message() {
    use antsy::styled;

    black_box_fmt(lazy_format_args!(
        "{}: {}",
        styled!("Error").bold().red(),
        styled!("something went wrong").italic()
    ))
}

fn _benchmark_colored_error_message() {
    use colored::Colorize;

    black_box_fmt(lazy_format_args!(
        "{}: {}",
        "Error".bold().red(),
        "something went wrong".italic()
    ));
}

fn benchmark_owo_colors_error_message_generic() {
    use owo_colors::OwoColorize;

    black_box_fmt(lazy_format_args!(
        "{}: {}",
        "Error".bold().red(),
        "something went wrong".italic(),
    ));
}

fn benchmark_owo_colors_error_message() {
    use owo_colors::OwoColorize;
    use owo_colors::Style;

    black_box_fmt(lazy_format_args!(
        "{}: {}",
        "Error".style(Style::new().bold().fg::<owo_colors::colors::Red>()),
        "something went wrong".style(Style::new().italic())
    ));
}

fn benchmark_yansi_error_message() {
    use yansi::Paint;

    black_box_fmt(lazy_format_args!(
        "{}: {}",
        "Error".bold().red(),
        "something went wrong".italic()
    ));
}

fn benchmark(c: &mut Criterion) {
    colored::control::set_override(true);
    antsy::set_style_mode(antsy::StyleMode::enable());

    c.bench_function("antsy", |b| b.iter(benchmark_antsy));
    // c.bench_function("colored", |b| b.iter(benchmark_colored));
    c.bench_function("owo-colors-generic", |b| {
        b.iter(benchmark_owo_colors_generic)
    });
    c.bench_function("owo-colors", |b| b.iter(benchmark_owo_colors));
    c.bench_function("yansi", |b| b.iter(benchmark_yansi));
}

fn benchmark_error_message(c: &mut Criterion) {
    colored::control::set_override(true);
    antsy::set_style_mode(antsy::StyleMode::enable());

    c.bench_function("err_antsy", |b| b.iter(benchmark_antsy_error_message));
    // c.bench_function("err_colored", |b| b.iter(benchmark_colored_error_message));
    c.bench_function("err_owo-colors-generic", |b| {
        b.iter(benchmark_owo_colors_error_message_generic)
    });
    c.bench_function("err_owo-colors", |b| {
        b.iter(benchmark_owo_colors_error_message)
    });
    c.bench_function("err_yansi", |b| b.iter(benchmark_yansi_error_message));
}

criterion_group!(benches, benchmark, benchmark_error_message);
criterion_main!(benches);
