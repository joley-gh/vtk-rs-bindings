use vtk_rs as vtk;

fn main() {
    println!("=== VTK Clip Poly Data Demo ===");
    println!("Clipping a sphere with a plane to create cross-sections\n");

    // Create a sphere source
    let mut sphere = vtk::SphereSource::new();
    sphere.set_center([0.0, 0.0, 0.0]);
    sphere.set_radius(5.0);
    sphere.set_theta_resolution(30);
    sphere.set_phi_resolution(30);
    
    println!("Created sphere: center=(0,0,0), radius=5.0");

    // Create a clipping plane
    let mut plane = vtk::Plane::new();
    plane.set_origin(0.0, 0.0, 0.0);  // Plane passes through origin
    plane.set_normal(1.0, 0.5, 0.2);  // Tilted plane
    
    println!("Created clipping plane: origin=(0,0,0), normal=(1.0, 0.5, 0.2)");
    
    // Create clipper
    let mut clipper = vtk::ClipPolyData::new();
    clipper.set_input_connection(sphere.get_output_port());
    clipper.set_clip_function(&mut plane);
    clipper.set_value(0.0);  // Clip at the plane (distance = 0)
    
    println!("Clipping sphere with plane\n");

    // Create mapper for clipped geometry
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(clipper.get_output_port());

    // Create actor
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);
    actor.get_property().set_edge_visibility(true);
    actor.get_property().set_line_width(2.0);

    // Create renderer
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.2);

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("Clip Poly Data - Sphere Cross-Section");

    // Add interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    println!("Rendering clipped sphere (half visible)");
    println!("The sphere is cut by the tilted plane");
    println!("Press 'q' to quit, mouse to rotate\n");

    render_window.render();
    interactor.start();
}
