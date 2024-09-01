#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod macros;

mod color;
mod style;

mod display;
mod enable;

pub use color::{AnsiColor, Color};
pub use style::{Attributes, Hyperlink, Style, Styled};

pub use enable::{set_style_mode, style_enabled, StyleMode};
