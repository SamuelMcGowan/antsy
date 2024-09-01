use stylic::Styled;

fn main() {
    for i in 0..255 {
        println!("{}", Styled::new(i).ansi256_color(i));
    }
}
