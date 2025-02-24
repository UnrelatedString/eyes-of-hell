use macroquad::prelude::*;
use functor_derive::Functor;

// OOH OOH todo: occlusion by reified *full rhombus cube faces* because that's really all it needs
// could be pretty simple since hexagonal tiling is deceptively simple to begin with. all just half offsets and whatnot
// still allows for stuff that's finer grained than that to just possibly render behind stuff because that is no big deal and the tiling helps determine how they layer anyways. probably. actually no that sucks for like non grid snapped stuff never mind D:

pub trait Drawable {
    fn draw_colored(&self, color: Color) -> (); // this is too stupid for there to be any way this is what I actually do with other stuff that could impl this but uhhhh yeah
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Functor)]
pub struct Quad<T>(pub T, pub T, pub T, pub T);

impl Drawable for Quad<Vec2> {
    fn draw_colored(&self, color: Color) -> () {
        draw_triangle(self.0, self.1, self.3, color);
        draw_triangle(self.1, self.2, self.3, color);
    }
}
