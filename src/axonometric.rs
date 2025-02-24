use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sign {
    Pos,
    Neg,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Octant {
    pub x: Sign,
    pub y: Sign,
    pub z: Sign,
}



impl Octant {
    pub fn project(self, coords: Vec3A) -> Vec2 {
        // because APPARENTLY I can't use sqrt in const...
        let PROJECTION: Mat3A = Mat3A::from_cols_array(&[
            3.0_f32.sqrt(), 1.0,  0.0,
            0.0,            2.0, 0.0,
           -3.0_f32.sqrt(), 1.0,  0.0,
       ]) / 6.0_f32.sqrt();
        // TODO: actually. use the octant lol
        let transformed = PROJECTION * coords;
        Vec2::new(
            transformed.x,
            transformed.y,
        )
    }
}
