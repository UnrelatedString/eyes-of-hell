use macroquad::prelude::*;
use std::f32::consts::PI;

mod util;
use crate::util::{
    draw_quad, // ironic
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

        draw_quad(
            o.project(Vec3A::new(-1.0, 0.0, 0.0)) * scale + center,
            o.project(Vec3A::new(0.0, 0.0, 0.0)) * scale + center,
            o.project(Vec3A::new(0.0, 0.0, -1.0)) * scale + center,
            o.project(Vec3A::new(-1.0, 0.0, -1.0)) * scale + center,
            WHITE
        );

        draw_quad(
            o.project(Vec3A::new(0.0, 0.0, 0.0)) * scale + center,
            o.project(Vec3A::new(-1.0, 0.0, 0.0)) * scale + center,
            o.project(Vec3A::new(-1.0, 1.0, 0.0)) * scale + center,
            o.project(Vec3A::new(0.0, 1.0, 0.0)) * scale + center,
            LIGHTGRAY
        );

        draw_quad(
            o.project(Vec3A::new(0.0, 0.0, 0.0)) * scale + center,
            o.project(Vec3A::new(0.0, 0.0, -1.0)) * scale + center,
            o.project(Vec3A::new(0.0, 1.0, -1.0)) * scale + center,
            o.project(Vec3A::new(0.0, 1.0, 0.0)) * scale + center,
            DARKGRAY
        );

        next_frame().await
    }
}

