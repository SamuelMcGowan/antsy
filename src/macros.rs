use core::fmt;

/// Create a styled value (see [`Styled`](crate::Styled)).
///
/// # Examples
///
/// ```rust
/// use antsy::{Color, styled};
///
/// println!(
///     "{}, {}!",
///     styled!("Hello").fg(Color::Red).bold(),
///     styled!("World").fg(Color::Cyan).inverse(),
/// );
/// ```
#[macro_export]
macro_rules! styled {
    ($($tt:tt)+) => {
        $crate::Styled::new($crate::macros::FormatArgsCallback::new(|f| write!(f, $($tt)+)))
    };
}

/// Create a styled hyperlink (see [`Hyperlink`](crate::Hyperlink)) to the given URI.
///
/// # Examples
///
/// ```rust
/// use antsy::{Color, hyperlinked};
///
/// println!(
///     "{}",
///     hyperlink!("https://google.com"; "Google")
///         .bold()
///         .fg(Color::Green)
/// );
/// ```
#[macro_export]
macro_rules! hyperlink {
    ($uri:expr; $($tt:tt)+) => {
        $crate::Hyperlink::new($uri, $crate::macros::FormatArgsCallback::new(|f| write!(f, $($tt)+)))
    };
}

// We use a callback to avoid storing a `fmt::Arguments`, since `fmt::Arguments` causes lifetime issues
// due to being (?) a temporary value.
pub struct FormatArgsCallback<F>(F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> FormatArgsCallback<F> {
    #[inline]
    pub const fn new(callback: F) -> Self {
        Self(callback)
    }
}

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for FormatArgsCallback<F> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}
