use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use comparator::Comparator;
use crate::figures::Figure;
use crate::suits::Suit;
use crate::symbol::CardSymbol;
#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};

pub trait Card2S<F: CardSymbol, S:CardSymbol>{
    const CARD_SPACE: usize = F::SYMBOL_SPACE * S::SYMBOL_SPACE;
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card<F: CardSymbol + Debug + Eq + PartialEq + Clone + Hash,
    S: CardSymbol + Debug + Eq + PartialEq + Clone + Hash> {
    pub(crate) suit: S,
    pub(crate) figure: F
}

//pub type CardL<F, S> = Card<F, S>;

impl<F:Figure, S:Suit> Card2S<F,S> for Card<F,S>{}

impl<F: Figure + Copy, S: Suit + Copy> Copy for Card<F, S>{}


impl<F:Figure, S: Suit > Card<F, S> {
    pub fn new(figure: F, suit: S) -> Self{
        Self{suit, figure}
    }

    pub fn suit(&self) -> &S {
        &self.suit
    }
    pub fn figure(&self) -> &F {
        &self.figure
    }

}

pub struct ComparatorCard<F: Figure, S: Suit, CF: Comparator<F> + Default,  CS: Comparator<S> + Default> {
    pub suit_comparator: CS,
    pub figure_comparator: CF,
    pub _phantom: PhantomData<(F, S)>,

}

impl<F: Figure, S: Suit, CS: Comparator<S> + Default, CF: Comparator<F> + Default> Default
for ComparatorCard<F, S, CF, CS>{
    fn default() -> Self {
        Self{suit_comparator: CS::default(), figure_comparator: CF::default(), _phantom: PhantomData::default()}
    }
}

impl<F: Figure, S: Suit, CS: Comparator<S> + Default, CF: Comparator<F> + Default>
ComparatorCard<F, S, CF, CS>{

/// ```
/// use karty::cards::{STANDARD_DECK};
/// use karty::figures::ComparatorF;
/// use karty::suits::ComparatorAHCD;
/// use karty::cards::*;
///
/// let mut deck = Vec::from(STANDARD_DECK);
/// let bridge_comparator = CARD_COMPARATOR_BRIDGE;
/// assert_eq!(deck[51], ACE_SPADES);
/// assert_eq!(deck[50], ACE_HEARTS);
/// deck.sort_by(|l, r | bridge_comparator.cmp_suit_figure(l, r));
/// assert_eq!(deck[51], ACE_SPADES);
/// assert_eq!(deck[50], KING_SPADES);
/// ```
    pub fn cmp_suit_figure(&self, l: &Card<F, S>, r: &Card<F, S>) -> Ordering {
        match self.suit_comparator.compare(l.suit(), r.suit()){
            Ordering::Equal => self.figure_comparator.compare(l.figure(), r.figure() ),
            ordering => ordering
        }
    }

/// ```
/// use karty::cards::{STANDARD_DECK};
/// use karty::figures::ComparatorF;
/// use karty::suits::ComparatorAHCD;
/// use karty::cards::*;
///
/// let mut deck = Vec::from(STANDARD_DECK);
/// let bridge_comparator = CARD_COMPARATOR_VISUAL;
/// assert_eq!(deck[51], ACE_SPADES);
/// assert_eq!(deck[50], ACE_HEARTS);
/// assert_eq!(deck[49], ACE_DIAMONDS);
/// assert_eq!(deck[48], ACE_CLUBS);
/// deck.sort_by(|l, r | bridge_comparator.cmp_figure_suit(l, r));
/// assert_eq!(deck[51], ACE_SPADES);
/// assert_eq!(deck[50], ACE_HEARTS);
/// assert_eq!(deck[49], ACE_CLUBS);
/// assert_eq!(deck[48], ACE_DIAMONDS);
/// ```
    pub fn cmp_figure_suit(&self, l: &Card<F, S>, r: &Card<F, S>) -> Ordering {
        match self.figure_comparator.compare(l.figure(), r.figure()){
            Ordering::Equal => self.suit_comparator.compare(l.suit(), r.suit() ),
            ordering => ordering
        }
    }
}






