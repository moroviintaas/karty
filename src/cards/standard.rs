use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use crate::symbol::CardSymbol;
use crate::cards::{Card2SGen, CardComparatorGen};
//use crate::error::CardError::{WrongMaskFormat, WrongPosition};
use crate::figures::{Ace, FigureComparator, F10, F2, F3, F4, F5, F6, F7, F8, F9, Figure, Jack, King, Queen};
use crate::suits::Suit::*;
use crate::suits::{ComparatorAHCD, ComparatorAHDC, Suit};

use super::Card2SymTrait;
//#[cfg(feature = "speedy")]
//use speedy::{Readable, Writable};


impl Card2SGen<Figure, Suit>{
    /// ```
    /// use karty::cards::KING_HEARTS;
    /// assert_eq!(KING_HEARTS.mask(), 0x2000000000);//37 bit from ls
    /// ```
    /// ```
    /// use karty::cards::STANDARD_DECK;
    /// let mut bin_sum = 0u64;
    /// for c in &STANDARD_DECK{
    ///     assert_eq!(bin_sum & c.mask(), 0);
    ///     bin_sum |= c.mask();
    /// }
    /// assert_eq!(bin_sum, 0x0fffffffffffff)
    /// ```
    pub fn mask(&self) -> u64{

        //self.figure().mask() << (self.suit().position() * 16)
        1u64<<self.position()
    }
    /// ```
    /// use karty::cards::{Card, TWO_CLUBS, KING_SPADES};
    /// assert_eq!(Card::from_mask(0x01).unwrap(), TWO_CLUBS);
    /// assert_eq!(Card::from_mask(0x4000000000000).unwrap(), KING_SPADES);
    /// ```
    pub fn from_mask(mask: u64) -> Option<Self>{

        if mask.count_ones() != 1{
            return None
        }


        let t0 = mask.trailing_zeros();
        Some(Self::from_position(t0 as usize).unwrap())
        /*
        let suit_mask = t0/16;
        let figure_mask = mask >> (suit_mask * 16);
        match Suit::from_position(suit_mask as usize){
            Ok(suit) => Figure::from_mask(figure_mask).map(|figure| Self{suit, figure}),
            /*match Figure::from_mask(figure_mask){
                Some(figure) => Some(Self{suit, figure}),
                None => None,

            },*/
            Err(_) => None
        }

         */




    }
}


/// ```
/// use karty::cards::Card;
/// assert_eq!(std::mem::size_of::<Card>(), 3);
/// ```
pub type Card =  Card2SGen<Figure, Suit>;

/*
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Card{
    mask: u64
}

impl Card{
    pub fn mask(&self) -> u64{
        self.mask
    }
    pub fn from_mask(mask: u64) -> Result<Self, CardError>{
        if mask.count_ones() != 1{
            return Err(CardError::WrongMaskFormat)
        }
        if mask > (0x1<<51){
            return Err(CardError::MaskSpaceViolated)
        }
        Ok(Self{mask})
    }
}

impl CardSymbol for Card{
    const SYMBOL_SPACE: usize = 52;

    fn position(&self) -> usize {
        self.mask.trailing_zeros() as usize
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        match position{
            good @ 0..=51 => Ok(Self{mask: 0x1<<good}),
            e => Err(CardError::WrongPosition(e))
        }
    }
}


impl From<Card2SGen<Figure, Suit>> for Card{
    fn from(crd: Card2SGen<Figure, Suit>) -> Self {
        Self{mask: crd.mask()}
    }
}
impl From<Card> for Card2SGen<Figure, Suit>{
    fn from(crd: Card) -> Self {
        Self{figure: crd.figure(), suit: crd.suit()}
    }
}

impl Card2SymTrait for Card{
    type Figure = Figure;
    type Suit = Suit;

    fn suit(&self) -> Self::Suit {
        match self.mask.trailing_zeros(){
            0..=12 => Clubs,
            13..=25 => Diamonds,
            26..=38 => Hearts,
            39..=51 => Spades,
            e => panic!("Bad mask with {} trailing zeros.", e)

        }
    }

    fn figure(&self) -> Self::Figure {
        Figure::from_mask(match self.mask.trailing_zeros() {
            n @ 0..=12 => self.mask,
            n @ 13..=25 => self.mask >> 13,
            n @ 26..=38 => self.mask >> 26,
            n @ 39..=51 => self.mask >> 39,
            e => panic!("Too big internal mask, this should not occur. It is a bug.")
        } as u64).unwrap()

    }

    fn from_figure_and_suit(figure: Self::Figure, suit: Self::Suit) -> Self {
        let suit_mask = 0x1u64 << (13 * suit.position());
        let figure_position = figure.position();
        Self{mask: suit_mask<<figure_position}
    }
}

*/





impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "{:#}{:#}", self.figure(), self.suit()),
            false => write!(f, "{} of {}", self.figure(), self.suit())
        }

    }
}


pub type ComparatorCardStd<CS> = CardComparatorGen<Figure, Suit, FigureComparator, CS>;

pub const CARD_COMPARATOR_BRIDGE: ComparatorCardStd<ComparatorAHDC> =
    CardComparatorGen {
        suit_comparator: ComparatorAHDC{},
        figure_comparator: FigureComparator {},
        _phantom: PhantomData{}
    };
pub const CARD_COMPARATOR_VISUAL: ComparatorCardStd<ComparatorAHCD> =
    CardComparatorGen {
        suit_comparator: ComparatorAHCD{},
        figure_comparator: FigureComparator {},
        _phantom: PhantomData{}
    };

//pub const TWO_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F2)};

pub const TWO_CLUBS: Card = Card { suit: Clubs, figure: F2};
pub const THREE_CLUBS: Card = Card { suit: Clubs, figure: F3};
pub const FOUR_CLUBS: Card = Card { suit: Clubs, figure: F4};
pub const FIVE_CLUBS: Card = Card { suit: Clubs, figure: F5};
pub const SIX_CLUBS: Card = Card { suit: Clubs, figure: F6};
pub const SEVEN_CLUBS: Card = Card { suit: Clubs, figure: F7};
pub const EIGHT_CLUBS: Card = Card { suit: Clubs, figure: F8};
pub const NINE_CLUBS: Card = Card { suit: Clubs, figure: F9};
pub const TEN_CLUBS: Card = Card { suit: Clubs, figure: F10};
pub const JACK_CLUBS: Card = Card { suit: Clubs, figure: Jack};
pub const QUEEN_CLUBS: Card = Card { suit: Clubs, figure: Queen};
pub const KING_CLUBS: Card = Card { suit: Clubs, figure: King};
pub const ACE_CLUBS: Card = Card { suit: Clubs, figure: Ace};

pub const TWO_DIAMONDS: Card = Card { suit: Diamonds, figure: F2};
pub const THREE_DIAMONDS: Card = Card { suit: Diamonds, figure: F3};
pub const FOUR_DIAMONDS: Card = Card { suit: Diamonds, figure: F4};
pub const FIVE_DIAMONDS: Card = Card { suit: Diamonds, figure: F5};
pub const SIX_DIAMONDS: Card = Card { suit: Diamonds, figure: F6};
pub const SEVEN_DIAMONDS: Card = Card { suit: Diamonds, figure: F7};
pub const EIGHT_DIAMONDS: Card = Card { suit: Diamonds, figure: F8};
pub const NINE_DIAMONDS: Card = Card { suit: Diamonds, figure: F9};
pub const TEN_DIAMONDS: Card = Card { suit: Diamonds, figure: F10};
pub const JACK_DIAMONDS: Card = Card { suit: Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: Card = Card { suit: Diamonds, figure: Queen};
pub const KING_DIAMONDS: Card = Card { suit: Diamonds, figure: King};
pub const ACE_DIAMONDS: Card = Card { suit: Diamonds, figure: Ace};

pub const TWO_HEARTS: Card = Card { suit: Hearts, figure: F2};
pub const THREE_HEARTS: Card = Card { suit: Hearts, figure: F3};
pub const FOUR_HEARTS: Card = Card { suit: Hearts, figure: F4};
pub const FIVE_HEARTS: Card = Card { suit: Hearts, figure: F5};
pub const SIX_HEARTS: Card = Card { suit: Hearts, figure: F6};
pub const SEVEN_HEARTS: Card = Card { suit: Hearts, figure: F7};
pub const EIGHT_HEARTS: Card = Card { suit: Hearts, figure: F8};
pub const NINE_HEARTS: Card = Card { suit: Hearts, figure: F9};
pub const TEN_HEARTS: Card = Card { suit: Hearts, figure: F10};
pub const JACK_HEARTS: Card = Card { suit: Hearts, figure: Jack};
pub const QUEEN_HEARTS: Card = Card { suit: Hearts, figure: Queen};
pub const KING_HEARTS: Card = Card { suit: Hearts, figure: King};
pub const ACE_HEARTS: Card = Card { suit: Hearts, figure: Ace};

pub const TWO_SPADES: Card = Card { suit: Spades, figure: F2};
pub const THREE_SPADES: Card = Card { suit: Spades, figure: F3};
pub const FOUR_SPADES: Card = Card { suit: Spades, figure: F4};
pub const FIVE_SPADES: Card = Card { suit: Spades, figure: F5};
pub const SIX_SPADES: Card = Card { suit: Spades, figure: F6};
pub const SEVEN_SPADES: Card = Card { suit: Spades, figure: F7};
pub const EIGHT_SPADES: Card = Card { suit: Spades, figure: F8};
pub const NINE_SPADES: Card = Card { suit: Spades, figure: F9};
pub const TEN_SPADES: Card = Card { suit: Spades, figure: F10};
pub const JACK_SPADES: Card = Card { suit: Spades, figure: Jack};
pub const QUEEN_SPADES: Card = Card { suit: Spades, figure: Queen};
pub const KING_SPADES: Card = Card { suit: Spades, figure: King};
pub const ACE_SPADES: Card = Card { suit: Spades, figure: Ace};

/*
pub const TWO_CLUBS: Card = Card{mask: 0x0001};
pub const THREE_CLUBS: Card = Card{mask: 0x0002};
pub const FOUR_CLUBS: Card = Card{mask: 0x0004};
pub const FIVE_CLUBS: Card = Card{mask: 0x0008};
pub const SIX_CLUBS: Card = Card{mask: 0x0010};
pub const SEVEN_CLUBS: Card = Card{mask: 0x0020};
pub const EIGHT_CLUBS: Card = Card{mask: 0x0040};
pub const NINE_CLUBS: Card = Card{mask: 0x0080};
pub const TEN_CLUBS: Card = Card{mask: 0x0100};
pub const JACK_CLUBS: Card = Card{mask: 0x0200};
pub const QUEEN_CLUBS: Card = Card{mask: 0x0400};
pub const KING_CLUBS: Card = Card{mask: 0x0800};
pub const ACE_CLUBS: Card = Card{mask: 0x1000};

pub const TWO_DIAMONDS: Card =      Card{mask:  0x00002000};
pub const THREE_DIAMONDS: Card =    Card{mask:  0x00004000};
pub const FOUR_DIAMONDS: Card =     Card{mask:  0x00008000};
pub const FIVE_DIAMONDS: Card =     Card{mask:  0x00010000};
pub const SIX_DIAMONDS: Card =      Card{mask:  0x00020000};
pub const SEVEN_DIAMONDS: Card =    Card{mask:  0x00040000};
pub const EIGHT_DIAMONDS: Card =    Card{mask:  0x00080000};
pub const NINE_DIAMONDS: Card =     Card{mask:  0x00100000};
pub const TEN_DIAMONDS: Card =      Card{mask:  0x00200000};
pub const JACK_DIAMONDS: Card =     Card{mask:  0x00400000};
pub const QUEEN_DIAMONDS: Card =    Card{mask:  0x00800000};
pub const KING_DIAMONDS: Card =     Card{mask:  0x01000000};
pub const ACE_DIAMONDS: Card =      Card{mask:  0x02000000};

pub const TWO_HEARTS: Card =        Card{mask:  0x04000000};
pub const THREE_HEARTS: Card =      Card{mask:  0x08000000};
pub const FOUR_HEARTS: Card =       Card{mask:  0x10000000};
pub const FIVE_HEARTS: Card =       Card{mask:  0x20000000};
pub const SIX_HEARTS: Card =        Card{mask:  0x40000000};
pub const SEVEN_HEARTS: Card =      Card{mask:  0x80000000};
pub const EIGHT_HEARTS: Card =      Card{mask:  0x000100000000};
pub const NINE_HEARTS: Card =       Card{mask:  0x000200000000};
pub const TEN_HEARTS: Card =        Card{mask:  0x000400000000};
pub const JACK_HEARTS: Card =       Card{mask:  0x000800000000};
pub const QUEEN_HEARTS: Card =      Card{mask:  0x001000000000};
pub const KING_HEARTS: Card =       Card{mask:  0x002000000000};
pub const ACE_HEARTS: Card =        Card{mask:  0x004000000000};

pub const TWO_SPADES: Card =        Card{mask:  0x008000000000};
pub const THREE_SPADES: Card =      Card{mask:  0x010000000000};
pub const FOUR_SPADES: Card =       Card{mask:  0x020000000000};
pub const FIVE_SPADES: Card =       Card{mask:  0x040000000000};
pub const SIX_SPADES: Card =        Card{mask:  0x080000000000};
pub const SEVEN_SPADES: Card =      Card{mask:  0x100000000000};
pub const EIGHT_SPADES: Card =      Card{mask:  0x200000000000};
pub const NINE_SPADES: Card =       Card{mask:  0x400000000000};
pub const TEN_SPADES: Card =        Card{mask:  0x800000000000};
pub const JACK_SPADES: Card =       Card{mask:  0x0001000000000000};
pub const QUEEN_SPADES: Card =      Card{mask:  0x0002000000000000};
pub const KING_SPADES: Card =       Card{mask:  0x0004000000000000};
pub const ACE_SPADES: Card =        Card{mask:  0x0008000000000000};
*/
pub const STANDARD_DECK: [Card; Card::SYMBOL_SPACE] = [
    TWO_CLUBS,	    TWO_DIAMONDS,	TWO_HEARTS,	    TWO_SPADES,
    THREE_CLUBS,	THREE_DIAMONDS,	THREE_HEARTS,	THREE_SPADES,
    FOUR_CLUBS,	    FOUR_DIAMONDS,	FOUR_HEARTS,	FOUR_SPADES,
    FIVE_CLUBS,	    FIVE_DIAMONDS,	FIVE_HEARTS,	FIVE_SPADES,
    SIX_CLUBS,	    SIX_DIAMONDS,	SIX_HEARTS,	    SIX_SPADES,
    SEVEN_CLUBS,	SEVEN_DIAMONDS,	SEVEN_HEARTS,	SEVEN_SPADES,
    EIGHT_CLUBS,	EIGHT_DIAMONDS,	EIGHT_HEARTS,	EIGHT_SPADES,
    NINE_CLUBS,	    NINE_DIAMONDS,	NINE_HEARTS,	NINE_SPADES,
    TEN_CLUBS,	    TEN_DIAMONDS,	TEN_HEARTS,	    TEN_SPADES,
    JACK_CLUBS,	    JACK_DIAMONDS,	JACK_HEARTS,	JACK_SPADES,
    QUEEN_CLUBS,	QUEEN_DIAMONDS,	QUEEN_HEARTS,	QUEEN_SPADES,
    KING_CLUBS,	    KING_DIAMONDS,	KING_HEARTS,	KING_SPADES,
    ACE_CLUBS,	    ACE_DIAMONDS,	ACE_HEARTS,	    ACE_SPADES
];
/* 
pub const MASK_CLUBS: u64 =                 0x7ffc;
pub const MASK_DIAMONDS: u64 =          0x7ffc0000;
pub const MASK_HEARTS: u64 =        0x7ffc00000000;
pub const MASK_SPADES: u64 =    0x7ffc000000000000;
*/
pub const MASK_CLUBS: u64 =                 0x1fff;
pub const MASK_DIAMONDS: u64 =          0x03ffe000;
pub const MASK_HEARTS: u64 =        0x007ffc000000;
pub const MASK_SPADES: u64 =    0x000fff8000000000;


#[cfg(test)]
mod tests{
    use crate::symbol::CardSymbol;
    use crate::cards::standard::{ *};

    #[test]
    fn display(){
        assert_eq!(format!("{:#}", KING_HEARTS), format!("𝑲♥"));
        assert_eq!(format!("{}", KING_HEARTS), format!("King of Hearts"));
    }

    #[test]
    fn test_card_element_for_card_std(){
        assert_eq!(Card::from_position(0).unwrap(), TWO_CLUBS);
        assert_eq!(Card::from_position(1).unwrap(), THREE_CLUBS);
        assert_eq!(Card::from_position(4).unwrap(), SIX_CLUBS);
        assert_eq!(Card::from_position(13).unwrap(), TWO_DIAMONDS);
        assert_eq!(Card::from_position(51).unwrap(), ACE_SPADES);
    }

    #[test]
    #[cfg(feature = "speedy")]
    fn test_speedy_card(){
        use speedy::{Readable, Writable};
        let serialized_jack_hearts = JACK_HEARTS.write_to_vec().unwrap();
        let deserialized_jack_hearts = Card::read_from_buffer(&serialized_jack_hearts).unwrap();
        let serialized_nine_clubs = NINE_CLUBS.write_to_vec().unwrap();
        let deserialized_nine_clubs = Card::read_from_buffer(&serialized_nine_clubs).unwrap();
        assert_eq!(deserialized_jack_hearts, JACK_HEARTS);
        assert_eq!(deserialized_nine_clubs, NINE_CLUBS);
    }
}

/*
///
/// ```
/// use karty::cards::{order_sf, STANDARD_DECK};
/// use karty::figures::FigureStd;
/// use karty::suits::ahcd;
/// use karty::cards::*;
/// let mut deck = Vec::from(STANDARD_DECK);
/// deck.sort_by(order_sf::<ahcd, FigureStd::cmp>);
/// assert_eq!(deck[51], ACE_SPADES);
/// assert_eq!(deck[50], ACE_HEARTS);
/// ```
pub fn order_sf<SuitOrder, FigureOrder>(l: &CardStd, r: &CardStd) -> Ordering
where SuitOrder: Fn(&SuitStd, &SuitStd) -> Ordering, FigureOrder: Fn(&FigureStd, &FigureStd){

    match SuitOrder(l.suit, r.suit){
        Ordering::Equal => FigureOrder(l.figure, r.figure),
        ordering => ordering
    }
}*/


