
use std::fmt::{Debug, Display};
use crate::error::CardErrorGen;
use crate::suits::{SuitTrait};
use crate::symbol::CardSymbol;

pub trait HandTrait: Debug + Clone + Eq + IntoIterator<Item=Self::CardType> + Display + IntoIterator{
    type CardType : CardSymbol;
    fn insert_card(&mut self, card: Self::CardType) -> Result<(), CardErrorGen<Self::CardType>>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), CardErrorGen<Self::CardType>>;
    fn empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool{
        self.len() == 0
    }
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn insert_card_noerr(&mut self, card: Self::CardType){
        self.insert_card(card).unwrap_or(());
    }

    /// Inserts cards from vector. All inserts are tried, if at least one produces `Err(e)`, an `Err(e)` is returned (first encountered).
    /// ```
    /// use karty::hand::{HandTrait, CardSet};
    /// use karty::card_set;
    /// use karty::cards::*;
    /// let mut hand = card_set![ACE_SPADES, KING_HEARTS, FOUR_CLUBS, TWO_HEARTS];
    /// hand.insert_from_iterator(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS].into_iter()).unwrap();
    /// assert_eq!(hand.len(), 8);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
    fn insert_from_iterator<I: Iterator<Item = Self::CardType>>(&mut self, iter: I) -> Result<(), CardErrorGen<Self::CardType>>{
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
    /// use karty::hand::{HandTrait, CardSet};
    /// use karty::card_set;
    /// use karty::cards::*;
    /// let mut hand = CardSet::from_iterator(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS].into_iter());
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

    fn to_vec(self) -> Vec<Self::CardType>{
        self.into_iter().collect()
    }

}


pub trait HandSuitedTrait: HandTrait{
    type SuitIterator: Iterator<Item = <Self as HandTrait>::CardType>;
    type St: SuitTrait;
    
    fn contains_in_suit(&self, suit: &Self::St) -> bool;
    fn suit_iterator(&self, suit: &Self::St) -> Self::SuitIterator;
}