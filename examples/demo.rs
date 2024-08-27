use antsy::{hyperlink, styled, Color};

fn main() {
    let hello = styled!("Hello").fg(Color::Red).bold();
    let world = styled!(
        "Wor{}ld",
        styled!("he{}he", styled!("haha")).fg(Color::Blue)
    )
    .fg(Color::Cyan)
    .inverted();

    println!("{hello}, {world}!");

    println!("{:?}", world.to_string());
    println!("{:?}", world.style);

    println!("{}", styled!("strikethrough").crossed());

    println!(
        "{}",
        hyperlink!("https://google.com"; "Google")
            .bold()
            .fg(Color::Green)
    );
}
