use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::digit1;
use nom::error::ErrorKind;
use nom::IResult;
use crate::figures::F10;

use crate::figures::standard::{Figure, MAX_NUMBER_FIGURE, MIN_NUMBER_FIGURE, NumberFigure};
/* In case nom parsers should be published
/// Parses Ace
/// ```
/// use karty::figures::FigureStd;
/// use karty::figures::parse::parse_ace;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_ace("acedd"), Ok(("dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("aCe dd"), Ok((" dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("qd dd"), Err(nom::Err::Error(nom::error::Error::new("qd dd", ErrorKind::Tag))));
/// ```

 */
fn parse_ace(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("ace"), tag_no_case("a")))(s)
        .map(|(i, _)| (i, Figure::Ace))
}
fn parse_king(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("king"), tag_no_case("k")))(s)
        .map(|(i, _)| (i, Figure::King))
}
fn parse_queen(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("queen"), tag_no_case("q")))(s)
        .map(|(i, _)| (i, Figure::Queen))
}
fn parse_jack(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("jack"), tag_no_case("j")))(s)
        .map(|(i, _)| (i, Figure::Jack))
}
fn parse_ten(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("ten"), tag_no_case("t")))(s)
        .map(|(i, _)| (i, F10))
}

fn parse_numbered_figure(s: &str) -> IResult<&str, Figure>{
    match digit1(s){
        Ok((i, ns)) => match ns.parse::<u8>(){
            Ok(n @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE )=> Ok((i, Figure::Numbered(NumberFigure::new(n)))),
            Ok(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit))),

            Err(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail)))
        }
        Err(e) => Err(e)
    }

}
/// Parses a figure
/// ```
/// use karty::figures::{Figure, NumberFigure};
/// use karty::figures::parse_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_figure("kc"), Ok(("c", Figure::King)));
/// assert_eq!(parse_figure("qdiamonds"), Ok(("diamonds", Figure::Queen)));
/// assert_eq!(parse_figure("deadfish"), Err(nom::Err::Error(nom::error::Error::new("deadfish", ErrorKind::Tag))));
/// ```
pub fn parse_figure(s: &str) -> IResult<&str, Figure>{
    alt((parse_ten, parse_numbered_figure, parse_ace, parse_king, parse_queen, parse_jack))(s)
}

#[cfg(test)]
mod tests{
    use nom::error::ErrorKind;
    use crate::figures::{Figure, NumberFigure, parse};

    #[test]
    fn parse_ace(){
        assert_eq!(parse::parse_ace("acedd"), Ok(("dd", Figure::Ace)));
        assert_eq!(parse::parse_ace("aCe dd"), Ok((" dd", Figure::Ace)));
        assert_eq!(parse::parse_ace("qd dd"), Err(nom::Err::Error(nom::error::Error::new("qd dd", ErrorKind::Tag))));

    }

    #[test]
    fn parse_numbered_figure(){
        assert_eq!(parse::parse_numbered_figure("10fg"), Ok(("fg", Figure::Numbered(NumberFigure::new(10)))));
        assert_eq!(parse::parse_numbered_figure("11fg"), Err(nom::Err::Error(nom::error::Error::new("11fg", ErrorKind::Digit))));
        assert_eq!(parse::parse_numbered_figure("512fg"), Err(nom::Err::Error(nom::error::Error::new("512fg", ErrorKind::Fail))));
        assert_eq!(parse::parse_figure("tfg"), Ok(("fg", Figure::Numbered(NumberFigure::new(10)))));
    }
}