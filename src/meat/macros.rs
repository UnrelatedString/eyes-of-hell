#![feature(macro_metavar_expr)]

/// Format expressions into what level geometry expects.
/// I thought of it in the shower after having the terrible idea to use *rat*ionals.
/// Still accepts proper and improper fraction syntax (within limits).
macro_rules! rats {
    // https://veykril.github.io/tlborm/decl-macros/patterns/callbacks.html 😭
    ($($t:tt)*) => {
        $crate::meat::macros::_count_the_rats(
            ( $($t)* ) // duplicated so one can continue being passed as tt
            ( $($t)* ) // while the other can be parsed as a list of exprs >:3
        )
    }
}

macro_rules! _count_the_rats {
    (
        $($t:tt)*
        
    )
}

macro_rules! _erase {
    ($($t:tt)*) => ()
}

macro_rules! _rats {
    (
        $($silence:tt $(ats:tt)*)? @
        $($wholel:literal)?$($wholei:ident)? +
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $(${ignore($silence)} $crate::meat::macros::_erase!)? (
        $($wholel)?$($wholei)? as f32 +
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
        )
        $(${ignore($silence)}
            $crate::meat::macros::_rats
        )
    };

    (
        $($silence:tt $(ats:tt)*)? @
        $($wholel:literal)?$($wholei:ident)? -
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $($wholel)?$($wholei)? as f32 -
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
    };

    (
        $($silence:tt $(ats:tt)*)? @
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32
    };

    (
        $($silence:tt $(ats:tt)*)? @
        $wholel:literal
        $($rest:tt)*
    ) => {
        $wholel as f32
    };

    (
        $($silence:tt $(ats:tt)*)? @
        $wholei:ident
        $($rest:tt)*
    ) => {
        $wholei as f32
    };

    () => {};
}

/// Given a comma-separated list of exprs, calls vec2, vec3, or vec4 as appropriate.
macro_rules! vecn {
    ($x:expr, $y:expr) => { three_d::prelude::vec2($x, $y) };
    ($x:expr, $y:expr, $z:expr) => { three_d::prelude::vec3($x, $y, $z) };
    ($x:expr, $y:expr, $z:expr, $w:expr) => { three_d::prelude::vec4($x, $y, $z, $w) };
}

// https://stackoverflow.com/a/31749071/11047396
pub(crate) use rats;
pub(crate) use _count_the_rats;
pub(crate) use _erase;
pub(crate) use _rats;
pub(crate) use vecn;
