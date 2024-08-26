pub mod style;

mod macros {
    // Imported for doc-comment.
    #[allow(unused_imports)]
    use crate::style::Styled;

    /// Like [`format_args!`], but returns a [`Styled`] value.
    #[macro_export]
    macro_rules! styled {
        ($($tt:tt)+) => {
            $crate::style::Styled::new(
                format_args!($($tt)+),
                $crate::style::Style::new()
            )
        };
    }
}
