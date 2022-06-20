#![cfg_attr(docsrs, feature(doc_cfg))]
use rand::{Rng};
use rand::distributions::{Distribution, Standard};
use crate::symbol::CardSymbol;
use crate::cards::Card;
use crate::figures::Figure;
use crate::suits::Suit;


#[cfg_attr(doc_cfg, doc(cfg(all(feature = "random"))))]
pub trait RandomSymbol<R: Rng>{
    fn random(rng: &mut R) -> Self;

}

impl<E: CardSymbol, R: Rng> RandomSymbol<R> for E
where Standard: Distribution<E>{
    fn random(rng: &mut R) -> Self {
        rng.sample(Standard)
    }
}

impl<R: Rng, F: Figure + RandomSymbol<R>, S: Suit + RandomSymbol<R>> RandomSymbol<R> for Card<F, S>{
    fn random(rng: &mut R) -> Self {
        Self{suit: S::random(rng), figure: F::random(rng)}
    }
}


#[cfg(test)]
mod test{
    use rand::thread_rng;
    use crate::symbol::CardSymbol;
    use crate::figures::FigureStd;
    use crate::random::RandomSymbol;
    use crate::suits::SuitStd;

    #[test]
    fn test_random_std_suit(){
        for _ in 0..=20 {
            let suit = SuitStd::random(&mut thread_rng());
            assert!(suit.position() < 5);
        }
    }

    #[test]
    fn test_random_std_figure(){
        for _ in 0..=20{
            let figure = FigureStd::random(&mut thread_rng());
            assert!(figure.position() < 13);
        }

    }
}