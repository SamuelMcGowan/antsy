use core::fmt;

/// Create a styled value (see [`Styled`](crate::Styled)).
///
/// # Examples
///
/// ```rust
/// use stylic::{Color, styled};
///
/// println!(
///     "{}, {}!",
///     styled!("Hello").red().bold(),
///     styled!("World").cyan().inverted(),
/// );
/// ```
#[macro_export]
macro_rules! styled {
    ($($tt:tt)+) => {
        $crate::Styled::new($crate::macros::FormatArgsCallback::new(|f| write!(f, $($tt)+)))
    };
}

/// Create a styled value with the given style (see [`Styled`](crate::Styled)).
///
/// # Examples
///
/// ```rust
/// use stylic::{apply, Color, Style};
///
/// const MY_STYLE: Style = Style::new().blue().bold();
/// println!("{}!", apply!(MY_STYLE => "Hello"));
/// ```
#[macro_export]
macro_rules! apply {
    ($style:expr => $($tt:tt)+) => {
        $crate::Styled {
            content: $crate::lazy_format_args!($($tt)+),
            style: $style,
        }
    };
}

/// Create a styled hyperlink with the given style and URI (see [`Hyperlink`](crate::Hyperlink)).
///
/// # Examples
///
/// ```rust
/// use stylic::{apply_hyperlink, Color, Style};
///
/// const MY_STYLE: Style = Style::new().yellow().italic();
/// println!(
///     "{}",
///     apply_hyperlink!(MY_STYLE => "https://google.com"; "Google")
/// );
/// ```
#[macro_export]
macro_rules! apply_hyperlink {
    ($style:expr => $uri:expr; $($tt:tt)+) => {
        $crate::Hyperlink {
            uri: $uri,
            content: $crate::lazy_format_args!($($tt)+),
            style: $style,
        }
    };
}

/// Create a styled hyperlink (see [`Hyperlink`](crate::Hyperlink)) to the given URI.
///
/// # Examples
///
/// ```rust
/// use stylic::{Color, hyperlink};
///
/// println!(
///     "{}",
///     hyperlink!("https://google.com"; "Google")
///         .bold()
///         .green()
/// );
/// ```
#[macro_export]
macro_rules! hyperlink {
    ($uri:expr; $($tt:tt)+) => {
        $crate::Hyperlink::new($uri, $crate::lazy_format_args!($($tt)+))
    };
}

/// Like [`format_args`], but evaluates the format expression whenever [`Display::fmt`](fmt::Display::fmt) is called,
/// rather than returning a [`fmt::Arguments`] directly.
///
/// This avoids lifetime issues caused by temporary lifetimes. See <https://github.com/rust-lang/rust/issues/92698>.
#[macro_export]
macro_rules! lazy_format_args {
    ($($tt:tt)+) => {
        $crate::macros::FormatArgsCallback::new(|f| write!(f, $($tt)+))
    };
}

/// See [`lazy_format_args`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
