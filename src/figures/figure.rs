
use std::fmt::Debug;
use std::hash::Hash;



pub trait Figure: Debug + Eq + Ord + Clone + Hash{
    const NUMBER_OF_FIGURES: usize;
    //fn power(&self) -> u8;
    //fn order_number(&self) -> usize;
}

