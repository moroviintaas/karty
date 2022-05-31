use crate::cards::Card;
use crate::figures::standard::{Ace, F10, F2, F3, F4, F5, F6, F7, F8, F9, FigureStd, Jack, King, Numbered, Queen};
use crate::suits::standard::SuitStd::*;
use crate::suits::{Suit, SuitStd};

impl Card<FigureStd, SuitStd>{
    pub fn mask(&self) -> u64{

        self.figure.mask() << (self.suit.order_number() * 16)
    }
}

pub type CardStd =  Card<FigureStd, SuitStd>;


//pub const TWO_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F2)};
pub const TWO_CLUBS: CardStd = CardStd{ suit: Clubs, figure: Numbered(F2)};
pub const THREE_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F3)};
pub const FOUR_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F4)};
pub const FIVE_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F5)};
pub const SIX_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F6)};
pub const SEVEN_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F7)};
pub const EIGHT_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F8)};
pub const NINE_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F9)};
pub const TEN_CLUBS: CardStd = Card { suit: Clubs, figure: Numbered(F10)};
pub const JACK_CLUBS: CardStd = Card { suit: Clubs, figure: Jack};
pub const QUEEN_CLUBS: CardStd = Card { suit: Clubs, figure: Queen};
pub const KING_CLUBS: CardStd = Card { suit: Clubs, figure: King};
pub const ACE_CLUBS: CardStd = Card { suit: Clubs, figure: Ace};

pub const TWO_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F2)};
pub const THREE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F3)};
pub const FOUR_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F4)};
pub const FIVE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F5)};
pub const SIX_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F6)};
pub const SEVEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F7)};
pub const EIGHT_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F8)};
pub const NINE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F9)};
pub const TEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Numbered(F10)};
pub const JACK_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Queen};
pub const KING_DIAMONDS: CardStd = Card { suit: Diamonds, figure: King};
pub const ACE_DIAMONDS: CardStd = Card { suit: Diamonds, figure: Ace};

pub const TWO_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F2)};
pub const THREE_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F3)};
pub const FOUR_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F4)};
pub const FIVE_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F5)};
pub const SIX_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F6)};
pub const SEVEN_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F7)};
pub const EIGHT_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F8)};
pub const NINE_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F9)};
pub const TEN_HEARTS: CardStd = Card { suit: Hearts, figure: Numbered(F10)};
pub const JACK_HEARTS: CardStd = Card { suit: Hearts, figure: Jack};
pub const QUEEN_HEARTS: CardStd = Card { suit: Hearts, figure: Queen};
pub const KING_HEARTS: CardStd = Card { suit: Hearts, figure: King};
pub const ACE_HEARTS: CardStd = Card { suit: Hearts, figure: Ace};

pub const TWO_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F2)};
pub const THREE_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F3)};
pub const FOUR_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F4)};
pub const FIVE_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F5)};
pub const SIX_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F6)};
pub const SEVEN_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F7)};
pub const EIGHT_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F8)};
pub const NINE_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F9)};
pub const TEN_SPADES: CardStd = Card { suit: Spades, figure: Numbered(F10)};
pub const JACK_SPADES: CardStd = Card { suit: Spades, figure: Jack};
pub const QUEEN_SPADES: CardStd = Card { suit: Spades, figure: Queen};
pub const KING_SPADES: CardStd = Card { suit: Spades, figure: King};
pub const ACE_SPADES: CardStd = Card { suit: Spades, figure: Ace};