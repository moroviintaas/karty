use std::cmp::Ordering;
use crate::cards::{Card, DECK_SIZE};
use crate::error::CardSetErrorGen;
use crate::error::CardSetErrorGen::{BadProbability, ProbabilityBelowZero, ProbabilityOverOne};
use crate::hand::FProbability::{Uncertain, Zero};

#[derive(Clone, Copy, Debug)]
pub enum FProbability{
    One,
    Zero,
    Uncertain(f32),
    Bad(f32)
}

impl FProbability{
    pub fn is_uncertain(&self) -> bool{
        match self{
            Uncertain(_) => true,
            _ => false
        }
    }
}



impl PartialEq<Self> for FProbability {
    fn eq(&self, other: &Self) -> bool {
        let left_asf32 = match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        let right_asf32 = match other{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        left_asf32.eq(&right_asf32)
    }
}

impl PartialOrd for FProbability{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_asf32 = match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };
        let right_asf32 = match other{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => *f,
            FProbability::Bad(b) => *b
        };

        left_asf32.partial_cmp(&right_asf32)
    }
}

impl Into<f32> for FProbability{
    fn into(self) -> f32 {
        match self{
            FProbability::One => 1.0,
            FProbability::Zero => 0.0,
            FProbability::Uncertain(f) => f,
            FProbability::Bad(b) => b
        }
    }
}




impl TryFrom<f32> for FProbability{
    type Error = CardSetErrorGen<Card>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value > 0.0 && value < 1.0{
            Ok(FProbability::Uncertain(value))
        } else if value == 0.0{
            Ok(FProbability::Zero)
        } else if value == 1.0{
            Ok(FProbability::One)
        } else if value < 0.0 {
            Err(ProbabilityBelowZero(value))
        } else if value > 1.0{
            Err(ProbabilityOverOne(value))
        } else {
            Err(BadProbability(value))
        }
    }
}