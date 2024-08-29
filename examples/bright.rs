use antsy::{styled, Color};

fn main() {
    println!("{}", styled!("normal").fg(Color::green()).bold());
    println!("{}", styled!("bright").fg(Color::bright_green()).bold());
}
