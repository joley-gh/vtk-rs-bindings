use vtk_rs::*;
use std::sync::{ Arc, Mutex };

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProjectionMode {
    ConstantDepth, // Project at constant depth from camera (default, most stable)
    ZPlane, // Project onto z=0 plane (good for 2D-like structures)
}

/// Shared visualization state that can be updated from callbacks
struct VisualizationState {
    nodes: Vec<[f64; 3]>,
    points: *mut Points,
    poly_data: *mut PolyData,
    window: *mut RenderWindow,
    renderer: *mut Renderer,
    projection_mode: ProjectionMode,
}

unsafe impl Send for VisualizationState {}
unsafe impl Sync for VisualizationState {}

impl VisualizationState {
    fn new(
        points: *mut Points,
        poly_data: *mut PolyData,
        window: *mut RenderWindow,
        renderer: *mut Renderer
    ) -> Self {
        VisualizationState {
            nodes: Vec::new(),
            points,
            poly_data,
            window,
            renderer,
            projection_mode: ProjectionMode::ConstantDepth, // Default to constant depth
        }
    }

    fn toggle_projection_mode(&mut self) {
        self.projection_mode = match self.projection_mode {
            ProjectionMode::ConstantDepth => ProjectionMode::ZPlane,
            ProjectionMode::ZPlane => ProjectionMode::ConstantDepth,
        };
        println!("\n🔄 Switched to {:?} projection mode", self.projection_mode);
    }

    fn screen_to_world(&self, screen_x: i32, screen_y: i32) -> [f64; 3] {
        unsafe {
            if self.renderer.is_null() {
                return [0.0, 0.0, 0.0];
            }

            let renderer = &mut *self.renderer;
            let display_x = screen_x as f64;
            let display_y = screen_y as f64;

            match self.projection_mode {
                ProjectionMode::ConstantDepth => {
                    // Project at a constant depth from the camera
                    // This is the most stable method - uses a fixed distance along the view ray

                    // Get two points along the viewing ray
                    renderer.set_display_point(display_x, display_y, 0.0);
                    renderer.display_to_world();
                    let (x1, y1, z1, w1) = renderer.get_world_point();
                    let near_point = if w1 != 0.0 {
                        [x1 / w1, y1 / w1, z1 / w1]
                    } else {
                        [x1, y1, z1]
                    };

                    renderer.set_display_point(display_x, display_y, 1.0);
                    renderer.display_to_world();
                    let (x2, y2, z2, w2) = renderer.get_world_point();
                    let far_point = if w2 != 0.0 {
                        [x2 / w2, y2 / w2, z2 / w2]
                    } else {
                        [x2, y2, z2]
                    };

                    // Calculate ray direction
                    let ray_dir = [
                        far_point[0] - near_point[0],
                        far_point[1] - near_point[1],
                        far_point[2] - near_point[2],
                    ];

                    // Normalize ray direction
                    let ray_length = (
                        ray_dir[0] * ray_dir[0] +
                        ray_dir[1] * ray_dir[1] +
                        ray_dir[2] * ray_dir[2]
                    ).sqrt();

                    if ray_length > 1e-10 {
                        let ray_unit = [
                            ray_dir[0] / ray_length,
                            ray_dir[1] / ray_length,
                            ray_dir[2] / ray_length,
                        ];

                        // Use a constant depth (e.g., distance from focal point to camera)
                        // Or use a reasonable fixed depth like 5.0 units
                        let depth = 5.0; // Fixed depth in world units

                        [
                            near_point[0] + ray_unit[0] * depth,
                            near_point[1] + ray_unit[1] * depth,
                            near_point[2] + ray_unit[2] * depth,
                        ]
                    } else {
                        near_point
                    }
                }
                ProjectionMode::ZPlane => {
                    // Project onto z=0 world plane via ray-plane intersection
                    // Get near and far points to construct ray
                    renderer.set_display_point(display_x, display_y, 0.0);
                    renderer.display_to_world();
                    let (x1, y1, z1, w1) = renderer.get_world_point();
                    let near_point = if w1 != 0.0 {
                        [x1 / w1, y1 / w1, z1 / w1]
                    } else {
                        [x1, y1, z1]
                    };

                    renderer.set_display_point(display_x, display_y, 1.0);
                    renderer.display_to_world();
                    let (x2, y2, z2, w2) = renderer.get_world_point();
                    let far_point = if w2 != 0.0 {
                        [x2 / w2, y2 / w2, z2 / w2]
                    } else {
                        [x2, y2, z2]
                    };

                    // Ray direction
                    let ray_dir = [
                        far_point[0] - near_point[0],
                        far_point[1] - near_point[1],
                        far_point[2] - near_point[2],
                    ];

                    // Intersect with z=0 plane
                    if ray_dir[2].abs() > 1e-10 {
                        let t = -near_point[2] / ray_dir[2];
                        [near_point[0] + t * ray_dir[0], near_point[1] + t * ray_dir[1], 0.0]
                    } else {
                        // Ray parallel to plane
                        [near_point[0], near_point[1], 0.0]
                    }
                }
            }
        }
    }

    fn add_node(&mut self, position: [f64; 3]) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(position);

        println!(
            "✓ Added node {} at ({:.2}, {:.2}, {:.2})",
            node_id,
            position[0],
            position[1],
            position[2]
        );

        // Update VTK structures
        unsafe {
            if !self.points.is_null() {
                (*self.points).insert_next_point(position[0], position[1], position[2]);
            }

            if !self.poly_data.is_null() {
                (*self.poly_data).modified();
            }

            // Trigger re-render
            if !self.window.is_null() {
                (*self.window).render();
            }
        }

        node_id
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn get_nodes(&self) -> &Vec<[f64; 3]> {
        &self.nodes
    }
}

fn main() {
    println!("=== Interactive FEM Node Placement ===\n");
    println!("Instructions:");
    println!("  - Click anywhere in the 3D viewport to place nodes");
    println!("  - Nodes appear at your click position as blue spheres");
    println!("  - Node positions are shown in console");
    println!("  - Right-click and drag to rotate view");
    println!("  - Middle-click and drag to pan");
    println!("  - Scroll to zoom");
    println!("  - Press 'q' to exit\n");
    println!("📍 Mode: Focal Plane projection (nodes at camera focal depth)");
    println!("   → Try rotating the view and clicking - nodes maintain depth!\n");

    // Create VTK visualization structures
    let mut points = Points::new();
    let mut poly_data = PolyData::new();

    // Create sphere for glyph visualization
    let mut sphere_source = SphereSource::new();
    sphere_source.set_radius(0.1);
    sphere_source.set_theta_resolution(16);
    sphere_source.set_phi_resolution(16);

    // Setup Glyph3D for efficient node rendering
    let mut glyph = Glyph3D::new();
    glyph.set_scale_mode_to_data_scaling_off();
    glyph.set_scale_factor(1.0);

    poly_data.set_points(&mut points);

    glyph.set_input_connection(poly_data.get_output_port());
    glyph.set_source_connection(SphereSource::get_output_port(&mut sphere_source));

    let mut mapper = PolyDataMapper::new();
    mapper.set_input_connection(glyph.get_output_port());

    let mut actor = Actor::new();
    actor.set_mapper(&mut mapper);

    // Blue nodes
    let mut property = actor.get_property();
    property.set_color(0.2, 0.5, 1.0);

    // Setup renderer and window
    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.2);
    renderer.add_actor(&mut actor);

    let mut window = RenderWindow::new();
    window.add_renderer(&mut renderer);
    window.set_size(1920, 1080); // Larger window, closer to fullscreen
    window.set_window_name("Interactive Node Placement - Focal Plane (press 'p' to toggle)");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut window);

    // Create visualization state with raw pointers for callback access
    let vis_state = Arc::new(
        Mutex::new(
            VisualizationState::new(
                &mut points as *mut Points,
                &mut poly_data as *mut PolyData,
                &mut window as *mut RenderWindow,
                &mut renderer as *mut Renderer
            )
        )
    );

    // Add initial demonstration nodes
    {
        let mut state = vis_state.lock().unwrap();
        state.add_node([0.0, 0.0, 0.0]);
        state.add_node([1.0, 0.0, 0.0]);
        state.add_node([0.0, 1.0, 0.0]);
        state.add_node([1.0, 1.0, 0.0]);
        println!("\n✓ Added 4 initial nodes");
    }

    // Set up better camera view
    renderer.reset_camera();
    let mut camera = renderer.get_active_camera();
    camera.set_position(0.5, 0.5, 3.0); // Look down at the plane from above
    camera.set_focal_point(0.5, 0.5, 0.0);
    camera.set_view_up(0.0, 1.0, 0.0);

    // Setup custom interactor style with callback
    let mut style = InteractorStyleCustom::new();

    // Create callback for left button press (node placement)
    let vis_state_clone = Arc::clone(&vis_state);

    style.set_left_button_press_callback(move |x, y| {
        println!("\n=== Click detected at screen position ({}, {}) ===", x, y);

        let mut state = vis_state_clone.lock().unwrap();

        // Convert screen coordinates to world coordinates
        let position = state.screen_to_world(x, y);

        println!(
            "Converted to world coordinates: ({:.2}, {:.2}, {:.2}) [mode: {:?}]",
            position[0],
            position[1],
            position[2],
            state.projection_mode
        );

        state.add_node(position);

        println!("✓ Node added at click position! Click to add more, or press 'q' to exit");
    });

    interactor.set_interactor_style_custom(&mut style);

    println!("=== Starting Interactive Session ===");
    println!("Window is now active - click anywhere to place nodes!");
    println!("Nodes will appear at your click position as blue spheres.\n");

    window.render();
    interactor.start();

    // Print final node list
    let state = vis_state.lock().unwrap();
    println!("\n=== Final Node List ===");
    println!("Total nodes placed: {}", state.node_count());
    for (i, node) in state.get_nodes().iter().enumerate() {
        println!("  Node {}: ({:.2}, {:.2}, {:.2})", i, node[0], node[1], node[2]);
    }
}
