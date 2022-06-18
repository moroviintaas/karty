
use crate::card_register::register::{Register};
use crate::cards::standard::CardStd;

#[derive(Debug, Default)]
pub struct CardUsageRegStd{
    memory: u64
}

impl Register<CardStd> for CardUsageRegStd{

    fn register(&mut self, card: CardStd) {
        self.memory |= card.mask();
    }

    fn unregister(&mut self, card: &CardStd) {
        self.memory &= !card.mask()
    }

    fn is_registered(&self, card: &CardStd) -> bool {
        !matches!(self.memory & card.mask(), 0)
    }
}