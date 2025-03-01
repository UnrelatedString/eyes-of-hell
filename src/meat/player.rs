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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Functor)]
pub struct Cardinals<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputDirection {
    LeftDown,
    Down,
    RightDown,
    Left,
    Neutral,
    Right,
    LeftUp,
    Up,
    RightUp,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Octant {
    pub x: Sign,
    pub y: Sign,
    pub z: Sign,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Quadrant {
    NE, // default 1,1,1 angle is NE; N is up-left = negative X
    NW,
    SW,
    SE,
}

impl Quadrant {
    pub fn y(self, vertical: Sign) -> Octant {
        use Quadrant::*;
        match self {
            NE => Octant{ x: Pos, y: vertical, z: Pos },
            NW => Octant{ x: Pos, y: vertical, z: Neg },
            SW => Octant{ x: Neg, y: vertical, z: Neg },
            SE => Octant{ x: Neg, y: vertical, z: Pos },
        }
    }
}
