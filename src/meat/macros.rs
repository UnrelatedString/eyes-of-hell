#![feature(macro_metavar_expr)]

/// Format expressions into what level geometry expects.
/// I thought of it in the shower after having the terrible idea to use *rat*ionals.
/// Still accepts proper and improper fraction syntax (within limits).
macro_rules! rats {
    // https://veykril.github.io/tlborm/decl-macros/patterns/callbacks.html 😭
    ($($t:tt)*) => {
        $crate::meat::macros::_rats!(@cursor $($t)* ,)
    }
}

macro_rules! _rats {
    (
        $($processed:expr,)* @cursor
        $($wholel:literal)?$($wholei:ident)? +
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)? ,
        $($rest:tt)*
    ) => {
        $crate::meat::macros::_rats!( $($processed,)*
        $($wholel)?$($wholei)? as f32 +
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
        , @cursor $($rest)*)
    };

    (
        $($processed:expr,)* @cursor
        $($wholel:literal)?$($wholei:ident)? -
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)? ,
        $($rest:tt)*
    ) => {
        $crate::meat::macros::_rats!( $($processed,)*
        $($wholel)?$($wholei)? as f32 -
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
        , @cursor $($rest)*)
    };

    (
        $($processed:expr,)* @cursor
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)? ,
        $($rest:tt)*
    ) => {
        $crate::meat::macros::_rats!( $($processed,)*
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
        , @cursor $($rest)*)
    };

    (
        $($processed:expr,)* @cursor
        $wholel:literal ,
        $($rest:tt)*
    ) => {
        $crate::meat::macros::_rats!( $($processed,)*
        $wholel as f32
        , @cursor $($rest)*)
    };

    (
        $($processed:expr,)* @cursor
        $wholei:ident ,
        $($rest:tt)*
    ) => {
        $crate::meat::macros::_rats!( $($processed,)*
        $wholei as f32
        , @cursor $($rest)*)
    };

    ($($processed:expr,)* @cursor) => {
        $crate::meat::macros::vecn!($($processed),*)
    };
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
