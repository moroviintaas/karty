use std::cmp::Ordering;
use std::marker::PhantomData;
use comparator::Comparator;
use crate::symbol::CardSymbol;

pub struct SimpleSymbolComparator<S: CardSymbol>{
    _phantom_symbol: PhantomData<S>
}

impl<S: CardSymbol> Comparator<S> for SimpleSymbolComparator<S>{
    fn compare(&self, a: &S, b: &S) -> Ordering {
        a.position().cmp(&b.position())
    }
}

