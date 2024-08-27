use core::sync::atomic::{AtomicU8, Ordering};

static STYLING_ENABLED: AtomicU8 = AtomicU8::new(0);

/// Set the style mode.
///
/// Returns `true` if styling was enabled.
///
/// By default, the style mode is [`StyleMode::Auto`], so unless the style mode
/// has been changed, there is no need to call `set_style_mode(StyleMode::Auto)`.
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

/// Whether to enable or disable styling.
///
/// Defaults to `Auto`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleMode {
    /// Auto-detect if styling is supported. See [`StyleMode::auto`] for details.
    #[default]
    Auto,

    /// Force styling on/off.
    Force(bool),
}

impl StyleMode {
    /// Auto-detect if styling is supported.
    ///
    /// Specifically, if the `TERM` environment variable is set to `dumb`
    /// or `NO_COLOR` is set, then styling is disabled. Otherwise it is enabled.
    ///
    /// In non-std environments, this always enables styling.
    #[inline]
    pub const fn auto() -> Self {
        Self::Auto
    }

    /// Force-enable styling.
    #[inline]
    pub const fn enable() -> Self {
        Self::Force(true)
    }

    /// Force-disable styling.
    #[inline]
    pub const fn disable() -> Self {
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
