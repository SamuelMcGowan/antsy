#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod macros;

mod display;
mod enable;

mod color;
mod style;

pub use color::{AnsiColor, Color};
pub use enable::*;
pub use style::*;
