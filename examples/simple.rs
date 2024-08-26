use antsy::{styled, Color};

fn main() {
    println!(
        "{}, {}!",
        styled!("Hello").fg(Color::Red).bold(),
        styled!("World").italic().inverse(),
    );
}
