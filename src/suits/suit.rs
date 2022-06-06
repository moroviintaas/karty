
use std::fmt::Debug;
use std::hash::Hash;
use crate::card_dimension::CardDimension;


pub trait Suit: Debug + Ord + Clone + Hash + CardDimension{
    const NUMBER_OF_SUITS: usize = Self::DIMENSION_SIZE;


}