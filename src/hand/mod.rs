mod hand_set;
mod stack_hand;
pub use stack_hand::*;
pub use hand_set::*;

use std::fmt::{Debug, Display};
use crate::error::HandError;
use crate::symbol::CardSymbol;

pub trait HandTrait: Debug + Clone + Eq + IntoIterator<Item=Self::CardType> + Display{
    type CardType : CardSymbol;
    fn insert_card(&mut self, card: Self::CardType) -> Result<(), HandError>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), HandError>;
    fn new_empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool{
        self.len() == 0
    }
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    /// ```
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::stack_hand;
    /// use karty::cards::*;
    /// let mut hand = stack_hand![ACE_SPADES, KING_HEARTS, FOUR_CLUBS, TWO_HEARTS];
    /// hand.insert_many_cards(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS]).unwrap();
    /// assert_eq!(hand.len(), 8);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
    fn insert_many_cards(&mut self, cards: Vec<Self::CardType>) -> Result<(), HandError>{
        let mut result = Ok(());
        for c in cards{
            match self.insert_card(c){
                Ok(()) => (),
                Err(e) => result = Err(e)
            }
        }
        result
    }

}