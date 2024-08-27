use antsy::{styled, Color};

fn main() {
    let hello = styled!("Hello").fg(Color::Red).bold();
    let world = styled!(
        "Wor{}ld",
        styled!("he{}he", styled!("haha")).fg(Color::Blue)
    )
    .fg(Color::Cyan)
    .inverse();

    println!("{hello}, {world}!");

    println!("{:?}", world.to_string());
    println!("{:?}", world.style);

    println!("{}", styled!("strikethrough").crossed());
}
