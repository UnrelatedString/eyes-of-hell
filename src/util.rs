use three_d::renderer::Camera;
use functor_derive::Functor;
use std::f32::consts::PI;
use std::ops;
pub use Sign::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sign {
    Pos,
    Neg,
}

impl ops::Neg for Sign {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Pos => Neg,
            Neg => Pos,
        }
    }
}

impl <T: ops::Neg<Output = T>> ops::Mul<T> for Sign {
    type Output = T;
    fn mul(self, rhs: T) -> T {
        match self {
            Pos => rhs,
            Neg => -rhs,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Octant {
    pub x: Sign,
    pub y: Sign,
    pub z: Sign,
}

// impl Octant {
//     pub fn projection(self) -> Mat3A {
//         let alpha = self.y * (PI / 6.0).tan().asin();
//         let beta = match self {
//             Octant{x: Pos, z: Pos, ..} => 3.0,
//             Octant{x: Neg, z: Pos, ..} => 5.0,
//             Octant{x: Neg, z: Neg, ..} => 7.0,
//             Octant{x: Pos, z: Neg, ..} => 1.0,
//         };
//         Mat3A::from_euler(EulerRot::ZXY, 0.0, alpha, beta * PI / 4.0)
//     }

//     pub fn project(self, coords: Vec3A) -> Vec2 {
//         let transformed = self.projection() * coords;
//         Vec2::new(
//             transformed.x,
//             transformed.y,
//         )
//     }
// }

// 
pub trait Region {
    
}
