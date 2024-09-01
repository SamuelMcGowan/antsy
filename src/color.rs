#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    #[default]
    Default,
    Ansi(AnsiColor),
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

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
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

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
                black
                red
                green
                yellow
                blue
                magenta
                cyan
                white

                bright_black
                bright_red
                bright_green
                bright_yellow
                bright_blue
                bright_magenta
                bright_cyan
                bright_white
            )

            bg(
                on_black
                on_red
                on_green
                on_yellow
                on_blue
                on_magenta
                on_cyan
                on_white

                on_bright_black
                on_bright_red
                on_bright_green
                on_bright_yellow
                on_bright_blue
                on_bright_magenta
                on_bright_cyan
                on_bright_white
            )
        }
    };

    (@ $self:ident, $color:ident => $output_fg:expr, $output_bg:expr; colors($($variant:ident)*) fg($($fg_name:ident)*) bg($($bg_name:ident)*)) => {
        #[inline]
        pub const fn rgb(mut $self: Self, r: u8, g: u8, b: u8) -> Self {
            let $color = $crate::Color::Rgb(r, g, b);
            $output_fg
        }

        #[inline]
        pub const fn ansi256(mut $self: Self, i: u8) -> Self {
            let $color = $crate::Color::Ansi256(i);
            $output_fg
        }

        #[inline]
        pub const fn on_rgb(mut $self: Self, r: u8, g: u8, b: u8) -> Self {
            let $color = $crate::Color::Rgb(r, g, b);
            $output_bg
        }

        #[inline]
        pub const fn on_ansi256(mut $self: Self, i: u8) -> Self {
            let $color = $crate::Color::Ansi256(i);
            $output_bg
        }

        $(
            #[inline]
            pub const fn $fg_name(mut $self: Self) -> Self {
                let $color = $crate::Color::Ansi($crate::AnsiColor::$variant);
                $output_fg
            }
        )*

        $(
            #[inline]
            pub const fn $bg_name(mut $self: Self) -> Self {
                let $color = $crate::Color::Ansi($crate::AnsiColor::$variant);
                $output_bg
            }
        )*
    };
}

pub(crate) use impl_color_builder_methods;
