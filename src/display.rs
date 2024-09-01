use core::{
    fmt::{self, Display},
    str,
};

use crate::style::{Attributes, Style, Styled};
use crate::{AnsiColor, Color, Hyperlink};

#[cfg(feature = "nested_styles")]
use core::cell::Cell;

#[cfg(feature = "nested_styles")]
thread_local! {
    static RESET_STYLE: Cell<Style> = const { Cell::new(Style::new()) };
}

impl<T: fmt::Display> fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if crate::is_style_enabled() {
            write_styled(f, self.style, &self.content)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !crate::is_style_enabled() {
            return Ok(());
        }

        write_style(f, *self)
    }
}

impl<T: fmt::Display, L: fmt::Display> fmt::Display for Hyperlink<T, L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !crate::is_style_enabled() {
            return self.content.fmt(f);
        }

        f.write_str("\x1b]8;;")?;
        self.uri.fmt(f)?;
        f.write_str("\x1b\\")?;

        write_styled(f, self.style, &self.content)?;

        f.write_str("\x1b]8;;\x1b\\")?;

        Ok(())
    }
}

#[cfg(not(feature = "nested_styles"))]
#[inline]
fn write_styled(f: &mut fmt::Formatter, style: Style, content: &impl fmt::Display) -> fmt::Result {
    style.fmt(f)?;
    content.fmt(f)?;
    f.write_str("\x1b[0")
}

#[cfg(feature = "nested_styles")]
#[inline]
fn write_styled(f: &mut fmt::Formatter, style: Style, content: &impl fmt::Display) -> fmt::Result {
    let reset_style = RESET_STYLE.get();

    RESET_STYLE.set(style);

    style.fmt(f)?;
    content.fmt(f)?;
    reset_style.fmt(f)?;

    RESET_STYLE.set(reset_style);

    Ok(())
}

#[inline]
fn write_style(f: &mut fmt::Formatter, style: Style) -> fmt::Result {
    f.write_str("\x1b[0")?;

    write_fg_color(f, style.fg)?;
    write_bg_color(f, style.bg)?;

    write_attributes(f, style.attributes)?;

    f.write_str("m")
}

// Separate functions for formatting colors shows performance improvement.
macro_rules! impl_write_color {
    ($name:ident $prefix:literal $bright_prefix:literal) => {
        #[inline]
        fn $name(f: &mut fmt::Formatter, color: Color) -> fmt::Result {
            match color {
                Color::Default => Ok(()),

                Color::Ansi(color) => match color {
                    AnsiColor::Black => f.write_str(concat!(";", $prefix, "0")),
                    AnsiColor::Red => f.write_str(concat!(";", $prefix, "1")),
                    AnsiColor::Green => f.write_str(concat!(";", $prefix, "2")),
                    AnsiColor::Yellow => f.write_str(concat!(";", $prefix, "3")),
                    AnsiColor::Blue => f.write_str(concat!(";", $prefix, "4")),
                    AnsiColor::Magenta => f.write_str(concat!(";", $prefix, "5")),
                    AnsiColor::Cyan => f.write_str(concat!(";", $prefix, "6")),
                    AnsiColor::White => f.write_str(concat!(";", $prefix, "7")),

                    AnsiColor::BrightBlack => f.write_str(concat!(";", $bright_prefix, "0")),
                    AnsiColor::BrightRed => f.write_str(concat!(";", $bright_prefix, "1")),
                    AnsiColor::BrightGreen => f.write_str(concat!(";", $bright_prefix, "2")),
                    AnsiColor::BrightYellow => f.write_str(concat!(";", $bright_prefix, "3")),
                    AnsiColor::BrightBlue => f.write_str(concat!(";", $bright_prefix, "4")),
                    AnsiColor::BrightMagenta => f.write_str(concat!(";", $bright_prefix, "5")),
                    AnsiColor::BrightCyan => f.write_str(concat!(";", $bright_prefix, "6")),
                    AnsiColor::BrightWhite => f.write_str(concat!(";", $bright_prefix, "7")),
                },

                Color::Ansi256(i) => {
                    f.write_str(concat!(";", $prefix, "8;5;"))?;
                    i.fmt(f)
                }

                Color::Rgb(r, g, b) => {
                    f.write_str(concat!(";", $prefix, "8;2;"))?;
                    r.fmt(f)?;
                    f.write_str(";")?;
                    g.fmt(f)?;
                    f.write_str(";")?;
                    b.fmt(f)
                }
            }
        }
    };
}

impl_write_color!(write_fg_color "3" "9");
impl_write_color!(write_bg_color "4" "10");

fn write_attributes(f: &mut fmt::Formatter, attributes: Attributes) -> fmt::Result {
    f.write_str(ATTRIBUTE_LOOKUP[attributes.into_bits() as usize])
}

// Since the attributes are only 8 bits, we can use a lookup table.
const ATTRIBUTE_LOOKUP: [&str; 256] = [
    "",
    ";1",
    ";2",
    ";2;1",
    ";3",
    ";3;1",
    ";3;2",
    ";3;2;1",
    ";4",
    ";4;1",
    ";4;2",
    ";4;2;1",
    ";4;3",
    ";4;3;1",
    ";4;3;2",
    ";4;3;2;1",
    ";5",
    ";5;1",
    ";5;2",
    ";5;2;1",
    ";5;3",
    ";5;3;1",
    ";5;3;2",
    ";5;3;2;1",
    ";5;4",
    ";5;4;1",
    ";5;4;2",
    ";5;4;2;1",
    ";5;4;3",
    ";5;4;3;1",
    ";5;4;3;2",
    ";5;4;3;2;1",
    ";7",
    ";7;1",
    ";7;2",
    ";7;2;1",
    ";7;3",
    ";7;3;1",
    ";7;3;2",
    ";7;3;2;1",
    ";7;4",
    ";7;4;1",
    ";7;4;2",
    ";7;4;2;1",
    ";7;4;3",
    ";7;4;3;1",
    ";7;4;3;2",
    ";7;4;3;2;1",
    ";7;5",
    ";7;5;1",
    ";7;5;2",
    ";7;5;2;1",
    ";7;5;3",
    ";7;5;3;1",
    ";7;5;3;2",
    ";7;5;3;2;1",
    ";7;5;4",
    ";7;5;4;1",
    ";7;5;4;2",
    ";7;5;4;2;1",
    ";7;5;4;3",
    ";7;5;4;3;1",
    ";7;5;4;3;2",
    ";7;5;4;3;2;1",
    ";8",
    ";8;1",
    ";8;2",
    ";8;2;1",
    ";8;3",
    ";8;3;1",
    ";8;3;2",
    ";8;3;2;1",
    ";8;4",
    ";8;4;1",
    ";8;4;2",
    ";8;4;2;1",
    ";8;4;3",
    ";8;4;3;1",
    ";8;4;3;2",
    ";8;4;3;2;1",
    ";8;5",
    ";8;5;1",
    ";8;5;2",
    ";8;5;2;1",
    ";8;5;3",
    ";8;5;3;1",
    ";8;5;3;2",
    ";8;5;3;2;1",
    ";8;5;4",
    ";8;5;4;1",
    ";8;5;4;2",
    ";8;5;4;2;1",
    ";8;5;4;3",
    ";8;5;4;3;1",
    ";8;5;4;3;2",
    ";8;5;4;3;2;1",
    ";8;7",
    ";8;7;1",
    ";8;7;2",
    ";8;7;2;1",
    ";8;7;3",
    ";8;7;3;1",
    ";8;7;3;2",
    ";8;7;3;2;1",
    ";8;7;4",
    ";8;7;4;1",
    ";8;7;4;2",
    ";8;7;4;2;1",
    ";8;7;4;3",
    ";8;7;4;3;1",
    ";8;7;4;3;2",
    ";8;7;4;3;2;1",
    ";8;7;5",
    ";8;7;5;1",
    ";8;7;5;2",
    ";8;7;5;2;1",
    ";8;7;5;3",
    ";8;7;5;3;1",
    ";8;7;5;3;2",
    ";8;7;5;3;2;1",
    ";8;7;5;4",
    ";8;7;5;4;1",
    ";8;7;5;4;2",
    ";8;7;5;4;2;1",
    ";8;7;5;4;3",
    ";8;7;5;4;3;1",
    ";8;7;5;4;3;2",
    ";8;7;5;4;3;2;1",
    ";9",
    ";9;1",
    ";9;2",
    ";9;2;1",
    ";9;3",
    ";9;3;1",
    ";9;3;2",
    ";9;3;2;1",
    ";9;4",
    ";9;4;1",
    ";9;4;2",
    ";9;4;2;1",
    ";9;4;3",
    ";9;4;3;1",
    ";9;4;3;2",
    ";9;4;3;2;1",
    ";9;5",
    ";9;5;1",
    ";9;5;2",
    ";9;5;2;1",
    ";9;5;3",
    ";9;5;3;1",
    ";9;5;3;2",
    ";9;5;3;2;1",
    ";9;5;4",
    ";9;5;4;1",
    ";9;5;4;2",
    ";9;5;4;2;1",
    ";9;5;4;3",
    ";9;5;4;3;1",
    ";9;5;4;3;2",
    ";9;5;4;3;2;1",
    ";9;7",
    ";9;7;1",
    ";9;7;2",
    ";9;7;2;1",
    ";9;7;3",
    ";9;7;3;1",
    ";9;7;3;2",
    ";9;7;3;2;1",
    ";9;7;4",
    ";9;7;4;1",
    ";9;7;4;2",
    ";9;7;4;2;1",
    ";9;7;4;3",
    ";9;7;4;3;1",
    ";9;7;4;3;2",
    ";9;7;4;3;2;1",
    ";9;7;5",
    ";9;7;5;1",
    ";9;7;5;2",
    ";9;7;5;2;1",
    ";9;7;5;3",
    ";9;7;5;3;1",
    ";9;7;5;3;2",
    ";9;7;5;3;2;1",
    ";9;7;5;4",
    ";9;7;5;4;1",
    ";9;7;5;4;2",
    ";9;7;5;4;2;1",
    ";9;7;5;4;3",
    ";9;7;5;4;3;1",
    ";9;7;5;4;3;2",
    ";9;7;5;4;3;2;1",
    ";9;8",
    ";9;8;1",
    ";9;8;2",
    ";9;8;2;1",
    ";9;8;3",
    ";9;8;3;1",
    ";9;8;3;2",
    ";9;8;3;2;1",
    ";9;8;4",
    ";9;8;4;1",
    ";9;8;4;2",
    ";9;8;4;2;1",
    ";9;8;4;3",
    ";9;8;4;3;1",
    ";9;8;4;3;2",
    ";9;8;4;3;2;1",
    ";9;8;5",
    ";9;8;5;1",
    ";9;8;5;2",
    ";9;8;5;2;1",
    ";9;8;5;3",
    ";9;8;5;3;1",
    ";9;8;5;3;2",
    ";9;8;5;3;2;1",
    ";9;8;5;4",
    ";9;8;5;4;1",
    ";9;8;5;4;2",
    ";9;8;5;4;2;1",
    ";9;8;5;4;3",
    ";9;8;5;4;3;1",
    ";9;8;5;4;3;2",
    ";9;8;5;4;3;2;1",
    ";9;8;7",
    ";9;8;7;1",
    ";9;8;7;2",
    ";9;8;7;2;1",
    ";9;8;7;3",
    ";9;8;7;3;1",
    ";9;8;7;3;2",
    ";9;8;7;3;2;1",
    ";9;8;7;4",
    ";9;8;7;4;1",
    ";9;8;7;4;2",
    ";9;8;7;4;2;1",
    ";9;8;7;4;3",
    ";9;8;7;4;3;1",
    ";9;8;7;4;3;2",
    ";9;8;7;4;3;2;1",
    ";9;8;7;5",
    ";9;8;7;5;1",
    ";9;8;7;5;2",
    ";9;8;7;5;2;1",
    ";9;8;7;5;3",
    ";9;8;7;5;3;1",
    ";9;8;7;5;3;2",
    ";9;8;7;5;3;2;1",
    ";9;8;7;5;4",
    ";9;8;7;5;4;1",
    ";9;8;7;5;4;2",
    ";9;8;7;5;4;2;1",
    ";9;8;7;5;4;3",
    ";9;8;7;5;4;3;1",
    ";9;8;7;5;4;3;2",
    ";9;8;7;5;4;3;2;1",
];
