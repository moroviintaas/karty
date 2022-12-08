mod iterator;
mod r#trait;
//mod symbol_map;
mod symbol_comparator;


pub use iterator::*;
pub use r#trait::*;
//pub use symbol_map::*;
pub use symbol_comparator::*;


#[cfg(test)]
mod tests{
    use crate::symbol::CardSymbolIterator;
    use crate::figures::{*};
    use crate::suits::Suit;
    use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};

    #[test]
    fn suit_iterator(){
        let iterator = CardSymbolIterator::<Suit>::new();
        let vec = Vec::from_iter(iterator);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[0], Clubs);
        assert_eq!(vec[1], Diamonds);
        assert_eq!(vec[2], Hearts);
        assert_eq!(vec[3], Spades);
    }

    #[test]
    fn figure_iterator(){
        let iterator = CardSymbolIterator::<Figure>::new();
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