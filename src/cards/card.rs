use std::fmt::Debug;
use std::hash::Hash;
use crate::figures::Figure;
use crate::suits::Suit;
use crate::symbol::CardSymbol;


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Card2S<F: CardSymbol + Debug + Eq + PartialEq + Clone + Hash,
    S: CardSymbol + Debug + Eq + PartialEq + Clone + Hash> {
    pub(crate) suit: S,
    pub(crate) figure: F
}
pub type Card<F, S> = Card2S<F, S>;

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






