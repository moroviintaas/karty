use std::ops::{Index, IndexMut};
use crate::suits::{Suit};
use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};



#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SuitMap<T>{
    pub spades: T,
    pub hearts: T,
    pub diamonds: T,
    pub clubs: T,
    priviledged_suit: Option<Suit>
}

impl<T> SuitMap<T>{
    pub fn new(spades: T, hearts: T, diamonds: T, clubs:T) -> Self{
        Self{
            spades,
            hearts,
            diamonds,
            clubs,
            priviledged_suit: None
        }
    }
    
    pub fn with_priviledge(mut self, priviledged: Suit) -> Self{
        self.priviledged_suit = Some(priviledged);
        self
    }

    pub fn new_from_f<F>(f: F) -> Self
    where F: Fn(Suit) -> T{
        Self{
            spades: f(Spades),
            hearts: f(Hearts),
            diamonds: f(Diamonds),
            clubs: f(Clubs),
            priviledged_suit: None
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
        let map = SuitMap::new(suit_map.clubs.into_iter(),
        suit_map.diamonds.into_iter(), suit_map.hearts.into_iter(), suit_map.spades.into_iter());
        Self{current_high_suit: Spades, current_low_suit: Clubs, map}
        //Self { current_low_suit: Suit::Clubs, current_low_iterator: (), current_high_suit: (), current_high_iterator: (), source: suit_map }
    }
}

impl<T: IntoIterator> Iterator for SuitMapIterator<T>{
    type Item = <T as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_low_suit.cmp(&self.current_high_suit){
            std::cmp::Ordering::Less => todo!(),
            std::cmp::Ordering::Equal => todo!(),
            std::cmp::Ordering::Greater => None,
        }
    }
}

