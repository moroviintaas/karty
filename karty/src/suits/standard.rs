//! Module containing implementation of standard suit.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use comparator::Comparator;
use crate::symbol::CardSymbol;
use crate::error::CardError;
use crate::error::CardError::WrongSuitPosition;
use crate::suits::standard::Suit::{Clubs, Diamonds, Hearts, Spades};
use crate::suits::SuitTrait;

//use karty_proc_macro::Rnd;
#[cfg(feature = "random")]
use rand::prelude::Distribution;
#[cfg(feature = "random")]
use rand::Rng;
#[cfg(feature = "random")]
use karty_proc_macro::*;


///Enum representing standard (52-card deck) suits of cards. It implements bridge order, it is:
///  [`Spades`][crate::suits::Suit::Spades] >
/// [`Hearts`][crate::suits::Suit::Hearts] > [`Diamonds`][crate::suits::Suit::Diamonds] >
/// [`Clubs`][crate::suits::Suit::Clubs]. If you need another order create similar Enum and
/// provide it with your implementation of order. Example implementation of trait is presented in
/// [`Suit`][crate::suits::Suit].
#[derive(Debug, Eq, PartialEq,Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomSymbol))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
//#[cfg_attr(feature = "random", derive(Rnd))]
pub enum Suit {
    /// symbol: ♠, position: 3
    Spades,
    /// symbol: ♥, position: 2
    Hearts,
    /// symbol: ♦, position: 1
    Diamonds,
    /// symbol: ♣, position: 0
    Clubs
}

impl Suit {


}

/// Array of standard suits `[Clubs, Diamonds, Hearts, Spades]`
pub const SUITS: [Suit; 4] = [Clubs, Diamonds, Hearts, Spades];


/// Implemented order is [`Spades`][crate::suits::Suit::Spades] >
/// [`Hearts`][crate::suits::Suit::Hearts] > [`Diamonds`][crate::suits::Suit::Diamonds] >
/// [`Clubs`][crate::suits::Suit::Clubs].
impl PartialOrd<Self> for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        //Some(self.age().cmp(&other.age()))
        Some(self.cmp(other))
    }
}
/// Implemented order is [`Spades`][crate::suits::Suit::Spades] >
/// [`Hearts`][crate::suits::Suit::Hearts] > [`Diamonds`][crate::suits::Suit::Diamonds] >
/// [`Clubs`][crate::suits::Suit::Clubs].
impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.usize_index().cmp(&other.usize_index())
        //self.age().cmp(&other.age())
    }
}

impl CardSymbol for Suit {
    const SYMBOL_SPACE: usize = 4;

    fn usize_index(&self) -> usize {
        match self{
            Spades => 3,
            Hearts => 2,
            Diamonds => 1,
            Clubs => 0
        }
    }

    fn from_usize_index(position: usize) -> Result<Self, CardError> {
        match position{
            3 => Ok(Spades),
            2 => Ok(Hearts),
            1 => Ok(Diamonds),
            0 => Ok(Clubs),
            s => Err(WrongSuitPosition(s))
        }
    }
}

impl SuitTrait for Suit {
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;

}
/// Implements [`Display`][std::fmt::Display]
/// # Examples:
/// ```
/// use karty::suits::Suit;
///
/// assert_eq!("Spades", format!("{}", Suit::Spades));
/// assert_eq!("♠", format!("{:#}", Suit::Spades));
/// ```
impl Display for Suit {
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
/// Comparator implementing ordering `Spades > Hearts > Clubs > Diamonds`
#[derive(Default, Copy, Clone)]
pub struct ComparatorDCHS {}

impl Comparator<Suit> for ComparatorDCHS {
    fn compare(&self, l: &Suit, r: &Suit) -> Ordering {
        match l{
            Spades => match r{
                Spades => Ordering::Equal,
                _ => Ordering::Greater
            },
            Hearts => match r {
                Spades => Ordering::Less,
                Hearts => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Clubs => match r {
                Diamonds => Ordering::Greater,
                Clubs => Ordering::Equal,
                _ => Ordering::Less
            },
            Diamonds => match r {
                Diamonds => Ordering::Equal,
                _ => Ordering::Less
            }
        }
    }
}
/// Comparator implementing ordering `Spades > Hearts > Diamonds > Clubs`
#[derive(Default, Copy, Clone)]
pub struct ComparatorCDHS {}


impl Comparator<Suit> for ComparatorCDHS {
    fn compare(&self, l: &Suit, r: &Suit) -> Ordering {
        match l{
            Spades => match r{
                Spades => Ordering::Equal,
                _ => Ordering::Greater
            },
            Hearts => match r {
                Spades => Ordering::Less,
                Hearts => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Diamonds => match r {
                Clubs => Ordering::Greater,
                Diamonds => Ordering::Equal,
                _ => Ordering::Less
            },
            Clubs => match r {
                Clubs => Ordering::Equal,
                _ => Ordering::Less
            }
        }
    }
}



#[cfg(test)]
mod tests{
    use crate::suits::standard::Suit;


    #[test]
    fn test_order(){
        let spades = Suit::Spades;
        let hearts = Suit::Hearts;
        let diamonds = Suit::Diamonds;
        let clubs = Suit::Clubs;

        assert_eq!( spades, spades);
        assert!(spades > hearts);
        assert!(spades > clubs);
        assert!(hearts > clubs && diamonds > clubs);
        assert!(clubs < spades);
    }
    #[test]
    #[cfg(feature = "speedy")]
    fn test_speedy_suit(){
        use speedy::{Readable, Writable};
        let serialized_spades = Suit::Spades.write_to_vec().unwrap();
        let deserialized_spades = Suit::read_from_buffer(&serialized_spades).unwrap();
        assert_eq!(deserialized_spades, Suit::Spades);
        assert_ne!(deserialized_spades, Suit::Hearts);
    }
}