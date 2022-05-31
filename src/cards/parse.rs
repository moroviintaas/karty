use std::str::FromStr;
use nom::branch::alt;
use nom::character::complete::space0;
use nom::IResult;
use nom::sequence::{separated_pair};
use crate::cards::Card;
use crate::figures::parse::parse_figure;
use crate::figures::standard::FigureStd;
use crate::suits::parse::parse_suit;
use crate::suits::standard::SuitStd;


fn parse_card_fs(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>>{
    match separated_pair(parse_figure, space0, parse_suit)(s){
        Ok((i, (fig, suit))) => Ok((i, Card::new(fig, suit))),
        Err(e) => Err(e)
    }

}
/* In case evere needed to publish
fn parse_card_fs_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    delimited(space0, parse_card_fs, space0)(s)
}
*/


pub fn parse_card_sf(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    match separated_pair(parse_suit, space0, parse_figure)(s) {
        Ok((i, (suit, figure))) => Ok((i, Card::new(figure, suit))),
        Err(e) => Err(e)
    }
}
/* In case ever needed to publish
fn parse_card_sf_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    delimited(space0, parse_card_sf, space0)(s)
}

 */

/// Parses card from &str (non delimeted way)
/// ```
/// use karty::cards::Card;
/// use karty::figures::{FigureStd, NumberFigureStd};
/// use karty::cards::parse::parse_card;
/// use karty::suits::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card("10 dxg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card("A  sdiax"), Ok(("diax", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card("h  jv"), Ok(("v", Card::new(FigureStd::Jack, SuitStd::Hearts))));
/// assert_eq!(parse_card("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("A10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    alt((parse_card_fs, parse_card_sf))(s)
}
/* In case ever needed to publish
/// Parses card from &str (delimited way)
/// ```
/// use karty::cards::Card;
/// use karty::figures::standard::{FigureStd, NumberFigureStd};
/// use karty::cards::parse::parse_card_delimited;
/// use karty::suits::standard::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card_delimited("10 d  xg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card_delimited("A  s\tdiax"), Ok(("diax", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card_delimited("   h  jv"), Ok(("v", Card::new(FigureStd::Jack, SuitStd::Hearts))));
/// assert_eq!(parse_card_delimited(" A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("A10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>>{
    delimited(space0, parse_card, space0)(s)
}


 */

/// Parses Card from str
/// ```
/// use karty::figures::{NumberFigureStd, FigureStd};
/// use karty::suits::SuitStd;
/// use karty::cards::Card;
/// use std::str::FromStr;
/// use karty::cards::standard::{ACE_SPADES, FOUR_CLUBS, NINE_SPADES};
/// assert_eq!(Card::from_str("A s"), Ok(ACE_SPADES));
/// assert_eq!(Card::from_str("4caa"), Ok(FOUR_CLUBS));
/// assert!(Card::from_str("jq").is_err());
/// assert_eq!(Card::from_str("9♠"), Ok(NINE_SPADES));
/// ```
impl FromStr for Card<FigureStd, SuitStd> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_card(s).map(|(_, card)| card).map_err(|e| format!("{}", e))
    }
}


#[cfg(test)]
mod tests{
    use nom::error::ErrorKind;
    use crate::cards::{parse};
    use crate::cards::standard::{ACE_SPADES, TEN_DIAMONDS};

    #[test]
    fn parse_card_fs(){
        assert_eq!(parse::parse_card_fs("10 dxg"), Ok(("xg", TEN_DIAMONDS)));
        assert_eq!(parse::parse_card_fs("A  sdiax"), Ok(("diax", ACE_SPADES)));
        assert_eq!(parse::parse_card_fs("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("10  sdiax", ErrorKind::Tag))));

    }
    /* In case delimited function should be published
    #[test]
    fn parse_card_fs_delimited(){
        assert_eq!(parse::parse_card_fs_delimited("  10 d\txg"), Ok(("xg", TEN_DIAMONDS)));
        assert_eq!(parse::parse_card_fs_delimited(" A  s\tdiax  "), Ok(("diax  ", ACE_SPADES)));
        assert_eq!(parse::parse_card_fs_delimited("\tA10  sdiax  "), Err(nom::Err::Error(nom::error::Error::new("10  sdiax  ", ErrorKind::Tag))));

    }*/
}