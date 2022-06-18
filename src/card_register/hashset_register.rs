use std::collections::{HashSet};
use crate::cards::Card;
use crate::figures::Figure;
use crate::card_register::register::{ Register};
use crate::suits::Suit;


#[derive(Debug)]
pub struct HashSetCardRegister<F: Figure, S: Suit>{
    set: HashSet<Card<F,S>>
}


impl<F: Figure, S: Suit> Default for HashSetCardRegister<F, S> {
    fn default() -> Self {
        Self{ set: HashSet::new() }
    }
}

impl<F: Figure, S: Suit> HashSetCardRegister<F, S>{
    pub fn new() -> Self{
        Self{set: HashSet::new()}
    }
}

impl<F: Figure, S: Suit> Register<Card<F,S>> for HashSetCardRegister<F, S>{
    fn register(&mut self, card: Card<F, S>) {
        self.set.insert(card.to_owned());
    }

    fn unregister(&mut self, card: &Card<F, S>) {
        self.set.remove(card);
    }

    fn is_registered(&self, card: &Card<F, S>) -> bool {
        self.set.contains(card)
    }
}

#[cfg(test)]
mod tests{
    use crate::cards::standard::ACE_SPADES;
    use crate::card_register::hashset_register::HashSetCardRegister;
    use crate::card_register::register::{Register};

    #[test]
    fn generic_memory_register(){
        let mut generic_record = HashSetCardRegister::new();
        assert!(!generic_record.is_registered(&ACE_SPADES));
        generic_record.register(ACE_SPADES);
        assert!(generic_record.is_registered(&ACE_SPADES));

    }
}

