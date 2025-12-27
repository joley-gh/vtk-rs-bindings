use vtk_rs as vtk;

fn main() {
    println!("=== Smooth PolyData Filter Demo ===");

    // Sphere source
    let mut sphere = vtk::SphereSource::new();
    sphere.set_radius(1.0);
    sphere.set_theta_resolution(15);
    sphere.set_phi_resolution(15);

    // Smooth filter
    let mut smoother = vtk::SmoothPolyDataFilter::new();
    smoother.set_number_of_iterations(5);
    smoother.set_relaxation_factor(0.05);
    smoother.set_feature_edge_smoothing(true);
    smoother.set_boundary_smoothing(true);
    smoother.set_input_connection(sphere.get_output_port());

    // Mapper and actor
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(smoother.get_output_port());

    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Create a mapper/actor for the original (unsmoothed) sphere for comparison
    let mut mapper_orig = vtk::PolyDataMapper::new();
    mapper_orig.set_input_connection(sphere.get_output_port());
    let mut actor_orig = vtk::Actor::new();
    actor_orig.set_mapper(&mut mapper_orig);

    // Position them side-by-side
    actor_orig.set_position(-1.5, 0.0, 0.0);
    actor.set_position(1.5, 0.0, 0.0);

    // Color the original and smoothed meshes differently
    actor_orig.get_property().set_color(1.0, 0.5, 0.1); // warm orange
    actor.get_property().set_color(0.2, 0.6, 1.0); // cool blue

    // Renderer and window
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor_orig);
    renderer.add_actor(&mut actor);
    renderer.set_background(0.2, 0.3, 0.4);

    let mut window = vtk::RenderWindow::new();
    window.add_renderer(&mut renderer);
    window.set_size(800, 600);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_interactor_style(&mut vtk::InteractorStyleTrackballCamera::new());
    interactor.set_render_window(&mut window);

    window.render();
    interactor.start();
}
