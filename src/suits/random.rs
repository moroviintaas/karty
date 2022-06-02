use rand::distributions::{Standard};
use rand::prelude::Distribution;
use rand::Rng;
use crate::suits::{Suit, SuitStd};

/*impl<S:Suit> Distribution<S> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> S {
        S::from_position(rng.gen_range(0..S::NUMBER_OF_SUITS)).unwrap()
    }
}*/

impl Distribution<SuitStd> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SuitStd {
        SuitStd::from_position(rng.gen_range(0..SuitStd::NUMBER_OF_SUITS)).unwrap()
    }
}

#[cfg(test)]
mod test{
    use rand::{Rng, thread_rng};
    use rand::distributions::Standard;
    use crate::suits::{Suit, SuitStd};

    #[test]
    fn test_sample(){
        let mut trng = thread_rng();
        for _ in 0..=1000{
            let suit:SuitStd = trng.sample(Standard);
            assert!(suit.position() <= SuitStd::NUMBER_OF_SUITS)

        }
    }
}