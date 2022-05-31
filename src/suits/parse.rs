use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use crate::suits::standard::SuitStd;

pub fn parse_spades(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("spades"), tag_no_case("s"),
         tag_no_case("♠"), tag_no_case("♤")))(s)
        .map(|(i,_) | (i, SuitStd::Spades))
}
pub fn parse_hearts(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("hearts"), tag_no_case("h"),
         tag_no_case("♥"), tag_no_case("♡")))(s)
        .map(|(i,_) | (i, SuitStd::Hearts))
}
pub fn parse_diamonds(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("diamonds"), tag_no_case("diax"), tag_no_case("d"),
         tag_no_case("♦"), tag_no_case("♢")))(s)
        .map(|(i,_) | (i, SuitStd::Diamonds))
}
pub fn parse_clubs(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("clubs"), tag_no_case("c"),
         tag_no_case("♣"), tag_no_case("♧")))(s)
        .map(|(i,_) | (i, SuitStd::Clubs))
}

/// Parses a suit
/// ```
/// use karty::suits::SuitStd;
/// use karty::suits::parse::parse_suit;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_suit("sgq"), Ok(("gq", SuitStd::Spades)));
/// assert_eq!(parse_suit("diamondsda"), Ok(("da", SuitStd::Diamonds)));
/// assert_eq!(parse_suit("eadfish"), Err(nom::Err::Error(nom::error::Error::new("eadfish", ErrorKind::Tag))));
/// assert_eq!(parse_suit("♦K"), Ok(("K", SuitStd::Diamonds)));
/// ```
pub fn parse_suit(s: &str) -> IResult<&str, SuitStd>{
    alt((parse_spades, parse_hearts, parse_diamonds, parse_clubs))(s)
}

#[cfg(test)]
mod tests{
    use crate::suits::{parse, SuitStd};
    #[test]
    fn parse_spades(){
        assert_eq!(parse::parse_spades("spadesacedd"), Ok(("acedd", SuitStd::Spades)));
        assert_eq!(parse::parse_spades("s aCe dd"), Ok((" aCe dd", SuitStd::Spades)));
        assert_eq!(parse::parse_spades("♠ aCe dd"), Ok((" aCe dd", SuitStd::Spades)));
    }
}