use std::fmt::Debug;



pub trait Register<E>: Debug + Default{
    fn register(&mut self, element: E);
    fn unregister(&mut self, element: &E);
    fn is_registered(&self, element: &E) -> bool;
}