#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

impl Style {
    #[inline]
    pub const fn new() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            attributes: Attributes::EMPTY,
        }
    }

    #[inline]
    pub const fn fg(self, color: Color) -> Self {
        Self { fg: color, ..self }
    }

    #[inline]
    pub const fn bg(self, color: Color) -> Self {
        Self { bg: color, ..self }
    }

    #[inline]
    pub const fn attributes(self, attributes: Attributes) -> Self {
        Self {
            attributes: self.attributes.or(attributes),
            ..self
        }
    }

    #[inline]
    pub const fn bold(self) -> Self {
        self.attributes(Attributes::BOLD)
    }

    #[inline]
    pub const fn dim(self) -> Self {
        self.attributes(Attributes::DIM)
    }

    #[inline]
    pub const fn italic(self) -> Self {
        self.attributes(Attributes::ITALIC)
    }

    #[inline]
    pub const fn underlined(self) -> Self {
        self.attributes(Attributes::UNDERLINED)
    }

    #[inline]
    pub const fn blinking(self) -> Self {
        self.attributes(Attributes::BLINKING)
    }

    #[inline]
    pub const fn inverse(self) -> Self {
        self.attributes(Attributes::INVERSE)
    }

    #[inline]
    pub const fn hidden(self) -> Self {
        self.attributes(Attributes::HIDDEN)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Rgb(u8, u8, u8),
    Indexed(u8),

    #[default]
    Default,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes(u8);

impl Attributes {
    pub const EMPTY: Self = Self(0);

    pub const BOLD: Self = Self(1 << 0);
    pub const DIM: Self = Self(1 << 1);
    pub const ITALIC: Self = Self(1 << 2);
    pub const UNDERLINED: Self = Self(1 << 3);
    pub const BLINKING: Self = Self(1 << 4);
    pub const INVERSE: Self = Self(1 << 5);
    pub const HIDDEN: Self = Self(1 << 6);
    pub const CROSSED: Self = Self(1 << 7);

    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
}
