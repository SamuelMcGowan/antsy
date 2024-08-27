use crate::Style;

pub use std::fmt::Formatter;

#[macro_export]
macro_rules! styled {
    ($($tt:tt)+) => {
        $crate::styled::Styled::new($crate::styled::FormatArgsCallback::new(|f| write!(f, $($tt)+)))
    };
}

use std::cell::Cell;
use std::fmt;

pub struct Styled<T> {
    pub style: Style,
    pub content: T,
}

impl<T> Styled<T> {
    #[inline]
    pub const fn new(content: T) -> Self {
        Self {
            style: Style::new(),
            content,
        }
    }
}

impl<F: fmt::Display> fmt::Display for Styled<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset_style = RESET_STYLE.get();

        RESET_STYLE.set(self.style);

        self.style.fmt(f)?;
        self.content.fmt(f)?;
        reset_style.fmt(f)?;

        RESET_STYLE.set(reset_style);

        Ok(())
    }
}

thread_local! {
    #[doc(hidden)]
    pub static RESET_STYLE: Cell<Style> = const { Cell::new(Style::new()) };
}

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
