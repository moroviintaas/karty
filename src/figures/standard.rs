use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
#[cfg(feature = "random")]
use karty_proc_macro::RandomSymbol;
#[cfg(feature = "random")]
use rand::prelude::Distribution;
#[cfg(feature = "random")]
use rand::distributions::Standard;
#[cfg(feature = "random")]
use rand::Rng;


pub const MAX_NUMBER_FIGURE: u8 = 10;
pub const MIN_NUMBER_FIGURE: u8 = 2;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomSymbol))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct NumberFigureStd {
    power: u8
}
impl NumberFigureStd {
    pub fn new(power: u8) -> Self{
        match power{
            legit @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE => Self{power: legit},
            e => panic!("Invalid power value {:?}", e)
        }
    }

    pub fn order_number(&self) -> usize {
        usize::from(self.power - 2 )
    }

    /*
    /// Returns a mask of a figure as in example. It can be used for optimised storing in true/false arrays.
    /// # Example:
    /// ```
    /// use karty::figures::standard;
    /// assert_eq!(standard::F2.mask(), 0x04);
    /// assert_eq!(standard::F3.mask(), 0x08);
    /// assert_eq!(standard::F4.mask(), 0x10);
    /// assert_eq!(standard::F5.mask(), 0x20);
    /// assert_eq!(standard::F6.mask(), 0x40);
    /// assert_eq!(standard::F7.mask(), 0x80);
    /// assert_eq!(standard::F8.mask(), 0x100);
    /// assert_eq!(standard::F9.mask(), 0x200);
    /// assert_eq!(standard::F10.mask(), 0x400);
    ///
    /// ```

     */
    pub(crate) fn mask(&self) -> u64{
        1u64<<self.power
    }
    /*
    fn power(&self) -> u8{
        self.power
    }*/
}

impl std::fmt::Display for NumberFigureStd{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.power)
    }
}

impl Ord for NumberFigureStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power.cmp(&other.power)
    }
}

impl PartialOrd<Self> for NumberFigureStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.power.partial_cmp(&other.power)
    }
}

impl CardSymbol for NumberFigureStd{
    const SYMBOL_SPACE: usize = 9;

    fn position(&self) -> usize {
        (self.power - 2) as usize
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            p@ 0..=8 => Ok(Self{power: (p + 2) as u8 }),
            s => Err(CardError::WrongFigurePosition(s))
        }
    }
}

impl Figure for NumberFigureStd{
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;

}




#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomSymbol))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub enum FigureStd {
    Ace,
    King,
    Queen,
    Jack,
    Numbered(NumberFigureStd)
}
impl FigureStd {

    /*
    /// Returns a mask for figure for efficient storing bool tables
    /// ```
    /// use karty::figures::standard::{F2, F10};
    /// use karty::figures::standard::FigureStd;
    /// assert_eq!(FigureStd::Ace.mask(),      0b0100000000000000);
    /// assert_eq!(FigureStd::King.mask(),     0b0010000000000000);
    /// assert_eq!(FigureStd::Queen.mask(),    0b0001000000000000);
    /// assert_eq!(FigureStd::Jack.mask(),     0b0000100000000000);
    /// assert_eq!(F10.mask(),      0b0000010000000000);
    /// assert_eq!(F2.mask(),       0b0000000000000100);
    /// ```

     */
    pub(crate) fn mask(&self) -> u64{
        match self{
            FigureStd::Ace => 0x4000,
            FigureStd::King => 0x2000,
            FigureStd::Queen => 0x1000,
            FigureStd::Jack => 0x800,
            Numbered(n) => n.mask()

        }
    }
    fn power(&self) -> u8{
        match self{
            Ace => 14,
            King=> 13,
            Queen=> 12,
            Jack=> 11,
            Numbered(fig) => fig.power
        }
    }



}

impl CardSymbol for FigureStd{
    const SYMBOL_SPACE: usize = 13;

    fn position(&self) -> usize {
        match self{
            Ace => 12,
            King => 11,
            Queen => 10,
            Jack => 9,
            Numbered(fig) => fig.order_number()
        }
    }
    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            p@ 0..=8 => Ok(Numbered(NumberFigureStd::from_position(p)?)),
            9 => Ok(Jack),
            10 => Ok(Queen),
            11 => Ok(King),
            12 => Ok(Ace),
            s => Err(CardError::WrongFigurePosition(s))
        }
    }
}

impl Figure for FigureStd{
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;

}

impl std::fmt::Display for FigureStd{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            match self{
                FigureStd::Ace => write!(f, "𝑨"),
                FigureStd::King => write!(f, "𝑲"),
                FigureStd::Queen => write!(f, "𝑸"),
                FigureStd::Jack => write!(f, "𝑱"),
                FigureStd::Numbered(n) => write!(f, "{}", n)
            }
        }
        else{
            match self{
                FigureStd::Ace => write!(f, "Ace"),
                FigureStd::King => write!(f, "King"),
                FigureStd::Queen => write!(f, "Queen"),
                FigureStd::Jack => write!(f, "Jack"),
                FigureStd::Numbered(n) => write!(f, "{}", n)
            }

        }
    }
}

pub const FIGURES: [FigureStd;13] = [Ace, King, Queen, Jack, Numbered(NumberFigureStd {power: 10}),
        Numbered(NumberFigureStd {power: 9}), Numbered(NumberFigureStd {power: 8}),
        Numbered(NumberFigureStd {power: 7}), Numbered(NumberFigureStd {power: 6}),
        Numbered(NumberFigureStd {power: 5}), Numbered(NumberFigureStd {power: 4}),
        Numbered(NumberFigureStd {power: 3}), Numbered(NumberFigureStd {power: 2})];

impl PartialOrd for FigureStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.power().cmp(&other.power()))
        //Some()
    }
}

impl Ord for FigureStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power().cmp(&other.power())
    }
}


#[cfg(test)]
mod tests{
    use crate::figures::{F10, F2};
    use crate::figures::standard::{FigureStd, NumberFigureStd};

    #[test]
    fn test_ordering(){
        let king = FigureStd::King;
        let ten = FigureStd::Numbered(NumberFigureStd::new(10));
        let four = FigureStd::Numbered(NumberFigureStd::new(4));
        let ace = FigureStd::Ace;
        let king2 = FigureStd::King;

        assert!(king > ten);
        assert!(four < ten);
        assert!(king < ace);

        assert_eq!(king, king2);
    }

    #[test]
    fn masks(){

        assert_eq!(FigureStd::Ace.mask(),      0b0100000000000000);
        assert_eq!(FigureStd::King.mask(),     0b0010000000000000);
        assert_eq!(FigureStd::Queen.mask(),    0b0001000000000000);
        assert_eq!(FigureStd::Jack.mask(),     0b0000100000000000);
        assert_eq!(F10.mask(),      0b0000010000000000);
        assert_eq!(F2.mask(),       0b0000000000000100);
    }

    #[test]
    fn formatting(){
        assert_eq!(format!("{:#}", FigureStd::Ace), String::from("𝑨"))
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn test_speedy(){
        use speedy::{Readable, Writable};
        let serialized_king = FigureStd::King.write_to_vec().unwrap();
        let serialized_10 = F10.write_to_vec().unwrap();
        let deserialized_king = FigureStd::read_from_buffer(&serialized_king).unwrap();
        let deserialized_10 = FigureStd::read_from_buffer(&serialized_10).unwrap();

        assert_eq!(deserialized_king, FigureStd::King);
        assert_eq!(deserialized_10, F10);
    }
}

pub const F2: FigureStd = Numbered(NumberFigureStd {power: 2});
pub const F3: FigureStd = Numbered(NumberFigureStd {power: 3});
pub const F4: FigureStd = Numbered(NumberFigureStd {power: 4});
pub const F5: FigureStd = Numbered(NumberFigureStd {power: 5});
pub const F6: FigureStd = Numbered(NumberFigureStd {power: 6});
pub const F7: FigureStd = Numbered(NumberFigureStd {power: 7});
pub const F8: FigureStd = Numbered(NumberFigureStd {power: 8});
pub const F9: FigureStd = Numbered(NumberFigureStd {power: 9});
pub const F10: FigureStd = Numbered(NumberFigureStd {power: 10});

pub use FigureStd::*;
use crate::symbol::CardSymbol;
use crate::error::CardError;
use crate::figures::Figure;
