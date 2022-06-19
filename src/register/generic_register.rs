use std::collections::{HashSet};
use std::fmt::{Debug};
use std::hash::Hash;
use crate::register::register_trait::{ Register};


#[derive(Debug)]
pub struct GenericRegister<E: Hash + Debug + Eq>{
    set: HashSet<E>
}

impl <E: Hash + Debug + Eq> Default for GenericRegister<E>{
    fn default() -> Self {
        Self{ set: HashSet::new() }
    }
}

impl <E: Hash + Debug + Eq> GenericRegister<E>{
    pub fn new() -> Self{
        Self{ set: HashSet::new()}
    }
}


impl<E: Hash + Debug + Eq> Register<E> for GenericRegister<E>{
    fn register(&mut self, element: E) {
        self.set.insert(element);
    }

    fn unregister(&mut self, element: &E) {
        self.set.remove(element);
    }

    fn is_registered(&self, element: &E) -> bool {
        self.set.contains(element)
    }
}


#[cfg(test)]
mod tests{
    use crate::cards::standard::ACE_SPADES;
    use crate::register::generic_register::GenericRegister;
    use crate::register::register_trait::{Register};

    #[test]
    fn generic_memory_register(){
        let mut generic_record = GenericRegister::new();
        assert!(!generic_record.is_registered(&ACE_SPADES));
        generic_record.register(ACE_SPADES);
        assert!(generic_record.is_registered(&ACE_SPADES));

    }
}

