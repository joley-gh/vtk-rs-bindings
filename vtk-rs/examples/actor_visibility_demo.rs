use std::sync::{ Arc, Mutex };
use vtk_rs::*;

fn main() {
    println!("=== Actor Visibility and Pickability Demo ===\n");

    // Create the renderer, window, and interactor
    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.1);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("Actor Visibility & Pickability Demo");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Create three spheres with different properties

    // Red sphere - visible and pickable (default)
    let mut sphere_source_a = SphereSource::new();
    sphere_source_a.set_center([-3.0, 0.0, 0.0]);
    sphere_source_a.set_radius(1.0);

    let mut mapper_a = PolyDataMapper::new();
    mapper_a.set_input_connection(sphere_source_a.get_output_port());

    let mut actor_a = Actor::new();
    actor_a.set_mapper(&mut mapper_a);
    actor_a.get_property().set_color(1.0, 0.2, 0.2);
    renderer.add_actor(&mut actor_a);

    // Green sphere - will be HIDDEN
    let mut sphere_source_b = SphereSource::new();
    sphere_source_b.set_center([0.0, 0.0, 0.0]);
    sphere_source_b.set_radius(1.0);

    let mut mapper_b = PolyDataMapper::new();
    mapper_b.set_input_connection(sphere_source_b.get_output_port());

    let mut actor_b = Actor::new();
    actor_b.set_mapper(&mut mapper_b);
    actor_b.get_property().set_color(0.2, 1.0, 0.2);
    renderer.add_actor(&mut actor_b);

    // Blue sphere - visible but NOT pickable
    let mut sphere_source_c = SphereSource::new();
    sphere_source_c.set_center([3.0, 0.0, 0.0]);
    sphere_source_c.set_radius(1.0);

    let mut mapper_c = PolyDataMapper::new();
    mapper_c.set_input_connection(sphere_source_c.get_output_port());

    let mut actor_c = Actor::new();
    actor_c.set_mapper(&mut mapper_c);
    actor_c.get_property().set_color(0.2, 0.2, 1.0);
    renderer.add_actor(&mut actor_c);

    // Test initial states (all should be visible and pickable by default)
    println!("Initial states:");
    println!(
        "  Red sphere:   visibility={}, pickable={}",
        actor_a.get_visibility(),
        actor_a.get_pickable()
    );
    println!(
        "  Green sphere: visibility={}, pickable={}",
        actor_b.get_visibility(),
        actor_b.get_pickable()
    );
    println!(
        "  Blue sphere:  visibility={}, pickable={}",
        actor_c.get_visibility(),
        actor_c.get_pickable()
    );
    println!();

    // Modify properties
    actor_b.set_visibility(false); // Hide green sphere
    actor_c.set_pickable(false); // Make blue sphere non-pickable

    println!("After modifications:");
    println!(
        "  Red sphere:   visibility={}, pickable={}",
        actor_a.get_visibility(),
        actor_a.get_pickable()
    );
    println!(
        "  Green sphere: visibility={}, pickable={} (HIDDEN)",
        actor_b.get_visibility(),
        actor_b.get_pickable()
    );
    println!(
        "  Blue sphere:  visibility={}, pickable={} (NOT PICKABLE)",
        actor_c.get_visibility(),
        actor_c.get_pickable()
    );
    println!();

    // Now wrap renderer in Arc<Mutex> for the callback
    let renderer_arc = Arc::new(Mutex::new(renderer));
    let renderer_clone = Arc::clone(&renderer_arc);

    // Create cell picker for interactive picking (also wrapped for the callback)
    let picker = Arc::new(Mutex::new(CellPicker::new()));
    picker.lock().unwrap().set_tolerance(0.005);
    let picker_clone = Arc::clone(&picker);

    // Create custom interactor style with picking callback
    let mut style = InteractorStyleCustom::new();
    style.set_left_button_press_callback(move |x, y| {
        let mut renderer_guard = renderer_clone.lock().unwrap();
        let mut picker_guard = picker_clone.lock().unwrap();

        // Perform pick at mouse coordinates
        picker_guard.pick(x as f64, y as f64, 0.0, &mut *renderer_guard);

        // Check if an actor was picked
        let picked_actor = picker_guard.get_actor();
        if !picked_actor.is_null() {
            println!("Picked an actor at ({}, {})", x, y);

            // Get the picked position
            let pos = picker_guard.get_pick_position();
            println!("  Pick position: ({:.2}, {:.2}, {:.2})", pos.0, pos.1, pos.2);
        } else {
            println!("No actor picked at ({}, {})", x, y);
        }
    });

    interactor.set_interactor_style_custom(&mut style);

    println!("\n=== Instructions ===");
    println!("You should see:");
    println!("  - RED sphere on the left (visible & pickable)");
    println!("  - GREEN sphere is HIDDEN");
    println!("  - BLUE sphere on the right (visible but NOT pickable)");
    println!("\nClick on the spheres to test picking:");
    println!("  - Red sphere: should print pick information");
    println!("  - Blue sphere: should print 'No actor picked' (not pickable)");
    println!("  - Green sphere: invisible, cannot be clicked");
    println!("\nPress 'q' to quit.\n");

    render_window.render();
    interactor.start();
}
