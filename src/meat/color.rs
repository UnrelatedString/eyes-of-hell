use three_d::{
    ColorMaterial,
    Srgba,
    RenderStates,
    WriteMask,
    Blend,
};

#[derive(Copy, Clone, Debug)]
pub struct Color(pub Srgba);

impl Color {
    pub const fn new(hex: u32) -> Color {
        let [blue, green, red, high] = hex.to_le_bytes();
        assert!(high == 0);
        Color(Srgba::new_opaque(
            red,
            green,
            blue,
        ))
    }

    pub fn opaque_material(&self) -> ColorMaterial {
        ColorMaterial {
            color: self.0,
            texture: None,
            render_states: 
                RenderStates {
                    blend: Blend::TRANSPARENCY,
                    write_mask: WriteMask::COLOR_AND_DEPTH,
                    ..Default::default()
                },
            is_transparent: true,
        }
    }

    pub fn transparent_material(&self) -> ColorMaterial {
        ColorMaterial {
            color: self.0,
            texture: None,
            render_states: 
                RenderStates {
                    blend: Blend::TRANSPARENCY,
                    write_mask: WriteMask::COLOR,
                    ..Default::default()
                },
            is_transparent: true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ThreeTonePalette {
    pub dark: Color,
    pub mid: Color,
    pub bright: Color,
}

#[derive(Copy, Clone, Debug)]
pub struct PrismFacePalette { // I feel like this is stupid and not even remotely going to survive into a polished finished product... with, like, textures... but for now it's good
    pub bottom: Color,
    pub ns: Color,
    pub ew: Color,
    pub top: Color,
}

pub const ATARI_WHITE: Color = Color::new(0xececec);
pub const ATARI_LIGHTGRAY: Color = Color::new(0xdcdcdc);
pub const ATARI_MIDGRAY: Color = Color::new(0xc8c8c8);
pub const ATARI_DARKGRAY: Color = Color::new(0xb0b0b0);

pub const WHITE_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_DARKGRAY,
    ns: ATARI_MIDGRAY,
    ew: ATARI_LIGHTGRAY,
    top: ATARI_WHITE,
};

pub const ATARI_LIGHTPINK: Color = Color::new(0xecb0e0);
pub const ATARI_MIDPINK1: Color = Color::new(0xdc9cd0);
pub const ATARI_MIDPINK2: Color = Color::new(0xd084c0);
pub const ATARI_MIDPINK3: Color = Color::new(0xc070b0);

pub const PINK_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_MIDPINK3,
    ns: ATARI_MIDPINK2,
    ew: ATARI_MIDPINK1,
    top: ATARI_LIGHTPINK,
};

pub const ATARI_LIGHTAMBER: Color = Color::new(0xfce08c);
pub const ATARI_MIDAMBER: Color = Color::new(0xe8cc7c);
pub const ATARI_DARKAMBER: Color = Color::new(0xd0b46c);
pub const ATARI_NOTREALLYAMBER: Color = Color::new(0xb89c58);

pub const AMBER_CUBE: PrismFacePalette = PrismFacePalette {
    bottom: ATARI_NOTREALLYAMBER,
    ns: ATARI_DARKAMBER,
    ew: ATARI_MIDAMBER,
    top: ATARI_LIGHTAMBER,
};
