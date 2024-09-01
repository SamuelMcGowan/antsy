use antsy::{apply, apply_hyperlink, Style};

fn main() {
    const MY_STYLE: Style = Style::new().bold().blue();

    println!("{}!", apply!(MY_STYLE => "Hello"));
    println!(
        "{}",
        apply_hyperlink!(MY_STYLE => "https://rust-lang.org"; "Rust Language")
    );
}
