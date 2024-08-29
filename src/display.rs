use core::fmt;

use crate::{
    style::{Attributes, Color, Style, Styled},
    Hyperlink,
};

#[cfg(feature = "nested_styles")]
use core::cell::Cell;

#[cfg(feature = "nested_styles")]
thread_local! {
    static RESET_STYLE: Cell<Style> = const { Cell::new(Style::new()) };
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    #[cfg(feature = "nested_styles")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reset_style = RESET_STYLE.get();

        RESET_STYLE.set(self.style);

        self.style.fmt(f)?;
        self.content.fmt(f)?;
        reset_style.fmt(f)?;

        RESET_STYLE.set(reset_style);

        Ok(())
    }

    #[cfg(not(feature = "nested_styles"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.style.fmt(f)?;
        self.content.fmt(f)?;
        write_reset(f)?;
        Ok(())
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !crate::style_enabled() {
            return Ok(());
        }

        f.write_str("\x1b[0")?;

        macro_rules! write_colors {
            ($name:ident = $prefix:literal) => {
                match self.$name {
                    Color::Rgb(r, g, b) => {
                        f.write_str(concat!(";", $prefix, "8;2;"))?;
                        r.fmt(f)?;
                        f.write_str(";")?;
                        g.fmt(f)?;
                        f.write_str(";")?;
                        b.fmt(f)?;
                    }

                    Color::Indexed(i) => {
                        f.write_str(concat!(";", $prefix, "8;5;"))?;
                        i.fmt(f)?;
                    }

                    Color::Default => {}

                    Color::Black => f.write_str(concat!(";", $prefix, "0"))?,
                    Color::Red => f.write_str(concat!(";", $prefix, "1"))?,
                    Color::Green => f.write_str(concat!(";", $prefix, "2"))?,
                    Color::Yellow => f.write_str(concat!(";", $prefix, "3"))?,
                    Color::Blue => f.write_str(concat!(";", $prefix, "4"))?,
                    Color::Magenta => f.write_str(concat!(";", $prefix, "5"))?,
                    Color::Cyan => f.write_str(concat!(";", $prefix, "6"))?,
                    Color::White => f.write_str(concat!(";", $prefix, "7"))?,
                }
            };
        }

        write_colors!(fg = "3");
        write_colors!(bg = "4");

        macro_rules! write_attributes {
            ($($name:ident = $ansi:expr),*) => {
                $(
                    if self.attributes.contains(Attributes::$name) {
                        f.write_str(concat!(";", $ansi))?;
                    }
                )*
            };
        }

        write_attributes!(
            BOLD = "1",
            DIM = "2",
            ITALIC = "3",
            UNDERLINED = "4",
            BLINKING = "5",
            INVERTED = "7",
            HIDDEN = "8",
            CROSSED = "9"
        );

        write!(f, "m")
    }
}

impl<T: fmt::Display, L: fmt::Display> fmt::Display for Hyperlink<T, L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: disable styling while formatting URI.

        f.write_str("\x1b]8;;")?;
        self.uri.fmt(f)?;
        f.write_str("\x1b\\")?;

        Styled {
            content: &self.content,
            style: self.style,
        }
        .fmt(f)?;

        f.write_str("\x1b]8;;\x1b\\")?;

        Ok(())
    }
}

fn write_reset(f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("\x1b[0")
}
