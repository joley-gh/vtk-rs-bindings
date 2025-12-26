use vtk_rs::*;

fn main() {
    println!("\n=== VTK Geometric Primitives Showcase ===\n");
    println!("Demonstrating all 7 basic VTK geometric primitives:");
    println!("1. Cone (red) - Top left");
    println!("2. Cylinder (green) - Top center");
    println!("3. Cube (blue) - Top right");
    println!("4. Plane (gold) - Bottom left");
    println!("5. Disk (purple) - Bottom center");
    println!("6. Arrow (cyan) - Bottom right");
    println!("7. Regular Polygon/Hexagon (orange) - Center\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.15, 0.15, 0.15);

    // 1. CONE - Red (Top Left)
    let mut cone = ConeSource::new();
    cone.set_radius(0.4);
    cone.set_height(0.8);
    cone.set_resolution(20);
    cone.set_direction(0.0, 1.0, 0.0);

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(cone.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    actor1.set_position(-2.5, 2.0, 0.0);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.9, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);

    // 2. CYLINDER - Green (Top Center)
    let mut cylinder = CylinderSource::new();
    cylinder.set_radius(0.3);
    cylinder.set_height(1.0);
    cylinder.set_resolution(24);
    cylinder.set_capping(true);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(cylinder.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    actor2.set_position(0.0, 2.0, 0.0);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);

    // 3. CUBE - Blue (Top Right)
    let mut cube = CubeSource::new();
    cube.set_x_length(0.8);
    cube.set_y_length(0.8);
    cube.set_z_length(0.8);

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(cube.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    actor3.set_position(2.5, 2.0, 0.0);
    actor3.rotate_x(15.0);
    actor3.rotate_y(30.0);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.4, 0.9); // Blue
    renderer.add_actor(&mut actor3);

    // 4. PLANE - Gold (Bottom Left)
    let mut plane = PlaneSource::new();
    plane.set_origin(-3.0, -2.5, 0.0);
    plane.set_point1(-2.0, -2.5, 0.0);
    plane.set_point2(-3.0, -1.5, 0.0);
    plane.set_x_resolution(10);
    plane.set_y_resolution(10);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(plane.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    prop4.set_representation(RepresentationType::Wireframe);
    renderer.add_actor(&mut actor4);

    // 5. DISK - Purple (Bottom Center)
    let mut disk = DiskSource::new();
    disk.set_inner_radius(0.2);
    disk.set_outer_radius(0.5);
    disk.set_radial_resolution(6);
    disk.set_circumferential_resolution(24);
    disk.set_center(0.0, -2.0, 0.0);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(disk.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);

    // 6. ARROW - Cyan (Bottom Right)
    let mut arrow = ArrowSource::new();
    arrow.set_tip_length(0.35);
    arrow.set_tip_radius(0.1);
    arrow.set_shaft_radius(0.03);
    arrow.set_tip_resolution(16);
    arrow.set_shaft_resolution(16);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(arrow.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    actor6.set_position(2.5, -2.0, 0.0);
    actor6.rotate_z(45.0);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    renderer.add_actor(&mut actor6);

    // 7. REGULAR POLYGON (Hexagon) - Orange (Center)
    let mut hexagon = RegularPolygonSource::new();
    hexagon.set_number_of_sides(6);
    hexagon.set_radius(0.5);
    hexagon.set_center(0.0, 0.0, 0.0);
    hexagon.set_normal(0.0, 0.0, 1.0);
    hexagon.generate_polygon_on();

    let mut mapper7 = PolyDataMapper::new();
    mapper7.set_input_connection(hexagon.get_output_port());

    let mut actor7 = Actor::new();
    actor7.set_mapper(&mut mapper7);
    let mut prop7 = actor7.get_property();
    prop7.set_color(0.9, 0.5, 0.1); // Orange
    renderer.add_actor(&mut actor7);

    // Setup rendering
    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1400, 900);
    render_window.set_window_name("VTK-RS Geometric Primitives Showcase");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();
    render_window.render();

    println!("All 7 primitive types displayed in a 3x3 grid layout.");
    println!("Each primitive demonstrates key parameters:");
    println!("- Cone: direction, resolution, radius/height ratio");
    println!("- Cylinder: capping enabled, high resolution");
    println!("- Cube: rotated to show 3D structure");
    println!("- Plane: wireframe mesh with resolution");
    println!("- Disk: washer (ring) shape with radial divisions");
    println!("- Arrow: proportional tip/shaft, angled placement");
    println!("- Hexagon: filled regular polygon");
    println!("\nUse mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");

    interactor.start();
}
