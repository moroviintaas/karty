use std::marker::PhantomData;
use crate::symbol::CardSymbol;

/// Iterator over CardSymbol space, starts with card associated with number `0` and ends on card
/// with associated number
/// # Example:
/// ```
/// use karty::suits::{Suit, Suit::*};
/// use karty::symbol::CardSymbol;
/// use std::iter::FromIterator;
///
/// let iterator = Suit::iterator();
/// let symbols = Vec::from_iter(iterator);
/// assert_eq!(symbols, [Clubs, Diamonds, Hearts, Spades]);
/// ```
#[derive(Clone)]
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