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
    Context,
};

// To be treated as the *actual* root module because of multiple entry point jank

mod geometry;
mod color;
mod player;
mod input;

use crate::meat::color::{ WHITE_CUBE, PINK_CUBE };
use crate::meat::geometry::AAPrism;
use crate::meat::player::Player;

const DISTANCE: f32 = 200.0;
const SCREEN_HEIGHT_WORLD_UNITS: f32 = 20.0;
const UP_VEC: Vec3 = Vec3::new(0.0, -1.0, 0.0);

pub async fn run() {
    let window = Window::new(three_d::WindowSettings {
        title: "Eyes of Hell".to_string(),
        min_size: (720, 480),
        initial_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    // TODO: keybinds

    let context = window.gl();


    // TODO: way way past MVP, make a custom Viewer that can smoothly transition between
    // orthographic and perspective. just for shits and giggles
    let mut camera = Camera::new_orthographic(
        // ooooh what if I make the sides of things tinted by the left and right eyes
        window.viewport(),
        Vec3::new(1.0, 1.0, 1.0) * DISTANCE,
        Vec3::new(0.0, 0.0, 0.0),
        UP_VEC,
        SCREEN_HEIGHT_WORLD_UNITS / DISTANCE / 3.0_f32.sqrt(),
        0.0, // maybe make the z bounds by region and, like, automatically use the min/max from neighbors
        DISTANCE * 2.0,
    );

    let cube = AAPrism::new(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(2.0, 2.0, 2.0),
        WHITE_CUBE,
    ).gms(&context);

    let north = AAPrism::new(
        Vec3::new(-3.5, -0.5, -0.5),
        Vec3::new(1.0, 1.0, 1.0),
        PINK_CUBE,
    ).gms(&context);

    let east = AAPrism::new(
        Vec3::new(-0.5, -0.5, -3.5),
        Vec3::new(1.0, 1.0, 1.0),
        PINK_CUBE,
    ).gms(&context);

    let mut player = Player::new();

    window.render_loop (move |frame_input| {

        for event in &frame_input.events {
            player.update(event);
        }

        player.tick(frame_input.elapsed_time);

        camera.set_view(
            player.pos + player.eye * DISTANCE,
            player.pos,
            UP_VEC,
        );

        let pwidth = 0.2;
        let pheight = 0.4;
        let bod = AAPrism::new(
            Vec3::new(pwidth/2.0, pheight, pwidth/2.0) + player.pos,
            Vec3::new(pwidth, pheight, pwidth),
            PINK_CUBE,
        ).gms(&context);


        let screen = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0));

        screen.render(&camera, &cube, &[]);
        screen.render(&camera, &north, &[]);
        screen.render(&camera, &east, &[]);
        screen.render(&camera, &bod, &[]);

        Default::default()
    });
}
