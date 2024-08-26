mod color_choice;
mod display;
mod style;

pub use color_choice::*;
pub use style::*;

mod macros {
    // Imported for doc-comment.
    #[allow(unused_imports)]
    use crate::style::Styled;

    /// Like [`format_args!`], but returns a [`Styled`] value.
    #[macro_export]
    macro_rules! styled {
        ($($tt:tt)+) => {
            $crate::Styled::new(
                format_args!($($tt)+),
                $crate::Style::new()
            )
        };
    }
}
