use antsy::styled;

fn main() {
    println!(
        "{}",
        styled!(
            "the quick {} fox jumps over the lazy dog",
            styled!("brown").rgb_color(161, 123, 90)
        )
        .cyan()
    );
}
