use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::digit1;
use nom::error::ErrorKind;
use nom::IResult;

use crate::figures::standard::{FigureStd, MAX_NUMBER_FIGURE, MIN_NUMBER_FIGURE, NumberFigureStd};

/// Parses Ace
/// ```
/// use karty::figures::FigureStd;
/// use karty::figures::parse::parse_ace;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_ace("acedd"), Ok(("dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("aCe dd"), Ok((" dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("qd dd"), Err(nom::Err::Error(nom::error::Error::new("qd dd", ErrorKind::Tag))));
/// ```
pub fn parse_ace(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("ace"), tag_no_case("a")))(s)
        .map(|(i, _)| (i, FigureStd::Ace))
}
pub fn parse_king(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("king"), tag_no_case("k")))(s)
        .map(|(i, _)| (i, FigureStd::King))
}
pub fn parse_queen(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("queen"), tag_no_case("q")))(s)
        .map(|(i, _)| (i, FigureStd::Queen))
}
pub fn parse_jack(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("jack"), tag_no_case("j")))(s)
        .map(|(i, _)| (i, FigureStd::Jack))
}

/// Parser numbered figure
/// ```
/// use karty::figures::{FigureStd, NumberFigureStd};
/// use karty::figures::parse::parse_numbered_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_numbered_figure("10fg"), Ok(("fg", FigureStd::Numbered(NumberFigureStd::new(10)))));
/// assert_eq!(parse_numbered_figure("11fg"), Err(nom::Err::Error(nom::error::Error::new("11fg", ErrorKind::Digit))));
/// assert_eq!(parse_numbered_figure("512fg"), Err(nom::Err::Error(nom::error::Error::new("512fg", ErrorKind::Fail))));
/// ```
pub fn parse_numbered_figure(s: &str) -> IResult<&str, FigureStd>{
    match digit1(s){
        Ok((i, ns)) => match ns.parse::<u8>(){
            Ok(n @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE )=> Ok((i, FigureStd::Numbered(NumberFigureStd::new(n)))),
            Ok(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit))),

            Err(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail)))
        }
        Err(e) => Err(e)
    }
    /*
    match map_res(digit1, |ns: &str| ns.parse::<u8>())(s){
        Ok((i, n@ MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE)) => Ok((i, Figure::Numbered(NumberFigure::new(n)))),
        Ok((_, _))  => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::TooLarge))),

        Err(e) => Err(e)
    }*/
}
/// Parses a figure
/// ```
/// use karty::figures::{FigureStd, NumberFigureStd};
/// use karty::figures::parse::parse_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_figure("kc"), Ok(("c", FigureStd::King)));
/// assert_eq!(parse_figure("qdiamonds"), Ok(("diamonds", FigureStd::Queen)));
/// assert_eq!(parse_figure("deadfish"), Err(nom::Err::Error(nom::error::Error::new("deadfish", ErrorKind::Tag))));
/// ```
pub fn parse_figure(s: &str) -> IResult<&str, FigureStd>{
    alt((parse_numbered_figure, parse_ace, parse_king, parse_queen, parse_jack))(s)
}