mod hand_set;
mod stack_hand;
pub use stack_hand::*;
pub use hand_set::*;

use std::fmt::{Debug, Display};
use crate::error::HandError;
use crate::symbol::CardSymbol;

pub trait HandTrait: Debug + Clone + Eq + IntoIterator<Item=Self::CardType> + Display{
    type CardType : CardSymbol;
    fn add_card(&mut self, card: Self::CardType) -> Result<(), HandError>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), HandError>;
    fn new_empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool{
        self.len() == 0
    }
}