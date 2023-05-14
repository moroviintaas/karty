#[macro_export]
macro_rules! f_proba {
    [ $( $x:expr ),* ] => {
        {
            let mut h = 0u64;
            $(
                 h |= $x.mask();
            )*
            $crate::hand::CardSet::from(h)

        }
    };
}