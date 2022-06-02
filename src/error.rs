use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CardError{
    WrongSuitPosition(usize),
    WrongFigurePosition(usize)
}
impl Display for CardError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}