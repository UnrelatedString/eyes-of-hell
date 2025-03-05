#![feature(macro_metavar_expr)]

/// Format expressions into what level geometry expects.
/// I thought of it in the shower after having the terrible idea to use *rat*ionals.
/// Still accepts proper and improper fraction syntax (within limits).
macro_rules! rats {
    ($(
        $( // I feel like this approach won't scale to anything other than floats...
            $($plusl:literal)?
            $($plusi:ident)?
            +
        )?
        $(
            $($minusl:literal)?
            $($minusi:ident)?
            -
        )?
        $(
            $($dividel:literal)?
            $($dividei:ident)?
            /
        )?
            $($dl:literal)?
            $($di:ident)?
    ),+) => {
        $crate::meat::macros::vecn!( $(
            $( $($plusl)?$($plusi)? as f32 +)?
            $( $($minusl)?$($minusi)? as f32 +)?
            $( $($dividel)?$($dividei)? as f32 +)?
               $($dl)?$($di)? as f32
        ),+)
    }
}

/// Given a comma-separated list of exprs, calls vec2, vec3, or vec4 as appropriate.
macro_rules! vecn {
    ($x:expr, $y:expr) => { three_d::prelude::vec2($x, $y) };
    ($x:expr, $y:expr, $z:expr) => { three_d::prelude::vec3($x, $y, $z) };
    ($x:expr, $y:expr, $z:expr, $w:expr) => { three_d::prelude::vec4($x, $y, $z, $w) };
}

// https://stackoverflow.com/a/31749071/11047396
pub(crate) use rats;
pub(crate) use _rats;
pub(crate) use vecn;
