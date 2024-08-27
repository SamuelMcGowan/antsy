use core::fmt;

#[macro_export]
macro_rules! styled {
    ($($tt:tt)+) => {
        $crate::Styled::new($crate::styled_macro::FormatArgsCallback::new(|f| write!(f, $($tt)+)))
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
