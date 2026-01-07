use vtk_rs as vtk;
use vtk::CameraObserverExt;

fn main() {
    println!("=== VTK Billboard Text Demo - FEM Node Labels ===\n");
    println!("Demonstrating vtkFollower + vtkVectorText for camera-facing labels");
    println!("Perfect for FEM node IDs, point annotations, and 3D labels\n");

    // Initialize VTK
    vtk::init_vtk();

    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.1, 0.15);

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1400, 900);
    render_window.set_window_name("VTK Billboard Text Demo - Node Labels with Follower");

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Create a simple mesh structure (like FEM nodes)
    // 5x5 grid of spheres representing nodes
    println!("Creating 5x5 grid of nodes (spheres)...");

    let grid_size = 5;
    let spacing = 2.0;
    let mut node_actors = Vec::new();
    let mut label_followers = Vec::new();
    let mut label_texts = Vec::new();
    let mut label_mappers = Vec::new();

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = ((i as f64) - 2.0) * spacing;
            let y = ((j as f64) - 2.0) * spacing;
            let z = 0.0;

            let node_id = i * grid_size + j;

            // Create sphere for node
            let mut sphere = vtk::SphereSource::new();
            sphere.set_radius(0.15);
            sphere.set_center([x, y, z]);
            sphere.set_theta_resolution(16);
            sphere.set_phi_resolution(16);

            let mut mapper = vtk::PolyDataMapper::new();
            mapper.set_input_connection(sphere.get_output_port());

            let mut actor = vtk::Actor::new();
            actor.set_mapper(&mut mapper);
            actor.get_property().set_color(0.3, 0.7, 1.0); // Light blue
            renderer.add_actor(&mut actor);
            node_actors.push(actor);

            // Create billboard text label for node ID
            let mut text = vtk::VectorText::new();
            text.set_text(&format!("{}", node_id));

            let mut text_mapper = vtk::PolyDataMapper::new();
            text_mapper.set_input_connection(text.output_port());

            // Create Follower (billboard actor that always faces camera)
            let mut follower = vtk::Follower::new();

            // Set the mapper for the follower
            follower.set_mapper(&mut text_mapper);

            // Position label above the node
            follower.set_position(x, y, z + 0.5);

            // Scale the text to appropriate size
            follower.set_scale(0.3, 0.3, 0.3);

            // Don't add to renderer yet - need to set camera first
            label_followers.push(follower);
            label_mappers.push(text_mapper);
            label_texts.push(text);
        }
    }

    println!("Created {} nodes with billboard labels", grid_size * grid_size);

    // Setup camera for good initial view
    let mut camera = renderer.get_active_camera();
    camera.set_position(15.0, 15.0, 15.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);
    renderer.reset_camera();

    // Get camera position for initial distance-based scaling
    let cam_pos = camera.get_position();

    // Assign camera to all followers and set initial scale
    for follower in &mut label_followers {
        follower.set_camera_ref(&mut camera);

        // Calculate initial distance-based scale
        let label_pos = follower.get_position();
        let dx = cam_pos.0 - label_pos[0];
        let dy = cam_pos.1 - label_pos[1];
        let dz = cam_pos.2 - label_pos[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        let scale = distance * 0.015;
        follower.set_scale(scale, scale, scale);

        // Add follower to renderer using safe wrapper
        renderer.add_follower(follower);
    }

    println!("Registering camera observer with closure callback...");

    // Register a closure-based observer on the camera
    // The closure captures follower_refs and updates their scales when camera moves
    let follower_refs: Vec<vtk::FollowerRef> = label_followers
        .iter_mut()
        .map(|f| vtk::FollowerRef::from_follower(f))
        .collect();

    let (_observer_tag, _command) = camera.on_modified(move |camera_ref| {
        let (x, y, z) = camera_ref.get_position();

        for follower_ref in &follower_refs {
            let label_pos = follower_ref.get_position();

            // Calculate distance from camera to label
            let dx = x - label_pos[0];
            let dy = y - label_pos[1];
            let dz = z - label_pos[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            // Scale proportional to distance for constant screen size
            let scale = distance * 0.015;
            follower_ref.set_scale(scale, scale, scale);
        }
    });

    println!("Observer registered successfully");

    println!("\n=== Rendering Configuration ===");
    println!("Window size: 1400x900");
    println!("Camera position: (15, 15, 15)");
    println!("Camera focal point: (0, 0, 0)");
    println!("Background: Dark blue (0.1, 0.1, 0.15)");

    println!("\n=== Billboard Text Behavior ===");
    println!("- Text labels ALWAYS face the camera (billboard effect)");
    println!("- Text scale DYNAMICALLY ADJUSTS to maintain constant screen size!");
    println!("- Uses closure-based camera observer (safe Rust API)");
    println!("- Perfect for FEM node IDs, point annotations, markers");

    println!("\n=== Controls ===");
    println!("- Left mouse: Rotate view");
    println!("- Middle mouse: Pan");
    println!("- Right mouse / Scroll: Zoom");
    println!("- Notice: Labels STAY SAME SIZE when you zoom in/out!");

    println!("\nImplementation: Closure callback fires on camera MODIFIED_EVENT");
    println!("No unsafe blocks or manual pointer management needed!");

    // Keep command alive for the duration of the program
    // (stored in _command, which will be dropped at end of main)

    render_window.render();
    interactor.start();
}
