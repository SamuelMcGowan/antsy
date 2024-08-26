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

        match self.fg {
            Color::Rgb(r, g, b) => write!(f, ";38;2;{r};{g};{b}")?,
            Color::Indexed(i) => write!(f, ";38;5;{i}")?,

            Color::Default => {}
            Color::Black => write!(f, ";30")?,
            Color::Red => write!(f, ";31")?,
            Color::Green => write!(f, ";32")?,
            Color::Yellow => write!(f, ";33")?,
            Color::Blue => write!(f, ";34")?,
            Color::Magenta => write!(f, ";35")?,
            Color::Cyan => write!(f, ";36")?,
            Color::White => write!(f, ";37")?,
        }

        match self.bg {
            Color::Rgb(r, g, b) => write!(f, ";48;2;{r};{g};{b}")?,
            Color::Indexed(i) => write!(f, ";48;5;{i}")?,

            Color::Default => {}
            Color::Black => write!(f, ";40")?,
            Color::Red => write!(f, ";41")?,
            Color::Green => write!(f, ";42")?,
            Color::Yellow => write!(f, ";43")?,
            Color::Blue => write!(f, ";44")?,
            Color::Magenta => write!(f, ";45")?,
            Color::Cyan => write!(f, ";46")?,
            Color::White => write!(f, ";47")?,
        }

        if self.attributes.contains(Attributes::BOLD) {
            write!(f, ";1")?;
        }

        if self.attributes.contains(Attributes::DIM) {
            write!(f, ";2")?;
        }

        if self.attributes.contains(Attributes::ITALIC) {
            write!(f, ";3")?;
        }

        if self.attributes.contains(Attributes::UNDERLINED) {
            write!(f, ";4")?;
        }

        if self.attributes.contains(Attributes::BLINKING) {
            write!(f, ";5")?;
        }

        if self.attributes.contains(Attributes::INVERSE) {
            write!(f, ";7")?;
        }

        if self.attributes.contains(Attributes::HIDDEN) {
            write!(f, ";8")?;
        }

        if self.attributes.contains(Attributes::CROSSED) {
            write!(f, ";9")?;
        }

        write!(f, "m")
    }
}
