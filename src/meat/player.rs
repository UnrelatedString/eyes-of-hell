use three_d::renderer::Camera;
use three_d::renderer::control::{Event, Key};
use three_d::core::prelude::*;
use functor_derive::Functor;
use num_traits::identities::One;
use std::f32::consts::PI;
use std::ops;
pub use Sign::*;

use crate::meat::input::*;

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

impl <S: One + ops::Neg<Output = S>> From<Sign> for S {
    fn from(sign: Sign) -> S {
        One::one() * sign
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Octant {
    q: Quadrant,
    vertical: Sign,
}

impl Octant {

    fn from_quadrant(q: Quadrant, vertical: Sign) -> Self {
        Octant { q, vertical }
    }

    fn cw(self) -> Self {
        Octant { q: self.q.cw(), vertical }
    }

    fn ccw(self) -> Self {
        Octant { q: self.q.ccw(), vertical }
    }

    fn x(self) -> Sign {
        match self.q {
            NE | NW => Pos,
            SE | SW => Neg
        }
    }

    fn y(self) -> Sign {
        self.vertical
    }

    fn z(self) -> Sign {
        match self.q {
            NE | SE => Pos,
            NW | SW => Neg
        }
    }

}

impl <S: One + ops::Neg<Output>> From<Octant> for Vector3<S> {
    fn from(oct: Octant) -> Vector3<S> {
        Vector3::new(
            oct.x().into(),
            oct.y().into(),
            oct.z().into(),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Quadrant {
    NE, // default 1,1,1 angle is NE; N is up-left = negative X
    NW,
    SW,
    SE,
}

impl Quadrant {

    fn cw(self) -> Self {
        use Quadrant::*;
        match self {
            NE => SE,
            NW => NE,
            SW => NW,
            SE => SW,
        }
    }

    fn ccw(self) -> Self {
        use Quadrant::*;
        match self {
            NE => NW,
            NW => SW,
            SW => SE,
            SE => NE,
        }
    }

    

    // flipping this for verticality would probably be more unintuitive anyways lol
    fn screen_up(self) -> Vec3 {

    }
}

// I wanted to originally have a struct Controls that was decoupled from
// other player state, but nahhh that would not be clean
struct Player {
    eye: Octant,
    pos: Vec3,
    wasd: Cardinals<KeyHoldState>,
}

impl Player {
    fn new() -> Player {
        Player {
            eye: Octant::from_quadrant(Quadrant::NE, Pos),
            pos: Vec3::new(0.0, 0.0, 0.0),
            wasd: Cardinals {
                up: KeyHoldState::new(Key::W),
                down: KeyHoldState::new(Key::S),
                left: KeyHoldState::new(Key::A),
                right: KeyHoldState::new(Key::D),
            }
        }
    }

    fn update(&mut self, event: &Event, delta: f32) {
        self.wasd.update(event);
        wasd: Cardinals<bool> = self.wasd.into();

        if was_pressed(Key::K, event) {
            if wasd.hardleft() {
                self.eye = self.eye.cw();  
            } else if wasd.hardright() {
                self.eye = self.eye.ccw();
            } else if wasd.harddown() {
                self.eye = self.eye.cw().cw();
            }
        } else {
            let mut velocity = Vec3::new(0.0, 0.0, 0.0);

            if wasd.hardup() {
                velocity += 
            }

            if !velocity.is_zero() {
                velocity = velocity.normalize();
            }
        }

    }
}
