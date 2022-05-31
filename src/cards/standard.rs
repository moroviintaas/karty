use crate::cards::Card;
use crate::figures::standard::{Ace, F10, F2, F3, F4, F5, F6, F7, F8, F9, FigureStd, Jack, King, Numbered, Queen};
use crate::suits::standard::SuitStd;
use crate::suits::Suit;

impl Card<FigureStd, SuitStd>{
    pub fn mask(&self) -> u64{

        self.figure.mask() << (self.suit.order_number() * 16)
    }
}

pub const TWO_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F2)};
pub const THREE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F3)};
pub const FOUR_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F4)};
pub const FIVE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F5)};
pub const SIX_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F6)};
pub const SEVEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F7)};
pub const EIGHT_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F8)};
pub const NINE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F9)};
pub const TEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(F10)};
pub const JACK_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Jack};
pub const QUEEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Queen};
pub const KING_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: King};
pub const ACE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Ace};

pub const TWO_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F2)};
pub const THREE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F3)};
pub const FOUR_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F4)};
pub const FIVE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F5)};
pub const SIX_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F6)};
pub const SEVEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F7)};
pub const EIGHT_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F8)};
pub const NINE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F9)};
pub const TEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(F10)};
pub const JACK_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Queen};
pub const KING_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: King};
pub const ACE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Ace};

pub const TWO_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F2)};
pub const THREE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F3)};
pub const FOUR_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F4)};
pub const FIVE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F5)};
pub const SIX_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F6)};
pub const SEVEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F7)};
pub const EIGHT_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F8)};
pub const NINE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F9)};
pub const TEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(F10)};
pub const JACK_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Jack};
pub const QUEEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Queen};
pub const KING_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: King};
pub const ACE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Ace};

pub const TWO_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F2)};
pub const THREE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F3)};
pub const FOUR_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F4)};
pub const FIVE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F5)};
pub const SIX_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F6)};
pub const SEVEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F7)};
pub const EIGHT_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F8)};
pub const NINE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F9)};
pub const TEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(F10)};
pub const JACK_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Jack};
pub const QUEEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Queen};
pub const KING_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: King};
pub const ACE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Ace};