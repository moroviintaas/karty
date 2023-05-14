use std::ops::Index;
use approx::abs_diff_ne;
use crate::cards::{Card, DECK_SIZE};
use crate::error::CardSetErrorGen;
use serde_big_array::BigArray;
use crate::figures::Figure;
use crate::suits::{Suit, SuitMap, SUITS};
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

    /// ```
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_ones_in_suit(&Clubs), 0);
    /// assert_eq!(fset.count_ones_in_suit(&Diamonds), 1);
    /// assert_eq!(fset.count_ones_in_suit(&Hearts), 0);
    /// assert_eq!(fset.count_ones_in_suit(&Spades), 0);
    /// ```
    ///
    pub fn count_ones_in_suit(&self, suit: &Suit) -> usize{
        /*self.probabilities[suit].iter().fold(0, |acc, x|{
            if *x>= 1.0{
                acc + 1
            } else {
                acc
            }
        })*/
        self.probabilities[suit].iter().filter(|&&x| x>= 1.0f32).count()
    }

    pub fn count_ones(&self) -> usize{
        SUITS.iter().map(|s|self.count_ones_in_suit(s)).sum()
    }

    /// ```
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_zeros_in_suit(&Clubs), 0);
    /// assert_eq!(fset.count_zeros_in_suit(&Diamonds), 12);
    /// assert_eq!(fset.count_zeros_in_suit(&Hearts), 0);
    /// assert_eq!(fset.count_zeros_in_suit(&Spades), 1);
    /// ```
    ///
    pub fn count_zeros_in_suit(&self, suit: &Suit) -> usize{
        self.probabilities[suit].iter().filter(|&&x| x<= 0.0f32).count()
    }

    pub fn count_zeros(&self) -> usize{
        SUITS.iter().map(|s|self.count_zeros_in_suit(s)).sum()
    }

    /// ```
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_uncertain_in_suit(&Clubs), 13);
    /// assert_eq!(fset.count_uncertain_in_suit(&Diamonds), 0);
    /// assert_eq!(fset.count_uncertain_in_suit(&Hearts), 13);
    /// assert_eq!(fset.count_uncertain_in_suit(&Spades), 12);
    /// ```
    pub fn count_uncertain_in_suit(&self, suit: &Suit) -> usize{
        self.probabilities[suit].into_iter().filter(|&x| 0.0 < x && x < 1.0).count()
    }

    pub fn count_uncertain(&self) -> usize{
        SUITS.iter().map(|s| self.count_uncertain_in_suit(s)).sum()

    }

    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_abs_diff_eq!(fset.sum_probabilities_in_suit(&Clubs), 3.9, epsilon=0.001);
    /// assert_eq!(fset.sum_probabilities_in_suit(&Diamonds), 1.0);
    /// assert_abs_diff_eq!(fset.sum_probabilities_in_suit(&Spades), 4.2, epsilon=0.001);
    ///
    /// ```
    ///
    pub fn sum_probabilities_in_suit(&self, suit: &Suit) -> f32{
        self.probabilities[suit].iter().sum()
    }

     /// ```
    /// use approx::assert_abs_diff_eq;
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// assert_abs_diff_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap().sum_probabilities(), 13.0, epsilon=0.001);
    /// ```
    pub fn sum_probabilities(&self) -> f32{
        SUITS.iter().map(|s| self.sum_probabilities_in_suit(s)).sum()
    }
}


impl Index<&Card> for FuzzyCardSet{
    type Output = f32;
    /// ```
    /// use karty::cards::TWO_DIAMONDS;
    /// use karty::suits::SuitMap;
    /// use karty::hand::FuzzyCardSet;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// assert_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs),13 ).unwrap()[&TWO_DIAMONDS], 1.0);
    /// ```
    fn index(&self, index: &Card) -> &Self::Output {
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
