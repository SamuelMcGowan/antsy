use stylic::{hyperlink, styled};

fn main() {
    let hello = styled!("Hello").bold().red().on_blue();
    let world = styled!("Wor{}ld", styled!("he{}he", styled!("haha")).blue())
        .cyan()
        .inverted();

    println!("{hello}, {world}!");

    println!("{:?}", world.to_string());
    println!("{:?}", world.style);

    println!("{}", styled!("strikethrough").crossed());

    println!(
        "{}",
        hyperlink!("https://google.com"; "Google").bold().green()
    );
}
