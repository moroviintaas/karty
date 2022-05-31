
use std::fmt::Debug;
use std::hash::Hash;


pub trait Suit: Debug + Eq + Ord + Clone + Hash{
    const NUMBER_OF_SUITS: usize;
    fn order_number(&self) -> usize;

}