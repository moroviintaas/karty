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
    iterator_position_low: usize,
    iterator_position_high: usize,
    phantom: PhantomData<E>,
    stop: bool
}

impl<E: CardSymbol> CardSymbolIterator<E>{
    pub fn new() -> Self{
        Self{iterator_position_low: 0, iterator_position_high: E::SYMBOL_SPACE-1 ,phantom: PhantomData, stop:false}
    }
}

impl<E: CardSymbol> Default for CardSymbolIterator<E>{
    fn default() -> Self {
        Self{iterator_position_low: 0, iterator_position_high: E::SYMBOL_SPACE-1, phantom: PhantomData, stop:false}
    }
}
/// ```
/// use karty::suits::{Suit, Suit::*};
/// use karty::symbol::CardSymbolIterator;
/// use karty::symbol::CardSymbol;
/// //let suits:Vec::<Suit> = Suit::iterator().collect();
/// //assert_eq!(vec![Clubs, Diamonds, Hearts, Spades], suits );
/// let mut suit_iterator = Suit::iterator();
/// assert_eq!(suit_iterator.next(), Some(Clubs));
/// assert_eq!(suit_iterator.next(), Some(Diamonds));
/// assert_eq!(suit_iterator.next(), Some(Hearts));
/// assert_eq!(suit_iterator.next(), Some(Spades));
/// assert_eq!(suit_iterator.next(), None);
/// ```
impl<E: CardSymbol> Iterator for CardSymbolIterator<E>{
    type Item = E;
    
    fn next(&mut self) -> Option<Self::Item> {
        /* 
        match self.iterator_position_low<=self.iterator_position_high{
            true => {
                let element = E::from_position(self.iterator_position_low).ok();
                //self.iterator_position_low = self.iterator_position_low.saturating_add(1);
                self.iterator_position_low +=1;
                element
            },
            false => None,
        } */

        if self.stop{
            return None;
        }
        match self.iterator_position_low.cmp(&self.iterator_position_high){
            std::cmp::Ordering::Greater => None,
            std::cmp::Ordering::Equal =>  match self.stop{
               
                true => None,
                false => {
                    let element = E::from_position(self.iterator_position_low).ok();
                    self.stop = true;
                    self.iterator_position_low = self.iterator_position_low.saturating_add(1);
                    element
                    
                },
                
                
            }
                ,
            std::cmp::Ordering::Less => {
                let element = E::from_position(self.iterator_position_low).ok();
                self.iterator_position_low = self.iterator_position_low.saturating_add(1);
                element
            },
        } 
        
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (E::SYMBOL_SPACE-self.iterator_position_low, Some(E::SYMBOL_SPACE-self.iterator_position_low))
    }
    
}

/// ```
/// use karty::suits::{Suit, Suit::*};
/// use karty::symbol::CardSymbolIterator;
/// use karty::symbol::CardSymbol;
/// //let suits:Vec::<Suit> = Suit::iterator().rev().collect();
/// let mut suit_iterator = Suit::iterator();
/// assert_eq!(suit_iterator.next_back(), Some(Spades));
/// assert_eq!(suit_iterator.next_back(), Some(Hearts));
/// assert_eq!(suit_iterator.next_back(), Some(Diamonds));
/// assert_eq!(suit_iterator.next_back(), Some(Clubs));
/// assert_eq!(suit_iterator.next_back(), None);
/// let mut suit_iterator = Suit::iterator();
/// assert_eq!(suit_iterator.next_back(), Some(Spades));
/// assert_eq!(suit_iterator.next_back(), Some(Hearts));
/// assert_eq!(suit_iterator.next_back(), Some(Diamonds));
/// assert_eq!(suit_iterator.next(), Some(Clubs));
/// assert_eq!(suit_iterator.next_back(), None);
/// ```
impl <E: CardSymbol> DoubleEndedIterator for CardSymbolIterator<E>{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.stop{
            return None;
        }
        match self.iterator_position_high.cmp(&self.iterator_position_low){
            std::cmp::Ordering::Less => None,
            std::cmp::Ordering::Equal =>  match self.stop{
               
                true => None,
                false => {
                    self.stop = true;
                    let element = E::from_position(self.iterator_position_high).ok();
                    self.iterator_position_high = self.iterator_position_high.saturating_sub(1);
                    element
                    
                },
                
                
            }
                ,
            std::cmp::Ordering::Greater => {
                let element = E::from_position(self.iterator_position_high).ok();
                self.iterator_position_high = self.iterator_position_high.saturating_sub(1);
                element
            },
        } 
    }
}

/*impl<E: CardSymbol> ExactSizeIterator for CardSymbolIterator<E>{
    
}*/