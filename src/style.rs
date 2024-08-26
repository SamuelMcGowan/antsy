pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attributes: Attributes,
}

pub enum Color {
    Rgb(u8, u8, u8),
    Indexed(u8),

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
