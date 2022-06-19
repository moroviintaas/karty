use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::symbol::CardSymbol;
use crate::error::CardError;
use crate::error::CardError::WrongSuitPosition;
use crate::suits::standard::SuitStd::{Clubs, Diamonds, Hearts, Spades};
use crate::suits::Suit;
//use karty_proc_macro::Rnd;
#[cfg(feature = "random")]
use rand::prelude::Distribution;
#[cfg(feature = "random")]
use rand::distributions::Standard;
#[cfg(feature = "random")]
use rand::Rng;
#[cfg(feature = "random")]
use karty_proc_macro::*;


///Enum representing suits of card
#[derive(Debug, Eq, PartialEq,Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomElement))]
//#[cfg_attr(feature = "random", derive(Rnd))]
pub enum SuitStd {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl SuitStd {


}

pub const SUITS: [SuitStd; 4] = [Spades, Hearts, Diamonds, Clubs];



impl PartialOrd<Self> for SuitStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        //Some(self.age().cmp(&other.age()))
        Some(self.cmp(other))
    }
}

impl Ord for SuitStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position().cmp(&other.position())
        //self.age().cmp(&other.age())
    }
}
impl CardSymbol for SuitStd{
    const SYMBOL_SPACE: usize = 4;

    fn position(&self) -> usize {
        match self{
            Spades => 3,
            Hearts => 2,
            Diamonds => 1,
            Clubs => 0
        }
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            3 => Ok(Spades),
            2 => Ok(Hearts),
            1 => Ok(Diamonds),
            0 => Ok(Clubs),
            s => Err(WrongSuitPosition(s))
        }
    }
}

impl Suit for SuitStd{
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;

}

impl Display for SuitStd{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            match self {
                Spades => write!(f, "♠"),
                Hearts => write!(f, "♥"),
                Diamonds => write!(f, "♦"),
                Clubs => write!(f, "♣")
            }
        }
        else {
            match self{
                Spades => write!(f, "Spades"),
                Hearts => write!(f, "Hearts"),
                Diamonds => write!(f, "Diamonds"),
                Clubs => write!(f, "Clubs")
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::suits::standard::SuitStd;

    #[test]
    fn test_order(){
        let spades = SuitStd::Spades;
        let hearts = SuitStd::Hearts;
        let diamonds = SuitStd::Diamonds;
        let clubs = SuitStd::Clubs;

        assert_eq!( spades, spades);
        assert!(spades > hearts);
        assert!(spades > clubs);
        assert!(hearts > clubs && diamonds > clubs);
        assert!(clubs < spades);
    }
}