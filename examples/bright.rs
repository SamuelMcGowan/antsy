use stylic::styled;

fn main() {
    println!("{}", styled!("normal").bold().green());
    println!("{}", styled!("bright").bold().bright_green());
}
