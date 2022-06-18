use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Standard};
use crate::card_element::CardElement;
use crate::cards::Card;
use crate::figures::Figure;
use crate::suits::Suit;
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "random"))))]
pub trait RandomElement{
    fn random() -> Self;

}

impl<E: CardElement> RandomElement for E
where Standard: Distribution<E>{
    fn random() -> Self {
        thread_rng().sample(Standard)
    }
}

impl<F: Figure + RandomElement, S: Suit + RandomElement> RandomElement for Card<F, S>{
    fn random() -> Self {
        Self{suit: S::random(), figure: F::random()}
    }
}

#[cfg(test)]
mod test{
    use crate::card_element::CardElement;
    use crate::figures::FigureStd;
    use crate::random::RandomElement;
    use crate::suits::SuitStd;

    #[test]
    fn test_random_std_suit(){
        for _ in 0..=20 {
            let suit = SuitStd::random();
            assert!(suit.position() < 5);
        }
    }

    #[test]
    fn test_random_std_figure(){
        for _ in 0..=20{
            let figure = FigureStd::random();
            assert!(figure.position() < 13);
        }

    }
}