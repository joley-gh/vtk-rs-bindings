use vtk_rs as vtk;

fn main() {
    println!("=== VTK Contour Filter Demo ===");
    println!("Creating 3D volumetric data and extracting isosurfaces\n");

    // Create a 3D ImageData (volumetric grid)
    let mut image_data = vtk::ImageData::new();

    // Set dimensions: 50x50x50 grid
    let dims = [50, 50, 50];
    image_data.set_dimensions(dims[0], dims[1], dims[2]);

    // Set spacing (voxel size)
    image_data.set_spacing(1.0, 1.0, 1.0);

    // Set origin (center the volume)
    image_data.set_origin(-25.0, -25.0, -25.0);

    // Allocate scalar values (one value per voxel)
    let total_points = dims[0] * dims[1] * dims[2];
    image_data.allocate_scalars(vtk::VtkDataType::Float, 1);

    println!("Created volume: {}x{}x{} = {} voxels", dims[0], dims[1], dims[2], total_points);

    // Fill with a 3D function: distance from center (sphere)
    for z in 0..dims[2] {
        for y in 0..dims[1] {
            for x in 0..dims[0] {
                // Calculate distance from center
                let cx = (x as f64) - 25.0;
                let cy = (y as f64) - 25.0;
                let cz = (z as f64) - 25.0;
                let distance = (cx * cx + cy * cy + cz * cz).sqrt();

                image_data.set_scalar_component(x, y, z, 0, distance);
            }
        }
    }

    println!("Filled volume with distance field (sphere function)");
    println!("Value range: 0.0 (center) to ~43.3 (corners)\n");

    // Create contour filter to extract isosurfaces
    let mut contour = vtk::ContourFilter::new();
    contour.set_input_data(&mut image_data);

    // Extract 3 isosurfaces at different radii
    contour.set_value(0, 10.0); // Small sphere
    contour.set_value(1, 15.0); // Medium sphere
    contour.set_value(2, 20.0); // Large sphere

    println!("Extracting 3 isosurfaces:");
    println!("  - Radius 10.0 (inner sphere)");
    println!("  - Radius 15.0 (middle sphere)");
    println!("  - Radius 20.0 (outer sphere)\n");

    // Create mapper for the contoured surface
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(contour.get_output_port());

    // Create actor
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);
    actor.get_property().set_opacity(0.5); // Semi-transparent
    actor.get_property().set_edge_visibility(true);

    // Create renderer
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.2); // Dark blue

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("Contour Filter - Isosurface Extraction");

    // Add interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_interactor_style(&mut vtk::InteractorStyleTrackballCamera::new());
    interactor.set_render_window(&mut render_window);

    println!("Rendering 3 nested spherical isosurfaces");
    println!("Press 'q' to quit, mouse to rotate\n");

    render_window.render();
    interactor.start();
}
