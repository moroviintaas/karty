use std::fmt::Debug;
use crate::cards::Card;
use crate::figures::Figure;
use crate::suits::Suit;


pub trait CardRegister<F:Figure, S: Suit>: Debug + Default{
    fn mark_used(&mut self, card: &Card<F, S>);
    fn is_card_used(&self, card: &Card<F, S>) -> bool;

}