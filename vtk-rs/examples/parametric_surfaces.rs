use vtk_rs::*;

/// Helper function to demonstrate the general pattern for using ParametricFunctionSource.
///
/// The ParametricFunctionSource takes ANY VTK parametric function and tessellates it
/// into a polygonal mesh. The pattern is always:
/// 1. Create a parametric function (torus, klein bottle, mobius, etc.)
/// 2. Create a ParametricFunctionSource
/// 3. Set the parametric function via as_parametric_function()
/// 4. Configure resolution (U, V, W) to control mesh density
/// 5. Connect to mapper and render
///
/// This allows VTK's parametric math to define complex surfaces using
/// 2D or 3D parameter spaces (u, v, w) mapped to 3D coordinates.
fn create_parametric_actor(
    parametric_func_ptr: *mut std::ffi::c_void,
    u_res: i32,
    v_res: i32,
    color: (f64, f64, f64),
    opacity: f64,
    position: (f64, f64, f64)
) -> Actor {
    // Step 1: Create the parametric function source (tessellator)
    let mut source = ParametricFunctionSource::new();

    // Step 2: Attach the parametric function to the source
    source.set_parametric_function(parametric_func_ptr);

    // Step 3: Set resolution - higher values = smoother but more polygons
    // U and V are the parametric coordinates (like longitude/latitude)
    source.set_u_resolution(u_res);
    source.set_v_resolution(v_res);

    // Step 4: Create mapper to convert polygons to graphics primitives
    let mut mapper = PolyDataMapper::new();
    mapper.set_input_connection(source.get_output_port());

    // Step 5: Create actor with visual properties
    let mut actor = Actor::new();
    actor.set_mapper(&mut mapper);
    actor.get_property().set_color(color.0, color.1, color.2);
    actor.get_property().set_opacity(opacity);
    actor.set_position(position.0, position.1, position.2);

    actor
}

fn main() {
    println!("=== Parametric Function Source Demo ===");
    println!("This example demonstrates the general pattern for using ParametricFunctionSource.");
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

    // Using the helper function to demonstrate the general pattern
    let mut torus_actor = create_parametric_actor(
        torus.as_parametric_function(),
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

    let mut klein_actor = create_parametric_actor(
        klein.as_parametric_function(),
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

    let mut mobius_actor = create_parametric_actor(
        mobius.as_parametric_function(),
        50,
        50,
        (1.0, 0.8, 0.2), // Gold
        0.8,
        (3.0, 0.0, 0.0)
    );

    println!("=== General Pattern Summary ===");
    println!("For ANY parametric surface:");
    println!("  1. Create parametric function object");
    println!("  2. Create ParametricFunctionSource");
    println!("  3. Call set_parametric_function(func.as_parametric_function())");
    println!("  4. Set U/V resolution for mesh density");
    println!("  5. Connect to mapper → actor → renderer");
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
