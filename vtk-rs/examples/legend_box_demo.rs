use vtk_rs as vtk;

fn main() {
    println!("=== VTK Legend Box Actor Demo ===");
    println!("Demonstrates multi-item legends for scene annotations");
    println!();

    // Create renderer and window
    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.1, 0.2);

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Set interactor style for better camera control
    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Create three different colored spheres representing different categories

    // Red sphere - Category A
    let mut sphere_a = vtk::SphereSource::new();
    sphere_a.set_center([-2.0, 0.0, 0.0]);
    sphere_a.set_radius(0.5);
    sphere_a.set_theta_resolution(30);
    sphere_a.set_phi_resolution(30);

    let mut mapper_a = vtk::PolyDataMapper::new();
    mapper_a.set_input_connection(sphere_a.get_output_port());

    let mut actor_a = vtk::Actor::new();
    actor_a.set_mapper(&mut mapper_a);
    actor_a.get_property().set_color(1.0, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor_a);

    // Green sphere - Category B
    let mut sphere_b = vtk::SphereSource::new();
    sphere_b.set_center([0.0, 0.0, 0.0]);
    sphere_b.set_radius(0.5);
    sphere_b.set_theta_resolution(30);
    sphere_b.set_phi_resolution(30);

    let mut mapper_b = vtk::PolyDataMapper::new();
    mapper_b.set_input_connection(sphere_b.get_output_port());

    let mut actor_b = vtk::Actor::new();
    actor_b.set_mapper(&mut mapper_b);
    actor_b.get_property().set_color(0.2, 1.0, 0.2); // Green
    renderer.add_actor(&mut actor_b);

    // Blue sphere - Category C
    let mut sphere_c = vtk::SphereSource::new();
    sphere_c.set_center([2.0, 0.0, 0.0]);
    sphere_c.set_radius(0.5);
    sphere_c.set_theta_resolution(30);
    sphere_c.set_phi_resolution(30);

    let mut mapper_c = vtk::PolyDataMapper::new();
    mapper_c.set_input_connection(sphere_c.get_output_port());

    let mut actor_c = vtk::Actor::new();
    actor_c.set_mapper(&mut mapper_c);
    actor_c.get_property().set_color(0.2, 0.2, 1.0); // Blue
    renderer.add_actor(&mut actor_c);

    // Yellow sphere - Category D
    let mut sphere_d = vtk::SphereSource::new();
    sphere_d.set_center([-1.0, 1.5, 0.0]);
    sphere_d.set_radius(0.5);
    sphere_d.set_theta_resolution(30);
    sphere_d.set_phi_resolution(30);

    let mut mapper_d = vtk::PolyDataMapper::new();
    mapper_d.set_input_connection(sphere_d.get_output_port());

    let mut actor_d = vtk::Actor::new();
    actor_d.set_mapper(&mut mapper_d);
    actor_d.get_property().set_color(1.0, 1.0, 0.2); // Yellow
    renderer.add_actor(&mut actor_d);

    // Create legend box with entries for each category
    let mut legend = vtk::LegendBoxActor::new();

    // Set number of entries
    legend.set_number_of_entries(4);

    // Add entry for each category with matching colors
    legend.set_entry(0, "Category A - Red Sphere", 1.0, 0.2, 0.2);
    legend.set_entry(1, "Category B - Green Sphere", 0.2, 1.0, 0.2);
    legend.set_entry(2, "Category C - Blue Sphere", 0.2, 0.2, 1.0);
    legend.set_entry(3, "Category D - Yellow Sphere", 1.0, 1.0, 0.2);

    // Position legend in lower left corner
    // Position is in normalized viewport coordinates [0,1]
    legend.set_position(0.05, 0.05); // x, y
    legend.set_position2(0.35, 0.25); // width, height

    // Configure appearance
    legend.set_border(true); // Show border around legend
    legend.set_box(true); // Show background box
    legend.set_padding(5); // Padding inside the box

    // Add legend to renderer as a 2D overlay
    renderer.add_actor(&mut legend);

    println!("Legend created with 4 entries:");
    println!("  - Red sphere (Category A) at position (-2, 0, 0)");
    println!("  - Green sphere (Category B) at position (0, 0, 0)");
    println!("  - Blue sphere (Category C) at position (2, 0, 0)");
    println!("  - Yellow sphere (Category D) at position (-1, 1.5, 0)");
    println!();
    println!("Legend positioned in lower left corner");
    println!("  - Border: enabled");
    println!("  - Background box: enabled");
    println!("  - Padding: 5 pixels");
    println!();
    println!("Starting interactive visualization...");
    println!("Close the window to exit.");
    println!();

    // Start interaction
    render_window.render();
    interactor.start();

    println!("Demo complete!");
}
