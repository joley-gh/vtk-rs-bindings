use vtk_rs as vtk;

fn main() {
    // Create a sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_center([0.0; 3]);
    sphere_source.set_radius(5.0);

    // Make the surface smooth
    sphere_source.set_phi_resolution(100);
    sphere_source.set_theta_resolution(100);

    // Create the mapper and connect it to the sphere source
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());

    // Create the actor and set its mapper
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Create the renderer and set background color
    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.2, 0.4); // Dark blue background
    renderer.add_actor(&mut actor);

    // Create a grid around the sphere using CubeAxesActor
    let mut cube_axes = vtk::CubeAxesActor::new();
    cube_axes.set_bounds(-6.0, 6.0, -6.0, 6.0, -6.0, 6.0); // Slightly larger than sphere
    cube_axes.set_x_label("X Axis");
    cube_axes.set_y_label("Y Axis");
    cube_axes.set_z_label("Z Axis");
    cube_axes.draw_x_gridlines_on();
    cube_axes.draw_y_gridlines_on();
    cube_axes.draw_z_gridlines_on();

    // Create axes actor to show orientation
    let mut axes = vtk::AxesActor::new();

    // Create orientation marker widget to display axes in corner
    let mut axes_widget = vtk::OrientationMarkerWidget::new();
    axes_widget.set_orientation_marker(&mut axes);
    axes_widget.set_viewport(0.0, 0.0, 0.2, 0.2); // Bottom-left corner, 20% size
    axes_widget.interactive_off(); // Make it non-interactive

    // Create the render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.set_window_name("Sphere Visualization with Axes and Grid");
    render_window.set_size(800, 600);
    render_window.add_renderer(&mut renderer);

    // Create the interactor for user interaction
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Set up the axes widget with the interactor
    axes_widget.set_interactor(&mut interactor);
    axes_widget.set_enabled(true);

    // Set up mouse interaction style for rotation, zoom, and pan
    // - Left mouse button: Rotate the camera around the scene
    // - Right mouse button: Zoom in/out
    // - Middle mouse button: Pan the camera
    // - Scroll wheel: Zoom in/out
    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Add the grid to the renderer and set up its camera
    // Must be done before rendering
    let mut camera = renderer.get_active_camera();
    cube_axes.set_camera(&mut camera);

    renderer.add_actor(&mut cube_axes);

    // Reset camera to fit all visible actors in the scene
    renderer.reset_camera();

    // Initialize and start the rendering loop
    render_window.render();
    interactor.initialize();
    interactor.start();
}
