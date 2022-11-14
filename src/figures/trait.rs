//! Module containing basic FigureTrait
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
/// use karty::figures::FigureTrait;
/// use karty::suits::SuitTrait;
/// // if enabled feature "random" you can derive RandomSymbol for structs implementing CardSymbol
/// const MIN_MY_FIGURE:u8 = 1;
/// const MAX_MY_FIGURE:u8 = 4;
/// #[derive(Eq, PartialEq, Clone, Hash, Debug)]
/// pub struct MyFigureNumbered {
///     power: u8
/// }
/// impl MyFigureNumbered{
///     pub fn new(power: u8) -> Self{
///         match power{
///             ok @MIN_MY_FIGURE..=MAX_MY_FIGURE => Self{power: ok},
///             e => panic!("Invalid power value {:?}", e)
///         }
///     }
/// }
///
/// impl CardSymbol for MyFigureNumbered{
///     //as space of suits does not include clubs it is now of size=3
///     const SYMBOL_SPACE: usize = 3;
///
///     fn position(&self) -> usize {
///         (self.power -1) as usize
///     }
///
///     fn from_position(position: usize) -> Result<Self, CardError> {
///         let power = (position + 1) as u8;
///         match power{
///             ok @MIN_MY_FIGURE..=MAX_MY_FIGURE => Ok(Self::new(power)),
///             e => Err(CardError::WrongFigurePosition(position))
///         }
///     }
/// }
/// impl PartialOrd<Self> for MyFigureNumbered {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         Some(self.cmp(&other))
///     }
/// }
/// impl Ord for MyFigureNumbered {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.power.cmp(&other.power)
///     }
/// }
/// impl FigureTrait for MyFigureNumbered{
///     const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;
///
/// }
/// ```
pub trait FigureTrait: Debug + Ord + Clone + Hash + CardSymbol {
    const NUMBER_OF_FIGURES: usize = Self::SYMBOL_SPACE;
    /*fn position(&self) -> usize;
    fn from_position(position: usize) -> Result<Self, CardError>;*/

}

