use std::fmt::Debug;
use std::hash::Hash;
use crate::figures::Figure;
use crate::suits::Suit;
use crate::symbol::CardSymbol;
#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};

pub trait Card2S<F: CardSymbol, S:CardSymbol>{
    const CARD_SPACE: usize = F::SYMBOL_SPACE * S::SYMBOL_SPACE;
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct Card<F: CardSymbol + Debug + Eq + PartialEq + Clone + Hash,
    S: CardSymbol + Debug + Eq + PartialEq + Clone + Hash> {
    pub(crate) suit: S,
    pub(crate) figure: F
}

//pub type CardL<F, S> = Card<F, S>;

impl<F:Figure, S:Suit> Card2S<F,S> for Card<F,S>{}

impl<F: Figure + Copy, S: Suit + Copy> Copy for Card<F, S>{}


impl<F:Figure, S: Suit > Card<F, S> {
    pub fn new(figure: F, suit: S) -> Self{
        Self{suit, figure}
    }

    pub fn suit(&self) -> &S {
        &self.suit
    }
    pub fn figure(&self) -> &F {
        &self.figure
    }

}






