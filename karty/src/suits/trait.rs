//! Module containing basic SuitTrait
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use std::fmt::Debug;
use std::hash::Hash;
use crate::symbol::CardSymbol;

/// Trait that is combination of [`CardSymbol`][crate::symbol::CardSymbol], [`Debug`][std::fmt::Debug]
/// [`Ord`][std::cmp::Ord], [`Clone`][std::clone::Clone] and [`Hash`][std::hash::Hash].
/// At the moment trait is only semantic sugar. It remains after early crate development,
/// now it is advised to use more generic super trait [`CardSymbol`][crate::symbol::CardSymbol].
/// Trait may be marked deprecated in future and marked to remove.
/// # Example implementation of trait:
/// ```
/// use karty::symbol::CardSymbol;
/// use karty::error::CardError;
/// use std::cmp::Ordering;
/// use karty::suits::SuitTrait;
/// // if enabled feature "random" you can derive RandomSymbol for structs implementing CardSymbol
/// #[derive(Eq, PartialEq, Clone, Hash, Debug)]
/// pub enum MySuit {
///     Spades,
///     Hearts,
///     Diamonds,
/// }
///
/// impl CardSymbol for MySuit{
///     //as space of suits does not include clubs it is now of size=3
///     const SYMBOL_SPACE: usize = 3;
///
///     fn usize_index(&self) -> usize {
///         match self{
///             MySuit::Spades => 2,
///             MySuit::Hearts => 1,
///             MySuit::Diamonds => 0,
///         }
///     }
///
///     fn from_usize_index(position: usize) -> Result<Self, CardError> {
///         match position{
///             2 => Ok(MySuit::Spades),
///             1 => Ok(MySuit::Hearts),
///             0 => Ok(MySuit::Diamonds),
///             s => Err(CardError::WrongSuitPosition(s))
///         }
///     }
/// }
/// //implement your own order
/// impl PartialOrd<Self> for MySuit {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         // order does not have to follow `position`
///         Some(self.cmp(&other))
///     }
/// }
///
/// impl Ord for MySuit {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.usize_index().cmp(&other.usize_index())
///     }
/// }
/// impl SuitTrait for MySuit{
///     const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;
///
/// }
/// ```
pub trait SuitTrait: Debug + Ord + Clone + Hash + CardSymbol {
    /// The value is name association to [`SYMBOL_SPACE`][crate::symbol::CardSymbol::SYMBOL_SPACE]
    const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;


}