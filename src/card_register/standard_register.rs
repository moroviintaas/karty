
use crate::cards::Card;
use crate::figures::standard::FigureStd;
use crate::card_register::register::CardRegister;
use crate::suits::standard::SuitStd;

#[derive(Debug, Default)]
pub struct CardUsageRegStd{
    memory: u64
}

impl CardRegister<FigureStd, SuitStd> for CardUsageRegStd{

    fn register(&mut self, card: &Card<FigureStd, SuitStd>) {
        self.memory |= card.mask();
    }

    fn unregister(&mut self, card: &Card<FigureStd, SuitStd>) {
        self.memory &= !card.mask()
    }

    fn is_registered(&self, card: &Card<FigureStd, SuitStd>) -> bool {
        !matches!(self.memory & card.mask(), 0)
    }
}