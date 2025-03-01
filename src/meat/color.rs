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

