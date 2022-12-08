use std::ops::Index;
use crate::symbol::CardSymbol;

#[derive(Clone, Debug)]
pub struct SymbolMap<T, const SIZE: usize > where {
    pub associated: [T;SIZE]
}

impl<T, const SIZE: usize, S:CardSymbol > Index<S> for SymbolMap<T, SIZE>
where {
    type Output = T;

    fn index(&self, index: S) -> &Self::Output {
        &self.associated[index.position()]
    }
}