//! A library for conveniently creating styled ANSI text.
//!
//! # Features
//! - `no_std` support
//! - `const` style constructors
//! - Hyperlink support
//! - Support for nested styled text (requires `std` feature to operate correctly)
//! - Convenient [`styled`] and [`hyperlink`] macros
//! - Enable or disable styling globally using the [`set_style_mode`](crate::set_style_mode) function
//!
//! # Nested styled text
//!
//! Consider the following case:
//!
//! ```rust
//! use antsy::{styled, Color};
//!
//! println!(
//!     "{}",
//!     styled!(
//!         "the quick {} fox jumps over the lazy dog",
//!         styled!("brown").fg(Color::rgb(161, 123, 90))
//!     )
//!     .fg(Color::Cyan)
//! );
//! ```
//!
//! This prints some text in cyan, with the word "brown" in brown.
//!
//! Without proper support, this will result in all styling being reset after the
//! word "brown".
//!
//! With the `std` feature enabled, this is handled correctly, and the remaining text
//! will be cyan.

#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod macros;

mod display;
mod enable;
mod style;

pub use enable::*;
pub use style::*;
