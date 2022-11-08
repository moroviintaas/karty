

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum HandError{
    CardNotInHand,
    EmptyHand,
    HandFull,
    CardDuplicated,
    HandNotInitialised,
}

