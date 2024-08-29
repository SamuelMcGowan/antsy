use antsy::{styled, Color};

fn main() {
    println!(
        "{}",
        styled!(
            "the quick {} fox jumps over the lazy dog",
            styled!("brown").fg(Color::rgb(161, 123, 90))
        )
        .fg(Color::cyan())
    );
}
