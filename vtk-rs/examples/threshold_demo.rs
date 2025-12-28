use vtk_rs as vtk;

fn main() {
    println!("=== Threshold Demo (simple) ===");

    // Create a synthetic ImageData volume
    let mut img = vtk::ImageData::new();
    img.set_dimensions(50, 50, 50);
    img.set_spacing(0.1, 0.1, 0.1);
    img.set_origin(-2.5, -2.5, -2.5);
    img.allocate_scalars(vtk::VtkDataType::Double, 1);

    // Fill with a spherical scalar field (distance from center)
    let (nx, ny, nz) = img.get_dimensions();
    for z in 0..nz {
        for y in 0..ny {
            for x in 0..nx {
                let px = ((x as f64) - (nx as f64) / 2.0) * 0.1;
                let py = ((y as f64) - (ny as f64) / 2.0) * 0.1;
                let pz = ((z as f64) - (nz as f64) / 2.0) * 0.1;
                let val = (px * px + py * py + pz * pz).sqrt();
                img.set_scalar_component(x, y, z, 0, val);
            }
        }
    }

    // Apply threshold to keep voxels within [0.0, 1.5]
    let mut thr = vtk::Threshold::new();
    thr.set_input_data(&mut img);
    thr.threshold_between(0.0, 1.5);

    // Feed the threshold output to a contour filter to extract an isosurface
    let mut contour = vtk::ContourFilter::new();
    contour.set_input_connection(thr.get_output_port());
    contour.set_value(0, 1.0);

    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(contour.get_output_port());

    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);
    actor.get_property().set_color(0.2, 0.6, 1.0);

    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.1);

    let mut window = vtk::RenderWindow::new();
    window.add_renderer(&mut renderer);
    window.set_size(800, 600);
    window.set_window_name("Threshold Demo - contour from threshold");

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_interactor_style(&mut vtk::InteractorStyleTrackballCamera::new());
    interactor.set_render_window(&mut window);

    // Frame the scene
    renderer.reset_camera();

    println!("Rendering contour extracted from thresholded volume (blue). Use mouse to rotate.");

    window.render();
    interactor.start();
}
