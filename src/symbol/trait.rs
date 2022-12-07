//! Module containing basic Symbol trait
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
use crate::error::CardError;
use std::fmt::Debug;
use crate::symbol::CardSymbolIterator;

/// Trait representing a symbol on a playing card.
/// Typical playing card (one of 52 card deck) is defined by two symbols - [`Figure`][crate::figures::Figure]  and [`Suit`][crate::suits::Suit]
/// Structure implementing this trait should have finite space of possible instances.
/// Instances should have associated number from `0` to [`SYMBOL_SPACE`][crate::symbol::CardSymbol::SYMBOL_SPACE] (excluded).
/// Numbering should be dense, which mean that there should not be a number in [0, [`SYMBOL_SPACE`][crate::symbol::CardSymbol::SYMBOL_SPACE])
/// with no associated [`CardSymbol`][crate::symbol::CardSymbol] instance.
/// For example, implemented in this crate `FigureStd` has `13` possible instances,
/// representing symbols (2,..., 10, Jack, Queen, King, Ace), but their associated numbers are `0..=12`.
pub trait CardSymbol: Sized + Eq +  std::hash::Hash  + Clone + Debug{
    /// Space of symbol, holds information how many possible instances of symbol exists.
    /// For standard figure it is `13`, for ls
    /// standard suit it is `4`.
    const SYMBOL_SPACE: usize;
    /// Information about symbol's associated number. Collision in returning numbers will cause
    /// wrong behaviour of functions utilising trait. Position should be unique for every symbol,
    /// it is used for example to iterate over space of symbols. It should be dense though.
    /// # Returns:
    /// Number associated with symbol.
    /// # Examples:
    /// For standard suit:
    /// ```
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(Spades.position(), 3);
    /// assert_eq!(Hearts.position(), 2);
    /// assert_eq!(Diamonds.position(), 1);
    /// assert_eq!(Clubs.position(), 0);
    /// ```
    /// For standard figures:
    /// ```
    /// use karty::figures::{F10, King};
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(F10.position(), 8);
    /// assert_eq!(King.position(), 11);
    /// ```
    fn position(&self) -> usize;
    /// Reverse function to [`position(&self)`][crate::symbol::CardSymbol::position], creates symbol instance, given it's associated number.
    /// # Returns:
    /// Instance of symbol associated with a number.
    /// # Example:
    /// ```
    /// use karty::cards::{Card, FIVE_CLUBS, JACK_SPADES};
    /// use karty::figures::{Figure, Queen};
    /// use karty::suits::Suit;
    /// use karty::suits::Suit::Hearts;
    /// use karty::symbol::CardSymbol;
    /// assert_eq!(Figure::from_position(10).unwrap(), Queen);
    /// assert_eq!(Suit::from_position(2).unwrap(), Hearts);
    /// assert_eq!(Card::from_position(3).unwrap(), FIVE_CLUBS);
    /// assert_eq!(Card::from_position(48).unwrap(), JACK_SPADES);
    /// ```
    fn from_position(position: usize) -> Result<Self, CardError>;
    /// Method constructs [`CardSymbolIterator`][crate::symbol::CardSymbolIterator] iterating symbols from the one numbered `0` to
    /// the last one (numbered [`SYMBOL_SPACE-1`][crate::symbol::CardSymbol::SYMBOL_SPACE].
    /// # Returns:
    /// Iterator of members of the type.
    /// `Sym::iterator()` is equivalent of `CardSymbolIterator<Sym>`.
    /// # Example:
    /// ```
    /// use karty::cards::Card;
    /// use karty::symbol::{CardSymbol, CardSymbolIterator};
    /// let cards_from_new: Vec<Card> = CardSymbolIterator::<Card>::new().collect();
    /// let cards_from_iter: Vec<Card> = Card::iterator().collect();
    /// assert_eq!(cards_from_new.len(), 52);
    /// for i in 0..=51{
    ///     assert_eq!(&cards_from_iter[i], &cards_from_new[i])
    /// }
    ///
    /// ```
    fn iterator() -> CardSymbolIterator<Self>{
        CardSymbolIterator::new()
    }
}