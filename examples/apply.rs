use antsy::{apply, apply_hyperlink, Color, Style};

fn main() {
    const MY_STYLE: Style = Style::new().fg(Color::blue()).bold();

    println!("{}!", apply!(MY_STYLE => "Hello"));
    println!(
        "{}",
        apply_hyperlink!(MY_STYLE => "https://rust-lang.org"; "Rust Language")
    );
}
