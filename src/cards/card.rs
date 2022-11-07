use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use comparator::Comparator;
use crate::figures::FigureTrait;
use crate::suits::SuitTrait;
use crate::symbol::CardSymbol;
#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};

pub trait Card2S<F: FigureTrait, S: SuitTrait>{
    const CARD_SPACE: usize = F::SYMBOL_SPACE * S::SYMBOL_SPACE;
}

pub trait Card2SymTrait: Debug + Clone + Eq{
    type Figure: FigureTrait;
    type Suit: SuitTrait;
    const CARD_SPACE: usize = Self::Figure::SYMBOL_SPACE * Self::Suit::SYMBOL_SPACE;
    fn suit(&self) -> &Self::Suit;
    fn figure(&self) -> &Self::Figure;
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card2SGen<F: FigureTrait,
    S: SuitTrait> {
    pub(crate) suit: S,
    pub(crate) figure: F
}
impl<F: FigureTrait, S: SuitTrait> Card2SymTrait for Card2SGen<F, S>{
    type Figure = F;

    type Suit = S;

    const CARD_SPACE: usize = Self::Figure::SYMBOL_SPACE * Self::Suit::SYMBOL_SPACE;

    fn suit(&self) -> &Self::Suit {
        &self.suit
    }

    fn figure(&self) -> &Self::Figure {
        &self.figure
    }

    

    
}

//pub type CardL<F, S> = Card<F, S>;

//impl<F:Figure, S:Suit> Card2S<F,S> for Card<F,S>{}

impl<F: FigureTrait + Copy, S: SuitTrait + Copy> Copy for Card2SGen<F, S>{}


impl<F: FigureTrait, S: SuitTrait> Card2SGen<F, S> {
    pub fn new(figure: F, suit: S) -> Self{
        Self{suit, figure}
    }
    /* 
    pub fn suit(&self) -> &S {
        &self.suit
    }
    pub fn figure(&self) -> &F {
        &self.figure
    }
    */

}

pub struct CardComparatorGen<F: FigureTrait, S: SuitTrait, CF: Comparator<F> + Default,  CS: Comparator<S> + Default> {
    pub suit_comparator: CS,
    pub figure_comparator: CF,
    pub _phantom: PhantomData<(F, S)>,

}

impl<F: FigureTrait, S: SuitTrait, CS: Comparator<S> + Default, CF: Comparator<F> + Default> Default
for CardComparatorGen<F, S, CF, CS>{
    fn default() -> Self {
        Self{suit_comparator: CS::default(), figure_comparator: CF::default(), _phantom: PhantomData::default()}
    }
}

impl<F: FigureTrait, S: SuitTrait, CS: Comparator<S> + Default, CF: Comparator<F> + Default>
CardComparatorGen<F, S, CF, CS>{

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
    pub fn cmp_suit_figure(&self, l: &Card2SGen<F, S>, r: &Card2SGen<F, S>) -> Ordering {
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
    pub fn cmp_figure_suit(&self, l: &Card2SGen<F, S>, r: &Card2SGen<F, S>) -> Ordering {
        match self.figure_comparator.compare(l.figure(), r.figure()){
            Ordering::Equal => self.suit_comparator.compare(l.suit(), r.suit() ),
            ordering => ordering
        }
    }
}






