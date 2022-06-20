//! Module containing basic Symbol trait
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::fmt::Debug;
use std::hash::Hash;
use crate::symbol::CardSymbol;

/// Trait that is combination of [`CardSymbol`][crate::symbol::CardSymbol], [`Debug`][std::fmt::Debug]
pub trait Suit: Debug + Ord + Clone + Hash + CardSymbol {
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;


}