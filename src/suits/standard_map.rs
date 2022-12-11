use std::ops::{Index, IndexMut};
use crate::suits::{Suit};
use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SuitMap<T>{
    pub spades: T,
    pub hearts: T,
    pub diamonds: T,
    pub clubs: T
}

impl<T> SuitMap<T>{
    pub fn new(spades: T, hearts: T, diamonds: T, clubs:T) -> Self{
        Self{
            spades,
            hearts,
            diamonds,
            clubs,
        }
    }
    pub fn new_from_f<F>(f: F) -> Self
    where F: Fn(Suit) -> T{
        Self{
            spades: f(Spades),
            hearts: f(Hearts),
            diamonds: f(Diamonds),
            clubs: f(Clubs),
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
