use crate::error::CardError;

pub trait CardDimension: Sized + Eq{
    const DIMENSION_SIZE: usize;
    fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;
}