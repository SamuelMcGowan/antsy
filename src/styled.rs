use crate::Style;

pub use std::fmt::Formatter;

#[macro_export]
macro_rules! styled {
    ($($tt:tt)+) => {
        $crate::styled::Styled::new(|f, style| {
                let reset_style = $crate::styled::RESET_STYLE.get();

                $crate::styled::RESET_STYLE.set(style);

                write!(f, "{style}")?;
                write!(f, $($tt)+)?;
                write!(f, "{reset_style}")?;


                $crate::styled::RESET_STYLE.set(reset_style);

                Ok(())
        })
    };
}

use std::cell::Cell;
use std::fmt;

pub struct Styled<F> {
    callback: F,
    pub style: Style,
}

impl<F: Fn(&mut fmt::Formatter, Style) -> fmt::Result> Styled<F> {
    pub const fn new(callback: F) -> Self {
        Self {
            callback,
            style: Style::new(),
        }
    }
}

impl<F: Fn(&mut fmt::Formatter, Style) -> fmt::Result> fmt::Display for Styled<F> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.callback)(f, self.style)
    }
}

thread_local! {
    #[doc(hidden)]
    pub static RESET_STYLE: Cell<Style> = const { Cell::new(Style::new()) };
}
