
pub trait Register<E>:  Default{
    fn register(&mut self, element: E);
    fn unregister(&mut self, element: &E);
    fn is_registered(&self, element: &E) -> bool;
}