use vtk_rs::*;

fn main() {
    println!("=== VTK Warp Vector Demo ===");
    println!("Visualizing FEM-style deformation with adjustable scale factor\n");

    // Create a simple beam grid (10x3x3 = 90 points)
    let mut grid = UnstructuredGrid::new();
    let mut points = Points::new();

    let nx = 10; // Length direction
    let ny = 3; // Width
    let nz = 3; // Height
    let spacing = 1.0;

    println!("Creating {}x{}x{} hexahedral beam mesh...", nx, ny, nz);

    // Create points
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let x = (i as f64) * spacing;
                let y = (j as f64) * spacing;
                let z = (k as f64) * spacing;
                points.insert_next_point(x, y, z);
            }
        }
    }

    // Create hexahedral cells
    let mut cell_count = 0;
    for k in 0..nz - 1 {
        for j in 0..ny - 1 {
            for i in 0..nx - 1 {
                cell_count += 1;
            }
        }
    }

    grid.set_points(&mut points);
    grid.allocate(cell_count);

    // Add hexahedron cells
    for k in 0..nz - 1 {
        for j in 0..ny - 1 {
            for i in 0..nx - 1 {
                let idx = |x: usize, y: usize, z: usize| -> i32 {
                    (z * ny * nx + y * nx + x) as i32
                };

                let point_ids = [
                    idx(i, j, k),
                    idx(i + 1, j, k),
                    idx(i + 1, j + 1, k),
                    idx(i, j + 1, k),
                    idx(i, j, k + 1),
                    idx(i + 1, j, k + 1),
                    idx(i + 1, j + 1, k + 1),
                    idx(i, j + 1, k + 1),
                ];

                grid.insert_next_cell(VtkCellType::Hexahedron, &point_ids);
            }
        }
    }

    println!(
        "Created beam: {} points, {} hexahedral cells",
        points.get_number_of_points(),
        cell_count
    );

    // Add displacement vectors (cantilever beam bending)
    // Fixed at left (x=0), increasing displacement toward right
    let mut displacements = DoubleArray::new();
    displacements.set_number_of_components(3);
    displacements.set_number_of_tuples(points.get_number_of_points() as i64);
    displacements.set_name("Displacement");

    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let point_idx = (k * ny * nx + j * nx + i) as i64;

                // Cantilever deflection: displacement increases with distance from fixed end
                let x_pos = (i as f64) * spacing;
                let max_x = ((nx - 1) as f64) * spacing;

                // Parabolic deflection in z-direction
                let deflection_factor = (x_pos / max_x).powi(2);

                let dx = 0.0;
                let dy = 0.0;
                let dz = -2.0 * deflection_factor; // Downward bending

                displacements.set_tuple3(point_idx, dx, dy, dz);
            }
        }
    }

    // Add vectors to point data
    let mut point_data = grid.get_point_data();
    point_data.add_array(&displacements);
    point_data.set_active_vectors("Displacement");

    println!("Added displacement vectors (cantilever deflection pattern)");
    println!("Fixed end at x=0, maximum deflection at x={}", ((nx - 1) as f64) * spacing);

    // Create warp vector filter
    let mut warp = WarpVector::new();
    warp.set_input_data(&mut grid);
    warp.set_scale_factor(3.0); // Amplify deformation 3x for visibility

    println!("\nWarp scale factor: {:.1}x (amplified for visualization)", warp.get_scale_factor());

    // Create mapper for deformed mesh
    let mut deformed_mapper = DataSetMapper::new();
    deformed_mapper.set_input_connection(warp.get_output_port());

    // Create mapper for original mesh (wireframe)
    let mut original_mapper = DataSetMapper::new();
    original_mapper.set_input_data(&mut grid);

    // Actor for deformed mesh (solid)
    let mut deformed_actor = Actor::new();
    deformed_actor.set_data_set_mapper(&mut deformed_mapper);
    let mut deformed_prop = deformed_actor.get_property();
    deformed_prop.set_color(0.9, 0.3, 0.3); // Bright red
    deformed_prop.set_edge_visibility(true);
    deformed_prop.set_line_width(2.0);

    // Actor for original mesh (wireframe only)
    let mut original_actor = Actor::new();
    original_actor.set_data_set_mapper(&mut original_mapper);
    let mut original_prop = original_actor.get_property();
    original_prop.set_representation(RepresentationType::Wireframe);
    original_prop.set_color(0.3, 0.7, 1.0); // Blue
    original_prop.set_line_width(2.0);
    original_prop.set_opacity(0.6);

    // Setup renderer and window
    let mut renderer = Renderer::new();
    renderer.add_actor(&mut deformed_actor);
    renderer.add_actor(&mut original_actor);
    renderer.set_background(0.1, 0.1, 0.15);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK Warp Vector - Cantilever Beam Deformation");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Add axes for orientation reference
    let mut axes = AxesActor::new();
    let mut axes_widget = OrientationMarkerWidget::new();
    axes_widget.set_orientation_marker(&mut axes);
    axes_widget.set_interactor(&mut interactor);
    axes_widget.set_viewport(0.0, 0.0, 0.2, 0.2);
    axes_widget.set_enabled(true);
    axes_widget.interactive_off();

    // Set good camera view
    let mut camera = renderer.get_active_camera();
    camera.set_position(15.0, -15.0, 10.0);
    camera.set_focal_point(4.5, 1.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);
    renderer.reset_camera();

    println!("\n=== Rendering ===");
    println!("Red mesh: Deformed shape (scale factor = 3.0x)");
    println!("Blue wireframe: Original undeformed shape");
    println!("The beam is fixed at left end (x=0) and bends downward");
    println!("Deformation is amplified 3x for better visibility");
    println!("\nPress 'q' to quit, mouse to rotate/zoom");

    render_window.render();
    interactor.start();

    println!("\nDemo completed.");
}
