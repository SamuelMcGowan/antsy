/// A color.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// The default color.
    #[default]
    Default,

    /// An ANSI 4-bit color.
    Ansi(AnsiColor),

    /// An ANSI 8-bit color.
    Ansi256(u8),

    /// An RGB color.
    Rgb(u8, u8, u8),
}

/// An ANSI 4-bit color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    /// Create an RGB color.
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    /// Create an ANSI 4-bit color.
    #[inline]
    pub const fn ansi(color: AnsiColor) -> Self {
        Self::Ansi(color)
    }

    /// Create an ANSI 8-bit color.
    #[inline]
    pub const fn ansi256(i: u8) -> Self {
        Self::Ansi256(i)
    }
}

macro_rules! impl_color_constructors {
    ($($name:ident $variant:ident,)*) => {
        $(
            #[inline]
            pub const fn $name() -> Self {
                Self::Ansi(AnsiColor::$variant)
            }
        )*
    };
}

impl Color {
    impl_color_constructors! {
        black Black,
        red Red,
        green Green,
        yellow Yellow,
        blue Blue,
        magenta Magenta,
        cyan Cyan,
        white White,

        bright_black BrightBlack,
        bright_red BrightRed,
        bright_green BrightGreen,
        bright_yellow BrightYellow,
        bright_blue BrightBlue,
        bright_magenta BrightMagenta,
        bright_cyan BrightCyan,
        bright_white BrightWhite,
    }
}

macro_rules! impl_color_builder_methods {
    ($self:ident, $color:ident => $output_fg:expr, $output_bg:expr) => {
        impl_color_builder_methods! {
            @ $self, $color => $output_fg, $output_bg;

            colors(
                Black
                Red
                Green
                Yellow
                Blue
                Magenta
                Cyan
                White

                BrightBlack
                BrightRed
                BrightGreen
                BrightYellow
                BrightBlue
                BrightMagenta
                BrightCyan
                BrightWhite
            )

            fg(
                /// Set the foreground color to black.
                black
                /// Set the foreground color to red.
                red
                /// Set the foreground color to green.
                green
                /// Set the foreground color to yellow.
                yellow
                /// Set the foreground color to blue.
                blue
                /// Set the foreground color to magenta.
                magenta
                /// Set the foreground color to cyan.
                cyan
                /// Set the foreground color to white.
                white

                /// Set the foreground color to bright black.
                bright_black
                /// Set the foreground color to bright red.
                bright_red
                /// Set the foreground color to bright green.
                bright_green
                /// Set the foreground color to bright yellow.
                bright_yellow
                /// Set the foreground color to bright blue.
                bright_blue
                /// Set the foreground color to bright magenta.
                bright_magenta
                /// Set the foreground color to bright cyan.
                bright_cyan
                /// Set the foreground color to bright white.
                bright_white
            )

            bg(
                /// Set the background color to black.
                on_black
                /// Set the background color to red.
                on_red
                /// Set the background color to green.
                on_green
                /// Set the background color to yellow.
                on_yellow
                /// Set the background color to blue.
                on_blue
                /// Set the background color to magenta.
                on_magenta
                /// Set the background color to cyan.
                on_cyan
                /// Set the background color to white.
                on_white

                /// Set the background color to bright black.
                on_bright_black
                /// Set the background color to bright red.
                on_bright_red
                /// Set the background color to bright green.
                on_bright_green
                /// Set the background color to bright yellow.
                on_bright_yellow
                /// Set the background color to bright blue.
                on_bright_blue
                /// Set the background color to bright magenta.
                on_bright_magenta
                /// Set the background color to bright cyan.
                on_bright_cyan
                /// Set the background color to bright white.
                on_bright_white
            )
        }
    };

    (
        @ $self:ident, $color:ident => $output_fg:expr, $output_bg:expr;
        colors($($variant:ident)*)
        fg($($(#[$fg_meta:meta])* $fg_name:ident)*)
        bg($($(#[$bg_meta:meta])* $bg_name:ident)*)
    ) => {
        /// Set the foreground color to an RGB color.
        #[inline]
        pub const fn color_rgb(mut $self: Self, r: u8, g: u8, b: u8) -> Self {
            let $color = $crate::Color::Rgb(r, g, b);
            $output_fg
        }

        /// Set the foreground color to an ANSI 4-bit color.
        #[inline]
        pub const fn color_ansi(mut $self: Self, color: $crate::AnsiColor) -> Self {
            let $color = $crate::Color::Ansi(color);
            $output_fg
        }

        /// Set the foreground color to an ANSI 8-bit color.
        #[inline]
        pub const fn color_ansi256(mut $self: Self, i: u8) -> Self {
            let $color = $crate::Color::Ansi256(i);
            $output_fg
        }

        /// Set the background color to an RGB color.
        #[inline]
        pub const fn on_color_rgb(mut $self: Self, r: u8, g: u8, b: u8) -> Self {
            let $color = $crate::Color::Rgb(r, g, b);
            $output_bg
        }

        /// Set the background color to an ANSI 4-bit color.
        #[inline]
        pub const fn on_color_ansi(mut $self: Self, color: $crate::AnsiColor) -> Self {
            let $color = $crate::Color::Ansi(color);
            $output_bg
        }

        /// Set the background color to an ANSI 8-bit color.
        #[inline]
        pub const fn on_color_ansi256(mut $self: Self, i: u8) -> Self {
            let $color = $crate::Color::Ansi256(i);
            $output_bg
        }

        $(
            #[inline]
            $(#[$fg_meta])*
            pub const fn $fg_name(mut $self: Self) -> Self {
                let $color = $crate::Color::Ansi($crate::AnsiColor::$variant);
                $output_fg
            }
        )*

        $(
            #[inline]
            $(#[$bg_meta])*
            pub const fn $bg_name(mut $self: Self) -> Self {
                let $color = $crate::Color::Ansi($crate::AnsiColor::$variant);
                $output_bg
            }
        )*
    };
}

pub(crate) use impl_color_builder_methods;
