use vtk_rs::*;

fn main() {
    println!("=== Parametric Function Source Demo ===");
    println!("This example demonstrates VTK parametric surfaces.");
    println!("VTK provides many parametric surfaces defined by mathematical equations.");
    println!("The ParametricFunctionSource tessellates these into polygon meshes.\n");

    // Create parametric torus
    println!("Creating Torus (donut shape)...");
    println!("  - Ring radius: 1.0, Cross-section radius: 0.3");
    println!("  - Parameters: u ∈ [0, 2π], v ∈ [0, 2π]");
    println!("  - Equation: x = (R + r*cos(v))*cos(u), y = (R + r*cos(v))*sin(u), z = r*sin(v)\n");

    let mut torus = ParametricTorus::new();
    torus.set_ring_radius(1.0);
    torus.set_cross_section_radius(0.3);

    // Using the convenience method to create actor, source, and mapper in one call
    let (mut torus_actor, _torus_source, _torus_mapper) = 
        ParametricFunctionSource::create_actor(
            &mut torus,
            50, // U resolution
            50, // V resolution
            (1.0, 0.3, 0.3), // Red color
            0.8, // Opacity
            (-3.0, 0.0, 0.0) // Position
        );

    // Create parametric Klein bottle
    println!("Creating Klein Bottle (non-orientable surface)...");
    println!("  - No distinct 'inside' or 'outside'");
    println!("  - Parameters: u ∈ [0, 2π], v ∈ [0, 2π]");
    println!("  - A 4D surface embedded in 3D space\n");

    let mut klein = ParametricKlein::new();

    let (mut klein_actor, _klein_source, _klein_mapper) = 
        ParametricFunctionSource::create_actor(
            &mut klein,
            50,
            50,
            (0.3, 0.7, 0.7), // Cyan
            0.8,
            (0.0, 0.0, 0.0)
        );

    // Create parametric Mobius strip
    println!("Creating Mobius Strip (one-sided surface)...");
    println!("  - Single continuous side");
    println!("  - Parameters: u ∈ [0, 2π], v ∈ [-1, 1]");
    println!("  - Classic topology demonstration\n");

    let mut mobius = ParametricMobius::new();
    mobius.set_radius(1.0);

    let (mut mobius_actor, _mobius_source, _mobius_mapper) = 
        ParametricFunctionSource::create_actor(
            &mut mobius,
            50,
            50,
            (1.0, 0.8, 0.2), // Gold
            0.8,
            (3.0, 0.0, 0.0)
        );

    println!("=== API Pattern ===");
    println!("For ANY parametric surface:");
    println!("  1. Create parametric function object (e.g., ParametricTorus::new())");
    println!("  2. Configure its parameters (e.g., set_ring_radius())");
    println!("  3. Call ParametricFunctionSource::create_actor()");
    println!("  4. Add actor to renderer");
    println!("\nThis pattern works for ALL VTK parametric surfaces:");
    println!("  - Geometric: Torus, Sphere, Ellipsoid, Cylinder");
    println!("  - Topological: Klein, Mobius, Boy, Roman");
    println!("  - Mathematical: Supertoroid, Pseudosphere, Enneper");
    println!("  - And many more...\n");

    // Create renderer and window
    let mut renderer = Renderer::new();
    renderer.add_actor(&mut torus_actor);
    renderer.add_actor(&mut klein_actor);
    renderer.add_actor(&mut mobius_actor);
    renderer.set_background(0.2, 0.3, 0.4); // Blue-gray background

    // Setup camera for better view
    let mut camera = renderer.get_active_camera();
    camera.set_position(0.0, -15.0, 5.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);
    renderer.reset_camera();

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 400);
    render_window.set_window_name("Parametric Surfaces: Torus, Klein Bottle, Mobius Strip");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    render_window.render();
    interactor.start();
}
