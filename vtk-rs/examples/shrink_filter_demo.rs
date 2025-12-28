use vtk_rs as vtk;

fn main() {
    println!("=== Shrink Filter Demo (Cube) ===");

    // Set up bounds for the box (as in the C++ example)
    let bounds = [-10.0, 10.0, 10.0, 20.0, -5.0, 5.0];

    // Cube source as a stand-in for TessellatedBoxSource
    let mut cube = vtk::CubeSource::new();
    cube.set_bounds(bounds[0], bounds[1], bounds[2], bounds[3], bounds[4], bounds[5]);

    // Shrink filter
    let mut shrink = vtk::ShrinkFilter::new();
    shrink.set_input_connection(cube.get_output_port());
    shrink.set_shrink_factor(0.8);

    // DataSetMapper for shrunk cube
    let mut mapper = vtk::DataSetMapper::new();
    mapper.set_input_connection(shrink.get_output_port());

    // Actor for shrunk cube
    let mut actor = vtk::Actor::new();
    actor.set_data_set_mapper(&mut mapper);
    actor.get_property().set_edge_visibility(true);
    // Banana color (approximate RGB)
    actor.get_property().set_color(0.98, 0.91, 0.71);
    // No backface property API available

    // Renderer, window, interactor
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    // Silver background (approximate RGB)
    renderer.set_background(0.75, 0.75, 0.75);

    let mut window = vtk::RenderWindow::new();
    window.add_renderer(&mut renderer);
    window.set_size(640, 480);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_interactor_style(&mut vtk::InteractorStyleTrackballCamera::new());
    interactor.set_render_window(&mut window);

    renderer.reset_camera();
    // Camera azimuth and elevation (if API available)
    // renderer.get_active_camera().azimuth(30.0);
    // renderer.get_active_camera().elevation(30.0);
    // renderer.reset_camera_clipping_range();

    window.render();
    interactor.start();
}
