use vtk_rs as vtk;

fn main() {
    println!("Interactive Picking Example");
    println!("Click on the sphere to see pick information\n");

    // Create a sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_center([0.0; 3]);
    sphere_source.set_radius(5.0);
    sphere_source.set_phi_resolution(50);
    sphere_source.set_theta_resolution(50);

    // Create the mapper and actor
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Style the sphere
    let mut property = actor.get_property();
    property.set_color(0.4, 0.7, 0.9);

    // Create the renderer and add actor
    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.2, 0.4);
    renderer.add_actor(&mut actor);

    // Setup camera
    let mut camera = renderer.get_active_camera();
    camera.set_position(15.0, 15.0, 15.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);

    // Create axes actor for orientation reference
    let mut axes = vtk::AxesActor::new();
    let mut axes_widget = vtk::OrientationMarkerWidget::new();
    axes_widget.set_orientation_marker(&mut axes);
    axes_widget.set_viewport(0.0, 0.0, 0.2, 0.2);
    axes_widget.interactive_off();

    // Create render window and interactor
    let mut render_window = vtk::RenderWindow::new();
    render_window.set_window_name("Picking Example - Click on the Sphere");
    render_window.set_size(800, 600);
    render_window.add_renderer(&mut renderer);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    axes_widget.set_interactor(&mut interactor);
    axes_widget.set_enabled(true);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Create a cell picker
    let mut picker = vtk::CellPicker::new();

    println!("Instructions:");
    println!("  - Left mouse button: Rotate view");
    println!("  - Middle mouse button: Pan");
    println!("  - Right mouse button: Zoom\n");

    println!("═══════════════════════════════════════════════════════");
    println!("This example demonstrates the VTK-RS picking API.");
    println!("It performs automated picks to show how the system works.");
    println!("═══════════════════════════════════════════════════════\n");

    // Demonstrate picking at various locations BEFORE opening window
    println!("Testing picks at different display coordinates:\n");

    // Test 1: Center of window (should hit sphere)
    println!("Test 1: Picking at center (400, 300)...");
    if picker.pick(400.0, 300.0, 0.0, &mut renderer) {
        let pos = picker.get_pick_position();
        let cell_id = picker.get_cell_id();
        println!("  ✓ Pick successful!");
        println!("    Position: ({:.3}, {:.3}, {:.3})", pos.0, pos.1, pos.2);
        println!("    Cell ID: {}", cell_id);
    } else {
        println!("  ✗ No geometry at that location");
    }

    // Test 2: Off to the side (likely to miss)
    println!("\nTest 2: Picking at edge (100, 100)...");
    if picker.pick(100.0, 100.0, 0.0, &mut renderer) {
        let pos = picker.get_pick_position();
        let cell_id = picker.get_cell_id();
        println!("  ✓ Pick successful!");
        println!("    Position: ({:.3}, {:.3}, {:.3})", pos.0, pos.1, pos.2);
        println!("    Cell ID: {}", cell_id);
    } else {
        println!("  ✗ No geometry at that location");
    }

    // Test 3: Another location
    println!("\nTest 3: Picking at (600, 400)...");
    if picker.pick(600.0, 400.0, 0.0, &mut renderer) {
        let pos = picker.get_pick_position();
        let cell_id = picker.get_cell_id();
        println!("  ✓ Pick successful!");
        println!("    Position: ({:.3}, {:.3}, {:.3})", pos.0, pos.1, pos.2);
        println!("    Cell ID: {}", cell_id);
    } else {
        println!("  ✗ No geometry at that location");
    }

    // Demonstrate coordinate conversion
    println!("\n═══════════════════════════════════════════════════════");
    println!("Coordinate Conversion Examples:");
    println!("═══════════════════════════════════════════════════════\n");
    // Display to world conversion
    renderer.set_display_point(400.0, 300.0, 0.0);
    renderer.display_to_world();
    let world = renderer.get_world_point();
    println!(
        "Display (400, 300, 0) → World ({:.3}, {:.3}, {:.3}, {:.3})",
        world.0,
        world.1,
        world.2,
        world.3
    );

    // World to display conversion
    renderer.set_world_point(0.0, 0.0, 0.0, 1.0);
    renderer.world_to_display();
    let display = renderer.get_display_point();
    println!("World (0, 0, 0) → Display ({:.1}, {:.1}, {:.1})", display.0, display.1, display.2);

    println!("\n═══════════════════════════════════════════════════════");
    println!("Opening interactive window...");
    println!("═══════════════════════════════════════════════════════");
    println!("\nThis demo shows programmatic picks (coordinates specified in code).");
    println!("For interactive clicking, see the 'interactive_picking' example!\n");

    // Start the interaction
    render_window.render();
    interactor.start();

    // Initial render
    render_window.render();

    // Start interactive mode
    interactor.initialize();
    interactor.start();
}
