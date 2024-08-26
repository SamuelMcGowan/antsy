use std::sync::atomic::{AtomicU8, Ordering};

static USE_COLOR: AtomicU8 = AtomicU8::new(0);

/// Set the color choice.
///
/// Returns `true` if the color choice means we should use color.
pub fn set_color_choice(color_choice: ColorChoice) -> bool {
    let use_color = color_choice.should_use_color();
    USE_COLOR.store(if use_color { 2 } else { 1 }, Ordering::Relaxed);
    use_color
}

/// Returns `true` if color is enabled.
pub fn color_enabled() -> bool {
    match USE_COLOR.load(Ordering::Relaxed) {
        // lazy initialization
        0 => set_color_choice(ColorChoice::Auto),

        1 => false,
        2 => true,

        _ => unreachable!(),
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorChoice {
    #[default]
    Auto,
    Force(bool),
}

impl ColorChoice {
    #[inline]
    pub const fn auto() -> Self {
        Self::Auto
    }

    #[inline]
    pub const fn always() -> Self {
        Self::Force(true)
    }

    #[inline]
    pub const fn never() -> Self {
        Self::Force(false)
    }

    fn should_use_color(&self) -> bool {
        match self {
            ColorChoice::Auto => env_supports_color(),
            ColorChoice::Force(b) => *b,
        }
    }
}

fn env_supports_color() -> bool {
    match std::env::var_os("TERM") {
        Some(s) if s == "dumb" => return false,
        None => return false,
        _ => {}
    }

    if std::env::var_os("NO_COLOR").is_some_and(|s| s != "0") {
        return false;
    }

    true
}
