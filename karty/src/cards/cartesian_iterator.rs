use itertools::{Itertools, Product};
use crate::cards::{Card2SGen, Card2SymTrait};
use crate::figures::FigureTrait;
use crate::suits::SuitTrait;

/// Cartesian iterator for Card2SGen Subset
/// ```
/// use karty::cards::{ACE_HEARTS, ACE_SPADES, Card, Card2SGenSubset, KING_HEARTS, KING_SPADES, QUEEN_HEARTS, QUEEN_SPADES};
/// use karty::figures::{Ace, King, Queen};
/// use karty::suits::Suit::{Hearts, Spades};
/// let cards_iterated: Vec<Card> = Card2SGenSubset::new(vec![Ace, King, Queen], vec![Spades, Hearts]).into_iter().collect();
/// assert_eq!(cards_iterated, vec![ACE_SPADES, KING_SPADES, QUEEN_SPADES, ACE_HEARTS, KING_HEARTS, QUEEN_HEARTS ]);
/// ```
pub struct Card2SGenSubset<
    IFigure: IntoIterator,
    ISuit: IntoIterator>{
    product: Product< ISuit::IntoIter, IFigure::IntoIter>
}

impl<F: FigureTrait, S: SuitTrait, IFigure: IntoIterator<Item = F>, ISuit: IntoIterator<Item = S>>
Card2SGenSubset<IFigure, ISuit>  {

    pub fn new(figures: IFigure, suits: ISuit) -> Self
    where <IFigure as IntoIterator>::IntoIter: Clone{
        let it = suits.into_iter();
        Self{product: it.cartesian_product(figures.into_iter())}
        //Self{product: Product::cartesian_product(suits.into_iter(), figures.into_iter())}
    }

}

impl <F: FigureTrait + Copy, S: SuitTrait + Copy, IFigure: IntoIterator<Item = F>, ISuit: IntoIterator<Item = S>>
Iterator for Card2SGenSubset<IFigure, ISuit>
where <IFigure as IntoIterator>::IntoIter: Clone{
    type Item = Card2SGen<F, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.product.next().map(|(suit, figure)| Card2SGen::from_figure_and_suit(figure, suit) )

    }
}