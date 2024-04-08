//! Module containing Error type used with cards.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!

/// Type used for indicating errors related with operations defined in crate
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CardError{
    /// Instance indicating that wrong position for [`Suit`][crate::suits::Suit] was used, associated value is violating value.
    #[error("Wrong suit index was used, associated value {0} is violating rules")]
    WrongSuitPosition(usize),
    /// Instance indicating that wrong position for [`Figure`][crate::figures::Figure] was used, associated value is violating value.
    #[error("Wrong figure index was used, associated value {0} is violating rules")]
    WrongFigurePosition(usize),
    #[error("Wrong index  was used, associated value {0} is violating rules")]
    WrongPosition(usize),
    #[error("Wrong mask format")]
    WrongMaskFormat,
    #[error("Mask space violated")]
    MaskSpaceViolated,
    #[error("Parse error")]
    ParseError
}