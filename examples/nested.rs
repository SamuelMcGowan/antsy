use antsy::styled;

fn main() {
    println!(
        "{}",
        styled!(
            "the quick {} fox jumps over the lazy dog",
            styled!("brown").color_rgb(161, 123, 90)
        )
        .cyan()
    );
}
