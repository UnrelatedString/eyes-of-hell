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

// mod util;
// use crate::util::{
//     Quad,
//     Drawable,
// };

// what if. Mystic Eyes of Depth Perception

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

pub async fn run() {
    let window = Window::new(three_d::WindowSettings {
        title: "Eyes of Hell".to_string(),
        min_size: (1280, 720),
        initial_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    // TODO: keybinds

    let camera = Camera::new_orthographic(
        // ooooh what if I make the sides of things tinted by the left and right eyes
        window.viewport(),
        Vec3::new(1.0, 1.0, 1.0)*2.0, // just trying stuff to see what actually. does the stuff I want at all
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        40.0,
    );

    let cube = Gm::new(
        Mesh::new(&window.gl(), &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::BLUE,
            ..Default::default()
        },
    );

    window.render_loop (move |frame_input| {
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(&camera, &[&cube], &[]);

        Default::default()
    });
}
