use antsy::Styled;

fn main() {
    for i in 0..255 {
        println!("{}", Styled::new(i).color_ansi256(i));
    }
}
