use std::fmt;

use crate::style::{Attributes, Color, Style, Styled};

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.style.fmt(f)?;
        self.content.fmt(f)?;

        // Reset to default style.
        Style::new().fmt(f)
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !crate::color_choice::color_enabled() {
            return Ok(());
        }

        write!(f, "\x1b[0")?;

        macro_rules! write_colors {
            ($name:ident = $prefix:literal) => {
                match self.$name {
                    Color::Rgb(r, g, b) => write!(f, ";{}8;2;{r};{g};{b}", $prefix)?,
                    Color::Indexed(i) => write!(f, ";{}8;5;{i}", $prefix)?,

                    Color::Default => {}

                    Color::Black => write!(f, ";{}0", $prefix)?,
                    Color::Red => write!(f, ";{}1", $prefix)?,
                    Color::Green => write!(f, ";{}2", $prefix)?,
                    Color::Yellow => write!(f, ";{}3", $prefix)?,
                    Color::Blue => write!(f, ";{}4", $prefix)?,
                    Color::Magenta => write!(f, ";{}5", $prefix)?,
                    Color::Cyan => write!(f, ";{}6", $prefix)?,
                    Color::White => write!(f, ";{}7", $prefix)?,
                }
            };
        }

        write_colors!(fg = "3");
        write_colors!(bg = "4");

        macro_rules! write_attributes {
            ($($name:ident = $ansi:expr),*) => {
                $(
                    if self.attributes.contains(Attributes::$name) {
                        write!(f, ";{}", $ansi)?;
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
            INVERSE = "7",
            HIDDEN = "8",
            CROSSED = "9"
        );

        write!(f, "m")
    }
}
