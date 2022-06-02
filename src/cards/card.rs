use crate::figures::Figure;
use crate::suits::Suit;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Card<F: Figure, S: Suit> {
    suit: S,
    figure: F
}

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



#[path = "standard.rs"]
pub mod standard;
