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

pub const ATARI_WHITE: Srgba = Srgba::new_opaque(0xec, 0xec, 0xec);
pub const ATARI_LIGHTGRAY: Srgba = Srgba::new_opaque(0xdc, 0xdc, 0xdc);
pub const ATARI_MIDGRAY: Srgba = Srgba::new_opaque(0xc8, 0xc8, 0xc8);
pub const ATARI_DARKGRAY: Srgba = Srgba::new_opaque(0xb0, 0xb0, 0xb0);

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
