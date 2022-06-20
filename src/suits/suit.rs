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
/// [`Ord`][std::cmp::Ord], [`Clone`][std::clone::Clone] and [`Hash`][std::hash::Hash].
/// At the moment trait is only semantic sugar. It remains after early crate development,
/// now it is advised to use more generic super trait [`CardSymbol`][crate::symbol::CardSymbol].
/// Trait may be marked deprecated in future and marked to remove.
pub trait Suit: Debug + Ord + Clone + Hash + CardSymbol {
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;


}