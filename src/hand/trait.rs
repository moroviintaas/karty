
use std::fmt::{Debug, Display};
use crate::error::HandError;
use crate::symbol::CardSymbol;

pub trait HandTrait: Debug + Clone + Eq + IntoIterator<Item=Self::CardType> + Display{
    type CardType : CardSymbol;
    fn insert_card(&mut self, card: Self::CardType) -> Result<(), HandError>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), HandError>;
    fn empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool{
        self.len() == 0
    }
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;

    /// Inserts cards from vector. All inserts are tried, if at least one produces `Err(e)`, an `Err(e)` is returned (first encountered).
    /// ```
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::stack_hand;
    /// use karty::cards::*;
    /// let mut hand = stack_hand![ACE_SPADES, KING_HEARTS, FOUR_CLUBS, TWO_HEARTS];
    /// hand.insert_from_iterator(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS].into_iter()).unwrap();
    /// assert_eq!(hand.len(), 8);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
    fn insert_from_iterator<I: Iterator<Item = Self::CardType>>(&mut self, iter: I) -> Result<(), HandError>{
        let mut result = Ok(());
        for c in iter{
            match self.insert_card(c){
                Ok(()) => (),

                Err(e) => if result.is_ok(){
                    result = Err(e)
                }
            }
        }
        result
    }

    /// ```
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::stack_hand;
    /// use karty::cards::*;
    /// let mut hand = StackHand::from_iterator(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS].into_iter());
    /// assert_eq!(hand.len(), 4);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
     fn from_iterator<I: Iterator<Item = Self::CardType>>(iter: I) -> Self{
         let mut hand = Self::empty();
         hand.insert_from_iterator(iter).unwrap_or(());
         hand
     }

}
