use core::sync::atomic::{AtomicU8, Ordering};

static STYLING_ENABLED: AtomicU8 = AtomicU8::new(0);

/// Set the style mode.
///
/// Returns `true` if styling is now enabled.
pub fn set_style_mode(mode: StyleMode) -> bool {
    let enable_styling = mode.should_enable_styling();
    STYLING_ENABLED.store(if enable_styling { 2 } else { 1 }, Ordering::Relaxed);
    enable_styling
}

/// Returns `true` if styling is enabled.
pub fn style_enabled() -> bool {
    match STYLING_ENABLED.load(Ordering::Relaxed) {
        // lazy initialization
        0 => set_style_mode(StyleMode::Auto),

        1 => false,
        2 => true,

        _ => unreachable!(),
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleMode {
    /// Auto-detect if styling is supported.
    #[default]
    Auto,

    /// Force styling on/off.
    Force(bool),
}

impl StyleMode {
    /// Auto-detect if styling is supported.
    #[inline]
    pub const fn auto() -> Self {
        Self::Auto
    }

    /// Force styling on.
    #[inline]
    pub const fn always() -> Self {
        Self::Force(true)
    }

    /// Force styling off.
    #[inline]
    pub const fn never() -> Self {
        Self::Force(false)
    }

    fn should_enable_styling(&self) -> bool {
        match self {
            StyleMode::Auto => env_supports_styling(),
            StyleMode::Force(b) => *b,
        }
    }
}

#[cfg(feature = "std")]
fn env_supports_styling() -> bool {
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

#[cfg(not(feature = "std"))]
fn env_supports_styling() -> bool {
    // If there is no `std` we can presume we should use styling since
    // there is no sign that we should not.
    true
}
