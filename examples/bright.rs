use antsy::styled;

fn main() {
    println!("{}", styled!("normal").green().bold());
    println!("{}", styled!("bright").bright_green().bold());
}
