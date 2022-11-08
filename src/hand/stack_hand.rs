use std::fmt::{Display, Formatter};
use crate::cards::Card;

use crate::error::HandError;
use crate::hand::HandTrait;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


const LARGEST_MASK:u64 = 1<<63;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct StackHand {
    pub(crate) cards: u64,
}

pub struct StackHandStdIterator{
    hand: StackHand,
    mask: u64,
    end: bool
}
impl StackHandStdIterator{
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
impl Iterator for StackHandStdIterator{
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

impl IntoIterator for StackHand {
    type Item = Card;

    type IntoIter = StackHandStdIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackHandStdIterator::new(self)
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