use vtk_rs::*;

fn main() {
    println!("=== FEM Beam with Glyph3D Node Markers ===\n");
    println!("Demonstrating efficient node visualization:");
    println!("  - Glyph3D filter places spheres at all nodes with one filter");
    println!("  - Much more efficient than creating individual SphereSource objects");
    println!("  - Ideal for models with many nodes\n");

    // 1. Define FEM beam geometry: 4 nodes, 3 elements
    let nodes = vec![
        [0.0, 0.0, 0.0], // Node 0: Fixed end
        [1.0, 0.0, 0.0], // Node 1
        [2.0, 0.0, 0.0], // Node 2
        [3.0, 0.0, 0.0] // Node 3: Free end
    ];

    // Displacement results (cantilever beam bending down)
    let displacements = vec![
        [0.0, 0.0, 0.0], // Node 0: Fixed (no displacement)
        [0.0, -0.3, 0.0], // Node 1: Small deflection
        [0.0, -0.7, 0.0], // Node 2: Medium deflection
        [0.0, -1.0, 0.0] // Node 3: Maximum deflection
    ];

    let elements = vec![
        [0, 1], // Element 0
        [1, 2], // Element 1
        [2, 3] // Element 2
    ];

    // Element stresses (MPa) - higher at fixed end
    let stresses = vec![180.0, 120.0, 60.0];

    let displacement_scale = 0.1;

    println!("✓ Creating geometry with deformed positions\n");

    // ==================== INITIAL GEOMETRY (Gray Lines) ====================
    println!("=== Creating Initial Geometry ===");

    let mut initial_lines = vec![];
    for (elem_idx, [n0, n1]) in elements.iter().enumerate() {
        let mut line = LineSource::new();
        line.set_point1(nodes[*n0][0], nodes[*n0][1], nodes[*n0][2]);
        line.set_point2(nodes[*n1][0], nodes[*n1][1], nodes[*n1][2]);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(LineSource::get_output_port(&mut line));

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);

        // Gray thin lines for reference
        let mut property = actor.get_property();
        property.set_color(0.5, 0.5, 0.5);
        property.set_line_width(2.0);

        initial_lines.push((line, mapper, actor));
    }

    println!("✓ Created initial geometry (thin gray lines)\n");

    // ==================== DEFORMED GEOMETRY (Colored Tubes) ====================
    println!("=== Creating Deformed Geometry with TubeFilter ===");

    // Calculate deformed positions
    let deformed_positions: Vec<[f64; 3]> = nodes
        .iter()
        .zip(displacements.iter())
        .map(|(pos, disp)| {
            [
                pos[0] + disp[0] * displacement_scale,
                pos[1] + disp[1] * displacement_scale,
                pos[2] + disp[2] * displacement_scale,
            ]
        })
        .collect();

    let mut deformed_tubes = vec![];
    for (elem_idx, [n0, n1]) in elements.iter().enumerate() {
        let mut line = LineSource::new();
        line.set_point1(
            deformed_positions[*n0][0],
            deformed_positions[*n0][1],
            deformed_positions[*n0][2]
        );
        line.set_point2(
            deformed_positions[*n1][0],
            deformed_positions[*n1][1],
            deformed_positions[*n1][2]
        );

        // Create 3D tube around line
        let mut tube = TubeFilter::new();
        tube.set_input_connection(LineSource::get_output_port(&mut line));
        tube.set_radius(0.05);
        tube.set_number_of_sides(12);
        tube.set_capping(true);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(tube.get_output_port());

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);

        // Color by stress: Red (high) -> Orange -> Green (low)
        let stress = stresses[elem_idx];
        let mut property = actor.get_property();
        if stress > 150.0 {
            property.set_color(1.0, 0.0, 0.0); // Red: High stress
        } else if stress > 90.0 {
            property.set_color(1.0, 0.5, 0.0); // Orange: Medium stress
        } else {
            property.set_color(0.0, 1.0, 0.0); // Green: Low stress
        }

        deformed_tubes.push((line, tube, mapper, actor));
    }

    println!("✓ Created tube actors with stress-based colors\n");

    // ==================== NODE MARKERS WITH GLYPH3D ====================
    println!("=== Creating Node Markers with Glyph3D ===");

    // Create PolyData with all node positions
    let mut node_points = Points::new();
    for [x, y, z] in &deformed_positions {
        node_points.insert_next_point(*x, *y, *z);
    }

    let mut node_poly_data = PolyData::new();
    node_poly_data.set_points(&mut node_points);

    // Create sphere geometry (this will be copied at each point)
    let mut sphere_source = SphereSource::new();
    sphere_source.set_radius(0.06);
    sphere_source.set_theta_resolution(16);
    sphere_source.set_phi_resolution(16);

    // Use Glyph3D to place spheres at all nodes efficiently
    let mut glyph = Glyph3D::new();
    glyph.set_input_connection(node_poly_data.get_output_port());
    glyph.set_source_connection(SphereSource::get_output_port(&mut sphere_source));
    glyph.set_scale_mode_to_data_scaling_off(); // Uniform size for all nodes
    glyph.set_scale_factor(1.0);

    let mut node_mapper = PolyDataMapper::new();
    node_mapper.set_input_connection(glyph.get_output_port());

    let mut node_actor = Actor::new();
    node_actor.set_mapper(&mut node_mapper);

    let mut node_property = node_actor.get_property();
    node_property.set_color(0.2, 0.4, 1.0); // Blue nodes

    println!("✓ Created Glyph3D node markers (4 spheres with 1 filter)\n");

    // ==================== RENDERING SETUP ====================
    println!("=== Setting up Renderer ===");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.2); // Dark blue background

    // Add all actors
    for (_, _, mut actor) in initial_lines {
        renderer.add_actor(&mut actor);
    }
    for (_, _, _, mut actor) in deformed_tubes {
        renderer.add_actor(&mut actor);
    }
    renderer.add_actor(&mut node_actor);

    println!("✓ Added all actors to renderer\n");

    let mut window = RenderWindow::new();
    window.add_renderer(&mut renderer);
    window.set_size(1024, 768);
    window.set_window_name("FEM Beam - Glyph3D Example");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("✓ Setup render window and interactor\n");

    // ==================== VISUALIZATION INFO ====================
    println!("=== Visualization Info ===");
    println!("OVERLAID GEOMETRY:");
    println!("  - Gray lines: Initial/undeformed configuration");
    println!("  - Colored tubes: Deformed configuration (3D beams)");
    println!("    • Red: High stress (180 MPa) - Fixed end");
    println!("    • Orange: Medium stress (120 MPa)");
    println!("    • Green: Low stress (60 MPa) - Free end");
    println!("  - Blue spheres: Node positions (Glyph3D)\n");

    println!("Glyph3D efficiency:");
    println!("  - Traditional approach: 4 SphereSource objects");
    println!("  - Glyph3D approach: 1 filter + 1 sphere definition");
    println!("  - Scales to thousands of nodes efficiently\n");

    println!("Displacement:");
    println!("  - Vertical deflection (downward)");
    println!("  - Scale factor: {} (for visibility)", displacement_scale);
    println!(
        "  - Max displacement: {:.3} units at free end\n",
        displacements[3][1].abs() * displacement_scale
    );

    println!("Interaction:");
    println!("  - Left mouse: Rotate");
    println!("  - Middle mouse: Pan");
    println!("  - Right mouse/Scroll: Zoom");
    println!("  - 'q' or close window to exit\n");

    window.render();
    interactor.start();
}
