//! Module containing Error type used with cards.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::fmt::{Display, Formatter};

/// Type used for indicating errors related with operations defined in crate
#[derive(Debug)]
pub enum CardError{
    /// Instance indicating that wrong position for [`Suit`][crate::suits::Suit] was used, associated value is violating value.
    WrongSuitPosition(usize),
    /// Instance indicating that wrong position for [`Figure`][crate::figures::Figure] was used, associated value is violating value.
    WrongFigurePosition(usize),
    WrongPosition(usize)
}
impl Display for CardError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}