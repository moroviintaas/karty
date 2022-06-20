//! Module containing `Suit` trait and implementation of standard card suit.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
mod suit;
mod standard;
pub use crate::suits::standard::*;
pub use suit::*;

#[cfg(feature = "parse")]
pub mod parse;
