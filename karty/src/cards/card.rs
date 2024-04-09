use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use comparator::Comparator;
use num_integer::div_rem;
use crate::figures::{FigureTrait};
use crate::suits::{ SuitTrait};
use crate::symbol::{CardSymbol};
#[cfg(feature = "speedy")]
use speedy::{Readable, Writable};
use crate::cards::Card2SGenSubset;
use crate::error::CardError;

pub trait Card2S<F: FigureTrait, S: SuitTrait>{
    const CARD_SPACE: usize = F::SYMBOL_SPACE * S::SYMBOL_SPACE;
}

pub trait Card2SymTrait: Debug + Clone + Eq + CardSymbol{
    type Figure: FigureTrait;
    type Suit: SuitTrait;
    const CARD_SPACE: usize = Self::Figure::SYMBOL_SPACE * Self::Suit::SYMBOL_SPACE;
    fn suit(&self) -> Self::Suit;
    fn figure(&self) -> Self::Figure;
    fn compare_suit_first(&self, other: &Self) -> Ordering{
        match self.suit().cmp(&other.suit()){
            Ordering::Equal => {
              self.figure().cmp(&other.figure())
            },
            ord => ord
        }
    }
    fn compare_figure_first(&self, other: &Self) -> Ordering{
        match self.figure().cmp(&other.figure()){
            Ordering::Equal => {
              self.suit().cmp(&other.suit())
            },
            ord => ord
        }
    }
    fn from_figure_and_suit(figure: Self::Figure, suit: Self::Suit) -> Self;

    /// ```
    /// use karty::cards::{ACE_HEARTS, ACE_SPADES, Card, Card2SGenSubset, Card2SymTrait, KING_HEARTS, KING_SPADES, QUEEN_HEARTS, QUEEN_SPADES};
    /// use karty::figures::{Ace, King, Queen};
    /// use karty::suits::Suit::{Hearts, Spades};
    /// let cards_iterated: Vec<Card> = Card::card_subset(vec![Ace, King, Queen], vec![Spades, Hearts]).into_iter().collect();
    /// assert_eq!(cards_iterated, vec![ACE_SPADES, KING_SPADES, QUEEN_SPADES, ACE_HEARTS, KING_HEARTS, QUEEN_HEARTS ]);
    /// ```
    /// ```
    /// use karty::cards::{Card, Card2SymTrait};
    /// use karty::figures::Figure;
    /// use karty::suits::Suit;
    /// use karty::symbol::CardSymbol;
    /// let all_cards: Vec<Card> = Card::card_subset(Figure::iterator(), Suit::iterator()).collect();
    /// assert_eq!(all_cards.len(), 52);
    /// ```
    fn card_subset<IFigure: IntoIterator<Item=Self::Figure>,
        ISuit: IntoIterator<Item=Self::Suit>>(figures: IFigure, suits: ISuit) -> Card2SGenSubset<IFigure, ISuit>
        where <IFigure as IntoIterator>::IntoIter: Clone{
        Card2SGenSubset::new(figures, suits)
    }
}

/// Generic 2-symbol card - one named figure, second suit. Traits [`FigureTrait`](crate::figures::FigureTrait) and [`SuitTrait`](crate::suits::SuitTrait) can be are actually organising markers, as these traits do not add new restrictions for basic [`CardSymbol`](crate::symbol::CardSymbol).
/// # Example of creation:
/// ```
/// // First we create our SuitTrait type:
/// use std::cmp::Ordering;
/// use std::fmt::{Debug, Formatter};
/// use std::hash::{Hash, Hasher};
/// use karty::cards::{Card2SGen, Card2SymTrait};
/// use karty::error::CardError;
/// use karty::figures::FigureTrait;
/// use karty::suits::SuitTrait;
/// use karty::symbol::{CardSymbol};
/// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
/// pub enum Color{
///     Red, Blue, Green, Yellow
/// }
/// impl Color{
///     fn value(&self) -> u8{
///         match self{
///             Self::Red => 3,
///             Self::Blue => 2,
///             Self::Green => 1,
///             Self::Yellow => 0,
///         }
///     }
/// }
/// // Needed traits for Color
/// impl Ord for Color {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.value().cmp(&other.value())
///     }
/// }
/// assert!(Color::Red > Color::Blue);
/// impl PartialOrd<Self> for Color {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         Some(self.cmp(&other))
///     }
/// }
/// impl CardSymbol for Color {
///     const SYMBOL_SPACE: usize = 4;
///
///     fn usize_index(&self) -> usize {
///         self.value() as usize
///     }
///
///     fn from_usize_index(position: usize) -> Result<Self, CardError> {
///         match position{
///             3 => Ok(Self::Red),
///             2 => Ok(Self::Blue),
///             1 => Ok(Self::Green),
///             0 => Ok(Self::Yellow),
///             e => Err(CardError::WrongSuitPosition(e))
///         }
///     }
/// }
/// impl SuitTrait for Color{
///     const NUMBER_OF_SUITS: usize = Self::SYMBOL_SPACE;
/// }
///
/// //Now the same for figures
/// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
/// pub enum Machine{
///     Train, Bike, Car
/// }
///
/// impl Machine{
///     fn value(&self) -> u8{
///         match self{
///             Self::Train => 2,
///             Self::Bike => 1,
///             Self::Car => 0,
///         }
///     }
/// }
/// impl Ord for Machine {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.value().cmp(&other.value())
///     }
/// }
/// assert!(Machine::Bike > Machine::Car);
/// impl PartialOrd<Self> for Machine {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         Some(self.cmp(&other))
///     }
/// }
/// impl CardSymbol for Machine {
///     const SYMBOL_SPACE: usize = 3;
///
///     fn usize_index(&self) -> usize {
///         self.value() as usize
///     }
///
///     fn from_usize_index(position: usize) -> Result<Self, CardError> {
///         match position{
///             2 => Ok(Self::Train),
///             1 => Ok(Self::Bike),
///             0 => Ok(Self::Car),
///             e => Err(CardError::WrongFigurePosition(e))
///         }
///     }
/// }
/// impl FigureTrait for Machine{
///     const NUMBER_OF_FIGURES:  usize = Self::SYMBOL_SPACE;
/// }
///
/// type MyCard = Card2SGen<Machine, Color>;
/// // Now we have access to few automatically implemented methods:
/// let card1 = MyCard::from_usize_index(10).unwrap();
/// let card2 = MyCard::from_usize_index(5).unwrap();
/// assert_eq!(card1.figure(), Machine::Bike);
/// assert_eq!(card1.suit(), Color::Red);
/// assert_eq!(card2.figure(), Machine::Train);
/// assert_eq!(card2.suit(), Color::Green);
/// // For example compare suit first//////
/// let mut v: Vec<MyCard> = MyCard::iterator().collect();
/// v.sort_by(|a,b| a.compare_suit_first(&b));
/// assert_eq!(&card1, &v[10]);
/// assert_eq!(&card2, &v[5]);
/// v.sort_by(|a,b| a.compare_figure_first(&b));
/// assert_eq!(&card1, &v[7]);
/// assert_eq!(&card2, &v[9]);
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card2SGen<F: FigureTrait,
    S: SuitTrait> {
    pub(crate) suit: S,
    pub(crate) figure: F
}
impl<F: FigureTrait + Copy, S: SuitTrait + Copy> Card2SymTrait for Card2SGen<F, S>{
    type Figure = F;

    type Suit = S;

    const CARD_SPACE: usize = Self::Figure::SYMBOL_SPACE * Self::Suit::SYMBOL_SPACE;

    fn suit(&self) -> Self::Suit {
        self.suit
    }

    fn figure(&self) -> Self::Figure {
        self.figure
    }

    fn from_figure_and_suit(figure: Self::Figure, suit: Self::Suit) -> Self {
        Self{suit, figure}
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
        Self{suit_comparator: CS::default(), figure_comparator: CF::default(), _phantom: Default::default()}
    }
}

impl<F: FigureTrait + Copy, S: SuitTrait + Copy, CS: Comparator<S> + Default, CF: Comparator<F> + Default>
CardComparatorGen<F, S, CF, CS>{

/// ```
/// use karty::cards::{STANDARD_DECK};
/// use karty::figures::FigureComparator;
/// use karty::suits::ComparatorDCHS;
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
        match self.suit_comparator.compare(&l.suit(), &r.suit()){
            Ordering::Equal => self.figure_comparator.compare(&l.figure(), &r.figure() ),
            ordering => ordering
        }
    }

/// ```
/// use karty::cards::{STANDARD_DECK};
/// use karty::figures::FigureComparator;
/// use karty::suits::ComparatorDCHS;
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
        match self.figure_comparator.compare(&l.figure(), &r.figure()){
            Ordering::Equal => self.suit_comparator.compare(&l.suit(), &r.suit() ),
            ordering => ordering
        }
    }
}



impl<F: FigureTrait+ Copy, S: SuitTrait + Copy> CardSymbol for Card2SGen<F, S> {
    const SYMBOL_SPACE: usize = F::SYMBOL_SPACE * S::SYMBOL_SPACE;

    fn usize_index(&self) -> usize {
        (self.suit().usize_index() * F::SYMBOL_SPACE) + self.figure.usize_index()
    }

    fn from_usize_index(position: usize) -> Result<Self, CardError> {
        let (suit, figure) = div_rem(position, F::SYMBOL_SPACE);
        Ok(Self{figure: F::from_usize_index(figure)?, suit: S::from_usize_index(suit)?})
    }
    /*
    fn position(&self) -> usize {
        (self.figure.position() * S::SYMBOL_SPACE) + self.suit.position()
    }

    fn from_position(position: usize) -> Result<Self, CardError> {
        let (figure, suit) = div_rem(position, S::SYMBOL_SPACE);
        Ok(Self{figure: F::from_position(figure)?, suit: S::from_position(suit)?})
    }

     */
}





