use vtk_rs as vtk;

fn main() {
    println!("=== VTK Text Actor Demo - 2D HUD Overlay ===\n");
    println!("Demonstrating vtkTextActor with normalized coordinates");
    println!("Text automatically maintains relative positioning when window is resized\n");

    // Initialize VTK
    vtk::init_vtk();

    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.1, 0.2);

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK Text Actor Demo - Resize Window to Test!");

    // Window dimensions for positioning
    let window_width = 1200;
    let window_height = 800;

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Create a simple 3D object for context
    let mut sphere = vtk::SphereSource::new();
    sphere.set_radius(2.0);
    sphere.set_theta_resolution(32);
    sphere.set_phi_resolution(32);

    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere.get_output_port());

    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);
    actor.get_property().set_color(0.3, 0.7, 1.0);
    renderer.add_actor(&mut actor);

    // Create text actors with normalized coordinates [0,1]
    // Title text - top center
    let mut title = vtk::TextActor::new();
    title.set_input("VTK Text Actor Demo - Resize Me!");
    title.set_position_normalized(0.25, 0.93, window_width, window_height);

    let mut title_prop = title.get_text_property();
    title_prop.set_font_size(28);
    title_prop.set_color(1.0, 1.0, 1.0); // White
    title_prop.set_bold(true);
    title_prop.set_font_family_to_arial();

    // Status text - bottom left
    let mut status = vtk::TextActor::new();
    status.set_input("Status: Interactive 3D Scene");
    status.set_position_normalized(0.02, 0.02, window_width, window_height);

    let mut status_prop = status.get_text_property();
    status_prop.set_font_size(14);
    status_prop.set_color(0.3, 1.0, 0.3); // Green
    status_prop.set_font_family_to_courier(); // Monospace

    // Info text - top right
    let mut info = vtk::TextActor::new();
    info.set_input("Normalized coordinates\nResize window to see\ntext stay in place");
    info.set_position_normalized(0.7, 0.75, window_width, window_height);

    let mut info_prop = info.get_text_property();
    info_prop.set_font_size(13);
    info_prop.set_color(1.0, 0.8, 0.3); // Yellow
    info_prop.set_italic(true);

    // Instructions - bottom center
    let mut instructions = vtk::TextActor::new();
    instructions.set_input("Left: Rotate | Middle: Pan | Right: Zoom | Resize Window!");
    instructions.set_position_normalized(0.15, 0.05, window_width, window_height);

    let mut inst_prop = instructions.get_text_property();
    inst_prop.set_font_size(12);
    inst_prop.set_color(0.7, 0.7, 0.7); // Gray

    // Label with transparency - left side middle
    let mut label = vtk::TextActor::new();
    label.set_input("Semi-transparent\nText Overlay");
    label.set_position_normalized(0.02, 0.45, window_width, window_height);

    let mut label_prop = label.get_text_property();
    label_prop.set_font_size(16);
    label_prop.set_color(1.0, 0.5, 0.0); // Orange
    label_prop.set_opacity(0.8); // Semi-transparent
    label_prop.set_bold(true);
    label_prop.set_font_family_to_times();

    // Top-right corner marker - for testing normalized coordinates
    let mut top_right = vtk::TextActor::new();
    top_right.set_input("TOP RIGHT\n(0.90, 0.90)");
    top_right.set_position_normalized(0.9, 0.9, window_width, window_height);

    let mut top_right_prop = top_right.get_text_property();
    top_right_prop.set_font_size(14);
    top_right_prop.set_color(1.0, 0.3, 0.3); // Red
    top_right_prop.set_bold(true);
    // Anchor text to top-right corner (position becomes top-right corner of text)
    top_right_prop.set_justification_to_right();
    top_right_prop.set_vertical_justification_to_top();

    // Add all text actors to renderer
    renderer.add_actor(&mut title);
    renderer.add_actor(&mut status);
    renderer.add_actor(&mut info);
    renderer.add_actor(&mut instructions);
    renderer.add_actor(&mut label);
    renderer.add_actor(&mut top_right);

    renderer.reset_camera();

    println!("=== Text Actor Configuration ===");
    println!("Position format: Normalized coordinates [0,1]");
    println!("Window size: {}x{}", window_width, window_height);
    println!("(0,0) = Bottom-left corner, (1,1) = Top-right corner");
    println!();
    println!("Title: (0.50, 0.95) - Top center (centered, top-aligned)");
    println!("Status: (0.02, 0.02) - Bottom left (left, bottom-aligned)");
    println!("Info: (0.98, 0.70) - Top right (right, top-aligned)");
    println!("Instructions: (0.50, 0.02) - Bottom center (centered, bottom-aligned)");
    println!("Label: (0.02, 0.50) - Left middle (left, vertically-centered)");
    println!("Top Right Marker: (0.90, 0.90) - Corner test (right, top-aligned)");
    println!();
    println!("=== Features Demonstrated ===");
    println!("✓ Normalized coordinates [0,1] (business logic in Rust)");
    println!("✓ Text automatically maintains relative positioning on resize");
    println!("✓ Multiple text actors with different positions");
    println!("✓ Font size control (12-28 points)");
    println!("✓ Font families (Arial, Courier, Times)");
    println!("✓ Bold and italic styles");
    println!("✓ Color customization (RGB)");
    println!("✓ Opacity/transparency control");
    println!("✓ Multi-line text support");
    println!();
    println!("Try resizing the window - text maintains relative positions automatically!");
    println!("Interact with the 3D sphere - text stays in place!");

    // Use TextActorResizeManager to automatically handle window resize
    // This keeps the text actors and sets up the resize observer
    let _resize_manager = vtk::TextActorResizeManager::new(
        &mut render_window,
        vec![title, status, info, instructions, label, top_right]
    );

    render_window.render();
    interactor.start();
}
