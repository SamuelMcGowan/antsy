use antsy::Styled;

fn main() {
    for i in 0..255 {
        println!("{}", Styled::new(i).ansi256(i));
    }
}
