use std::{collections::HashSet};
use std::fmt::{Display, Formatter};


use crate::{error::CardErrorGen};
use crate::cards::{Card};
use crate::hand::HandTrait;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};
use crate::symbol::CardSymbol;


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct CardGenHashSet<Crd: CardSymbol + Display>{
    cards: HashSet<Crd>,
    //_phantom: PhantomData<>
}

impl <Crd: CardSymbol + Display> IntoIterator for CardGenHashSet<Crd>{
    type Item = Crd;

    type IntoIter = std::collections::hash_set::IntoIter<Crd>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<Crd: CardSymbol + Display> HandTrait for CardGenHashSet<Crd>{
    type CardType = Crd;

    fn insert_card(&mut self, card: Crd) -> Result<(), crate::error::CardErrorGen<Crd>> {
        if self.cards.insert(card.to_owned()){
            Ok(())
        }
        else{
            Err(CardErrorGen::CardDuplicated(card))
        }
    }

    fn remove_card(&mut self, card: &Crd) -> Result<(), crate::error::CardErrorGen<Crd>> {
        match self.cards.remove(card){
            true => Ok(()),
            false => Err(CardErrorGen::CardNotInHand(card.to_owned()))
        }
    }

    fn empty() -> Self {
        Self{cards: HashSet::new()}
    }

    fn contains(&self, card: &Crd) -> bool {
        self.cards.contains(card)
    }
    fn len(&self) -> usize{
        self.cards.len()
    }

    fn union(&self, other: &Self) -> Self {
        Self{cards:  self.cards.union(&other.cards).cloned().collect()}
    }

    fn intersection(&self, other: &Self) -> Self {
        Self{cards: self.cards.intersection(&other.cards).cloned().collect()}
    }
}

pub type HandSetStd = CardGenHashSet<Card>;


impl<Crd: CardSymbol + Display> CardGenHashSet<Crd>{
    

}
impl<Crd: CardSymbol + Display> Display for CardGenHashSet<Crd>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //let v: Vec<CardStd> = self.cards.iter().collect();
        write!(f,  "[")?;
        if f.alternate(){
            for e in self.cards.iter(){
                write!(f, "{e:#}, ")?;
            }


        }
        else{
            for e in self.cards.iter(){
                write!(f, "{e}, ")?;
            }
        }
        write!(f, "]")
    }
}