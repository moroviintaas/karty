
use crate::register::register_trait::{Register};
use crate::cards::Card;

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardRegister {
    memory: u64
}

impl Register<Card> for CardRegister {

    fn register(&mut self, card: Card){
        self.memory |= card.mask();
    }

    fn unregister(&mut self, card: &Card) {

        self.memory &= !card.mask()
    }

    fn is_registered(&self, card: &Card) -> bool {
        !matches!(self.memory & card.mask(), 0)
    }
}

