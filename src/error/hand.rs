use crate::cards::{Card};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::symbol::CardSymbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum HandErrorGen<Crd: CardSymbol>{
    CardNotInHand(Crd),
    EmptyHand,
    HandFull,
    CardDuplicated(Crd),
    HandNotInitialised,
    DifferentLengths(usize, usize)
}

pub type HandError = HandErrorGen<Card>;

