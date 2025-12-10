//#![cfg_attr(docsrs, feature(doc_cfg))]
use rand::{Rng};
use rand::distr::{Distribution, StandardUniform};
use crate::symbol::CardSymbol;
use crate::cards::Card2SGen;
use crate::figures::FigureTrait;
use crate::suits::SuitTrait;


//#[cfg_attr(doc_cfg, doc(cfg(all(feature = "random"))))]
pub trait RandomSymbol<R: Rng>{
    fn random(rng: &mut R) -> Self;

}

impl<E: CardSymbol, R: Rng> RandomSymbol<R> for E
where StandardUniform: Distribution<E>{
    fn random(rng: &mut R) -> Self {
        rng.sample(StandardUniform)
    }
}

impl<R: Rng, F: FigureTrait + RandomSymbol<R>, S: SuitTrait + RandomSymbol<R>> RandomSymbol<R> for Card2SGen<F, S>{
    fn random(rng: &mut R) -> Self {
        Self{suit: S::random(rng), figure: F::random(rng)}
    }
}


#[cfg(test)]
mod test{
    use crate::symbol::CardSymbol;
    use crate::figures::Figure;
    use crate::random::RandomSymbol;
    use crate::suits::Suit;

    #[test]
    fn test_random_std_suit(){
        for _ in 0..=20 {
            let suit = Suit::random(&mut rand::rng());
            assert!(suit.usize_index() < 5);
        }
    }

    #[test]
    fn test_random_std_figure(){
        for _ in 0..=20{
            let figure = Figure::random(&mut rand::rng());
            assert!(figure.usize_index() < 13);
        }

    }
}