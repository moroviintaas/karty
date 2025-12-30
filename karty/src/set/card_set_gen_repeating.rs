use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::vec;
use crate::error::CardSetErrorGen;
use crate::set::CardSet;
use crate::symbol::CardSymbol;


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CardSetGenericRepeating<Crd: CardSymbol> {
    cards: HashMap<Crd, u64>,
    size_estimator: usize,
}
impl <Crd: CardSymbol> CardSetGenericRepeating<Crd>{
    fn insert_copies(&mut self, card: Crd, number: u64){
        match self.cards.get_mut(&card){
            None => {
                self.cards.insert(card, number);
                self.size_estimator += number as usize;
            }
            Some(count) => {
                *count += number;
                self.size_estimator += number as usize;
            }
        }
    }
}
impl <Crd: CardSymbol> IntoIterator for CardSetGenericRepeating<Crd>{
    type Item = Crd;
    type IntoIter = vec::IntoIter<Crd>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::with_capacity(self.size_estimator);
        for (card, &count) in self.cards.iter(){
            for _ in 0..count{
                vec.push(card.clone())
            }
        }
        vec.into_iter()
    }
}

impl<Crd: CardSymbol> CardSet for CardSetGenericRepeating<Crd>{
    type CardType = Crd;

    fn insert_card(&mut self, card: Self::CardType) -> Result<(), CardSetErrorGen<Self::CardType>> {

        if let Some(count) = self.cards.get_mut(&card){
            *count += 1;
            self.size_estimator += 1;
        } else {
            self.cards.insert(card, 1);
            self.size_estimator += 1;
        }
        Ok(())
    }

    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), CardSetErrorGen<Self::CardType>> {

        if let Some(count) = self.cards.get_mut(card){
            match count{
                0 => Err(CardSetErrorGen::CardNotInSet(card.clone())),
                1 => {
                    self.cards.remove(card);
                    self.size_estimator -= 1;
                    Ok(())
                },
                n => {
                    *n -= 1;
                    self.size_estimator -= 1;
                    Ok(())
                }
            }
        } else {
            Err(CardSetErrorGen::CardNotInSet(card.clone()))
        }

    }

    fn empty() -> Self {
       Self{
           cards: HashMap::new(),
           size_estimator: 0,
       }
    }

    fn contains(&self, card: &Self::CardType) -> bool {
        match self.cards.get(card){
            None | Some(0) => false,
            Some(_) => true
        }
    }

    fn len(&self) -> usize {
        self.size_estimator
    }

    fn union(&self, other: &Self) -> Self {
        let hm = HashMap::with_capacity(max(self.cards.len(), other.cards.len()));
        let mut hs = CardSetGenericRepeating{
            cards: hm,
            size_estimator: 0,
        };
        for (card, left_count) in &self.cards{
            match other.cards.get(card){
                None => {
                    hs.insert_copies(card.clone(), *left_count);

                },
                Some(right_count) => {
                    hs.insert_copies(card.clone(), max(*left_count, *right_count));
                }
            }
        }
        for (card, right_count) in &other.cards{
            if !self.contains(card){
                hs.insert_copies(card.clone(), *right_count);
            }
        }
        hs

    }

    fn intersection(&self, other: &Self) -> Self {
        let hm = HashMap::with_capacity(max(self.cards.len(), other.cards.len()));
        let mut hs = CardSetGenericRepeating{
            cards: hm,
            size_estimator: 0,
        };
        for (card, left_count) in &self.cards{
            if let Some(right_count) = other.cards.get(card){
                hs.insert_copies(card.clone(), min(*left_count, *right_count))
            }
        }

        hs
    }
}

impl<Crd: CardSymbol + Display> Display for CardSetGenericRepeating<Crd>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,  "[")?;
        if f.alternate(){
            for (card, count) in self.cards.iter(){
                write!(f, "{card:#} (x{count}), ")?;
            }


        }
        else{
            for (card, count) in self.cards.iter(){
                write!(f, "{card} (x{count}), ")?;
            }
        }
        write!(f, "]")
    }
}