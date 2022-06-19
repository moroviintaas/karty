use rand::Rng;
use crate::register::Register;



pub trait RandomSamplingRegister<E, R: Rng>: Register<E> {
    fn peek_sample(&self, rng: &mut R) -> Option<&E>;
    fn pop_sample(&mut self, rng: &mut R) -> Option<E>;
}

pub trait RandomSamplingRegisterCompl<E,R: Rng>: RandomSamplingRegister<E, R>{
    fn peek_sample_unr(&self, rng: &mut R) -> Option<&E>;
    fn pop_sample_unr(&mut self, rng: &mut R) -> Option<E>;
}