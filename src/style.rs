use core::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

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
        pub const fn inverted(self) -> Self {
            self.attributes(Attributes::INVERTED)
        }

        #[inline]
        pub const fn hidden(self) -> Self {
            self.attributes(Attributes::HIDDEN)
        }

        #[inline]
        pub const fn crossed(self) -> Self {
            self.attributes(Attributes::CROSSED)
        }
    };
}

/// A style that can be applied to a value.
///
/// Styles can be written directly (since they implement [`fmt::Display`]) or
/// using a `Styled` value, which can be conveniently created using the
/// [`styled!`](crate::styled) macro.
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

/// A styled value.
///
/// Can be created using the [`styled!`](crate::styled) macro, which supports
/// formatting arguments, or directly with [`Styled::new`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Styled<T> {
    pub content: T,
    pub style: Style,
}

impl<T> Styled<T> {
    /// Create a styled value.
    ///
    /// Use [`styled!`](crate::styled) instead if you want to format the content.
    #[inline]
    pub const fn new(content: T) -> Self {
        Self {
            style: Style::new(),
            content,
        }
    }

    impl_style_builder_methods!(self => self.style);
}

/// A styled hyperlink.
///
/// Can be created using the [`hyperlink!`](crate::hyperlink) macro, which supports
/// formatting arguments in the hyperlink content, or directly with [`Hyperlink::new`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hyperlink<T, L> {
    pub uri: L,
    pub content: T,

    pub style: Style,
}

impl<T, L> Hyperlink<T, L> {
    /// Create a styled hyperlink with the given content and URI.
    ///
    /// Use [`hyperlink!`](crate::hyperlink) instead if you want to format the hyperlink content.
    #[inline]
    pub const fn new(uri: L, content: T) -> Self {
        Self {
            uri,
            content,
            style: Style::new(),
        }
    }

    impl_style_builder_methods!(self => self.style);
}

/// A color.
///
/// RGB and [indexed colors](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
/// may not be supported by all terminals.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// An RGB color.
    Rgb(u8, u8, u8),

    /// An indexed color.
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
    /// Create an RGB color.
    ///
    /// May not be supported by all terminals.
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    /// Create an indexed color.
    ///
    /// See: <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
    ///
    /// May not be supported by all terminals.
    #[inline]
    pub const fn indexed(i: u8) -> Self {
        Self::Indexed(i)
    }
}

/// A set of attributes (bold, italic, etc).
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes(u8);

impl Attributes {
    /// An empty set of attributes.
    pub const EMPTY: Self = Self(0);

    /// An attribute set that enables bold text. See [`Style::bold`].
    pub const BOLD: Self = Self(1 << 0);

    /// An attribute set that enables dimmed text. See [`Style::dim`].
    pub const DIM: Self = Self(1 << 1);

    /// An attribute set that enables italic text. See [`Style::italic`].
    pub const ITALIC: Self = Self(1 << 2);

    /// An attribute set that enables underlined text. See [`Style::underlined`].
    pub const UNDERLINED: Self = Self(1 << 3);

    /// An attribute set that enables blinking text. See [`Style::blinking`].
    pub const BLINKING: Self = Self(1 << 4);

    /// An attribute set that enables inverted text. See [`Style::inverted`].
    pub const INVERTED: Self = Self(1 << 5);

    /// An attribute set that enables hidden text. See [`Style::hidden`].
    pub const HIDDEN: Self = Self(1 << 6);

    /// An attribute set that enables crossed out text. See [`Style::crossed`].
    pub const CROSSED: Self = Self(1 << 7);

    /// Returns `true` if all attributes enabled in `other` are enabled in `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Return a set of attributes containing all the attributes in `self` OR `other`.
    #[inline]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Return a set of attributes containing all the attributes in `self` AND `other`.
    #[inline]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Return a set of attributes containing all the attributes NOT in `self`.
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
            INVERTED
            HIDDEN
            CROSSED
        }

        f.finish()
    }
}

impl BitOr<Attributes> for Attributes {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Attributes {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd<Attributes> for Attributes {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Attributes {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for Attributes {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
