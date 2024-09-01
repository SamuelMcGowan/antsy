use core::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

use crate::color::{impl_color_builder_methods, Color};

macro_rules! impl_style_builder_methods {
    ($self:ident => $style:expr) => {
        impl_color_builder_methods!($self, color =>
            {
                $style.fg = color;
                $self
            },
            {
                $style.bg = color;
                $self
            }
        );

        /// Set the attributes. See [`Attributes`].
        #[inline]
        pub const fn attributes(mut $self: Self, attributes: Attributes) -> Self {
            $style.attributes = $style.attributes.or(attributes);
            $self
        }

        /// Set the bold attribute.
        #[inline]
        pub const fn bold(self) -> Self {
            self.attributes(Attributes::BOLD)
        }

        /// Set the dim attribute.
        #[inline]
        pub const fn dim(self) -> Self {
            self.attributes(Attributes::DIM)
        }

        /// Set the italic attribute.
        #[inline]
        pub const fn italic(self) -> Self {
            self.attributes(Attributes::ITALIC)
        }

        /// Set the underlined attribute.
        #[inline]
        pub const fn underlined(self) -> Self {
            self.attributes(Attributes::UNDERLINED)
        }

        /// Set the blinking attribute.
        ///
        /// Not supported by many terminals.
        #[inline]
        pub const fn blinking(self) -> Self {
            self.attributes(Attributes::BLINKING)
        }

        /// Set the inverted attribute.
        ///
        /// Inverts the foreground and background colors.
        #[inline]
        pub const fn inverted(self) -> Self {
            self.attributes(Attributes::INVERTED)
        }

        /// Set the hidden attribute.
        ///
        /// Makes the text invisible.
        #[inline]
        pub const fn hidden(self) -> Self {
            self.attributes(Attributes::HIDDEN)
        }

        /// Set the crossed attribute.
        ///
        /// Makes the text struck through.
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

    #[inline]
    pub const fn is_default(&self) -> bool {
        matches!(self.fg, Color::Default)
            && matches!(self.bg, Color::Default)
            && self.attributes.is_empty()
    }
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
pub struct Hyperlink<U, T> {
    pub uri: U,
    pub content: T,

    pub style: Style,
}

impl<U, T> Hyperlink<U, T> {
    /// Create a styled hyperlink with the given content and URI.
    ///
    /// Use [`hyperlink!`](crate::hyperlink) instead if you want to format the hyperlink content.
    #[inline]
    pub const fn new(uri: U, content: T) -> Self {
        Self {
            uri,
            content,
            style: Style::new(),
        }
    }

    impl_style_builder_methods!(self => self.style);
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

    #[inline]
    pub(crate) const fn as_bits(&self) -> u8 {
        self.0
    }

    /// Returns `true` if all attributes enabled in `other` are enabled in `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
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
