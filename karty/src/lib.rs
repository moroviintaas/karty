//! Crate providing playing cards, which can be use in standard games like contract bridge or poker.
//!
//!
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)

/// Module dedicated to whole distinguished cards
pub mod cards;
/// Module for figure symbols (Ace, King, etc...)
pub mod figures;
/// Module for suits (Spades, Hearts, Diamonds, Clubs)
pub mod suits;

#[cfg(feature = "register")]
/// Module for registering usage of cards
pub mod register;
/// Module for basic distinguished symbols (Figures, Suits, ...)
pub mod symbol;

/// Random card sampling
#[cfg(feature = "random")]
pub mod random;
/// Card aggregation (set of cards)
pub mod set;
/// Crate errors
pub mod error;


#[cfg(feature = "speedy")]
pub use speedy;




