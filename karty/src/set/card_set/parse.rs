use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::fold_many0;
use nom::sequence::tuple;
use crate::cards::{Card, Card2SymTrait};
use crate::error::CardSetError;
use crate::figures::{parse_figure};
use crate::set::{CardSetStd, CardSet};
use crate::suits::Suit::{Clubs, Diamonds, Hearts, Spades};

pub(crate) fn parse_card_set(s: &str) -> IResult<&str, CardSetStd>{

    tuple((
        fold_many0(
            parse_figure,
            CardSetStd::empty,
            |mut set: CardSetStd, fig|{
                set.insert_card_noerr(Card::from_figure_and_suit(fig, Spades));
                set

            }
        ),
        tag("."),
        fold_many0(
            parse_figure,
            CardSetStd::empty,
            |mut set: CardSetStd, fig|{
                set.insert_card_noerr(Card::from_figure_and_suit(fig, Hearts));
                set

            }
        ),
        tag("."),
        fold_many0(
            parse_figure,
            CardSetStd::empty,
            |mut set: CardSetStd, fig|{
                set.insert_card_noerr(Card::from_figure_and_suit(fig, Diamonds));
                set

            }
        ),
        tag("."),
        fold_many0(
            parse_figure,
            CardSetStd::empty,
            |mut set: CardSetStd, fig|{
                set.insert_card_noerr(Card::from_figure_and_suit(fig, Clubs));
                set

            }
        ),


    ))(s)
        .map(| (rem,(s, _, h, _, d, _, c))|
            (rem, (s.union(&h).union(&d).union(&c))))

}

impl FromStr for CardSetStd {
    type Err = CardSetError;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card_set(s){
            Ok((_rem, c)) => Ok(c),
            Err(_e) => Err(CardSetError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests{
    use std::str::FromStr;
    use nom::multi::fold_many0;
    use crate::card_set;
    use crate::cards::{*};
    use crate::figures::{ parse_figure};
    use crate::set::{CardSetStd, CardSet};
    use crate::suits::Suit::Spades;


    #[test]
    fn parse_4_figures(){
        assert_eq!(
            fold_many0(
                parse_figure,
                CardSetStd::empty,
                |mut set: CardSetStd, fig| {
                set.insert_card(Card::from_figure_and_suit(fig, Spades)).unwrap();
                set
            }


        )("AJ86."), Ok((".", card_set!(ACE_SPADES, JACK_SPADES, EIGHT_SPADES, SIX_SPADES))))

    }

    #[test]
    fn card_set_from_str_correct(){
        let card_set = CardSetStd::from_str("AT86.KJT93.4T.2A").unwrap();
        let card_vec: Vec<Card> = card_set.into_iter().collect();
        assert_eq!(card_vec, [TWO_CLUBS, ACE_CLUBS, FOUR_DIAMONDS, TEN_DIAMONDS, THREE_HEARTS, NINE_HEARTS, TEN_HEARTS, JACK_HEARTS, KING_HEARTS, SIX_SPADES, EIGHT_SPADES, TEN_SPADES, ACE_SPADES]);
        let card_set = CardSetStd::from_str("AT86...2A").unwrap();
        let card_vec: Vec<Card> = card_set.into_iter().collect();
        assert_eq!(card_vec, [TWO_CLUBS, ACE_CLUBS, SIX_SPADES, EIGHT_SPADES, TEN_SPADES, ACE_SPADES]);
    }
}