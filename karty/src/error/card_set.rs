use crate::cards::{Card};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::symbol::CardSymbol;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum CardSetErrorGen<Crd: CardSymbol>{
    #[error("Card {0:?} was not in set")]
    CardNotInSet(Crd),
    #[error("Hand is empty")]
    EmptyHand,
    #[error("Hand is full")]
    HandFull,
    #[error("Card duplicate found")]
    CardDuplicated(Crd),
    #[error("Hand is not initialised yes")]
    HandNotInitialised,
    #[error("Difference of lengths found: {0} and {1}")]
    DifferentLengths(usize, usize),
    #[error("Parse error")]
    ParseError,
    #[error("Card sets are not equal (but they were expected to be). Expected: {expected:?}, found: {found:?}")]
    ExpectedEqualCardSets{
        expected: Vec<Crd>,
        found: Vec<Crd>
    }

}

pub type CardSetError = CardSetErrorGen<Card>;

