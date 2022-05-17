
use std::fmt::Debug;



pub trait Suit: Debug + Eq + Ord + Clone{
    const NUMBER_OF_SUITS: u8;

}