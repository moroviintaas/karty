use std::fmt::{Display, Formatter};
use crate::cards::{Card, MASK_CLUBS, MASK_DIAMONDS, MASK_HEARTS, MASK_SPADES};

use crate::error::HandError;
use crate::hand::HandTrait;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::suits::Suit;
use crate::symbol::CardSymbol;


const LARGEST_MASK:u64 = 1<<63;

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
    /// let mut hand = StackHand::new_empty();
    /// hand.add_card(ACE_HEARTS).unwrap();
    /// hand.add_card(ACE_DIAMONDS).unwrap();
    /// hand.add_card(ACE_CLUBS).unwrap();
    /// hand.add_card(ACE_SPADES).unwrap();
    /// hand.add_card(TWO_SPADES).unwrap();
    /// hand.add_card(THREE_SPADES).unwrap();
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

impl Into<u64> for StackHand{
    fn into(self) -> u64 {
        self.cards
    }
}



pub struct StackHandIterator {
    hand: StackHand,
    mask: u64,
    end: bool
}


impl StackHandIterator {
    pub fn new(hand: StackHand) -> Self{

        Self{mask: 1, hand, end: false}
    }
}
/// ```
/// use karty::cards::{ACE_CLUBS, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS};
/// use karty::hand::{HandTrait, StackHand};
/// let mut hand = StackHand::new_empty();
/// hand.add_card(ACE_CLUBS);
/// hand.add_card( KING_HEARTS);
/// hand.add_card( QUEEN_DIAMONDS);
/// hand.add_card( JACK_SPADES);
/// let v: Vec<Card> = hand.into_iter().collect();
/// assert_eq!(v.len(), 4);
/// assert_eq!(v[0], ACE_CLUBS);
/// ```
impl Iterator for StackHandIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.end{
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
        }
    }
}

pub struct StackHandSuitIterator {
    hand: StackHand,
    mask: u64,
    guard: u64,
    end: bool,

}
impl StackHandSuitIterator {
    pub fn new(hand: StackHand, suit: Suit) -> Self{

        let mask = 1u64 <<(suit.position()*16);
        let guard = 1u64<<(suit.position()*16 + 15);
        Self{mask, hand, end: false, guard}
    }
}

impl Iterator for StackHandSuitIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.end{
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
        }
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

    fn add_card(&mut self, card: Self::CardType) -> Result<(), crate::error::HandError> {
        match self.contains(&card){
            true => Err(HandError::CardDuplicated),
            false => {
                self.cards |= card.mask();
                Ok(())
            }
        }
    }

    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), crate::error::HandError> {
        match self.contains(card){
            true => {
                self.cards ^= card.mask();
                Ok(())
            },
            false => Err(HandError::CardNotInHand)
        }
    }

    fn new_empty() -> Self {
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

pub const HAND_OF_SPADES: StackHand = StackHand{cards: MASK_SPADES};
pub const HAND_OF_HEARTS: StackHand = StackHand{cards: MASK_HEARTS};
pub const HAND_OF_DIAMONDS: StackHand = StackHand{cards: MASK_DIAMONDS};
pub const HAND_OF_CLUBS: StackHand = StackHand{cards: MASK_CLUBS};