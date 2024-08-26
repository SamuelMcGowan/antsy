use ansi_style_3::{styled, Color};

fn main() {
    println!(
        "{}, {}!",
        styled!("Hello").fg(Color::Red).bold(),
        styled!("World").italic().inverse(),
    );
}
