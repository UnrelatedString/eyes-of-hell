use three_d::core::prelude::*;
use three_d::{
    Window,
    Camera,
    CpuMesh,
    Mesh,
    Gm,
    ColorMaterial,
    Srgba,
    ClearState,
};

// To be treated as the *actual* root module because of multiple entry point jank

mod geometry;
mod color;
mod player;
mod input;

use crate::meat::color::WHITE_CUBE;
use crate::meat::geometry::AAPrism;

const DISTANCE: f32 = 200.0;
const SCREEN_HEIGHT_WORLD_UNITS: f32 = 20.0;

pub async fn run() {
    let window = Window::new(three_d::WindowSettings {
        title: "Eyes of Hell".to_string(),
        min_size: (720, 480),
        initial_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    // TODO: keybinds

    let context = &window.gl();


    // TODO: way way past MVP, make a custom Viewer that can smoothly transition between
    // orthographic and perspective. just for shits and giggles
    let mut camera = Camera::new_orthographic(
        // ooooh what if I make the sides of things tinted by the left and right eyes
        window.viewport(),
        Vec3::new(1.0, 1.0, 1.0) * DISTANCE,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        SCREEN_HEIGHT_WORLD_UNITS / DISTANCE / 3.0_f32.sqrt(),
        0.0, // maybe make the z bounds by region and, like, automatically use the min/max from neighbors
        DISTANCE * 2.0,
    );

    let cube = AAPrism::new(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(2.0, 2.0, 2.0),
        WHITE_CUBE,
    ).gms(&context);

    window.render_loop (move |mut frame_input| {
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(&camera, &cube, &[]);

        Default::default()
    });
}
