use macroquad::prelude::*;
use std::f32::consts::PI;

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
            -8.0,
            0.0,
        ) * scale + center;

        draw_quad(
            Vec2::new(0.0, 1.0) * scale + left,
            Vec2::new(2.0, 0.0) * scale + left,
            Vec2::new(0.0, -1.0) * scale + left,
            Vec2::new(-2.0, 0.0) * scale + left,
            WHITE
        );

        draw_quad(
            Vec2::new(-2.0, 0.0) * scale + left,
            Vec2::new(0.0, 1.0) * scale + left,
            Vec2::new(0.0, 3.0) * scale + left,
            Vec2::new(-2.0, 2.0) * scale + left,
            LIGHTGRAY
        );

        draw_quad(
            Vec2::new(2.0, 0.0) * scale + left,
            Vec2::new(0.0, 1.0) * scale + left,
            Vec2::new(0.0, 3.0) * scale + left,
            Vec2::new(2.0, 2.0) * scale + left,
            DARKGRAY
        );

        // true isometric (hopefully)

        let rightish = Vec2::new(
            6.0,
            0.0,
        ) * scale + center;

        draw_hexagon(
            rightish.x,
            rightish.y,
            2.0 * scale,
            0.0,
            true,
            WHITE,
            WHITE
        );

        let right = Vec2::new(
            10.0,
            0.0,
        ) * scale + center;

        draw_quad(
            Vec2::new(0.0, (PI / 6.0).sin() * 4.0) * scale + right,
            Vec2::new((PI / 6.0).cos() * 2.0, 0.0) * scale + right,
            Vec2::new(0.0, -(PI / 6.0).sin() * 4.0) * scale + right,
            Vec2::new(-(PI / 6.0).cos() * 2.0, 0.0) * scale + right,
            WHITE
        );

        next_frame().await
    }
}

