//! Module containing `Suit` trait and implementation of standard card suit.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
mod r#trait;
mod standard;
pub use crate::suits::standard::*;
pub use r#trait::*;

#[cfg(feature = "parse")]
pub mod parse;
mod standard_map;
pub use standard_map::*;
