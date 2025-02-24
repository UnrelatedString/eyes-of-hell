use macroquad::prelude::*;
use std::f32::consts::PI;

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
    pub fn projection(self) -> Mat3A {
        let alpha = (PI / 6.0).tan().asin();
        Mat3A::from_euler(EulerRot::ZXY, 0.0, alpha, 3.0 * PI / 4.0)
    }

    pub fn project(self, coords: Vec3A) -> Vec2 {
        let transformed = self.projection() * coords;
        Vec2::new(
            transformed.x,
            transformed.y,
        )
    }
}
