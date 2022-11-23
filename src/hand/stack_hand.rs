use std::fmt::{Display, Formatter};
use crate::cards::{Card, MASK_CLUBS, MASK_DIAMONDS, MASK_HEARTS, MASK_SPADES};

use crate::error::HandErrorGen;
use crate::figures::Figure;
use crate::hand::HandTrait;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::suits::Suit;
use crate::symbol::CardSymbol;


const CARD_MASK_GUARD:u64 = 1<<52;
const MASK_STACK_HAND_LEGAL: u64 = MASK_DIAMONDS | MASK_CLUBS | MASK_HEARTS | MASK_SPADES;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct StackHand {
    pub(crate) cards: u64,
}

impl StackHand{
    ///
    /// ```
    /// use karty::cards::{ACE_CLUBS, ACE_DIAMONDS, ACE_HEARTS, ACE_SPADES, Card, THREE_SPADES, TWO_SPADES};
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::suits::Suit::{Clubs, Spades};
    /// let mut hand = StackHand::empty();
    /// hand.insert_card(ACE_HEARTS).unwrap();
    /// hand.insert_card(ACE_DIAMONDS).unwrap();
    /// hand.insert_card(ACE_CLUBS).unwrap();
    /// hand.insert_card(ACE_SPADES).unwrap();
    /// hand.insert_card(TWO_SPADES).unwrap();
    /// hand.insert_card(THREE_SPADES).unwrap();
    /// let spades: Vec<Card> = hand.cards_in_suit(Spades).collect();
    /// let clubs: Vec<Card> = hand.cards_in_suit(Clubs).collect();
    /// assert_eq!(spades, vec![TWO_SPADES, THREE_SPADES, ACE_SPADES]);
    /// assert_eq!(clubs, vec![ACE_CLUBS]);
    ///
    /// ```
    pub fn cards_in_suit(&self, suit: Suit) -> StackHandSuitIterator {
        StackHandSuitIterator::new(*self, suit)
    }



}


impl From<StackHand> for u64{
    fn from(hand: StackHand) -> Self {
        hand.cards
    }
}

impl From<u64> for StackHand{
    fn from(cards: u64) -> Self {
        Self{cards: cards & MASK_STACK_HAND_LEGAL}
    }
}



pub struct StackHandIterator {
    hand: StackHand,
    mask: u64,
}


impl StackHandIterator {
    pub fn new(hand: StackHand) -> Self{

        Self{mask: 1, hand}
    }
}
/// ```
/// use karty::cards::{ACE_CLUBS, ACE_SPADES, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS};
/// use karty::hand::{HandTrait, StackHand};
/// let mut hand = StackHand::empty();
/// hand.insert_card(ACE_CLUBS).unwrap();
/// hand.insert_card( KING_HEARTS).unwrap();
/// hand.insert_card( QUEEN_DIAMONDS).unwrap();
/// hand.insert_card( JACK_SPADES).unwrap();
/// hand.insert_card( ACE_SPADES).unwrap();
/// let v: Vec<Card> = hand.into_iter().collect();
/// assert_eq!(v.len(), 5);
/// assert_eq!(v[0], ACE_CLUBS);
/// assert_eq!(v[4], ACE_SPADES);
/// ```
impl Iterator for StackHandIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        /*if !self.end{
            while self.mask != (LARGEST_MASK){
                if self.mask & self.hand.cards != 0{
                    let card = Card::from_mask(self.mask).unwrap();
                    self.mask <<=1;
                    return Some(card);
                }
                else{
                    self.mask<<=1;
                }
            }
            self.end = true;
            Card::from_mask(self.mask)

        }
        else{
            None
        }*/
        while self.mask < (CARD_MASK_GUARD){
            if self.mask & self.hand.cards != 0{
                let card = Card::from_mask(self.mask).unwrap();
                self.mask <<=1;
                return Some(card);
            }
            else{
                self.mask<<=1;
            }
        }
        None
    }
}

pub struct StackHandSuitIterator {
    hand: StackHand,
    mask: u64,
    guard: u64,

}
impl StackHandSuitIterator {
    pub fn new(hand: StackHand, suit: Suit) -> Self{

        let mask = 1u64 <<(suit.position()*Figure::SYMBOL_SPACE);
        let guard = 1u64<<(suit.position()*Figure::SYMBOL_SPACE + Figure::SYMBOL_SPACE);
        Self{mask, hand, guard}
    }
}

impl Iterator for StackHandSuitIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        /*if !self.end{
            while self.mask != (self.guard){
                if self.mask & self.hand.cards != 0{
                    let card = Card::from_mask(self.mask).unwrap();
                    self.mask <<=1;
                    return Some(card);
                }
                else{
                    self.mask<<=1;
                }
            }
            self.end = true;
            Card::from_mask(self.mask)

        }
        else{
            None
        }*/
        while self.mask < (self.guard){
            if self.mask & self.hand.cards != 0{
                let card = Card::from_mask(self.mask).unwrap();
                self.mask <<=1;
                return Some(card);
            }
            else{
                self.mask<<=1;
            }
        }
        None

    }
}



impl IntoIterator for StackHand {
    type Item = Card;

    type IntoIter = StackHandIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackHandIterator::new(self)
    }
}

impl HandTrait for StackHand {
    type CardType = Card;

    fn insert_card(&mut self, card: Self::CardType) -> Result<(), crate::error::HandErrorGen<Self::CardType>> {
        match self.contains(&card){
            true => Err(HandErrorGen::CardDuplicated(card)),
            false => {
                self.cards |= card.mask();
                Ok(())
            }
        }
    }

    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), crate::error::HandErrorGen<Self::CardType>> {
        match self.contains(card){
            true => {
                self.cards ^= card.mask();
                Ok(())
            },
            false => Err(HandErrorGen::CardNotInHand(*card))
        }
    }

    fn empty() -> Self {
        Self{cards: 0u64}
    }

    fn contains(&self, card: &Self::CardType) -> bool {
        card.mask() & self.cards != 0
    }

    fn len(&self) -> usize {
        self.cards.count_ones() as usize
    }

    fn union(&self, other: &Self) -> Self {
        Self{cards: self.cards | other.cards}
    }

    fn intersection(&self, other: &Self) -> Self {
        Self{cards: self.cards & other.cards}
    }
}

impl Display for StackHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v: Vec<Card> = self.into_iter().collect();
        write!(f,  "[")?;
        if f.alternate(){
            for e in v.into_iter(){
                write!(f, "{:#}, ", e)?;
            }


        }
        else{
            for e in v.into_iter(){
                write!(f, "{}, ", e)?;
            }
        }
        write!(f, "]")
    }
}

impl FromIterator<Card> for StackHand{
    /// ```
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::stack_hand;
    /// use karty::cards::*;
    /// let mut hand = StackHand::from_iter(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS]);
    /// assert_eq!(hand.len(), 4);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
    fn from_iter<T: IntoIterator<Item=Card>>(iter: T) -> Self {
        let mut hand = StackHand::empty();
        hand.insert_from_iterator(iter.into_iter()).unwrap_or(());
        hand
    }
}

#[macro_export]
macro_rules! stack_hand {
    [ $( $x:expr ),* ] => {
        {
            let mut h = 0u64;
            $(
                h = h | $x.mask();
            )*
            StackHand::from(h)
        }
    };
}

pub const HAND_OF_SPADES: StackHand = StackHand{cards: MASK_SPADES};
pub const HAND_OF_HEARTS: StackHand = StackHand{cards: MASK_HEARTS};
pub const HAND_OF_DIAMONDS: StackHand = StackHand{cards: MASK_DIAMONDS};
pub const HAND_OF_CLUBS: StackHand = StackHand{cards: MASK_CLUBS};



#[cfg(test)]
mod tests{
    use crate::cards::{ACE_SPADES, KING_HEARTS, TEN_DIAMONDS, FOUR_SPADES};
    use crate::hand::{HandTrait, StackHand};

    #[test]
    fn stack_hand_macro(){

        let hand = stack_hand![ACE_SPADES, KING_HEARTS, TEN_DIAMONDS, FOUR_SPADES];
        assert_eq!(hand.len(), 4);
        assert!(hand.contains(&ACE_SPADES));
    }
}