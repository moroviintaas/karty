//! Module containing implementation of standard figures.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use comparator::Comparator;
#[cfg(feature = "random")]
use karty_proc_macro::RandomSymbol;
#[cfg(feature = "random")]
use rand::prelude::Distribution;
#[cfg(feature = "random")]
use rand::distributions::Standard;
#[cfg(feature = "random")]
use rand::Rng;

///Maximal symbol on standard deck figure `=10`
pub const MAX_NUMBER_FIGURE: u8 = 10;
///Minimal symbol on standard deck figure `=2`
pub const MIN_NUMBER_FIGURE: u8 = 2;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomSymbol))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberFigure {
    power: u8
}
impl NumberFigure {
    /// Constructor of [`NumberedFigure`](crate::figures::NumberFigure).
    /// # Panics:
    /// When power is lesser than [`MIN_NUMBER_FIGURE`](crate::figures::MIN_NUMBER_FIGURE) or greater than [`MAX_NUMBER_FIGURE`](crate::figures::MAX_NUMBER_FIGURE).
    ///
    pub fn new(power: u8) -> Self{
        match power{
            legit @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE => Self{power: legit},
            e => panic!("Invalid power value {e:?}")
        }
    }

    /// Returns order number (could be interpreted as [`CardSymbol`](crate::symbol::CardSymbol) if figure is stand alone symbol).
    /// # Warn!
    /// Might be deleted in the future.
    pub fn order_number(&self) -> usize {
        usize::from(self.power - 2 )
    }


    /// Returns a mask of a figure as in example. It can be used for optimised storing in true/false arrays.
    /// # Example:
    /// ```
    /// use karty::figures::{*};
    /// assert_eq!(F2.mask(), 0x04);
    /// assert_eq!(F3.mask(), 0x08);
    /// assert_eq!(F4.mask(), 0x10);
    /// assert_eq!(F5.mask(), 0x20);
    /// assert_eq!(F6.mask(), 0x40);
    /// assert_eq!(F7.mask(), 0x80);
    /// assert_eq!(F8.mask(), 0x100);
    /// assert_eq!(F9.mask(), 0x200);
    /// assert_eq!(F10.mask(), 0x400);
    ///
    /// ```
    pub fn mask(&self) -> u64{
        1u64<<self.power
    }

}

impl std::fmt::Display for NumberFigure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.power)
    }
}

impl Ord for NumberFigure {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power.cmp(&other.power)
    }
}

impl PartialOrd<Self> for NumberFigure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.power.partial_cmp(&other.power)
    }
}

impl CardSymbol for NumberFigure {
    const SYMBOL_SPACE: usize = 9;

    /// ```
    /// use karty::figures::{NumberFigure};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(NumberFigure::new(2).position(), 0);
    /// assert_eq!(NumberFigure::new(10).position(), 8);
    /// ```
    fn position(&self) -> usize {
        (self.power - 2) as usize
    }

    ///
    /// ```
    /// use karty::figures::{NumberFigure};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(NumberFigure::from_position(6).unwrap(), NumberFigure::new(8));
    /// assert_eq!(NumberFigure::from_position(3).unwrap(), NumberFigure::new(5));
    /// ```
    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            p@ 0..=8 => Ok(Self{power: (p + 2) as u8 }),
            s => Err(CardError::WrongFigurePosition(s))
        }
    }
}

impl FigureTrait for NumberFigure {
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;

}




#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "random", derive(RandomSymbol))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Figure {
    Ace,
    King,
    Queen,
    Jack,
    ///Figure with number symbol (2-10)
    Numbered(NumberFigure)
}
impl Figure {


    /// Returns a mask for figure for efficient storing bool tables
    /// ```
    /// use karty::figures::{F2, F10};
    /// use karty::figures::Figure;
    /// assert_eq!(Figure::Ace.mask(),      0b0100000000000000);
    /// assert_eq!(Figure::King.mask(),     0b0010000000000000);
    /// assert_eq!(Figure::Queen.mask(),    0b0001000000000000);
    /// assert_eq!(Figure::Jack.mask(),     0b0000100000000000);
    /// assert_eq!(F10.mask(),              0b0000010000000000);
    /// assert_eq!(F2.mask(),               0b0000000000000100);
    /// ```
    pub fn mask(&self) -> u64{
        match self{
            Figure::Ace => 0x4000,
            Figure::King => 0x2000,
            Figure::Queen => 0x1000,
            Figure::Jack => 0x800,
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
    fn from_power(power: u8) -> Option<Self>{
        match power{
            14 => Some(Self::Ace),
            13 => Some(Self::King),
            12 => Some(Self::Queen),
            11 => Some(Self::Jack),
            n @ 2..=10 => Some(Self::Numbered(NumberFigure {power: n})),
            _ => None
        }
    }
    ///
    /// ```
    /// use karty::figures::Figure;
    /// use karty::figures::{F2, King};
    /// assert_eq!(Figure::from_mask(0b0100).unwrap(), F2);
    /// assert_eq!(Figure::from_mask(0x2000).unwrap(), King);
    ///
    /// ```
    pub fn from_mask(mask: u64) -> Option<Self>{
        if mask.count_ones() != 1{
            None
        }
        else{
            let power = mask.trailing_zeros() as u8;
            Self::from_power(power)
        }
        
    }



}

impl CardSymbol for Figure {
    const SYMBOL_SPACE: usize = 13;

    /// ```
    /// use karty::figures::{F10, F2, Queen};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(F2.position(), 0);
    /// assert_eq!(F10.position(), 8);
    /// assert_eq!(Queen.position(), 10);
    /// ```
    fn position(&self) -> usize {
        match self{
            Ace => 12,
            King => 11,
            Queen => 10,
            Jack => 9,
            Numbered(fig) => fig.order_number()
        }
    }
    /// ```
    /// use karty::figures::{F5, F8, Figure, King};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(Figure::from_position(6).unwrap(), F8);
    /// assert_eq!(Figure::from_position(3).unwrap(), F5);
    /// assert_eq!(Figure::from_position(11).unwrap(), King);
    /// ```
    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            p@ 0..=8 => Ok(Numbered(NumberFigure::from_position(p)?)),
            9 => Ok(Jack),
            10 => Ok(Queen),
            11 => Ok(King),
            12 => Ok(Ace),
            s => Err(CardError::WrongFigurePosition(s))
        }
    }
}

impl FigureTrait for Figure {
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;

}

impl std::fmt::Display for Figure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            match self{
                Ace => write!(f, "𝑨"),
                King => write!(f, "𝑲"),
                Queen => write!(f, "𝑸"),
                Jack => write!(f, "𝑱"),
                Numbered(n) => write!(f, "{n}")
            }
        }
        else{
            match self{
                Ace => write!(f, "Ace"),
                King => write!(f, "King"),
                Queen => write!(f, "Queen"),
                Jack => write!(f, "Jack"),
                Numbered(n) => write!(f, "{n}")
            }

        }
    }
}
/// Array of standard figures in ascending order
pub const FIGURES: [Figure;13] = [Numbered(NumberFigure {power: 2}), Numbered(NumberFigure {power: 3}),
    Numbered(NumberFigure {power: 4}),Numbered(NumberFigure {power: 5}),
    Numbered(NumberFigure {power: 6}), Numbered(NumberFigure {power: 7}),
    Numbered(NumberFigure {power: 8}), Numbered(NumberFigure {power: 9}),
    Numbered(NumberFigure {power: 10}), Jack,  Queen, King, Ace        ];

impl PartialOrd for Figure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.power().cmp(&other.power()))
        //Some()
    }
}

impl Ord for Figure {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power().cmp(&other.power())
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct FigureComparator {
}
impl Comparator<Figure> for FigureComparator {
    fn compare(&self, l: &Figure, r: &Figure) -> Ordering {
        l.power().cmp(&r.power())
    }
}

#[cfg(test)]
mod tests{
    use crate::figures::{F10, F2};
    use crate::figures::standard::{Figure, NumberFigure};

    #[test]
    fn test_ordering(){
        let king = Figure::King;
        let ten = Figure::Numbered(NumberFigure::new(10));
        let four = Figure::Numbered(NumberFigure::new(4));
        let ace = Figure::Ace;
        let king2 = Figure::King;

        assert!(king > ten);
        assert!(four < ten);
        assert!(king < ace);

        assert_eq!(king, king2);
    }

    #[test]
    fn masks(){

        assert_eq!(Figure::Ace.mask(), 0b0100000000000000);
        assert_eq!(Figure::King.mask(), 0b0010000000000000);
        assert_eq!(Figure::Queen.mask(), 0b0001000000000000);
        assert_eq!(Figure::Jack.mask(), 0b0000100000000000);
        assert_eq!(F10.mask(),      0b0000010000000000);
        assert_eq!(F2.mask(),       0b0000000000000100);
    }

    #[test]
    fn formatting(){
        assert_eq!(format!("{:#}", Figure::Ace), String::from("𝑨"))
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn test_speedy(){
        use speedy::{Readable, Writable};
        let serialized_king = Figure::King.write_to_vec().unwrap();
        let serialized_10 = F10.write_to_vec().unwrap();
        let deserialized_king = Figure::read_from_buffer(&serialized_king).unwrap();
        let deserialized_10 = Figure::read_from_buffer(&serialized_10).unwrap();

        assert_eq!(deserialized_king, Figure::King);
        assert_eq!(deserialized_10, F10);
    }

    #[test]
    #[cfg(feature = "serde_ron")]
    fn test_serde_ron(){
        use ron::ser::to_string_pretty;
        let f: NumberFigure = ron::from_str("NumberFigure(power: 7)").unwrap();
        assert_eq!(f, NumberFigure::new(7));

        let mut pc = ron::ser::PrettyConfig::new();
        pc.struct_names = true;
        let king = to_string_pretty(&Figure::King, pc).unwrap();
        assert_eq!(king, "King");
    }
}

/// Alias for Figure with symbol 2
pub const F2: Figure = Numbered(NumberFigure {power: 2});
/// Alias for Figure with symbol 3
pub const F3: Figure = Numbered(NumberFigure {power: 3});
/// Alias for Figure with symbol 4
pub const F4: Figure = Numbered(NumberFigure {power: 4});
/// Alias for Figure with symbol 5
pub const F5: Figure = Numbered(NumberFigure {power: 5});
/// Alias for Figure with symbol 6
pub const F6: Figure = Numbered(NumberFigure {power: 6});
/// Alias for Figure with symbol 7
pub const F7: Figure = Numbered(NumberFigure {power: 7});
/// Alias for Figure with symbol 8
pub const F8: Figure = Numbered(NumberFigure {power: 8});
/// Alias for Figure with symbol 9
pub const F9: Figure = Numbered(NumberFigure {power: 9});
/// Alias for Figure with symbol 10
pub const F10: Figure = Numbered(NumberFigure {power: 10});

pub use Figure::{Ace, King, Queen, Jack};
use crate::symbol::CardSymbol;
use crate::error::CardError;
use crate::figures::Figure::Numbered;
use crate::figures::FigureTrait;
