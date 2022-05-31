use std::cmp::Ordering;


pub const MAX_NUMBER_FIGURE: u8 = 10;
pub const MIN_NUMBER_FIGURE: u8 = 2;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

    /// Returns a mask of a figure in manner:
    ///
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
    pub fn mask(&self) -> u64{
        1u64<<self.power
    }
    /*
    fn power(&self) -> u8{
        self.power
    }*/
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

impl Figure for NumberFigureStd{
    const NUMBER_OF_FIGURES: usize = 9;

    fn order_number(&self) -> usize {
        usize::from(self.power - 2 )
    }



}



#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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
impl Figure for FigureStd{
    const NUMBER_OF_FIGURES: usize = 13;

    fn order_number(&self) -> usize {
        match self{
            Ace => 12,
            King => 11,
            Queen => 10,
            Jack => 9,
            Numbered(fig) => fig.order_number()
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
}

pub const F2: NumberFigureStd = NumberFigureStd {power: 2};
pub const F3: NumberFigureStd = NumberFigureStd {power: 3};
pub const F4: NumberFigureStd = NumberFigureStd {power: 4};
pub const F5: NumberFigureStd = NumberFigureStd {power: 5};
pub const F6: NumberFigureStd = NumberFigureStd {power: 6};
pub const F7: NumberFigureStd = NumberFigureStd {power: 7};
pub const F8: NumberFigureStd = NumberFigureStd {power: 8};
pub const F9: NumberFigureStd = NumberFigureStd {power: 9};
pub const F10: NumberFigureStd = NumberFigureStd {power: 10};

pub use FigureStd::*;
use crate::figures::Figure;