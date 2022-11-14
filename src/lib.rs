//! Crate providing playing cards, which can be use in standard games like contract bridge or poker.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
pub mod cards;
pub mod figures;
pub mod suits;

#[cfg(feature = "register")]
pub mod register;
pub mod symbol;

#[cfg(feature = "random")]
pub mod random;
pub mod hand;
pub mod error;

#[cfg(feature = "speedy")]
pub use speedy;




