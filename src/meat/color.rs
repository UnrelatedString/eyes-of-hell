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
