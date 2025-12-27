use vtk_rs as vtk;

fn main() {
    println!("=== VTK Text Source Demo ===\n");
    println!("Demonstrating 3D extruded text rendering");
    println!("6 text objects showing different text and backing options\n");

    // Initialize VTK
    vtk::init_vtk();

    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.2, 0.3);

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK Text Source Demo - 3D Extruded Text");

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Row 1: Text without backing
    println!("Row 1: Text without backing plane");

    // Text 1: "VTK" - Red, no backing
    println!("  Text 1: 'VTK' (Red, no backing)");
    let mut text1 = vtk::TextSource::new();
    text1.set_text("VTK");
    text1.set_backing(false);

    let mut mapper1 = vtk::PolyDataMapper::new();
    mapper1.set_input_connection(text1.get_output_port());

    let mut actor1 = vtk::Actor::new();
    actor1.set_mapper(&mut mapper1);
    actor1.get_property().set_color(1.0, 0.0, 0.0);
    actor1.set_position(0.0, 2.0, 0.0);
    renderer.add_actor(&mut actor1);

    // Text 2: "Rust" - Green, no backing
    println!("  Text 2: 'Rust' (Green, no backing)");
    let mut text2 = vtk::TextSource::new();
    text2.set_text("Rust");
    text2.set_backing(false);

    let mut mapper2 = vtk::PolyDataMapper::new();
    mapper2.set_input_connection(text2.get_output_port());

    let mut actor2 = vtk::Actor::new();
    actor2.set_mapper(&mut mapper2);
    actor2.get_property().set_color(0.0, 1.0, 0.0);
    actor2.set_position(0.0, 20.0, 0.0);
    renderer.add_actor(&mut actor2);

    // Text 3: "3D" - Blue, no backing
    println!("  Text 3: '3D' (Blue, no backing)");
    let mut text3 = vtk::TextSource::new();
    text3.set_text("3D");
    text3.set_backing(false);

    let mut mapper3 = vtk::PolyDataMapper::new();
    mapper3.set_input_connection(text3.get_output_port());

    let mut actor3 = vtk::Actor::new();
    actor3.set_mapper(&mut mapper3);
    actor3.get_property().set_color(0.0, 0.0, 1.0);
    actor3.set_position(0.0, 40.0, 0.0);
    renderer.add_actor(&mut actor3);

    // Row 2: Text with backing plane
    println!("\nRow 2: Text with backing plane");

    // Text 4: "Text" - Yellow, with backing
    println!("  Text 4: 'Text' (Yellow, with backing)");
    let mut text4 = vtk::TextSource::new();
    text4.set_text("Text");
    text4.set_backing(true);

    let mut mapper4 = vtk::PolyDataMapper::new();
    mapper4.set_input_connection(text4.get_output_port());

    let mut actor4 = vtk::Actor::new();
    actor4.set_mapper(&mut mapper4);
    actor4.get_property().set_color(1.0, 1.0, 0.0);
    actor4.set_position(0.0, 60.0, 0.0);
    renderer.add_actor(&mut actor4);

    // Text 5: "Source" - Magenta, with backing
    println!("  Text 5: 'Source' (Magenta, with backing)");
    let mut text5 = vtk::TextSource::new();
    text5.set_text("Source");
    text5.set_backing(true);

    let mut mapper5 = vtk::PolyDataMapper::new();
    mapper5.set_input_connection(text5.get_output_port());

    let mut actor5 = vtk::Actor::new();
    actor5.set_mapper(&mut mapper5);
    actor5.get_property().set_color(1.0, 0.0, 1.0);
    actor5.set_position(0.0, 80.0, 0.0);
    renderer.add_actor(&mut actor5);

    // Text 6: "Demo" - Cyan, with backing
    println!("  Text 6: 'Demo' (Cyan, with backing)");
    let mut text6 = vtk::TextSource::new();
    text6.set_text("Demo");
    text6.set_backing(true);

    let mut mapper6 = vtk::PolyDataMapper::new();
    mapper6.set_input_connection(text6.get_output_port());

    let mut actor6 = vtk::Actor::new();
    actor6.set_mapper(&mut mapper6);
    actor6.get_property().set_color(0.0, 1.0, 1.0);
    actor6.set_position(0.0, 100.0, 0.0);
    renderer.add_actor(&mut actor6);

    // Test get_text() method
    println!("\n=== Testing get_text() method ===");
    println!("Text 1 content: '{}'", text1.get_text());
    println!("Text 4 content: '{}'", text4.get_text());

    // Test get_backing() method
    println!("\n=== Testing get_backing() method ===");
    println!("Text 1 backing: {}", text1.get_backing());
    println!("Text 4 backing: {}", text4.get_backing());

    // Setup camera
    let mut camera = renderer.get_active_camera();
    camera.set_position(0.0, 0.0, 25.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    renderer.reset_camera();

    println!("\n=== Rendering Configuration ===");
    println!("Window size: 1200x800");
    println!("Camera position: (0, 0, 25)");
    println!("Camera focal point: (0, 0, 0)");
    println!("Background: Dark blue (0.1, 0.2, 0.3)");

    println!("\n=== Layout ===");
    println!("Row 1 (y=2.0): VTK, Rust, 3D - No backing plane");
    println!("Row 2 (y=-2.0): Text, Source, Demo - With backing plane");
    println!("\nRotate the view with the mouse to see the 3D extrusion!");

    render_window.render();
    interactor.start();
}
