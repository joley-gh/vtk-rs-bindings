// Example: Trackball Actor interaction with snapping behavior (Rust adaptation)
use vtk_rs as vtk;
use std::sync::{ Arc, Mutex };

fn main() {
    println!(
        "Interactor Trackball Actor Demo - move the cube and try middle-click to test snapping"
    );

    // Colors
    let mut colors = vtk::NamedColors::new();

    // Sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_radius(2.0);

    let mut sphere_mapper = vtk::PolyDataMapper::new();
    sphere_mapper.set_input_connection(sphere_source.get_output_port());

    let mut sphere_actor = vtk::Actor::new();
    sphere_actor.set_mapper(&mut sphere_mapper);
    let mut sphere_prop = sphere_actor.get_property();
    sphere_prop.set_color(0.8, 0.2, 0.2);

    // Cube
    let mut cube_source = vtk::CubeSource::new();
    cube_source.set_center(5.0, 0.0, 0.0);

    let mut cube_mapper = vtk::PolyDataMapper::new();
    cube_mapper.set_input_connection(cube_source.get_output_port());

    let mut cube_actor = vtk::Actor::new();
    cube_actor.set_mapper(&mut cube_mapper);
    let mut cube_prop = cube_actor.get_property();
    cube_prop.set_color(0.2, 0.2, 0.8);

    // Renderer + window
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut sphere_actor);
    renderer.add_actor(&mut cube_actor);
    renderer.set_background(0.87, 0.72, 0.53);
    renderer.reset_camera();

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_window_name("Trackball Actor Demo");
    render_window.set_size(800, 600);

    // Interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Shared references for callbacks
    let renderer_shared = Arc::new(Mutex::new(renderer));

    // Create the trackball-actor style and register callbacks
    let mut style = vtk::InteractorStyleTrackballActor::new();

    // Middle button release: check distance between cube position and sphere center; snap if inside radius
    let renderer_for_cb = Arc::clone(&renderer_shared);
    style.set_middle_button_release_callback_with_actor(move |_x, _y, actor_opt| {
        // Sphere center is origin (0,0,0) in this example
        let sphere_center = (0.0_f64, 0.0_f64, 0.0_f64);
        let sphere_radius = 2.0_f64;

        if let Some(mut actor_ref) = actor_opt {
            let (x, y, z) = actor_ref.position();

            let dx = x - sphere_center.0;
            let dy = y - sphere_center.1;
            let dz = z - sphere_center.2;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            if dist <= sphere_radius + 0.0001 {
                println!("A point of the cube is inside the sphere — snapping back to origin");
                // Reset position and orientation
                actor_ref.set_position(0.0, 0.0, 0.0);
                actor_ref.set_orientation(0.0, 0.0, 0.0);

                // Reset camera and request render
                let mut renderer = renderer_for_cb.lock().unwrap();
                renderer.reset_camera();
            }
        }
    });

    // Assign style to interactor
    interactor.set_interactor_style_trackball_actor(&mut style);

    // Initialize and start
    interactor.initialize();
    render_window.render();

    println!("Window opened — try dragging the cube and releasing middle mouse button to snap it.");

    interactor.start();
}
