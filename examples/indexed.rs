use antsy::{Color, Styled};

fn main() {
    for i in 0..255 {
        println!("{}", Styled::new(i).fg(Color::indexed(i)));
    }
}
