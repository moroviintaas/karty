use std::fmt::Debug;
use crate::cards::Card;
use crate::figures::Figure;
use crate::suits::Suit;


pub trait CardRegister<F:Figure, S: Suit>: Debug + Default{
    fn register(&mut self, card: &Card<F, S>);
    fn unregister(&mut self, card: &Card<F, S>);
    fn is_registered(&self, card: &Card<F, S>) -> bool;

}