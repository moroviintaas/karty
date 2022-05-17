
use std::fmt::Debug;




pub trait Figure: Debug + Eq + Ord + Clone{
    const NUMBER_OF_FIGURES: u8;
    fn power(&self) -> u8;
}

