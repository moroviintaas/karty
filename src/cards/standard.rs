use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use num_integer::div_rem;
use crate::symbol::CardSymbol;
use crate::cards::{Card2SGen, CardComparatorGen};
use crate::error::CardError;
use crate::figures::{Ace, ComparatorF, F10, F2, F3, F4, F5, F6, F7, F8, F9, Figure, Jack, King, Queen};
use crate::suits::Suit::*;
use crate::suits::{ComparatorAHCD, ComparatorAHDC, Suit};

use super::Card2SymTrait;


impl Card2SGen<Figure, Suit>{
    /// ```
    /// use karty::cards::KING_HEARTS;
    /// assert_eq!(KING_HEARTS.mask(), 0x200000000000);
    /// ```
    /// ```
    /// use karty::cards::STANDARD_DECK;
    /// let mut bin_sum = 0u64;
    /// for c in &STANDARD_DECK{
    ///     assert_eq!(bin_sum & c.mask(), 0);
    ///     bin_sum |= c.mask();
    /// }
    /// assert_eq!(bin_sum, 0x7ffc7ffc7ffc7ffc)
    /// ```
    pub fn mask(&self) -> u64{

        self.figure().mask() << (self.suit().position() * 16)
    }
    /// ```
    /// use karty::cards::{Card, TWO_CLUBS, KING_SPADES};
    /// assert_eq!(Card::from_mask(0x04).unwrap(), TWO_CLUBS);
    /// assert_eq!(Card::from_mask(0x2000000000000000).unwrap(), KING_SPADES);
    /// ```
    pub fn from_mask(mask: u64) -> Option<Self>{
        if mask.count_ones() != 1{
            return None
        }
        let t0 = mask.trailing_zeros();
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
        

    

    }
}



pub type Card =  Card2SGen<Figure, Suit>;

impl CardSymbol for Card {
    const SYMBOL_SPACE: usize = Figure::SYMBOL_SPACE * Suit::SYMBOL_SPACE;

    fn position(&self) -> usize {
        (self.figure.position() * Suit::SYMBOL_SPACE) + self.suit.position()
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        let (figure, suit) = div_rem(position, Suit::SYMBOL_SPACE);
        Ok(Self{figure: Figure::from_position(figure)?, suit: Suit::from_position(suit)?})
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "{:#}{:#}", self.figure(), self.suit()),
            false => write!(f, "{} of {}", self.figure(), self.suit())
        }

    }
}

pub type ComparatorCardStd<CS> = CardComparatorGen<Figure, Suit, ComparatorF, CS>;

pub const CARD_COMPARATOR_BRIDGE: ComparatorCardStd<ComparatorAHDC> =
    CardComparatorGen {
        suit_comparator: ComparatorAHDC{},
        figure_comparator: ComparatorF{},
        _phantom: PhantomData{}
    };
pub const CARD_COMPARATOR_VISUAL: ComparatorCardStd<ComparatorAHCD> =
    CardComparatorGen {
        suit_comparator: ComparatorAHCD{},
        figure_comparator: ComparatorF{},
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
        assert_eq!(Card::from_position(1).unwrap(), TWO_DIAMONDS);
        assert_eq!(Card::from_position(4).unwrap(), THREE_CLUBS);
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


