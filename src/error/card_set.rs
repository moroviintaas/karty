use crate::cards::{Card};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::symbol::CardSymbol;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum CardSetErrorGen<Crd: CardSymbol>{
    CardNotInHand(Crd),
    EmptyHand,
    HandFull,
    CardDuplicated(Crd),
    HandNotInitialised,
    DifferentLengths(usize, usize),
    ParseError,

}

pub type CardSetError = CardSetErrorGen<Card>;

