use three_d::renderer::{ Camera, Viewer, Light };
use three_d::renderer::control::{Event, Key};
use three_d::renderer::object::Object;
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

impl From<Sign> for f32 {
    fn from(sign: Sign) -> f32 {
        sign * 1.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Octant {
    q: Quadrant,
    vertical: Sign,
}

impl Octant {

    pub fn from_quadrant(q: Quadrant, vertical: Sign) -> Self {
        Octant { q, vertical }
    }

    pub fn cw(self) -> Self {
        Octant { q: self.q.cw(), ..self }
    }

    pub fn ccw(self) -> Self {
        Octant { q: self.q.ccw(), ..self }
    }

    pub fn x(self) -> Sign {
        self.q.x()
    }

    pub fn y(self) -> Sign {
        self.vertical
    }

    pub fn z(self) -> Sign {
        self.q.z()
    }

    pub fn quadrant(self) -> Quadrant {
        self.q
    }

}

impl From<Octant> for Vec3 {
    fn from(oct: Octant) -> Vec3 {
        Vec3::new(
            oct.x().into(),
            oct.y().into(),
            oct.z().into(),
        )
    }
}

impl <T: Copy + ops::Neg<Output = T>> ops::Mul<T> for Octant {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3::<T>::new(
            self.x() * rhs,
            self.y() * rhs,
            self.x() * rhs,
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

    pub fn cw(self) -> Self {
        use Quadrant::*;
        match self {
            NE => SE,
            NW => NE,
            SW => NW,
            SE => SW,
        }
    }

    pub fn ccw(self) -> Self {
        use Quadrant::*;
        match self {
            NE => NW,
            NW => SW,
            SW => SE,
            SE => NE,
        }
    }

    pub fn x(self) -> Sign {
        use Quadrant::*;
        match self {
            NE | NW => Pos,
            SE | SW => Neg
        }
    }

    pub fn z(self) -> Sign {
        use Quadrant::*;
        match self {
            NE | SE => Pos,
            NW | SW => Neg
        }
    }

    pub fn input_to_spatial(self, input: Vec2) -> Vec3 {
        // just bullshitting this for now lol
        let raw = Vec3::new(
            self.x() * input.x - self.z() * input.y,
            0.0,
            self.z() * input.y - self.x() * input.x,
        );
        if raw.is_zero() {
            raw
        } else {
            raw.normalize()
        }
    }
}

// I wanted to originally have a struct Controls that was decoupled from
// other player state, but nahhh that would not be clean
#[derive(Debug)]
pub struct Player {
    pub eye: Octant,
    pub pos: Vec3,
    wasd: Cardinals<KeyHoldState>,
    velocity2: Vec2,
}

impl Player {
    // Units per *milli*second!
    const SPEED: f32 = 0.005;

    pub fn new() -> Player {
        Player {
            eye: Octant::from_quadrant(Quadrant::NE, Pos),
            pos: Vec3::new(0.0, 0.0, 0.0),
            wasd: Cardinals {
                up: KeyHoldState::new(Key::W),
                down: KeyHoldState::new(Key::S),
                left: KeyHoldState::new(Key::A),
                right: KeyHoldState::new(Key::D),
            },
            velocity2: Vec2::zero(),
        }
    }

    pub fn update(&mut self, event: &Event) {
        self.velocity2 = Vec2::new(0.0, 0.0);
        self.wasd.update(event);
        let wasd: Cardinals<bool> = (&self.wasd).into();

        if was_pressed(event, Key::K) {
            if wasd.hardleft() {
                self.eye = self.eye.cw();  
            } else if wasd.hardright() {
                self.eye = self.eye.ccw();
            } else if wasd.harddown() {
                self.eye = self.eye.cw().cw();
            }
            println!("{:?} {:?}{:?}{:?}", self.eye, self.eye.x(), self.eye.y(),self.eye.z());
            println!("{:?}", self.eye * 1.0);
        } else {
            if wasd.hardup() {
                self.velocity2 -= Vec2::unit_y();
            }
            if wasd.harddown() {
                self.velocity2 += Vec2::unit_y();
            }
            if wasd.hardleft() {
                self.velocity2 -= Vec2::unit_x();
            }
            if wasd.hardright() {
                self.velocity2 += Vec2::unit_x();
            }
        }
    }

    pub fn tick(&mut self, delta: f64) {
        self.pos += self.eye.quadrant().input_to_spatial(self.velocity2) * (delta as f32) * Player::SPEED;
    }
}

// impl Object for Player {
//     fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) -> () {

//     }
// }
