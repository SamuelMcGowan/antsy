#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod styled_macro;

mod color_choice;
mod display;
mod style;

pub use color_choice::*;
pub use style::*;
