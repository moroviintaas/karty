/*use std::marker::PhantomData;
use array2d::Array2D;
use crate::cards::Card;
use crate::figures::Figure;
use crate::suits::Suit;


#[derive(Debug)]
pub struct CardRegisterArray<F: Figure, S: Suit>{
    array: Array2D<bool>,
    phantom: PhantomData<Card<F, S>>
}


impl<F: Figure, S: Suit> Default for CardRegisterArray<F, S> {
    fn default() -> Self {
        Self{phantom: PhantomData, array: Array2D::filled_with(false,  F::NUMBER_OF_FIGURES,S::NUMBER_OF_SUITS )}
    }
}

 */

/*
impl <F: Figure, S: Suit> CardRegister<F, S> for CardRegisterArray<F, S>{
    fn mark_used(&mut self, card: &Card<F, S>) {

    }

    fn is_card_used(&self, card: &Card<F, S>) -> bool {
        todo!()
    }
}*/