use std::ops::Index;
use approx::abs_diff_ne;
use crate::cards::{Card, DECK_SIZE};
use crate::error::CardSetErrorGen;
use serde_big_array::BigArray;
use crate::figures::Figure;
use crate::suits::{SuitMap, SUITS};
use crate::symbol::CardSymbol;

const FUZZY_CARD_SET_TOLERANCE: f32 = 0.01;
//#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[derive(Clone)]
pub struct FuzzyCardSet {
    //#[serde(with = "BigArray")]
    //probabilities: [f32; DECK_SIZE],
    probabilities: SuitMap<[f32; Figure::SYMBOL_SPACE]>,
    expected_card_number : f32,

}
impl FuzzyCardSet{
    /*
    pub fn new_unchecked(probabilities: [f32; DECK_SIZE], expected_card_number: u8) -> Self{
        Self{probabilities, expected_card_number: expected_card_number as f32}
    }

     */
    pub fn new_unchecked(probabilities: SuitMap<[f32; Figure::SYMBOL_SPACE]>, expected_card_number: u8) -> Self{
        Self{probabilities, expected_card_number: expected_card_number as f32}
    }

    /// ```
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let cards_spades_bad =  [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35];
    ///
    /// assert!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades_bad, cards_hearts, cards_diamonds, cards_clubs), 13).is_err());
    /// assert_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap().expected_card_number(), 13);
    /// ```
    ///
    pub fn new_check_epsilon(probabilities: SuitMap<[f32; Figure::SYMBOL_SPACE]>, expected_card_number: u8) -> Result<Self, CardSetErrorGen<Card>>{
        //let sum = probabilities.iter().sum();
        let sum = SUITS.iter().fold(0.0, |acc, x|{
            acc + probabilities[*x].iter().sum::<f32>()
        });
        let expected = expected_card_number as f32;
        if abs_diff_ne!(expected, sum, epsilon = FUZZY_CARD_SET_TOLERANCE){
            return Err(CardSetErrorGen::BadProbabilitiesSum(sum, expected))
        }
        Ok(Self{probabilities, expected_card_number: expected})
    }

    pub fn probabilities(&self) -> &SuitMap<[f32; Figure::SYMBOL_SPACE]>{
        &self.probabilities
    }
    pub fn expected_card_number(&self) -> u8{
        self.expected_card_number as u8
    }
}

impl Index<Card> for FuzzyCardSet{
    type Output = f32;

    fn index(&self, index: Card) -> &Self::Output {
        &self.probabilities[index.suit][index.figure.position()]
    }
}


#[cfg(feature = "serde")]
mod serde{
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::hand::FuzzyCardSet;

    impl Serialize for FuzzyCardSet{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            todo!()
        }
    }

    impl<'de> Deserialize<'de> for FuzzyCardSet{
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            todo!()
        }
    }
}
