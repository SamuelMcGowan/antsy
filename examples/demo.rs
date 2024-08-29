use antsy::{hyperlink, styled, Color};

fn main() {
    let hello = styled!("Hello").fg(Color::red()).bold();
    let world = styled!(
        "Wor{}ld",
        styled!("he{}he", styled!("haha")).fg(Color::blue())
    )
    .fg(Color::cyan())
    .inverted();

    println!("{hello}, {world}!");

    println!("{:?}", world.to_string());
    println!("{:?}", world.style);

    println!("{}", styled!("strikethrough").crossed());

    println!(
        "{}",
        hyperlink!("https://google.com"; "Google")
            .bold()
            .fg(Color::green())
    );
}
