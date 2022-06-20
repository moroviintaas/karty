//! Module containing basic Symbol trait
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::marker::PhantomData;
use crate::error::CardError;

/// Trait representing a symbol on a playing card.
/// Typical playing card (one of 52 card deck) is defined by two symbols - [`Figure`][crate::figures::Figure]  and [`Suit`][crate::suits::Suit]
/// Structure implementing this trait should have finite space of possible instances.
/// Instances should have associated number from `0` to [`SYMBOL_SPACE`][crate::symbol::CardSymbol::SYMBOL_SPACE] (excluded).
/// Numbering should be dense, which mean that there should not be a number in [0, [`SYMBOL_SPACE`][crate::symbol::CardSymbol::SYMBOL_SPACE])
/// with no associated [`CardSymbol`][crate::symbol::CardSymbol] instance.
/// For example, implemented in this crate `FigureStd` has `13` possible instances,
/// representing symbols (2,..., 10, Jack, Queen, King, Ace), but their associated numbers are `0..=12`.
pub trait CardSymbol: Sized + Eq{
    /// Space of symbol, holds information how many possible instances of symbol exists.
    /// For standard figure it is `13`, for ls
    /// standard suit it is `4`.
    const SYMBOL_SPACE: usize;
    /// Information about symbol's associated number. Collision in returning numbers will cause
    /// wrong behaviour of functions utilising trait. Position should be unique for every symbol,
    /// it is used for example to iterate over space of symbols. It should be dense though.
    /// # Returns:
    /// Number associated with symbol.
    fn position(&self) -> usize;
    /// Reverse function to [`position(&self)`][crate::symbol::CardSymbol::position], creates symbol instance, given it's associated number.
    /// # Returns:
    /// Instance of symbol associated with a number.
    fn from_position(position: usize) -> Result<Self, CardError>;
    /// Method constructs [`CardSymbolIterator`][crate::symbol::CardSymbolIterator] iterating symbols from the one numbered `0` to
    /// the last one (numbered [`SYMBOL_SPACE-1`][crate::symbol::CardSymbol::SYMBOL_SPACE].
    fn iterator() -> CardSymbolIterator<Self>{
        CardSymbolIterator::new()
    }
}
/// Iterator over CardSymbol space, starts with card associated with number `0` and ends on card
/// with associated number
/// # Example:
/// ```
/// use karty::suits::{SuitStd, SuitStd::*};
/// use karty::symbol::CardSymbol;
/// use std::iter::FromIterator;
///
/// let iterator = SuitStd::iterator();
/// let symbols = Vec::from_iter(iterator);
/// assert_eq!(symbols, [Clubs, Diamonds, Hearts, Spades]);
/// ```
pub struct CardSymbolIterator<E: CardSymbol>{
    iterator_position: usize,
    phantom: PhantomData<E>,
}

impl<E: CardSymbol> CardSymbolIterator<E>{
    pub fn new() -> Self{
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardSymbol> Default for CardSymbolIterator<E>{
    fn default() -> Self {
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardSymbol> Iterator for CardSymbolIterator<E>{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let element = E::from_position(self.iterator_position).ok();
        self.iterator_position += 1;
        element
    }
}


#[cfg(test)]
mod tests{
    use crate::symbol::CardSymbolIterator;
    use crate::figures::{*};
    use crate::suits::SuitStd;
    use crate::suits::SuitStd::{Clubs, Diamonds, Hearts, Spades};

    #[test]
    fn suit_iterator(){
        let iterator = CardSymbolIterator::<SuitStd>::new();
        let vec = Vec::from_iter(iterator);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[0], Clubs);
        assert_eq!(vec[1], Diamonds);
        assert_eq!(vec[2], Hearts);
        assert_eq!(vec[3], Spades);
    }

    #[test]
    fn figure_iterator(){
        let iterator = CardSymbolIterator::<FigureStd>::new();
        let vec = Vec::from_iter(iterator);
        assert_eq!(vec.len(), 13);
        assert_eq!(vec[0], F2);
        assert_eq!(vec[1], F3);
        assert_eq!(vec[2], F4);
        assert_eq!(vec[3], F5);
        assert_eq!(vec[4], F6);
        assert_eq!(vec[5], F7);
        assert_eq!(vec[6], F8);
        assert_eq!(vec[7], F9);
        assert_eq!(vec[8], F10);
        assert_eq!(vec[9], Jack);
        assert_eq!(vec[10], Queen);
        assert_eq!(vec[11], King);
        assert_eq!(vec[12], Ace);
    }

}