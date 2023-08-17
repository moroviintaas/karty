
use std::iter::Chain;
use std::ops::{Add, Index, IndexMut};
use crate::suits::{Suit};
use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};


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

    pub fn new_symmetric(val: T) -> Self
    where T: Clone{
        Self{
            spades: val.clone(),

            hearts: val.clone(),
            diamonds: val.clone(),
            clubs: val,
            privileged_suit: None,
        }
    }


    pub fn set_at(&mut self, suit: &Suit, value: T){
        self[*suit] = value;
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
    pub fn map<R, F: Fn(&T) -> R>(&self, f: F) -> SuitMap<R>{

        SuitMap::<R>{
            spades: f(&self.spades),
            hearts: f(&self.hearts),
            diamonds: f(&self.diamonds),
            clubs: f(&self.clubs),
            privileged_suit: self.privileged_suit,
        }
    }

    pub fn and<F: Fn(&T) -> bool>(&self, f: F) -> bool{
        f(&self.spades) && f(&self.hearts) && f(&self.diamonds) && f(&self.clubs)
    }
}

impl<T: Add<Output = T> + Copy> SuitMap<T>{
    pub fn sum(&self) -> T{
        self.spades.add(self.hearts.add(self.diamonds.add(self.clubs)))
    }
}




impl <T:Default> SuitMap<T>{
    pub fn single(suit: &Suit, value: T) -> Self{
        let mut s = Self::default();
        s.set_at(suit, value);
        s
    }
}


/*
impl<I, const C: usize> SuitMap<SmallVec<[I; C]>>{
    pub fn is_empty(&self) -> bool{
        self.spades.is_empty()
    }
}*/

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

impl<T> Index<&Suit> for SuitMap<T>{
    type Output = T;

    fn index(&self, index: &Suit) -> &Self::Output {
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

impl<T> IndexMut<&Suit> for SuitMap<T>{
    fn index_mut(&mut self, index: &Suit) -> &mut Self::Output {
        match index{
            Suit::Spades => &mut self.spades,
            Suit::Hearts => &mut self.hearts,
            Suit::Diamonds => &mut self.diamonds,
            Suit::Clubs => &mut self.clubs
        }
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
/// # DoubleSidedIterator:
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
#[allow(clippy::type_complexity)]
pub struct SuitMapIterator<T: IntoIterator>{
    iter: Chain<
    <T as IntoIterator>::IntoIter, Chain<
    <T as IntoIterator>::IntoIter, Chain<
    <T as IntoIterator>::IntoIter, <T as IntoIterator>::IntoIter>
    >>


}
#[allow(clippy::type_complexity)]
impl<T: IntoIterator> SuitMapIterator<T>{
    fn new(iter: Chain<
    <T as IntoIterator>::IntoIter, Chain<
    <T as IntoIterator>::IntoIter, Chain<
    <T as IntoIterator>::IntoIter, <T as IntoIterator>::IntoIter>
    >>) -> Self{
        Self{iter}
    }
}
impl<T: IntoIterator> Iterator for SuitMapIterator<T>{
    type Item = <T as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }


}
impl<T: IntoIterator<IntoIter=I, Item=It>, I: DoubleEndedIterator + Iterator<Item=It>, It> DoubleEndedIterator for SuitMapIterator<T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}
/*
/// ```
/// use karty::cards::{FOUR_HEARTS, KING_HEARTS, SEVEN_CLUBS, SIX_DIAMONDS, TEN_SPADES, THREE_CLUBS, THREE_SPADES};
/// use karty::suits::SuitMap;
/// let sm = SuitMap::new(vec![THREE_SPADES, TEN_SPADES], vec![FOUR_HEARTS, KING_HEARTS], vec![SIX_DIAMONDS], vec![THREE_CLUBS, SEVEN_CLUBS]);
/// let sm_iter = sm.into_iter();
/// assert_eq!(sm_iter.size_hint(), (8, Some(8)));
/// ```
impl<T: IntoIterator<IntoIter=I, Item=It>, I: ExactSizeIterator + Iterator<Item=It>, It> ExactSizeIterator for SuitMapIterator<T>
{
    
}*/


impl<T: IntoIterator> IntoIterator for SuitMap<T>{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = SuitMapIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.clubs.into_iter()
            .chain(self.diamonds.into_iter()
                .chain(self.hearts.into_iter()
                    .chain(self.spades.into_iter()))))

    }
}