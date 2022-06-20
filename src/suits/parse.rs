//! Module containing parsing functions for [`SuitStd`][crate::suits::SuitStd].
//! To parse suits crate [`nom`][nom] is used.
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
//! # Enable:
//! Use feature `parse`.
//!
use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use crate::suits::standard::SuitStd;

fn parse_spades(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("spades"), tag_no_case("s"),
         tag_no_case("♠"), tag_no_case("♤")))(s)
        .map(|(i,_) | (i, SuitStd::Spades))
}
fn parse_hearts(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("hearts"), tag_no_case("h"),
         tag_no_case("♥"), tag_no_case("♡")))(s)
        .map(|(i,_) | (i, SuitStd::Hearts))
}
fn parse_diamonds(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("diamonds"), tag_no_case("diax"), tag_no_case("d"),
         tag_no_case("♦"), tag_no_case("♢")))(s)
        .map(|(i,_) | (i, SuitStd::Diamonds))
}
fn parse_clubs(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("clubs"), tag_no_case("c"),
         tag_no_case("♣"), tag_no_case("♧")))(s)
        .map(|(i,_) | (i, SuitStd::Clubs))
}

/// Parses [`SuitStd`][crate::suits::SuitStd] from `&str`. Consumes initial sequences of:
/// ## names (case insensitive):
/// spades; hearts; diamonds; clubs;
/// ## first letters (case_insensitive):
/// s; h; d; c;
/// ## unicode symbols:
/// ♠; ♥; ♦; ♣;
///
/// consuming it and returning `Result::Ok()` with corresponding [`SuitStd`][crate::suits::SuitStd]
///
/// # Returns
/// If successful returns `Ok((remaining: &str, result: SuitStd)), else returns `Err(...)`
/// containing given &str that could not be parsed. Refer to [`nom`][nom] documentation: [`IResult`][nom::IResult].
///
/// # Example:
/// ```
/// use karty::suits::parse::parse_suit;
/// use karty::suits::SuitStd;
/// use nom::error::{Error, ErrorKind};
///
/// assert_eq!(parse_suit("sgq"), Ok(("gq", SuitStd::Spades)));
/// assert_eq!(parse_suit("diamondsda"), Ok(("da", SuitStd::Diamonds)));
/// assert_eq!(parse_suit("eadfish"), Err(nom::Err::Error(Error::new("eadfish", ErrorKind::Tag))));
/// assert_eq!(parse_suit("♦K"), Ok(("K", SuitStd::Diamonds)));
/// ```


pub fn parse_suit(s: &str) -> IResult<&str, SuitStd>{
    alt((parse_spades, parse_hearts, parse_diamonds, parse_clubs))(s)
}
/// Parses [`SuitStd`][crate::suits::SuitStd] from `&str`. Consumes initial sequences of:
/// ## names (case insensitive):
/// spades, hearts, diamonds, clubs;
/// ## first letters (case_insensitive):
/// s, h, d, c,
/// ## unicode symbols:
/// ♠, ♥, ♦, ♣,
///
/// consuming it and returning `Result::Ok()` with corresponding [`SuitStd`][crate::suits::SuitStd]
///
/// # Returns
/// If successful returns `Ok((remaining: &str, result: SuitStd)), else returns `Err(...)`
/// containing given &str that could not be parsed. Refer to [`nom`][nom] documentation: [`IResult`][nom::IResult].
///
/// # Example:
/// ```
/// use karty::suits::parse::parse_suit;
/// use karty::suits::SuitStd;
/// use nom::error::ErrorKind;
/// use nom::Err::Error;
/// use std::str::FromStr;
///
/// assert_eq!(SuitStd::from_str("sgq"), Ok(SuitStd::Spades));
/// assert_eq!(SuitStd::from_str("diamondsda"), Ok(SuitStd::Diamonds));
/// assert!(SuitStd::from_str("eadfish").is_err());
/// assert_eq!(SuitStd::from_str("♦K"), Ok(SuitStd::Diamonds));
/// ```
impl FromStr for SuitStd{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_suit(s).map(|(_, suit)| suit).map_err(|e| format!("{:?}", e))
    }
}

#[cfg(test)]
mod tests{
    use crate::suits::{parse, standard::SuitStd};
    #[test]
    fn parse_spades(){
        assert_eq!(parse::parse_spades("spadesacedd"), Ok(("acedd", SuitStd::Spades)));
        assert_eq!(parse::parse_spades("s aCe dd"), Ok((" aCe dd", SuitStd::Spades)));
        assert_eq!(parse::parse_spades("♠ aCe dd"), Ok((" aCe dd", SuitStd::Spades)));
    }
}