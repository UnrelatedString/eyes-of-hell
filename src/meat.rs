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

pub async fn run() {
    let window = Window::new(three_d::WindowSettings {
        title: "Eyes of Hell".to_string(),
        min_size: (1280, 720),
        initial_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    // TODO: keybinds

    let context = &window.gl();

    let mut camera = Camera::new_orthographic(
        // ooooh what if I make the sides of things tinted by the left and right eyes
        window.viewport(),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        40.0,
    );

    let cube = Gm::new(
        Mesh::new(context, &CpuMesh::cube()),
        ColorMaterial {
            color: Srgba::BLUE,
            ..Default::default()
        },
    );

    // I HAVE NO IDEA WHAT I'M DOING AAAAAA
    let mut gui = three_d::GUI::new(context);
    let mut pos_scale = 1.0;
    let mut height = 20.0;
    let mut zn = 0.0;
    let mut zf = 40.0;

    window.render_loop (move |mut frame_input| {
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0));

        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui;
                use three_d::egui::Slider;
                egui::Window::new("camera shit").vscroll(true).show(gui_context, |ui| {
                    ui.add(Slider::new(&mut pos_scale, 0.0..=10.0).text("position scale"));
                    ui.add(Slider::new(&mut height, -20.0..=40.0).text("height"));
                    ui.add(Slider::new(&mut zn, 0.0..=40.0).text("z near"));
                    ui.add(Slider::new(&mut zf, 0.0..=40.0).text("z far"));
                });
            }
        );

        camera.set_orthographic_projection(height, zn, zf);
        camera.set_view(
            Vec3::new(1.0, 1.0, 1.0) * pos_scale,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );       

        frame_input.screen().render(&camera, &[&cube], &[]).write(|| gui.render());

        Default::default()
    });
}
