use std::fmt;

macro_rules! impl_style_builder_methods {
    ($self:ident => $style:expr) => {
        #[inline]
        pub const fn fg(mut $self: Self, color: Color) -> Self {
            $style.fg = color;
            $self
        }

        #[inline]
        pub const fn bg(mut $self: Self, color: Color) -> Self {
            $style.bg = color;
            $self
        }

        #[inline]
        pub const fn attributes(mut $self: Self, attributes: Attributes) -> Self {
            $style.attributes = $style.attributes.or(attributes);
            $self
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
    };
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    impl_style_builder_methods!(self => self);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Styled<T> {
    pub content: T,
    pub style: Style,
}

impl<T> Styled<T> {
    #[inline]
    pub const fn new(content: T) -> Self {
        Self {
            style: Style::new(),
            content,
        }
    }

    impl_style_builder_methods!(self => self.style);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Color {
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    #[inline]
    pub const fn indexed(i: u8) -> Self {
        Self::Indexed(i)
    }
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

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_set();

        macro_rules! impl_debug {
            ($($name:ident)*) => {
                $(
                    if self.contains(Attributes::$name) {
                        f.entry(&format_args!("{}", stringify!($name)));
                    }
                )*
            };
        }

        impl_debug! {
            BOLD
            DIM
            ITALIC
            UNDERLINED
            BLINKING
            INVERSE
            HIDDEN
            CROSSED
        }

        f.finish()
    }
}
