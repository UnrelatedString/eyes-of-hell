/// Format expressions into what level geometry expects.
/// I thought of it in the shower after having the terrible idea to use *rat*ionals.
/// Still accepts proper and improper fraction syntax (within limits).
macro_rules! rats {
    // https://veykril.github.io/tlborm/decl-macros/patterns/callbacks.html 😭
    ($($t:tt)*) => {
        $crate::meat::macros::_vecn!($($t)*) (
            $crate::meat::macros::_rats!($($t)*)
        )
    }
}

macro_rules! _rats {
    (
        $(,)?
        $($wholel:literal)?$($wholei:ident)? +
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $($wholel)?$($wholei)? as f32 +
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32 ,
        $crate::meat::macros::_rats!($($rest)*)
    };

    (
        $(,)?
        $($wholel:literal)?$($wholei:ident)? -
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $($wholel)?$($wholei)? as f32 -
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32 ,
        $crate::meat::macros::_rats!($($rest)*)
    };

    (
        $(,)?
        $($nl:literal)?$($ni:ident)? /
        $($dl:literal)?$($di:ident)?
        $($rest:tt)*
    ) => {
        $($nl)?$($ni)? as f32 /
        $($dl)?$($di)? as f32 ,
        $crate::meat::macros::_rats!($($rest)*)
    };

    (
        $(,)?
        $wholel:literal
        $($rest:tt)*
    ) => {
        $wholel as f32 ,
        $crate::meat::macros::_rats!($($rest)*)
    };

    (
        $(,)?
        $wholei:ident
        $($rest:tt)*
    ) => {
        $wholei as f32 ,
        $crate::meat::macros::_rats!($($rest)*)
    };

    ($(,)?) => {};
}

/// Given a comma-separated list of exprs, calls vec2, vec3, or vec4 as appropriate.
macro_rules! vecn {
    ($x:expr, $y:expr) => { three_d::prelude::vec2($x, $y) };
    ($x:expr, $y:expr, $z:expr) => { three_d::prelude::vec3($x, $y, $z) };
    ($x:expr, $y:expr, $z:expr, $w:expr) => { three_d::prelude::vec4($x, $y, $z, $w) };
}

/// Given a comma-separated list of exprs, selects vec2, vec3, or vec4 as appropriate,
/// with no call since vecn! can't take macros arguments itself.
macro_rules! _vecn {
    ($x:expr, $y:expr) => { three_d::prelude::vec2 };
    ($x:expr, $y:expr, $z:expr) => { three_d::prelude::vec3 };
    ($x:expr, $y:expr, $z:expr, $w:expr) => { three_d::prelude::vec4 };
}

// https://stackoverflow.com/a/31749071/11047396
pub(crate) use rats;
pub(crate) use _rats;
pub(crate) use vecn;
pub(crate) use _vecn;
