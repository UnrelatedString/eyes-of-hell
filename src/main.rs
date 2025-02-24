use macroquad::prelude::*;
use functor_derive::Functor;

mod util;
use crate::util::{
    Quad,
    Drawable,
};

mod axonometric;
use crate::axonometric::Octant;
use crate::axonometric::Sign::*;

#[macroquad::main("EyesOfHell")]
async fn main() -> Result<(), ()> { // TODO: error type whenever I actually have errors
    loop {
        clear_background(BLACK);

        let center = Vec2::new(
            screen_width() / 2.0,
            screen_height() / 2.0,
        );

        // just, like, blindly assume a 4:3 aspect ratio for now,
        // and let shit clip off the sides if it's narrower than that
        let scale = screen_height() / 20.0;

        let o = Octant{x: Pos, y: Pos, z: Pos};
            
        Quad(
            Vec3A::new(-1.0, 0.0, 0.0),
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(0.0, 0.0, -1.0),
            Vec3A::new(-1.0, 0.0, -1.0),
        ).fmap(|v| o.project(v) * scale + center).draw_colored(WHITE);

        Quad(
            Vec3A::new(-1.0, 0.0, 0.0),
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(0.0, 1.0, 0.0),
            Vec3A::new(-1.0, 1.0, 0.0),
        ).fmap(|v| o.project(v) * scale + center).draw_colored(LIGHTGRAY);

        Quad(
            Vec3A::new(0.0, 0.0, -1.0),
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(0.0, 1.0, 0.0),
            Vec3A::new(0.0, 1.0, -1.0),
        ).fmap(|v| o.project(v) * scale + center).draw_colored(DARKGRAY);

        next_frame().await
    }
}

