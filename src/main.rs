use macroquad::prelude::*;

mod util;
use util::{
    draw_quad, // ironic
};

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
        let scale = screen_height() / 40.0;

        // traditional dimetric

        let left = Vec2::new(
            -4.0,
            0.0,
        ) * scale + center;

        draw_quad(
            Vec2::new(0.0, 1.0) * scale + left,
            Vec2::new(2.0, 0.0) * scale + left,
            Vec2::new(0.0, -1.0) * scale + left,
            Vec2::new(-2.0, 0.0) * scale + left,
            WHITE
        );

        // true isometric (hopefully)

        // ...

        next_frame().await
    }
}

