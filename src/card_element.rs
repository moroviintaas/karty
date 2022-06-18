use std::marker::PhantomData;
use crate::error::CardError;

pub trait CardElement: Sized + Eq{
    const DIMENSION_SIZE: usize;
    fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;
}

pub struct CardElementIterator<E: CardElement>{
    iterator_position: usize,
    phantom: PhantomData<E>,
}

impl<E: CardElement> CardElementIterator<E>{
    pub fn new() -> Self{
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardElement> Default for CardElementIterator<E>{
    fn default() -> Self {
        Self{iterator_position: 0, phantom: PhantomData}
    }
}

impl<E: CardElement> Iterator for CardElementIterator<E>{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let element = E::from_position(self.iterator_position).ok();
        self.iterator_position += 1;
        element
    }
}


#[cfg(test)]
mod tests{
    use crate::card_element::CardElementIterator;
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
        assert_eq!(vec[0], Numbered(F2));
        assert_eq!(vec[1], Numbered(F3));
        assert_eq!(vec[2], Numbered(F4));
        assert_eq!(vec[3], Numbered(F5));
        assert_eq!(vec[4], Numbered(F6));
        assert_eq!(vec[5], Numbered(F7));
        assert_eq!(vec[6], Numbered(F8));
        assert_eq!(vec[7], Numbered(F9));
        assert_eq!(vec[8], Numbered(F10));
        assert_eq!(vec[9], Jack);
        assert_eq!(vec[10], Queen);
        assert_eq!(vec[11], King);
        assert_eq!(vec[12], Ace);
    }

}