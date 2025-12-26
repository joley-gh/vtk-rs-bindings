use vtk_rs::*;

fn main() {
    println!("\n=== VTK WorldPointPicker Demo ===\n");
    println!("This demonstrates WorldPointPicker for screen-to-world coordinate conversion.");
    println!(
        "WorldPointPicker is specifically designed to convert any screen point to 3D world coordinates.\n"
    );

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

    // Add some reference objects to show the 3D space
    // Reference Cone at origin
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

    // Reference Cube
    let mut cube = CubeSource::new();
    cube.set_x_length(1.0);
    cube.set_y_length(1.0);
    cube.set_z_length(1.0);

    let mut cube_mapper = PolyDataMapper::new();
    cube_mapper.set_input_connection(cube.get_output_port());

    let mut cube_actor = Actor::new();
    cube_actor.set_mapper(&mut cube_mapper);
    cube_actor.set_position(-3.0, 2.0, 0.5);
    cube_actor.rotate_z(30.0);
    let mut cube_prop = cube_actor.get_property();
    cube_prop.set_color(0.2, 0.8, 0.2);
    renderer.add_actor(&mut cube_actor);

    // Reference Cylinder
    let mut cylinder = CylinderSource::new();
    cylinder.set_radius(0.4);
    cylinder.set_height(1.5);
    cylinder.set_resolution(24);

    let mut cyl_mapper = PolyDataMapper::new();
    cyl_mapper.set_input_connection(cylinder.get_output_port());

    let mut cyl_actor = Actor::new();
    cyl_actor.set_mapper(&mut cyl_mapper);
    cyl_actor.set_position(3.0, -2.0, 0.75);
    let mut cyl_prop = cyl_actor.get_property();
    cyl_prop.set_color(0.2, 0.4, 0.9);
    renderer.add_actor(&mut cyl_actor);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("WorldPointPicker Demo - Screen to 3D Conversion");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();

    // IMPORTANT: Initialize interactor and render before picking
    interactor.initialize();
    render_window.render();

    println!("Scene setup complete:");
    println!("- Grid plane at z=0 (reference ground)");
    println!("- Red cone at origin (0, 0, 0.5)");
    println!("- Green cube at (-3, 2, 0.5)");
    println!("- Blue cylinder at (3, -2, 0.75)");

    // Demonstrate WorldPointPicker functionality
    println!("\n═══════════════════════════════════════════════════════");
    println!("WorldPointPicker: Screen-to-World Coordinate Conversion");
    println!("═══════════════════════════════════════════════════════\n");

    let mut picker = WorldPointPicker::new();

    // Window size is 1200x800 (set above)
    let width = 1200.0;
    let height = 800.0;
    println!("Window size: {}x{}", width as i32, height as i32);

    // Test several screen positions and convert to 3D world coordinates
    let test_points = vec![
        (width / 2.0, height / 2.0, "Center of window"),
        (width / 4.0, height / 4.0, "Lower-left quadrant"),
        ((3.0 * width) / 4.0, height / 4.0, "Lower-right quadrant"),
        (width / 4.0, (3.0 * height) / 4.0, "Upper-left quadrant"),
        ((3.0 * width) / 4.0, (3.0 * height) / 4.0, "Upper-right quadrant")
    ];

    println!("\nConverting screen coordinates to 3D world positions:");
    println!("(WorldPointPicker projects screen points onto the focal plane)\n");

    for (x, y, description) in test_points {
        // WorldPointPicker.pick() converts screen coordinates to world coordinates
        // The z parameter (0.0) means we're picking on the view plane
        let success = picker.pick(x, y, 0.0, &mut renderer);
        if success {
            let (wx, wy, wz) = picker.get_pick_position();
            println!("  Screen ({:.0}, {:.0}) [{}]", x, y, description);
            println!("    → World: ({:.2}, {:.2}, {:.2})", wx, wy, wz);
        } else {
            println!("  Screen ({:.0}, {:.0}) [{}] - Pick failed", x, y, description);
        }
    }

    // Compare with Renderer coordinate conversion methods
    println!("\n═══════════════════════════════════════════════════════");
    println!("Comparison: Renderer.display_to_world() vs WorldPointPicker");
    println!("═══════════════════════════════════════════════════════\n");

    // Using Renderer's coordinate conversion
    renderer.set_display_point(600.0, 400.0, 0.0);
    renderer.display_to_world();
    let (wx1, wy1, wz1, ww) = renderer.get_world_point();
    // Homogeneous coordinates need to be normalized
    let (wx1, wy1, wz1) = (wx1 / ww, wy1 / ww, wz1 / ww);
    println!("Renderer.display_to_world(600, 400):");
    println!("  → World: ({:.2}, {:.2}, {:.2})", wx1, wy1, wz1);

    // Using WorldPointPicker
    picker.pick(600.0, 400.0, 0.0, &mut renderer);
    let (wx2, wy2, wz2) = picker.get_pick_position();
    println!("\nWorldPointPicker.pick(600, 400):");
    println!("  → World: ({:.2}, {:.2}, {:.2})", wx2, wy2, wz2);

    println!("\n═══════════════════════════════════════════════════════");
    println!("WorldPointPicker API Summary");
    println!("═══════════════════════════════════════════════════════\n");

    println!("Key Methods:");
    println!("  picker.pick(x, y, z, &mut renderer) -> bool");
    println!("    - x, y: Screen coordinates (display space)");
    println!("    - z: Depth (typically 0.0 for focal plane)");
    println!("    - Returns true if conversion successful");
    println!("\n  picker.get_pick_position() -> (f64, f64, f64)");
    println!("    - Returns world coordinates (x, y, z)");
    println!("    - Valid after successful pick() call");
    println!("\nUse Cases:");
    println!("  • Interactive 3D object placement");
    println!("  • Click-to-position interfaces");
    println!("  • Drawing tools in 3D space");
    println!("  • Screen-to-world coordinate mapping");
    println!("  • Converting mouse clicks to 3D positions");

    println!("\n═══════════════════════════════════════════════════════");
    println!("Opening interactive window...");
    println!("═══════════════════════════════════════════════════════\n");
    println!("This demo shows programmatic picks (coordinates specified in code).");
    println!("For interactive clicking, see the 'interactive_world_point_picker' example!");
    println!("\nUse mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");

    interactor.start();
}
