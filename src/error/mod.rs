mod card;
pub use card::CardError;

mod card_set;
#[cfg(feature = "fuzzy")]
mod fuzzy_card_set;

pub use card_set::{CardSetErrorGen, CardSetError};
#[cfg(feature = "fuzzy")]
pub use fuzzy_card_set::*;