use three_d::core::prelude::*;
use three_d::{
    Window,
    WindowSettings,
    WindowError,
    Camera,
    Srgba,
    ClearState,
};

// To be treated as the *actual* root module because of multiple entry point jank

mod geometry;
mod color;
mod player;
mod input;

use crate::meat::color::{ WHITE_CUBE, PINK_CUBE, AMBER_CUBE };
use crate::meat::geometry:: { AAPrism, AAPrismMeshes, pain, rats };
use crate::meat::player::Player;

/**

== IMPORTANT ==

UP IS +Y
N IS +Z
E IS +X

YES THIS IS KIND OF BACKWARDS BUT I WANT TO KEEP THE NE QUADRANT POSITIVE
AND THEN I CAN JUST MAKE THE SW QUADRANT THE ACTUAL "MAIN" ONE SO NE IS AT THE TOP OF THE ROSE

*/

const DISTANCE: f32 = 200.0;
const SCREEN_HEIGHT_WORLD_UNITS: f32 = 20.0;
const UP_VEC: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub async fn run(window_defaults: WindowSettings) -> Result<(), WindowError> {
    let window = Window::new(WindowSettings {
        title: "Eyes of Hell".to_string(),
        min_size: (720, 480),
        ..window_defaults
    })?;

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
        [rats![-1, 0, -1],
        rats![2, 2, 2]],
        WHITE_CUBE,
    ).gms(&context);

    let up = pain(
        Mat4::from_translation(UP_VEC * 3.0) * Mat4::from_angle_x(Rad::turn_div_4()),
        &context,
        Srgba::RED,
        false,
    );

    let fakeup = pain(
        Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0) * 3.0) * Mat4::from_translation(UP_VEC * 3.0) * Mat4::from_angle_x(Rad::turn_div_4()),
        &context,
        Srgba::WHITE,
        false
    );

    let east = AAPrism::new(
        [rats![-1/2, 0, -3 - 1/2],
        Vec3::new(1.0, 1.0, 1.0)],
        PINK_CUBE,
    ).gms(&context);

    let big_floor = AAPrism::new(
        [rats![-5, -1, -5],
        rats![10, 1, 10]],
        AMBER_CUBE,
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
        let bod = AAPrismMeshes::new(
            Vec3::new(pwidth/2.0, pheight, pwidth/2.0) + player.pos,
            Vec3::new(pwidth, pheight, pwidth),
            PINK_CUBE,
        ).gms(&context);


        let screen = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0));

        screen.render(&camera, &big_floor, &[]);

        screen.render(&camera, &cube, &[]);
        screen.render(&camera, &up, &[]);
        screen.render(&camera, &fakeup, &[]);
        screen.render(&camera, &east, &[]);
        screen.render(&camera, &bod, &[]);

        Default::default()
    });

    Ok(())
}
