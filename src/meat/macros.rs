#![feature(macro_metavar_expr)]

/// Format expressions into what level geometry expects.
/// I thought of it in the shower after having the terrible idea to use *rat*ionals.
/// Still accepts proper and improper fraction syntax (within limits).
macro_rules! rats {
    ($($t:tt),+) => {
        [
            $crate::meat::macros::_rats!($($t,)+)
        ].into()
    }
}

macro_rules! _rats {
    ($n:literal $punct:tt $($rest:tt)*) => {

    }
}

// https://stackoverflow.com/a/31749071/11047396
pub(crate) use rats;
pub(crate) use _rats;
