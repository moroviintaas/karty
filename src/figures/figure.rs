
use std::fmt::Debug;




pub trait Figure: Debug + Eq + Ord + Clone{
    const NUMBER_OF_FIGURES: usize;
    //fn power(&self) -> u8;
    fn order_number(&self) -> usize;
}

