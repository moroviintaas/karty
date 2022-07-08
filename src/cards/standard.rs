use std::fmt::{Display, Formatter};
use num_integer::div_rem;
use crate::symbol::CardSymbol;
use crate::cards::Card2S;
use crate::error::CardError;
use crate::figures::{Ace, F10, F2, F3, F4, F5, F6, F7, F8, F9, FigureStd, Jack, King, Queen};
use crate::suits::SuitStd::*;
use crate::suits::SuitStd;


impl Card2S<FigureStd, SuitStd>{
    pub fn mask(&self) -> u64{

        self.figure().mask() << (self.suit().position() * 16)
    }
}



pub type CardStd =  Card2S<FigureStd, SuitStd>;

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
pub const THREE_CLUBS: CardStd = CardStd { suit: Clubs, figure: F3};
pub const FOUR_CLUBS: CardStd = CardStd { suit: Clubs, figure: F4};
pub const FIVE_CLUBS: CardStd = CardStd { suit: Clubs, figure: F5};
pub const SIX_CLUBS: CardStd = CardStd { suit: Clubs, figure: F6};
pub const SEVEN_CLUBS: CardStd = CardStd { suit: Clubs, figure: F7};
pub const EIGHT_CLUBS: CardStd = CardStd { suit: Clubs, figure: F8};
pub const NINE_CLUBS: CardStd = CardStd { suit: Clubs, figure: F9};
pub const TEN_CLUBS: CardStd = CardStd { suit: Clubs, figure: F10};
pub const JACK_CLUBS: CardStd = CardStd { suit: Clubs, figure: Jack};
pub const QUEEN_CLUBS: CardStd = CardStd { suit: Clubs, figure: Queen};
pub const KING_CLUBS: CardStd = CardStd { suit: Clubs, figure: King};
pub const ACE_CLUBS: CardStd = CardStd { suit: Clubs, figure: Ace};

pub const TWO_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F2};
pub const THREE_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F3};
pub const FOUR_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F4};
pub const FIVE_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F5};
pub const SIX_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F6};
pub const SEVEN_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F7};
pub const EIGHT_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F8};
pub const NINE_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F9};
pub const TEN_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: F10};
pub const JACK_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: Queen};
pub const KING_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: King};
pub const ACE_DIAMONDS: CardStd = CardStd { suit: Diamonds, figure: Ace};

pub const TWO_HEARTS: CardStd = CardStd { suit: Hearts, figure: F2};
pub const THREE_HEARTS: CardStd = CardStd { suit: Hearts, figure: F3};
pub const FOUR_HEARTS: CardStd = CardStd { suit: Hearts, figure: F4};
pub const FIVE_HEARTS: CardStd = CardStd { suit: Hearts, figure: F5};
pub const SIX_HEARTS: CardStd = CardStd { suit: Hearts, figure: F6};
pub const SEVEN_HEARTS: CardStd = CardStd { suit: Hearts, figure: F7};
pub const EIGHT_HEARTS: CardStd = CardStd { suit: Hearts, figure: F8};
pub const NINE_HEARTS: CardStd = CardStd { suit: Hearts, figure: F9};
pub const TEN_HEARTS: CardStd = CardStd { suit: Hearts, figure: F10};
pub const JACK_HEARTS: CardStd = CardStd { suit: Hearts, figure: Jack};
pub const QUEEN_HEARTS: CardStd = CardStd { suit: Hearts, figure: Queen};
pub const KING_HEARTS: CardStd = CardStd { suit: Hearts, figure: King};
pub const ACE_HEARTS: CardStd = CardStd { suit: Hearts, figure: Ace};

pub const TWO_SPADES: CardStd = CardStd { suit: Spades, figure: F2};
pub const THREE_SPADES: CardStd = CardStd { suit: Spades, figure: F3};
pub const FOUR_SPADES: CardStd = CardStd { suit: Spades, figure: F4};
pub const FIVE_SPADES: CardStd = CardStd { suit: Spades, figure: F5};
pub const SIX_SPADES: CardStd = CardStd { suit: Spades, figure: F6};
pub const SEVEN_SPADES: CardStd = CardStd { suit: Spades, figure: F7};
pub const EIGHT_SPADES: CardStd = CardStd { suit: Spades, figure: F8};
pub const NINE_SPADES: CardStd = CardStd { suit: Spades, figure: F9};
pub const TEN_SPADES: CardStd = CardStd { suit: Spades, figure: F10};
pub const JACK_SPADES: CardStd = CardStd { suit: Spades, figure: Jack};
pub const QUEEN_SPADES: CardStd = CardStd { suit: Spades, figure: Queen};
pub const KING_SPADES: CardStd = CardStd { suit: Spades, figure: King};
pub const ACE_SPADES: CardStd = CardStd { suit: Spades, figure: Ace};

#[cfg(test)]
mod tests{
    use crate::symbol::CardSymbol;
    use crate::cards::standard::{ ACE_SPADES, CardStd, KING_HEARTS,  JACK_HEARTS, NINE_CLUBS, THREE_CLUBS, TWO_CLUBS, TWO_DIAMONDS};

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

    #[test]
    #[cfg(feature = "speedy")]
    fn test_speedy_card(){
        use speedy::{Readable, Writable};
        let serialized_jack_hearts = JACK_HEARTS.write_to_vec().unwrap();
        let deserialized_jack_hearts = CardStd::read_from_buffer(&serialized_jack_hearts).unwrap();
        let serialized_nine_clubs = NINE_CLUBS.write_to_vec().unwrap();
        let deserialized_nine_clubs = CardStd::read_from_buffer(&serialized_nine_clubs).unwrap();
        assert_eq!(deserialized_jack_hearts, JACK_HEARTS);
        assert_eq!(deserialized_nine_clubs, NINE_CLUBS);
    }
}