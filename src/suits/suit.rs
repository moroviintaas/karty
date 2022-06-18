
use std::fmt::Debug;
use std::hash::Hash;
use crate::card_element::CardElement;


pub trait Suit: Debug + Ord + Clone + Hash + CardElement {
    const NUMBER_OF_SUITS: usize = Self::DIMENSION_SIZE;


}