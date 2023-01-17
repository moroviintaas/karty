use std::fmt::{Display, Formatter};
use crate::cards::{Card, MASK_CLUBS, MASK_DIAMONDS, MASK_HEARTS, MASK_SPADES};

use crate::error::HandErrorGen;
use crate::figures::Figure;
use crate::hand::HandTrait;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::suits::Suit;
use crate::symbol::CardSymbol;

use super::HandSuitedTrait;


//const CARD_MASK_GUARD:u64 = 1<<52;
const MASK_STACK_HAND_LEGAL: u64 = MASK_DIAMONDS | MASK_CLUBS | MASK_HEARTS | MASK_SPADES;
//const STACK_HAND_LARGEST_MASK:u64 = 0x1<<53;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct StackHand {
    pub(crate) cards: u64,
}

impl StackHand{
    

    

    fn suit_mask(suit: Suit) -> u64{
        match suit{
            Suit::Spades => MASK_SPADES,
            Suit::Hearts => MASK_HEARTS,
            Suit::Diamonds => MASK_DIAMONDS,
            Suit::Clubs => MASK_CLUBS
        }
    }

    pub fn only_in_suit(&self, suit: &Suit) -> Self{
        Self{cards: self.cards & Self::suit_mask(*suit)}
    }


    /// ```
    /// use karty::cards::{*};
    /// use karty::stack_hand;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let hand = stack_hand![TWO_DIAMONDS, THREE_CLUBS, JACK_CLUBS, KING_CLUBS, ACE_HEARTS, TWO_SPADES];
    /// assert_eq!(hand.highest_in_suit(&Spades).unwrap(), TWO_SPADES);
    /// assert_eq!(hand.highest_in_suit(&Clubs).unwrap(), KING_CLUBS);
    /// assert_eq!(hand.highest_in_suit(&Diamonds).unwrap(), TWO_DIAMONDS);
    /// assert_eq!(hand.highest_in_suit(&Hearts).unwrap(), ACE_HEARTS);
    /// ```
    /// ```
    /// use karty::cards::{*};
    /// use karty::stack_hand;
    /// use karty::suits::Suit::Hearts;
    /// let hand = stack_hand![ACE_HEARTS, KING_CLUBS];
    /// assert_eq!(hand.highest_in_suit(&Hearts).unwrap(), ACE_HEARTS);
    /// ```
    pub fn highest_in_suit(&self, suit: &Suit) -> Option<Card>{
        let leading_zeros =(self.cards & Self::suit_mask(*suit)).leading_zeros() as u64;
        match leading_zeros{
            some @ 0..=63 => {
                let pos = 63 - some;
                Some(Card::from_mask(1<<pos).unwrap())
            },
            _ => None,
        }
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
    lower_position: u64,
    higher_position: u64,

}


impl StackHandIterator {
    pub fn new(hand: StackHand) -> Self{

        Self{ lower_position: 1, hand, higher_position:1<<51}
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

        while self.lower_position <= self.higher_position{
            if self.lower_position & self.hand.cards != 0{
                let card = Card::from_mask(self.lower_position).unwrap();
                self.lower_position <<=1;
                return Some(card);
            }
            else{
                self.lower_position <<=1;
            }
        }
        None

    }

    /// ```
    /// use karty::cards::*;
    /// use karty::stack_hand;
    /// let mut v = stack_hand![TWO_CLUBS, THREE_CLUBS, NINE_DIAMONDS].into_iter();
    /// assert_eq!(v.size_hint(), (3,Some(3)));
    /// assert_eq!(v.next_back(), Some(NINE_DIAMONDS));
    /// assert_eq!(v.size_hint(), (2, Some(2)));
    ///
    /// ```
    ///
    /// # Full StackHand
    /// ```
    /// use karty::cards::Card;
    /// use karty::hand::StackHand;
    /// use karty::symbol::CardSymbol;
    /// let mut full_hand = StackHand::from_iter(Card::iterator()).into_iter();
    /// assert_eq!(full_hand.size_hint(), (52, Some(52)));
    /// for _ in 0..5{
    ///     full_hand.next();
    /// }
    /// assert_eq!(full_hand.size_hint(), (47, Some(47)));
    ///
    /// ```
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut mask: u64 = 0;
        let mut m =self.lower_position;
        while m <= self.higher_position{
            mask |= m;
            m<<=1;
        }
        let sum = (self.hand.cards & mask).count_ones();
        (sum as usize, Some(sum as usize))

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
/// let v: Vec<Card> = hand.into_iter().rev().collect();
/// assert_eq!(v.len(), 5);
/// assert_eq!(v[0], ACE_SPADES);
/// assert_eq!(v[4], ACE_CLUBS);
/// ```
impl DoubleEndedIterator for StackHandIterator{
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.higher_position >= self.lower_position{
            if self.higher_position & self.hand.cards != 0{
                let card = Card::from_mask(self.higher_position).unwrap();
                self.higher_position >>=1;
                return Some(card);
            }
            else{
                self.higher_position >>=1;
            }
        }
        None
        
    }
}

impl ExactSizeIterator for StackHandIterator{}

pub struct StackHandSuitIterator {
    hand: StackHand,
    //mask: u64,
    //lower_guard: u64,
    higher_position: u64,
    lower_position: u64
    //higher_guard: u64,

}
impl StackHandSuitIterator {
    pub fn new(hand: StackHand, suit: Suit) -> Self{

        //let guard = 1u64 <<(suit.position()*Figure::SYMBOL_SPACE);
        //let mask = 1u64<<(suit.position()*Figure::SYMBOL_SPACE + Figure::SYMBOL_SPACE -1);
        let lower_position = 1u64 <<(suit.position()*Figure::SYMBOL_SPACE);
        let higher_position = 1u64<<(suit.position()*Figure::SYMBOL_SPACE + Figure::SYMBOL_SPACE -1);
        //Self{mask, hand, lower_guard: guard, /*higher_guard: mask*/}
        Self{lower_position, higher_position, hand}

    }
}


/// ```
/// use karty::cards::{ACE_CLUBS, ACE_SPADES, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS, TWO_SPADES};
/// use karty::hand::{HandTrait, StackHand, StackHandSuitIterator};
/// use karty::suits::Suit::Spades;
/// let mut hand = StackHand::empty();
/// hand.insert_card(ACE_CLUBS).unwrap();
/// hand.insert_card( KING_HEARTS).unwrap();
/// hand.insert_card( QUEEN_DIAMONDS).unwrap();
/// hand.insert_card( JACK_SPADES).unwrap();
/// hand.insert_card( ACE_SPADES).unwrap();
/// hand.insert_card( TWO_SPADES).unwrap();
/// let v: Vec<Card> = StackHandSuitIterator::new(hand, Spades).collect();
/// assert_eq!(v.len(), 3);
/// assert_eq!(v[0], TWO_SPADES);
/// assert_eq!(v[1], JACK_SPADES);
/// assert_eq!(v[2], ACE_SPADES);
/// ```
impl Iterator for StackHandSuitIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {

        while self.lower_position <= self.higher_position{
            if self.lower_position & self.hand.cards != 0{
                let card = Card::from_mask(self.lower_position).unwrap();
                self.lower_position <<=1;
                return Some(card);
            }
            else{
                self.lower_position <<=1;
            }
        }
        None

    }
}

/// ```
/// use karty::cards::{ACE_CLUBS, ACE_SPADES, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS, TEN_SPADES, TWO_SPADES};
/// use karty::hand::{HandTrait, StackHand, StackHandSuitIterator};
/// use karty::suits::Suit::Spades;
/// let mut hand = StackHand::empty();
/// hand.insert_card(ACE_CLUBS).unwrap();
/// hand.insert_card( KING_HEARTS).unwrap();
/// hand.insert_card( QUEEN_DIAMONDS).unwrap();
/// hand.insert_card( JACK_SPADES).unwrap();
/// hand.insert_card( ACE_SPADES).unwrap();
/// hand.insert_card( TWO_SPADES).unwrap();
/// hand.insert_card( TEN_SPADES).unwrap();
/// let mut it = StackHandSuitIterator::new(hand, Spades);
/// assert_eq!(it.next(), Some(TWO_SPADES));
/// assert_eq!(it.next_back(), Some(ACE_SPADES));
/// assert_eq!(it.next_back(), Some(JACK_SPADES));
/// assert_eq!(it.next(), Some(TEN_SPADES));
/// assert_eq!(it.next(), None);
/// assert_eq!(it.next_back(), None);
///
///
/// ```
impl DoubleEndedIterator for StackHandSuitIterator{
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.higher_position >= self.lower_position{
            if self.higher_position & self.hand.cards != 0{
                let card = Card::from_mask(self.higher_position).unwrap();
                self.higher_position >>=1;
                return Some(card);
            }
            else{
                self.higher_position >>=1;
            }
        }
        None

    }
}

/// ```
///  use karty::cards::{Card, EIGHT_HEARTS, JACK_HEARTS, KING_HEARTS, NINE_HEARTS, QUEEN_HEARTS, SEVEN_HEARTS};
/// use karty::hand::StackHandIntervalIterator;
/// use karty::stack_hand;
/// let mut hand = stack_hand![KING_HEARTS, QUEEN_HEARTS, JACK_HEARTS, NINE_HEARTS];
/// assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &EIGHT_HEARTS).next(), None);
/// assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &QUEEN_HEARTS).collect::<Vec<Card>>(), vec![NINE_HEARTS, JACK_HEARTS, QUEEN_HEARTS]);
/// assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &KING_HEARTS).rev().collect::<Vec<Card>>(), vec![KING_HEARTS, QUEEN_HEARTS, JACK_HEARTS, NINE_HEARTS]);
///
/// ```
pub struct StackHandIntervalIterator{
    hand: StackHand,
    higher_position: u64,
    lower_position: u64
}


impl Iterator for StackHandIntervalIterator{
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {

        while self.lower_position <= self.higher_position{
            if self.lower_position & self.hand.cards != 0{
                let card = Card::from_mask(self.lower_position).unwrap();
                self.lower_position <<=1;
                return Some(card);
            }
            else{
                self.lower_position <<=1;
            }
        }
        None

    }
}

impl DoubleEndedIterator for StackHandIntervalIterator{
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.higher_position >= self.lower_position{
            if self.higher_position & self.hand.cards != 0{
                let card = Card::from_mask(self.higher_position).unwrap();
                self.higher_position >>=1;
                return Some(card);
            }
            else{
                self.higher_position >>=1;
            }
        }
        None

    }
}

impl StackHandIntervalIterator{
    pub fn new(hand: StackHand, lower_card: &Card, higher_card: &Card) -> Self{
        Self{hand, lower_position: lower_card.mask(), higher_position: higher_card.mask()}
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




impl HandSuitedTrait for StackHand{
    type SuitIterator = StackHandSuitIterator;
    type St = Suit;

    /// ```
    /// use karty::cards::{ACE_HEARTS, KING_HEARTS, KING_SPADES};
    /// use karty::hand::{HandTrait, StackHand};
    /// use crate::karty::hand::HandSuitedTrait;
    /// use karty::suits::Suit::{Hearts, Spades};
    /// let mut hand = StackHand::empty();
    /// assert!(!hand.contains_in_suit(&Spades));
    /// assert!(!hand.contains_in_suit(&Hearts));
    /// hand.insert_card(ACE_HEARTS).unwrap();
    /// assert!(hand.contains_in_suit(&Hearts));
    /// hand.insert_card(KING_SPADES).unwrap();
    /// assert!(hand.contains_in_suit(&Spades))
    /// ```
    fn contains_in_suit(&self, suit: &Suit) -> bool{
        self.cards & Self::suit_mask(*suit) != 0
    }

    ///
    /// ```
    /// use karty::cards::{ACE_CLUBS, ACE_DIAMONDS, ACE_HEARTS, ACE_SPADES, Card, THREE_SPADES, TWO_SPADES};
    /// use karty::hand::{HandTrait, StackHand};
    /// use karty::suits::Suit::{Clubs, Spades};
    /// use crate::karty::hand::HandSuitedTrait;
    /// let mut hand = StackHand::empty();
    /// hand.insert_card(ACE_HEARTS).unwrap();
    /// hand.insert_card(ACE_DIAMONDS).unwrap();
    /// hand.insert_card(ACE_CLUBS).unwrap();
    /// hand.insert_card(ACE_SPADES).unwrap();
    /// hand.insert_card(TWO_SPADES).unwrap();
    /// hand.insert_card(THREE_SPADES).unwrap();
    /// let spades: Vec<Card> = hand.create_suit_iterator(&Spades).collect();
    /// let clubs: Vec<Card> = hand.create_suit_iterator(&Clubs).collect();
    /// let spades_reverse: Vec<Card> = hand.create_suit_iterator(&Spades).rev().collect();
    /// assert_eq!(spades, vec![ TWO_SPADES, THREE_SPADES, ACE_SPADES]);
    /// assert_eq!(spades_reverse, vec![ ACE_SPADES, THREE_SPADES, TWO_SPADES]);
    /// assert_eq!(clubs, vec![ACE_CLUBS]);
    ///
    /// ```
    fn create_suit_iterator(&self, suit: &Suit) -> Self::SuitIterator {
        StackHandSuitIterator::new(*self, *suit)
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
                 h |= $x.mask();
            )*
            $crate::hand::StackHand::from(h)
            /*let mut hand = StackHand::empty();
            $(
                hand.insert_card($x).unwrap();
            )*
            hand*/
        }
    };
}

pub const HAND_OF_SPADES: StackHand = StackHand{cards: MASK_SPADES};
pub const HAND_OF_HEARTS: StackHand = StackHand{cards: MASK_HEARTS};
pub const HAND_OF_DIAMONDS: StackHand = StackHand{cards: MASK_DIAMONDS};
pub const HAND_OF_CLUBS: StackHand = StackHand{cards: MASK_CLUBS};



#[cfg(test)]
mod tests{
    use crate::cards::{ACE_SPADES, KING_HEARTS, TEN_DIAMONDS, FOUR_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS};
    use crate::hand::{HandTrait, StackHand};

    #[test]
    fn stack_hand_macro(){

        let hand = stack_hand![ACE_SPADES, KING_HEARTS, TEN_DIAMONDS, FOUR_SPADES];
        assert_eq!(hand.len(), 4);
        assert!(hand.contains(&ACE_SPADES));
        let hand_2 = stack_hand![ACE_SPADES, JACK_CLUBS, KING_CLUBS, QUEEN_SPADES];
        assert!(hand_2.contains(&ACE_SPADES));
        assert!(hand_2.contains(&QUEEN_SPADES));
        assert!(hand_2.contains(&JACK_CLUBS));
        assert!(hand_2.contains(&KING_CLUBS));
        //assert_eq!(hand_2.len(), 4);
    }

    #[test]
    fn insert_qeen_spades(){
        assert_eq!(QUEEN_SPADES.mask(), 1u64<<49);
        let mut hand = StackHand::empty();
        hand.insert_card(QUEEN_SPADES).unwrap();
        assert!(hand.contains(&QUEEN_SPADES));
        assert_eq!(stack_hand![QUEEN_SPADES].cards, hand.cards);
    }
}