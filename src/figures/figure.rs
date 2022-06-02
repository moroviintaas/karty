
use std::fmt::Debug;
use std::hash::Hash;
use crate::error::CardError;


pub trait Figure: Debug + Eq + Ord + Clone + Hash{
    const NUMBER_OF_FIGURES: usize;
    fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;

}

