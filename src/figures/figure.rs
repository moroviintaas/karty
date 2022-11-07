
use std::fmt::Debug;
use std::hash::Hash;
use crate::symbol::CardSymbol;


pub trait FigureTrait: Debug + Ord + Clone + Hash + CardSymbol {
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;
    /*fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;*/

}

