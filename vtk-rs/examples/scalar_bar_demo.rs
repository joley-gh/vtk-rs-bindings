/// Scalar Bar Actor Demo
///
/// Demonstrates vtkScalarBarActor and vtkLookupTable for creating color legends.
/// Shows multiple spheres colored by a data value with a color bar indicating the mapping.

use vtk_rs as vtk;

fn main() {
    println!("=== VTK Scalar Bar Actor Demo ===\n");

    // Create a lookup table for blue-to-red color mapping
    let mut lut = vtk::LookupTable::new();
    lut.set_range(0.0, 100.0); // Map values from 0 to 100
    lut.set_hue_range(0.66, 0.0); // Blue (0.66) to Red (0.0)
    lut.set_saturation_range(1.0, 1.0); // Fully saturated
    lut.set_value_range(1.0, 1.0); // Full brightness
    lut.set_number_of_table_values(256); // Smooth gradient
    lut.build();

    println!("Created lookup table:");
    println!("  Range: 0 - 100");
    println!("  Colors: Blue → Cyan → Green → Yellow → Red");
    println!("  Resolution: 256 discrete colors\n");

    // Create renderer and window
    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.1, 0.15);

    // Create a grid of spheres with different "temperature" values
    let rows = 5;
    let cols = 5;
    let spacing = 3.0;
    let mut spheres = Vec::new();
    let mut mappers = Vec::new();
    let mut actors = Vec::new();

    println!("Creating {} spheres with temperature values:\n", rows * cols);

    for row in 0..rows {
        for col in 0..cols {
            let x = ((col as f64) - (cols as f64) / 2.0) * spacing;
            let y = ((row as f64) - (rows as f64) / 2.0) * spacing;

            // Temperature value based on position (0-100)
            let temp = (((row * cols + col) as f64) / ((rows * cols - 1) as f64)) * 100.0;

            // Create sphere
            let mut sphere = vtk::SphereSource::new();
            sphere.set_radius(1.0);
            sphere.set_theta_resolution(32);
            sphere.set_phi_resolution(32);

            // Create mapper
            let mut mapper = vtk::PolyDataMapper::new();
            mapper.set_input_connection(sphere.get_output_port());

            // Create actor
            let mut actor = vtk::Actor::new();
            actor.set_mapper(&mut mapper);
            actor.set_position(x, y, 0.0); // Position the actor instead of the sphere

            // Get color from lookup table and apply to actor
            let (r, g, b) = lut.get_color(temp);
            actor.get_property().set_color(r, g, b);

            renderer.add_actor(&mut actor);

            if row == 0 && col == 0 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Blue)", x, y, temp);
            } else if row == rows / 2 && col == cols / 2 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Green)", x, y, temp);
            } else if row == rows - 1 && col == cols - 1 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Red)", x, y, temp);
            }

            spheres.push(sphere);
            mappers.push(mapper);
            actors.push(actor);
        }
    }

    println!("\n");
    let rows = 5;
    let cols = 5;
    let spacing = 3.0;
    let mut spheres = Vec::new();
    let mut mappers = Vec::new();
    let mut actors = Vec::new();

    println!("Creating {} spheres with temperature values:\n", rows * cols);

    for row in 0..rows {
        for col in 0..cols {
            let x = ((col as f64) - (cols as f64) / 2.0) * spacing;
            let y = ((row as f64) - (rows as f64) / 2.0) * spacing;

            // Temperature value based on position (0-100)
            let temp = (((row * cols + col) as f64) / ((rows * cols - 1) as f64)) * 100.0;

            // Create sphere
            let mut sphere = vtk::SphereSource::new();
            sphere.set_radius(1.0);
            sphere.set_theta_resolution(32);
            sphere.set_phi_resolution(32);

            // Create mapper
            let mut mapper = vtk::PolyDataMapper::new();
            mapper.set_input_connection(sphere.get_output_port());

            // Create actor
            let mut actor = vtk::Actor::new();
            actor.set_mapper(&mut mapper);
            actor.set_position(x, y, 0.0); // Position the actor instead of the sphere

            // Get color from lookup table and apply to actor
            let (r, g, b) = lut.get_color(temp);
            actor.get_property().set_color(r, g, b);

            renderer.add_actor(&mut actor);

            if row == 0 && col == 0 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Blue)", x, y, temp);
            } else if row == rows / 2 && col == cols / 2 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Green)", x, y, temp);
            } else if row == rows - 1 && col == cols - 1 {
                println!("  Sphere at ({:5.1}, {:5.1}) → Temp: {:5.1}°C (Red)", x, y, temp);
            }

            spheres.push(sphere);
            mappers.push(mapper);
            actors.push(actor);
        }
    }

    println!("\n");

    // Create scalar bar actor (color legend)
    let mut scalar_bar = vtk::ScalarBarActor::new();
    scalar_bar.set_lookup_table(&mut lut);
    scalar_bar.set_title("Temperature (°C)");
    scalar_bar.set_number_of_labels(6);

    // Position on right side of window
    scalar_bar.set_position(0.88, 0.1); // (x, y) in normalized coords [0,1]
    scalar_bar.set_width(0.1); // 10% of window width
    scalar_bar.set_height(0.8); // 80% of window height

    // Customize appearance
    let mut title_prop = scalar_bar.get_title_text_property();
    title_prop.set_font_size(18);
    title_prop.set_bold(true);
    title_prop.set_color(1.0, 1.0, 1.0); // White

    let mut label_prop = scalar_bar.get_label_text_property();
    label_prop.set_font_size(14);
    label_prop.set_color(0.9, 0.9, 0.9); // Light gray

    renderer.add_actor(&mut scalar_bar);

    println!("Scalar bar configuration:");
    println!("  Position: Right side (88% from left, 10% from bottom)");
    println!("  Size: 10% width × 80% height");
    println!("  Title: 'Temperature (°C)'");
    println!("  Labels: 6 evenly spaced (0, 20, 40, 60, 80, 100)");
    println!("  Orientation: Vertical\n");

    // Setup camera for good view
    let mut camera = renderer.get_active_camera();
    camera.set_position(0.0, 0.0, 40.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 1.0, 0.0);

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 900);
    render_window.set_window_name("VTK Scalar Bar Demo - Temperature Visualization");

    // Create interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("=== Demo Instructions ===");
    println!("• Drag with LEFT mouse button to rotate");
    println!("• Drag with MIDDLE mouse button to pan");
    println!("• Scroll or drag with RIGHT mouse button to zoom");
    println!("• Press 'q' to exit\n");
    println!("The color bar on the right shows the temperature scale.");
    println!("Cool (blue) objects are at the bottom-left,");
    println!("hot (red) objects are at the top-right.\n");

    render_window.render();
    interactor.start();

    println!("\nDemo complete.");
}
