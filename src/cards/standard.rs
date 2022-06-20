use std::fmt::{Display, Formatter};
use num_integer::div_rem;
use crate::symbol::CardSymbol;
use crate::cards::Card;
use crate::error::CardError;
use crate::figures::{Ace, F10, F2, F3, F4, F5, F6, F7, F8, F9, FigureStd, Jack, King, Queen};
use crate::suits::SuitStd::*;
use crate::suits::SuitStd;

impl Card<FigureStd, SuitStd>{
    pub fn mask(&self) -> u64{

        self.figure().mask() << (self.suit().position() * 16)
    }
}



pub type CardStd =  Card<FigureStd, SuitStd>;

impl CardSymbol for CardStd{
    const SYMBOL_SPACE: usize = FigureStd::SYMBOL_SPACE * SuitStd::SYMBOL_SPACE;

    fn position(&self) -> usize {
        (self.figure.position() * SuitStd::SYMBOL_SPACE) + self.suit.position()
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        let (figure, suit) = div_rem(position, SuitStd::SYMBOL_SPACE);
        Ok(Self{figure: FigureStd::from_position(figure)?, suit: SuitStd::from_position(suit)?})
    }
}

impl Display for CardStd{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "{:#}{:#}", self.figure(), self.suit()),
            false => write!(f, "{} of {}", self.figure(), self.suit())
        }

    }
}

//pub const TWO_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F2)};
pub const TWO_CLUBS: CardStd = CardStd{ suit: Clubs, figure: F2};
pub const THREE_CLUBS: CardStd = Card { suit: Clubs, figure: F3};
pub const FOUR_CLUBS: CardStd = Card { suit: Clubs, figure: F4};
pub const FIVE_CLUBS: CardStd = Card { suit: Clubs, figure: F5};
pub const SIX_CLUBS: CardStd = Card { suit: Clubs, figure: F6};
pub const SEVEN_CLUBS: CardStd = Card { suit: Clubs, figure: F7};
pub const EIGHT_CLUBS: CardStd = Card { suit: Clubs, figure: F8};
pub const NINE_CLUBS: CardStd = Card { suit: Clubs, figure: F9};
pub const TEN_CLUBS: CardStd = Card { suit: Clubs, figure: F10};
pub const JACK_CLUBS: CardStd = Card { suit: Clubs, figure: Jack};
pub const QUEEN_CLUBS: CardStd = Card { suit: Clubs, figure: Queen};
pub const KING_CLUBS: CardStd = Card { suit: Clubs, figure: King};
pub const ACE_CLUBS: CardStd = Card { suit: Clubs, figure: Ace};

pub const TWO_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F2};
pub const THREE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F3};
pub const FOUR_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F4};
pub const FIVE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F5};
pub const SIX_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F6};
pub const SEVEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F7};
pub const EIGHT_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F8};
pub const NINE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F9};
pub const TEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: F10};
pub const JACK_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Queen};
pub const KING_DIAMONDS: CardStd = Card { suit: Diamonds, figure: King};
pub const ACE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Ace};

pub const TWO_HEARTS: CardStd = Card { suit: Hearts, figure: F2};
pub const THREE_HEARTS: CardStd = Card { suit: Hearts, figure: F3};
pub const FOUR_HEARTS: CardStd = Card { suit: Hearts, figure: F4};
pub const FIVE_HEARTS: CardStd = Card { suit: Hearts, figure: F5};
pub const SIX_HEARTS: CardStd = Card { suit: Hearts, figure: F6};
pub const SEVEN_HEARTS: CardStd = Card { suit: Hearts, figure: F7};
pub const EIGHT_HEARTS: CardStd = Card { suit: Hearts, figure: F8};
pub const NINE_HEARTS: CardStd = Card { suit: Hearts, figure: F9};
pub const TEN_HEARTS: CardStd = Card { suit: Hearts, figure: F10};
pub const JACK_HEARTS: CardStd = Card { suit: Hearts, figure: Jack};
pub const QUEEN_HEARTS: CardStd = Card { suit: Hearts, figure: Queen};
pub const KING_HEARTS: CardStd = Card { suit: Hearts, figure: King};
pub const ACE_HEARTS: CardStd = Card { suit: Hearts, figure: Ace};

pub const TWO_SPADES: CardStd = Card { suit: Spades, figure: F2};
pub const THREE_SPADES: CardStd = Card { suit: Spades, figure: F3};
pub const FOUR_SPADES: CardStd = Card { suit: Spades, figure: F4};
pub const FIVE_SPADES: CardStd = Card { suit: Spades, figure: F5};
pub const SIX_SPADES: CardStd = Card { suit: Spades, figure: F6};
pub const SEVEN_SPADES: CardStd = Card { suit: Spades, figure: F7};
pub const EIGHT_SPADES: CardStd = Card { suit: Spades, figure: F8};
pub const NINE_SPADES: CardStd = Card { suit: Spades, figure: F9};
pub const TEN_SPADES: CardStd = Card { suit: Spades, figure: F10};
pub const JACK_SPADES: CardStd = Card { suit: Spades, figure: Jack};
pub const QUEEN_SPADES: CardStd = Card { suit: Spades, figure: Queen};
pub const KING_SPADES: CardStd = Card { suit: Spades, figure: King};
pub const ACE_SPADES: CardStd = Card { suit: Spades, figure: Ace};

#[cfg(test)]
mod tests{
    use crate::symbol::CardSymbol;
    use crate::cards::standard::{ ACE_SPADES, CardStd, KING_HEARTS,  THREE_CLUBS, TWO_CLUBS, TWO_DIAMONDS};

    #[test]
    fn display(){
        assert_eq!(format!("{:#}", KING_HEARTS), format!("𝑲♥"));
        assert_eq!(format!("{}", KING_HEARTS), format!("King of Hearts"));
    }

    #[test]
    fn test_card_element_for_card_std(){
        assert_eq!(CardStd::from_position(0).unwrap(), TWO_CLUBS);
        assert_eq!(CardStd::from_position(1).unwrap(), TWO_DIAMONDS);
        assert_eq!(CardStd::from_position(4).unwrap(), THREE_CLUBS);
        assert_eq!(CardStd::from_position(51).unwrap(), ACE_SPADES);
    }
}