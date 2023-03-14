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
pub struct CardSet {
    pub(crate) cards: u64,
}

impl CardSet {
    

    

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

    /// ```
    ///  use karty::cards::{Card,*};
    /// use karty::hand::{CardSet, StackHandIntervalIterator};
    /// use karty::symbol::CardSymbol;
    /// let hand = CardSet::from_iter(Card::iterator().filter(|c| c.position()%4==1));
    /// assert_eq!(hand.interval_iterator(&THREE_DIAMONDS, &FIVE_DIAMONDS).collect::<Vec<Card>>(), Vec::new());
    /// assert_eq!(hand.interval_iterator(&TWO_DIAMONDS, &QUEEN_DIAMONDS).collect::<Vec<Card>>(), vec![TWO_DIAMONDS, SIX_DIAMONDS, TEN_DIAMONDS]);
    /// //assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &QUEEN_HEARTS).collect::<Vec<Card>>(), vec![NINE_HEARTS, JACK_HEARTS, QUEEN_HEARTS]);
    /// //assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &KING_HEARTS).rev().collect::<Vec<Card>>(), vec![KING_HEARTS, QUEEN_HEARTS, JACK_HEARTS, NINE_HEARTS]);
    ///
    /// ```
    pub fn interval_iterator(self, lower_card: &Card, higher_card: &Card) -> StackHandIntervalIterator{
        StackHandIntervalIterator::new(self, lower_card, higher_card)
    }

    /// ```
    ///  use karty::cards::{Card,*};
    /// use karty::hand::{CardSet, StackHandIntervalIterator};
    /// use karty::symbol::CardSymbol;
    /// let hand = CardSet::from_iter(Card::iterator().filter(|c| c.position()%4==1));
    /// assert_eq!(hand.excluding_interval_iterator(&THREE_DIAMONDS, &FIVE_DIAMONDS).collect::<Vec<Card>>(), Vec::new());
    /// assert_eq!(hand.excluding_interval_iterator(&TWO_DIAMONDS, &QUEEN_DIAMONDS).collect::<Vec<Card>>(), vec![SIX_DIAMONDS, TEN_DIAMONDS]);
    /// assert_eq!(hand.excluding_interval_iterator(&FOUR_SPADES, &ACE_SPADES).collect::<Vec<Card>>(), vec![EIGHT_SPADES, QUEEN_SPADES]);
    /// //assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &QUEEN_HEARTS).collect::<Vec<Card>>(), vec![NINE_HEARTS, JACK_HEARTS, QUEEN_HEARTS]);
    /// //assert_eq!(StackHandIntervalIterator::new(hand, &SEVEN_HEARTS, &KING_HEARTS).rev().collect::<Vec<Card>>(), vec![KING_HEARTS, QUEEN_HEARTS, JACK_HEARTS, NINE_HEARTS]);
    ///
    /// ```
    pub fn excluding_interval_iterator(self,lower_card: &Card, higher_card: &Card) -> StackHandIntervalIterator {
        StackHandIntervalIterator::new_excluding(self, lower_card, higher_card)
    }


}


impl From<CardSet> for u64{
    fn from(hand: CardSet) -> Self {
        hand.cards
    }
}

impl From<u64> for CardSet {
    fn from(cards: u64) -> Self {
        Self{cards: cards & MASK_STACK_HAND_LEGAL}
    }
}



pub struct StackHandIterator {
    hand: CardSet,
    lower_position: u64,
    higher_position: u64,

}


impl StackHandIterator {
    /// ```
    /// use karty::cards::{*};
    /// use karty::hand::StackHandIterator;
    /// use karty::stack_hand;
    /// let mut iter_empty = StackHandIterator::new(stack_hand![]);
    /// assert!(iter_empty.next().is_none());
    /// let mut iter_empty = StackHandIterator::new(stack_hand![KING_SPADES, ACE_HEARTS, JACK_CLUBS, TEN_DIAMONDS]);
    /// assert_eq!(iter_empty.collect::<Vec<Card>>(), vec![JACK_CLUBS, TEN_DIAMONDS, ACE_HEARTS, KING_SPADES]);
    /// ```
    pub fn new(hand: CardSet) -> Self{
        match hand.cards.trailing_zeros(){
            n @ 0..=63 => Self{ lower_position: 1<<n, hand, higher_position:1<<(63-hand.cards.leading_zeros())},
            _ => Self{lower_position: 1<<63, hand, higher_position: 0}
        }

        //Self{ lower_position: 1<<hand.cards.trailing_zeros(), hand, higher_position:1<<(63-hand.cards.leading_zeros())}//51
    }
}
/// ```
/// use karty::cards::{ACE_CLUBS, ACE_SPADES, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS};
/// use karty::hand::{HandTrait, CardSet};
/// let mut hand = CardSet::empty();
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
/// ```
/// use karty::cards::{Card, *};
/// use karty::hand::CardSet;
/// use karty::symbol::CardSymbol;
/// let hand = CardSet::from_iter(Card::iterator());
/// let v: Vec<Card> = hand.into_iter().collect();
/// assert_eq!(&v[13..26], &[TWO_DIAMONDS, THREE_DIAMONDS, FOUR_DIAMONDS, FIVE_DIAMONDS, SIX_DIAMONDS, SEVEN_DIAMONDS,
/// EIGHT_DIAMONDS, NINE_DIAMONDS, TEN_DIAMONDS, JACK_DIAMONDS, QUEEN_DIAMONDS, KING_DIAMONDS, ACE_DIAMONDS ]);
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
    /// use karty::hand::CardSet;
    /// use karty::symbol::CardSymbol;
    /// let mut full_hand = CardSet::from_iter(Card::iterator()).into_iter();
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
/// use karty::hand::{HandTrait, CardSet};
/// let mut hand = CardSet::empty();
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
    hand: CardSet,
    //mask: u64,
    //lower_guard: u64,
    higher_position: u64,
    lower_position: u64
    //higher_guard: u64,

}
impl StackHandSuitIterator {
    pub fn new(hand: CardSet, suit: Suit) -> Self{
        match hand.is_empty(){
            true => Self{lower_position: 1<<63, higher_position: 0, hand},
            false => {
                let lower_position = 1u64 <<(suit.position()*Figure::SYMBOL_SPACE);
                let higher_position = 1u64<<(suit.position()*Figure::SYMBOL_SPACE + Figure::SYMBOL_SPACE -1);
                Self{lower_position, higher_position, hand}
            }
        }



    }
}


/// ```
/// use karty::cards::{ACE_CLUBS, ACE_SPADES, Card, JACK_SPADES, KING_HEARTS, QUEEN_DIAMONDS, TWO_SPADES};
/// use karty::hand::{HandTrait, CardSet, StackHandSuitIterator};
/// use karty::suits::Suit::Spades;
/// let mut hand = CardSet::empty();
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
/// use karty::hand::{HandTrait, CardSet, StackHandSuitIterator};
/// use karty::suits::Suit::Spades;
/// let mut hand = CardSet::empty();
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
    hand: CardSet,
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
    pub fn new(hand: CardSet, lower_card: &Card, higher_card: &Card) -> Self{
        Self{hand, lower_position: lower_card.mask(), higher_position: higher_card.mask()}
    }
    pub fn new_excluding(hand: CardSet, lower_card: &Card, higher_card: &Card) -> Self{
        Self{hand, lower_position: lower_card.mask()<<1, higher_position: higher_card.mask()>>1}
    }
}

impl IntoIterator for CardSet {
    type Item = Card;

    type IntoIter = StackHandIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackHandIterator::new(self)
    }
}

impl HandTrait for CardSet {
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




impl HandSuitedTrait for CardSet {
    type SuitIterator = StackHandSuitIterator;
    type St = Suit;

    /// ```
    /// use karty::cards::{ACE_HEARTS, KING_HEARTS, KING_SPADES};
    /// use karty::hand::{HandTrait, CardSet};
    /// use crate::karty::hand::HandSuitedTrait;
    /// use karty::suits::Suit::{Hearts, Spades};
    /// let mut hand = CardSet::empty();
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
    /// use karty::hand::{HandTrait, CardSet};
    /// use karty::suits::Suit::{Clubs, Spades};
    /// use crate::karty::hand::HandSuitedTrait;
    /// let mut hand = CardSet::empty();
    /// hand.insert_card(ACE_HEARTS).unwrap();
    /// hand.insert_card(ACE_DIAMONDS).unwrap();
    /// hand.insert_card(ACE_CLUBS).unwrap();
    /// hand.insert_card(ACE_SPADES).unwrap();
    /// hand.insert_card(TWO_SPADES).unwrap();
    /// hand.insert_card(THREE_SPADES).unwrap();
    /// let spades: Vec<Card> = hand.suit_iterator(&Spades).collect();
    /// let clubs: Vec<Card> = hand.suit_iterator(&Clubs).collect();
    /// let spades_reverse: Vec<Card> = hand.suit_iterator(&Spades).rev().collect();
    /// assert_eq!(spades, vec![ TWO_SPADES, THREE_SPADES, ACE_SPADES]);
    /// assert_eq!(spades_reverse, vec![ ACE_SPADES, THREE_SPADES, TWO_SPADES]);
    /// assert_eq!(clubs, vec![ACE_CLUBS]);
    ///
    /// ```
    fn suit_iterator(&self, suit: &Suit) -> Self::SuitIterator {
        StackHandSuitIterator::new(*self, *suit)
    }



    
    
    
}

impl Display for CardSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v: Vec<Card> = self.into_iter().collect();
        write!(f,  "[")?;
        if f.alternate(){
            for e in v.into_iter(){
                write!(f, "{e:#}, ")?;
            }


        }
        else{
            for e in v.into_iter(){
                write!(f, "{e}, ")?;
            }
        }
        write!(f, "]")
    }
}

impl FromIterator<Card> for CardSet {
    /// ```
    /// use karty::hand::{HandTrait, CardSet};
    /// use karty::stack_hand;
    /// use karty::cards::*;
    /// let mut hand = CardSet::from_iter(vec![KING_SPADES, QUEEN_DIAMONDS, FOUR_SPADES, THREE_CLUBS]);
    /// assert_eq!(hand.len(), 4);
    /// assert!(hand.contains(&QUEEN_DIAMONDS));
    /// assert!(!hand.contains(&NINE_CLUBS));
    ///
    /// ```
    fn from_iter<T: IntoIterator<Item=Card>>(iter: T) -> Self {
        let mut hand = CardSet::empty();
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
            $crate::hand::CardSet::from(h)
            /*let mut hand = StackHand::empty();
            $(
                hand.insert_card($x).unwrap();
            )*
            hand*/
        }
    };
}

pub const HAND_OF_SPADES: CardSet = CardSet {cards: MASK_SPADES};
pub const HAND_OF_HEARTS: CardSet = CardSet {cards: MASK_HEARTS};
pub const HAND_OF_DIAMONDS: CardSet = CardSet {cards: MASK_DIAMONDS};
pub const HAND_OF_CLUBS: CardSet = CardSet {cards: MASK_CLUBS};



#[cfg(test)]
mod tests{
    use crate::cards::{ACE_SPADES, KING_HEARTS, TEN_DIAMONDS, FOUR_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS};
    use crate::hand::{HandTrait, CardSet};

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
        let mut hand = CardSet::empty();
        hand.insert_card(QUEEN_SPADES).unwrap();
        assert!(hand.contains(&QUEEN_SPADES));
        assert_eq!(stack_hand![QUEEN_SPADES].cards, hand.cards);
    }
}