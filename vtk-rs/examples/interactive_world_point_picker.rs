use vtk_rs::*;

/// Interactive WorldPointPicker Demo
///
/// This example demonstrates how to use WorldPointPicker with VTK event callbacks
/// to convert 2D screen clicks into 3D world coordinates for interactive object placement.
///
/// Key Components:
/// - WorldPointPicker: Converts screen coordinates (x, y) to 3D world coordinates
/// - InteractorStyleCustom: Provides event callbacks for mouse interactions
/// - Raw pointer handling: Uses usize casting for thread-safe callback access
///
/// Pattern:
/// 1. Store renderer/window pointers as usize (which is Send/Sync)
/// 2. In callback, convert back to raw pointers
/// 3. Use WorldPointPicker.pick() to get 3D position from 2D click
/// 4. Create and add new actors at the picked position
/// 5. Render to update the scene
///
/// Safety: This uses unsafe code with raw pointers. The pointers remain valid
/// because the renderer and window outlive the callback.

fn main() {
    println!("\n═══════════════════════════════════════════════════════");
    println!("Interactive WorldPointPicker Demo");
    println!("═══════════════════════════════════════════════════════\n");
    println!("Click anywhere in the 3D view to place spheres!");
    println!("The WorldPointPicker converts your 2D click to 3D world coordinates.\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.15);

    // Create a reference grid (plane) to show the ground
    let mut plane = PlaneSource::new();
    plane.set_origin(-5.0, -5.0, 0.0);
    plane.set_point1(5.0, -5.0, 0.0);
    plane.set_point2(-5.0, 5.0, 0.0);
    plane.set_x_resolution(10);
    plane.set_y_resolution(10);

    let mut plane_mapper = PolyDataMapper::new();
    plane_mapper.set_input_connection(plane.get_output_port());

    let mut plane_actor = Actor::new();
    plane_actor.set_mapper(&mut plane_mapper);
    let mut plane_prop = plane_actor.get_property();
    plane_prop.set_color(0.3, 0.3, 0.3);
    plane_prop.set_representation(RepresentationType::Wireframe);
    plane_prop.set_opacity(0.3);
    renderer.add_actor(&mut plane_actor);

    // Add reference objects
    let mut cone = ConeSource::new();
    cone.set_radius(0.5);
    cone.set_height(1.0);
    cone.set_resolution(16);

    let mut cone_mapper = PolyDataMapper::new();
    cone_mapper.set_input_connection(cone.get_output_port());

    let mut cone_actor = Actor::new();
    cone_actor.set_mapper(&mut cone_mapper);
    cone_actor.set_position(0.0, 0.0, 0.5);
    let mut cone_prop = cone_actor.get_property();
    cone_prop.set_color(0.8, 0.2, 0.2);
    renderer.add_actor(&mut cone_actor);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("Interactive WorldPointPicker - Click to Place Spheres");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Create custom style for event handling
    let mut style = InteractorStyleCustom::new();

    // Convert raw pointers to usize for Send/Sync
    let renderer_ptr = &mut renderer as *mut Renderer as usize;
    let render_window_ptr = &mut render_window as *mut RenderWindow as usize;

    println!("Setting up interactive click handler...");
    println!("Scene ready - click anywhere to place spheres!\n");

    // Set up click handler
    style.set_left_button_press_callback(move |x, y| {
        unsafe {
            println!("\n=== Click detected at screen position ({}, {}) ===", x, y);

            // Create WorldPointPicker
            let mut picker = WorldPointPicker::new();

            // Convert usize back to raw pointer
            let renderer = &mut *(renderer_ptr as *mut Renderer);

            // Convert screen coordinates to 3D world position
            let success = picker.pick(x as f64, y as f64, 0.0, renderer);

            if success {
                let (wx, wy, wz) = picker.get_pick_position();
                println!("  ✓ Pick successful!");
                println!("  World position: ({:.2}, {:.2}, {:.2})", wx, wy, wz);

                // Create a sphere at the picked position
                let sphere = Box::leak(Box::new(SphereSource::new()));
                sphere.set_radius(0.2);
                sphere.set_theta_resolution(16);
                sphere.set_phi_resolution(16);
                sphere.set_center([wx, wy, wz]);

                let mapper = Box::leak(Box::new(PolyDataMapper::new()));
                mapper.set_input_connection(sphere.get_output_port());

                let actor = Box::leak(Box::new(Actor::new()));
                actor.set_mapper(mapper);

                // Random color for each sphere
                use std::time::{ SystemTime, UNIX_EPOCH };
                let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
                let r = ((seed * 12.9898).sin() * 43758.5453).fract().abs();
                let g = ((seed * 78.233).sin() * 43758.5453).fract().abs();
                let b = ((seed * 45.164).sin() * 43758.5453).fract().abs();

                let mut prop = actor.get_property();
                prop.set_color(r, g, b);

                renderer.add_actor(actor);

                // Render the updated scene
                let render_window = &mut *(render_window_ptr as *mut RenderWindow);
                render_window.render();

                println!("  → Sphere placed with color ({:.2}, {:.2}, {:.2})", r, g, b);
            } else {
                println!("  ✗ Pick failed - coordinates may be outside view");
            }
        }
    });

    interactor.set_interactor_style_custom(&mut style);

    renderer.reset_camera();
    interactor.initialize();
    render_window.render();

    println!("═══════════════════════════════════════════════════════");
    println!("Instructions:");
    println!("═══════════════════════════════════════════════════════");
    println!("  • Left-click: Place a sphere at the 3D location");
    println!("  • Right-drag: Rotate camera");
    println!("  • Scroll: Zoom in/out");
    println!("  • Close window: Exit demo\n");

    interactor.start();

    println!("\nDemo completed!");
}
