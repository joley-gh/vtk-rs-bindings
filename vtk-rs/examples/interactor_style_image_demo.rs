// Minimal example for vtk_interactor_style_image
use vtk_rs as vtk;
use std::sync::{ Arc, Mutex };

fn main() {
    println!("Interactor Image Demo - try left/right/middle clicks and keys (c: change color)");

    // Create a sphere so clicks are visible
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_radius(5.0);
    sphere_source.set_theta_resolution(24);
    sphere_source.set_phi_resolution(24);

    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());

    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);
    let mut prop = actor.get_property();
    prop.set_color(0.2, 0.6, 0.8);
    prop.set_representation(vtk::RepresentationType::Surface);

    // Renderer + window
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.12);
    renderer.reset_camera();

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(900, 700);
    render_window.set_window_name("Interactor Style Image Demo");

    // Interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Use Arc<Mutex<>> to share renderer state with callbacks
    let renderer_shared = Arc::new(Mutex::new(renderer));

    // Create the image interactor style and register callbacks
    let mut style = vtk::InteractorStyleImage::new();

    // Left click: toggle renderer background color for visual feedback
    let renderer_left = Arc::clone(&renderer_shared);
    style.set_left_button_press_callback(move |x, y| {
        println!("Left press at ({}, {})", x, y);
        let mut renderer = renderer_left.lock().unwrap();
        // simple toggle between two background presets
        renderer.set_background(0.05, 0.05, 0.08);
    });

    // Right click: print and rotate camera a little
    let renderer_right = Arc::clone(&renderer_shared);
    style.set_right_button_press_callback(move |x, y| {
        println!("Right press at ({}, {}) — rotate camera", x, y);
        let mut renderer = renderer_right.lock().unwrap();
        renderer.get_active_camera().azimuth(10.0);
        renderer.reset_camera();
    });

    // Middle click: pan camera
    let renderer_mid = Arc::clone(&renderer_shared);
    style.set_middle_button_press_callback(move |x, y| {
        println!("Middle press at ({}, {}) — panning (simulated)", x, y);
        let mut renderer = renderer_mid.lock().unwrap();
        renderer.get_active_camera().elevation(-5.0);
        renderer.reset_camera();
    });

    // Mouse move: print coordinates (note: frequent)
    style.set_mouse_move_callback(|x, y| {
        // Keep output lightweight
        if x % 50 == 0 && y % 50 == 0 {
            println!("Mouse move at ({}, {})", x, y);
        }
    });

    // Key press: toggle renderer background or simulate a window/level action
    let renderer_key = Arc::clone(&renderer_shared);
    style.set_key_press_callback(move |key| {
        println!("Key pressed: {}", key);
        match key {
            "c" | "C" => {
                let mut renderer = renderer_key.lock().unwrap();
                renderer.set_background(0.8, 0.2, 0.2);
                renderer.reset_camera();
                println!("Changed renderer background via key");
                // Consume the event so VTK doesn't apply its default behavior
                true
            }
            "w" | "W" => {
                // Simulate a window/level-like visual change by zooming the camera
                let mut renderer = renderer_key.lock().unwrap();
                renderer.get_active_camera().zoom(1.1);
                renderer.reset_camera();
                println!("Simulated window/level via camera zoom");
                // Consume to prevent default wireframe toggle
                true
            }
            _ => {
                // Not handled here; allow default behavior
                false
            }
        }
    });

    // Give interactor the style (we need a mutable reference to the style we registered on)
    interactor.set_interactor_style_image(&mut style);

    // Actor was already added to the renderer earlier; no further action required.

    // Initialize and start
    interactor.initialize();
    render_window.render();

    println!("Window opened. Try left/right/middle clicks and press 'c' or 'w' keys.");

    interactor.start();
}
