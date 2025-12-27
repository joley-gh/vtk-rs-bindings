use vtk_rs::*;

fn main() {
    println!("=== Superquadric Shape Morphing Demo ===");
    println!("Demonstrates smooth transitions between cube, sphere, and toroid shapes");
    println!("using theta and phi roundness parameters.\n");

    // Row 1: Theta roundness progression (cube → sphere)
    println!("Row 1 (top): Theta roundness 0.0 → 1.0 (horizontal morphing)");
    let theta_values = [0.1, 0.3, 0.5, 0.7, 1.0];
    let mut row1_actors = Vec::new();

    for (i, &theta) in theta_values.iter().enumerate() {
        let mut source = SuperquadricSource::new();
        source.set_theta_roundness(theta);
        source.set_phi_roundness(1.0); // Keep phi round
        source.set_theta_resolution(30);
        source.set_phi_resolution(30);
        source.set_size(0.8);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(source.get_output_port());

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        actor.get_property().set_color(1.0, 0.4, 0.4); // Red
        actor.set_position(((i as f64) - 2.0) * 2.5, 3.0, 0.0);

        row1_actors.push(actor);
        println!("  Position {}: Theta={:.1}, Phi=1.0 (sphere-ish)", i + 1, theta);
    }

    // Row 2: Phi roundness progression
    println!("\nRow 2 (middle): Phi roundness 0.0 → 1.0 (vertical morphing)");
    let phi_values = [0.1, 0.3, 0.5, 0.7, 1.0];
    let mut row2_actors = Vec::new();

    for (i, &phi) in phi_values.iter().enumerate() {
        let mut source = SuperquadricSource::new();
        source.set_theta_roundness(1.0); // Keep theta round
        source.set_phi_roundness(phi);
        source.set_theta_resolution(30);
        source.set_phi_resolution(30);
        source.set_size(0.8);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(source.get_output_port());

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        actor.get_property().set_color(0.4, 1.0, 0.4); // Green
        actor.set_position(((i as f64) - 2.0) * 2.5, 0.0, 0.0);

        row2_actors.push(actor);
        println!("  Position {}: Theta=1.0, Phi={:.1}", i + 1, phi);
    }

    // Row 3: Both roundness equal (symmetric shapes)
    println!("\nRow 3 (bottom): Equal roundness 0.0 → 1.0 (cube to sphere)");
    let equal_values = [0.1, 0.3, 0.5, 0.7, 1.0];
    let mut row3_actors = Vec::new();

    for (i, &roundness) in equal_values.iter().enumerate() {
        let mut source = SuperquadricSource::new();
        source.set_theta_roundness(roundness);
        source.set_phi_roundness(roundness);
        source.set_theta_resolution(30);
        source.set_phi_resolution(30);
        source.set_size(0.8);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(source.get_output_port());

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        actor.get_property().set_color(0.4, 0.4, 1.0); // Blue
        actor.set_position(((i as f64) - 2.0) * 2.5, -3.0, 0.0);

        row3_actors.push(actor);
        println!("  Position {}: Theta={:.1}, Phi={:.1}", i + 1, roundness, roundness);
    }

    // Row 4: Toroidal mode with varying roundness (cube-torus to round-torus)
    println!("\nRow 4 (toroidal): Toroidal mode with roundness 0.1 → 1.0");
    println!("  (Thickness=0.33 constant, roundness morphs cube-donut to round-donut)");
    let toroid_roundness_values = [0.1, 0.3, 0.5, 0.7, 1.0];
    let mut row4_actors = Vec::new();

    for (i, &roundness) in toroid_roundness_values.iter().enumerate() {
        let mut source = SuperquadricSource::new();
        source.set_toroidal(true); // Enable toroid mode
        source.set_thickness(0.33); // Fixed thickness (controls hole size)
        source.set_theta_roundness(roundness);
        source.set_phi_roundness(roundness);
        source.set_theta_resolution(40);
        source.set_phi_resolution(40);
        source.set_size(0.8);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(source.get_output_port());

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        actor.get_property().set_color(1.0, 0.8, 0.2); // Gold
        actor.set_position(((i as f64) - 2.0) * 2.5, -6.0, 0.0);

        row4_actors.push(actor);
        println!("  Position {}: Toroid roundness={:.1}, thickness=0.33", i + 1, roundness);
    }

    println!("\n=== Key Concepts ===");
    println!("- Theta roundness: Controls east-west curvature");
    println!("- Phi roundness: Controls north-south curvature");
    println!("- 0.0 = rectangular/flat, 1.0 = circular/round");
    println!("- Equal roundness: Cube (0.0) → Sphere (1.0)");
    println!("- Toroidal mode: Creates donut shapes (ALWAYS has a hole)");
    println!("  * Thickness controls hole size (NOT whether hole exists)");
    println!("  * Roundness still controls cube-ness vs sphere-ness of the torus\n");

    // Create renderer and add all actors
    let mut renderer = Renderer::new();
    renderer.set_background(0.15, 0.15, 0.2); // Dark blue-gray background

    for mut actor in row1_actors {
        renderer.add_actor(&mut actor);
    }
    for mut actor in row2_actors {
        renderer.add_actor(&mut actor);
    }
    for mut actor in row3_actors {
        renderer.add_actor(&mut actor);
    }
    for mut actor in row4_actors {
        renderer.add_actor(&mut actor);
    }

    // Setup camera for good overview
    let mut camera = renderer.get_active_camera();
    camera.set_position(0.0, 0.0, 25.0);
    camera.set_focal_point(0.0, -1.5, 0.0);
    camera.set_view_up(0.0, 1.0, 0.0);
    renderer.reset_camera();

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1400, 1000);
    render_window.set_window_name("Superquadric Shape Morphing - Cube to Sphere to Toroid");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("Rendering 20 superquadric shapes...");
    println!("Use mouse to rotate the view and explore the shape transitions.\n");

    render_window.render();
    interactor.start();
}
