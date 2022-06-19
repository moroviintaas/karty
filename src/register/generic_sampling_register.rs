use std::collections::{HashMap};

use std::hash::Hash;
use rand::{Rng};
use crate::symbol::{CardSymbol, CardElementIterator};
use crate::register::{RandomSamplingRegister, RandomSamplingRegisterCompl, Register};

#[derive(Debug)]
pub struct GenericSamplingRegister<E: Hash + Clone + Eq>{
    register_map: HashMap< E, usize>,
    vector: Vec<E>
}

impl<E: Hash + Clone + Eq> GenericSamplingRegister<E>{
    pub fn new() -> Self{
        Self{ register_map: HashMap::new(), vector: Vec::new()}
    }
}

impl<E: Hash + Clone + Eq> Default for GenericSamplingRegister<E>{
    fn default() -> Self {
         Self{ register_map: HashMap::new(), vector: Vec::new()}
    }
}




impl<E: Hash + Clone + Eq> Register<E> for GenericSamplingRegister<E> {
    fn register(&mut self, element: E) {
        if !self.is_registered(&element){
            self.vector.push(element.to_owned());
            self.register_map.insert(element, self.vector.len()-1);
        }
    }

    fn unregister(&mut self, element: &E) {
        match self.register_map.remove(element){
            Some(index) =>{
                self.vector.swap_remove(index);
            }
            None => {}
        }
    }

    fn is_registered(&self, element: &E) -> bool {
        self.register_map.contains_key(element)
    }
}

impl<E: Hash + Clone + Eq, R: Rng> RandomSamplingRegister<E, R> for GenericSamplingRegister<E>{
    fn peek_sample(&self, rng: &mut R) -> Option<&E> {
        let index = rng.gen_range(0..self.vector.len());
        self.vector.get(index)
    }

    fn pop_sample(&mut self, rng: &mut R) -> Option<E> {
        let index = rng.gen_range(0..self.vector.len());
        let result = self.vector.swap_remove(index);
        self.register_map.remove(&result);
        Some(result)
    }
}

#[derive(Debug)]
pub struct GenericSamplingRegisterCompl<E: CardSymbol + Hash + Clone + Eq>{
    register: GenericSamplingRegister<E>,
    complementary_register: GenericSamplingRegister<E>
}
impl<E: CardSymbol + Hash + Clone + Eq> GenericSamplingRegisterCompl<E>{
    pub fn new() -> Self{
        let mut complementary_register = GenericSamplingRegister::<E>::new();
        let iter = CardElementIterator::new();
        iter.for_each(|x| complementary_register.register(x));
        Self{register: GenericSamplingRegister::new(), complementary_register}
    }
}

impl<E: CardSymbol + Hash + Clone + Eq> Default for GenericSamplingRegisterCompl<E>{
    fn default() -> Self {
        Self::new()
    }
}

impl<E: CardSymbol + Hash + Clone + Eq> Register<E> for GenericSamplingRegisterCompl<E>{
    fn register(&mut self, element: E) {
        self.complementary_register.unregister(&element);
        self.register.register(element);

    }

    fn unregister(&mut self, element: &E) {
        self.register.unregister(element);
        self.complementary_register.register(element.to_owned());
    }

    fn is_registered(&self, element: &E) -> bool {
        self.register.is_registered(element)
    }
}

impl<E: CardSymbol + Hash + Clone + Eq, R: Rng> RandomSamplingRegister<E, R> for GenericSamplingRegisterCompl<E>{
    fn peek_sample(&self, rng: &mut R) -> Option<&E> {
        self.register.peek_sample(rng)
    }

    fn pop_sample(&mut self, rng: &mut R) -> Option<E> {
        self.register.pop_sample(rng).map(|e| {
            self.complementary_register.register(e.to_owned());
            e
        })
    }
}

impl <E: CardSymbol + Hash + Clone + Eq, R: Rng> RandomSamplingRegisterCompl<E, R> for GenericSamplingRegisterCompl<E>{
    fn peek_sample_unr(&self, rng: &mut R) -> Option<&E> {
        self.complementary_register.peek_sample(rng)
    }

    fn pop_sample_unr(&mut self, rng: &mut R) -> Option<E> {
        self.complementary_register.pop_sample(rng).map(|e| {
            self.register.register(e.to_owned());
            e
        })
    }
}

#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use rand::thread_rng;
    use crate::cards::standard::{ACE_SPADES, KING_SPADES, QUEEN_HEARTS};
    use crate::register::{GenericSamplingRegister, GenericSamplingRegisterCompl, RandomSamplingRegister, RandomSamplingRegisterCompl, Register};

    #[test]
    fn test_generic_sample_register(){
        let mut reg = GenericSamplingRegister::new();
        let mut thrng = thread_rng();
        reg.register(ACE_SPADES);
        reg.register(QUEEN_HEARTS);
        assert!(reg.is_registered(&ACE_SPADES));
        assert!(!reg.is_registered(&KING_SPADES));


        let mut hs = HashSet::new();
        hs.insert(reg.pop_sample(&mut thrng).unwrap());
        hs.insert(reg.pop_sample(&mut thrng).unwrap());

        assert!(!reg.is_registered(&ACE_SPADES));
        assert!(hs.contains(&ACE_SPADES));
        assert!(hs.contains(&QUEEN_HEARTS));
    }

    #[test]
    fn test_generic_sample_register_compl(){
        let mut reg = GenericSamplingRegisterCompl::new();
        let mut thrng = thread_rng();
        reg.register(ACE_SPADES);
        reg.register(QUEEN_HEARTS);
        assert!(reg.is_registered(&ACE_SPADES));
        assert!(!reg.is_registered(&KING_SPADES));


        let mut hs = HashSet::new();
        hs.insert(reg.pop_sample(&mut thrng).unwrap());
        hs.insert(reg.pop_sample(&mut thrng).unwrap());

        assert!(!reg.is_registered(&ACE_SPADES));
        assert!(hs.contains(&ACE_SPADES));
        assert!(hs.contains(&QUEEN_HEARTS));

        let unregistered = reg.pop_sample_unr(&mut thrng).unwrap();
        reg.register(unregistered.to_owned());
        assert_eq!(unregistered, reg.pop_sample(&mut thrng).unwrap())


    }
}