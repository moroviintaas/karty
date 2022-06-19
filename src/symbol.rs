//! Module containing basic Symbol trait
use std::marker::PhantomData;
use crate::error::CardError;

/// Trait representing a symbol on a playing card.
/// Typical playing card (one of 52 card deck) is defined by two symbols - `Figure` and `Suit`
/// Structure implementing this trait should have finite space of possible instances.
/// Instances should have associated number from `0` to `SYMBOL_SPACE` (excluded).
/// Numbering should be dense, which mean that there should not be a number in `[0, SYMBOL_SPACE)`
/// with no associated `CardSymbol` instance.
/// For example, implemented in this crate `FigureStd` has `13` possible instances,
/// representing symbols (2,..., 10, Jack, Queen, King, Ace), but their associated numbers are `0..=12`.
pub trait CardSymbol: Sized + Eq{
    const SYMBOL_SPACE: usize;
    fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;
    fn iterator() -> CardElementIterator<Self>{
        CardElementIterator::new()
    }
}

pub struct CardElementIterator<E: CardSymbol>{
    iterator_position: usize,
    phantom: PhantomData<E>,
}

impl<E: CardSymbol> CardElementIterator<E>{
    pub fn new() -> Self{
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardSymbol> Default for CardElementIterator<E>{
    fn default() -> Self {
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardSymbol> Iterator for CardElementIterator<E>{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let element = E::from_position(self.iterator_position).ok();
        self.iterator_position += 1;
        element
    }
}


#[cfg(test)]
mod tests{
    use crate::symbol::CardElementIterator;
    use crate::figures::{*};
    use crate::suits::SuitStd;
    use crate::suits::SuitStd::{Clubs, Diamonds, Hearts, Spades};

    #[test]
    fn suit_iterator(){
        let iterator = CardElementIterator::<SuitStd>::new();
        let vec = Vec::from_iter(iterator);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[0], Clubs);
        assert_eq!(vec[1], Diamonds);
        assert_eq!(vec[2], Hearts);
        assert_eq!(vec[3], Spades);
    }

    #[test]
    fn figure_iterator(){
        let iterator = CardElementIterator::<FigureStd>::new();
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