use three_d::{
    ColorMaterial,
    Srgba,
};

#[derive(Copy, Clone, Debug)]
pub struct ThreeTonePalette {
    pub dark: Srgba,
    pub mid: Srgba,
    pub bright: Srgba,
}

#[derive(Copy, Clone, Debug)]
pub struct PrismFacePalette { // I feel like this is stupid and not even remotely going to survive into a polished finished product... with, like, textures... but for now it's good
    pub bottom: Srgba,
    pub ns: Srgba,
    pub ew: Srgba,
    pub top: Srgba,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NTSCAtariHue {
    Gray,
    Yellow,
    Brown,
    Peach,
    Red,
    Mulberry,
    Purple,
    Indigo,
    RoyalBlue,
    StonyBlue,
    Turquoise,
    Green,
    Chartreuse,
    GenuinelyWhatDoICallThisColor,
    Amber,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NTSCAtariLuminance {
    L0,
    L2,
    L4,
    L6,
    L8,
    La,
    Lc,
    Le,
}

// color values from https://www.biglist.com/lists/stella/archives/200109/msg00285.html
// also wait didn't I already do this like a month ago or did I give up or what
// includes alpha because maybe it could be convenient somehow and also it's just.
// that byte is there anyways so yeah may as well make the placeholder meaningful

const ATARI_PALETTE: [[u32;8];16] =
    [ [ 0x000000ff, 0x404040ff, 0x6c6c6cff, 0x909090ff
      , 0xb0b0b0ff, 0xc8c8c8ff, 0xdcdcdcff, 0xecececff ]
    , [ 0x444400ff, 0x646410ff, 0x848424ff, 0xa0a034ff
      , 0xb8b840ff, 0xd0d050ff, 0xe8e85cff, 0xfcfc68ff ]
    , [ 0x702800ff, 0x844414ff, 0x985c28ff, 0xac783cff
      , 0xbc8c4cff, 0xcca05cff, 0xdcb468ff, 0xecc878ff ]
    , [ 0x841800ff, 0x983418ff, 0xac5030ff, 0xc06848ff
      , 0xd0805cff, 0xe09470ff, 0xeca880ff, 0xfcbc94ff ]
    , [ 0x880000ff, 0x9c2020ff, 0xb03c3cff, 0xc05858ff
      , 0xd07070ff, 0xe08888ff, 0xeca0a0ff, 0xfcb4b4ff ]
    , [ 0x78005cff, 0x8c2074ff, 0xa03c88ff, 0xb0589cff
      , 0xc070b0ff, 0xd084c0ff, 0xdc9cd0ff, 0xecb0e0ff ]
    , [ 0x480078ff, 0x602090ff, 0x783ca4ff, 0x8c58b8ff
      , 0xa070ccff, 0xb484dcff, 0xc49cecff, 0xd4b0fcff ]
    , [ 0x140084ff, 0x302098ff, 0x4c3cacff, 0x6858c0ff
      , 0x7c70d0ff, 0x9488e0ff, 0xa8a0ecff, 0xbcb4fcff ]
    , [ 0x000088ff, 0x1c209cff, 0x3840b0ff, 0x505cc0ff
      , 0x6874d0ff, 0x7c8ce0ff, 0x90a4ecff, 0xa4b8fcff ]
    , [ 0x00187cff, 0x1c3890ff, 0x3854a8ff, 0x5070bcff
      , 0x6888ccff, 0x7c9cdcff, 0x90b4ecff, 0xa4c8fcff ]
    , [ 0x002c5cff, 0x1c4c78ff, 0x386890ff, 0x5084acff
      , 0x689cc0ff, 0x7cb4d4ff, 0x90cce8ff, 0xa4e0fcff ]
    , [ 0x003c2cff, 0x1c5c48ff, 0x387c64ff, 0x509c80ff
      , 0x68b494ff, 0x7cd0acff, 0x90e4c0ff, 0xa4fcd4ff ]
    , [ 0x003c00ff, 0x205c20ff, 0x407c40ff, 0x5c9c5cff
      , 0x74b474ff, 0x8cd08cff, 0xa4e4a4ff, 0xb8fcb8ff ]
    , [ 0x143800ff, 0x345c1cff, 0x507c38ff, 0x6c9850ff
      , 0x84b468ff, 0x9ccc7cff, 0xb4e490ff, 0xc8fca4ff ]
    , [ 0x2c3000ff, 0x4c501cff, 0x687034ff, 0x848c4cff
      , 0x9ca864ff, 0xb4c078ff, 0xccd488ff, 0xe0ec9cff ]
    , [ 0x442800ff, 0x644818ff, 0x846830ff, 0xa08444ff
      , 0xb89c58ff, 0xd0b46cff, 0xe8cc7cff, 0xfce08cff ]
    ];

const fn palette_material(hue: NTSCAtariHue, lum: NTSCAtariLuminance) -> Srgba {
    let rgba = ATARI_PALETTE[hue as usize][lum as usize].to_be_bytes();
    Srgba::new_opaque(rgba[0], rgba[1], rgba[2])
}

use NTSCAtariHue::*;
use NTSCAtariLuminance::*;

pub const ATARI_WHITE: Srgba = palette_material(Gray, Le);
pub const ATARI_LIGHTGRAY: Srgba = palette_material(Gray, Lc);
pub const ATARI_MIDGRAY: Srgba = palette_material(Gray, La);
pub const ATARI_DARKGRAY: Srgba = palette_material(Gray, L8);

pub const WHITE_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_DARKGRAY,
    ns: ATARI_MIDGRAY,
    ew: ATARI_LIGHTGRAY,
    top: ATARI_WHITE,
};

pub const ATARI_LIGHTPINK: Srgba = Srgba::new_opaque(0xec, 0xb0, 0xe0);
pub const ATARI_MIDPINK1: Srgba = Srgba::new_opaque(0xdc, 0x9c, 0xd0);
pub const ATARI_MIDPINK2: Srgba = Srgba::new_opaque(0xd0, 0x84, 0xc0);
pub const ATARI_MIDPINK3: Srgba = Srgba::new_opaque(0xc0, 0x70, 0xb0);

pub const PINK_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_MIDPINK3,
    ns: ATARI_MIDPINK2,
    ew: ATARI_MIDPINK1,
    top: ATARI_LIGHTPINK,
};

pub const ATARI_LIGHTAMBER: Srgba = Srgba::new_opaque(0xfc, 0xe0, 0x8c);
pub const ATARI_MIDAMBER: Srgba = Srgba::new_opaque(0xe8, 0xcc, 0x7c);
pub const ATARI_DARKAMBER: Srgba = Srgba::new_opaque(0xd0, 0xb4, 0x6c);
pub const ATARI_NOTREALLYAMBER: Srgba = Srgba::new_opaque(0xb8, 0x9c, 0x58);

pub const AMBER_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_NOTREALLYAMBER,
    ns: ATARI_DARKAMBER,
    ew: ATARI_MIDAMBER,
    top: ATARI_LIGHTAMBER,
};
