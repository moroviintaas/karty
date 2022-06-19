
use std::fmt::Debug;
use std::hash::Hash;
use crate::symbol::CardSymbol;


pub trait Suit: Debug + Ord + Clone + Hash + CardSymbol {
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;


}