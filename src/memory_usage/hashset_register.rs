use std::collections::{HashSet};
use crate::cards::Card;
use crate::figures::Figure;
use crate::memory_usage::register::CardRegister;
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

impl<F: Figure, S: Suit> CardRegister<F, S> for HashSetCardRegister<F, S>{
    fn mark_used(&mut self, card: &Card<F, S>) {
        self.set.insert(card.to_owned());
    }

    fn is_card_used(&self, card: &Card<F, S>) -> bool {
        self.set.contains(card)
    }
}

#[cfg(test)]
mod tests{
    use crate::cards::standard::ACE_SPADES;
    use crate::memory_usage::hashset_register::HashSetCardRegister;
    use crate::memory_usage::register::CardRegister;

    #[test]
    fn generic_memory_register(){
        let mut generic_record = HashSetCardRegister::new();
        assert!(!generic_record.is_card_used(&ACE_SPADES));
        generic_record.mark_used(&ACE_SPADES);
        assert!(generic_record.is_card_used(&ACE_SPADES));

    }
}

