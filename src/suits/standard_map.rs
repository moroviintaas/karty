use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use crate::suits::{Suit};
use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
use crate::symbol::CardSymbol;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SuitMap<T>{
    pub spades: T,
    pub hearts: T,
    pub diamonds: T,
    pub clubs: T,
    privileged_suit: Option<Suit>
}

impl<T> SuitMap<T>{
    pub fn new(spades: T, hearts: T, diamonds: T, clubs:T) -> Self{
        Self{
            spades,
            hearts,
            diamonds,
            clubs,
            privileged_suit: None
        }
    }
    
    pub fn with_privilege(mut self, privileged: Suit) -> Self{
        self.privileged_suit = Some(privileged);
        self
    }

    pub fn new_from_f<F>(f: F) -> Self
    where F: Fn(Suit) -> T{
        Self{
            spades: f(Spades),
            hearts: f(Hearts),
            diamonds: f(Diamonds),
            clubs: f(Clubs),
            privileged_suit: None
        }
    }
}


impl<T> Index<Suit> for SuitMap<T>{
    type Output = T;

    fn index(&self, index: Suit) -> &Self::Output {
        match index{
            Suit::Spades => &self.spades,
            Suit::Hearts => &self.hearts,
            Suit::Diamonds => &self.diamonds,
            Suit::Clubs => &self.clubs
        }
    }
}

impl<T> IndexMut<Suit> for SuitMap<T>{
    fn index_mut(&mut self, index: Suit) -> &mut Self::Output {
        match index{
            Suit::Spades => &mut self.spades,
            Suit::Hearts => &mut self.hearts,
            Suit::Diamonds => &mut self.diamonds,
            Suit::Clubs => &mut self.clubs
        }
    }
}


pub struct SuitMapIterator<T: IntoIterator>{
    current_low_suit: Suit,
    current_high_suit: Suit,
    map: SuitMap<<T as IntoIterator>::IntoIter>,
}

impl<T: IntoIterator> SuitMapIterator<T>{
    pub fn new(suit_map: SuitMap<T>) -> Self{
        let map = SuitMap::new(suit_map.spades.into_iter(),
        suit_map.hearts.into_iter(), suit_map.diamonds.into_iter(), suit_map.clubs.into_iter());
        Self{current_high_suit: Spades, current_low_suit: Clubs, map}
        //Self { current_low_suit: Suit::Clubs, current_low_iterator: (), current_high_suit: (), current_high_iterator: (), source: suit_map }
    }
}


/// ```
/// use karty::cards::{ACE_HEARTS, ACE_SPADES, Card, FIVE_SPADES, FOUR_HEARTS, JACK_CLUBS, NINE_DIAMONDS, NINE_HEARTS, NINE_SPADES, QUEEN_HEARTS, SIX_DIAMONDS, SIX_SPADES, THREE_HEARTS, TWO_CLUBS, TWO_SPADES};
/// use karty::figures::{*, Figure};
/// use karty::suits::SuitMap;
/// let suit_map_cards = SuitMap::<Vec<Card>>::new(
///     vec![TWO_SPADES, FIVE_SPADES, SIX_SPADES, NINE_SPADES, ACE_SPADES],
///     vec![THREE_HEARTS, FOUR_HEARTS, NINE_HEARTS, QUEEN_HEARTS, ACE_HEARTS],
///     vec![SIX_DIAMONDS, NINE_DIAMONDS],
///     vec![JACK_CLUBS]
/// );
/// let mut suit_map_card_iterator = suit_map_cards.into_iter();
///
/// //assert_eq!(suit_map_card_iterator.next(), Some(JACk_CLUBS));
///
/// assert_eq!(suit_map_card_iterator.into_iter().collect::<Vec<Card>>(), vec![JACK_CLUBS, SIX_DIAMONDS, NINE_DIAMONDS, THREE_HEARTS, FOUR_HEARTS, NINE_HEARTS, QUEEN_HEARTS, ACE_HEARTS,
/// TWO_SPADES, FIVE_SPADES, SIX_SPADES, NINE_SPADES, ACE_SPADES]);
/// ```
impl<T: IntoIterator> Iterator for SuitMapIterator<T>{
    type Item = <T as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_low_suit.cmp(&self.current_high_suit){
            std::cmp::Ordering::Less => match self.map[self.current_low_suit].next(){
                None => {
                    self.current_low_suit = self.current_low_suit.higher_n(1).unwrap_or(self.current_high_suit);
                    self.next()
                }
                Some(item) => Some(item)
            },
            std::cmp::Ordering::Equal => self.map[self.current_low_suit].next() ,
            std::cmp::Ordering::Greater => None,
        }
    }
}

impl<T: IntoIterator> IntoIterator for SuitMap<T>{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = SuitMapIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SuitMapIterator::new(self)
    }
}
/// ```
/// use karty::cards::{ACE_HEARTS, ACE_SPADES, Card, FIVE_SPADES, FOUR_HEARTS, JACK_CLUBS, NINE_DIAMONDS, NINE_HEARTS, NINE_SPADES, QUEEN_HEARTS, SIX_DIAMONDS, SIX_SPADES, THREE_HEARTS, TWO_CLUBS, TWO_SPADES};
/// use karty::figures::{*, Figure};
/// use karty::suits::SuitMap;
/// let suit_map_cards = SuitMap::<Vec<Card>>::new(
///     vec![TWO_SPADES, FIVE_SPADES, SIX_SPADES, NINE_SPADES, ACE_SPADES],
///     vec![THREE_HEARTS, FOUR_HEARTS, NINE_HEARTS, QUEEN_HEARTS, ACE_HEARTS],
///     vec![SIX_DIAMONDS, NINE_DIAMONDS],
///     vec![JACK_CLUBS]
/// );
/// let mut suit_map_card_iterator = suit_map_cards.into_iter();
/// assert_eq!(suit_map_card_iterator.into_iter().rev().collect::<Vec<Card>>(), vec![ACE_SPADES, NINE_SPADES, SIX_SPADES, FIVE_SPADES, TWO_SPADES,
///     ACE_HEARTS, QUEEN_HEARTS, NINE_HEARTS, FOUR_HEARTS, THREE_HEARTS, NINE_DIAMONDS, SIX_DIAMONDS, JACK_CLUBS]);
/// ```
impl<T: IntoIterator<IntoIter=I, Item=It>, I: DoubleEndedIterator + Iterator<Item=It>, It> DoubleEndedIterator for SuitMapIterator<T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_high_suit.cmp(&self.current_low_suit){
            Ordering::Less => None,
            Ordering::Equal => self.map[self.current_high_suit].next_back(),
            Ordering::Greater => match self.map[self.current_high_suit].next_back(){
                None => {
                    self.current_high_suit = self.current_high_suit.lower_n(1).unwrap_or(self.current_low_suit);
                    self.next_back()
                }
                Some(item) => Some(item)
            }
        }
    }
}

