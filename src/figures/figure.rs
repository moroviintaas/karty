
use std::fmt::Debug;
use std::hash::Hash;
use crate::card_element::CardElement;


pub trait Figure: Debug + Ord + Clone + Hash + CardElement {
    const NUMBER_OF_FIGURES: usize = Self::DIMENSION_SIZE;
    /*fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;*/

}

