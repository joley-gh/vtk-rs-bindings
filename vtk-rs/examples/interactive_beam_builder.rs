use vtk_rs::*;
use std::sync::{ Arc, Mutex };

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProjectionMode {
    ConstantDepth, // Project at constant depth from camera (default, most stable)
    ZPlane, // Project onto z=0 plane (good for 2D-like structures)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InteractionMode {
    AddNode, // Click to add new nodes
    AddBeam, // Click two nodes to create beam
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BeamSelection {
    first_node: Option<usize>,
}

impl BeamSelection {
    fn new() -> Self {
        BeamSelection { first_node: None }
    }

    fn is_empty(&self) -> bool {
        self.first_node.is_none()
    }

    fn clear(&mut self) {
        self.first_node = None;
    }

    fn set_first(&mut self, node_id: usize) {
        self.first_node = Some(node_id);
    }

    fn get_first(&self) -> Option<usize> {
        self.first_node
    }
}

/// Shared visualization state that can be updated from callbacks
struct VisualizationState {
    nodes: Vec<[f64; 3]>,
    beams: Vec<[usize; 2]>, // Beam connectivity: pairs of node IDs

    // Node rendering
    node_points: *mut Points,
    node_poly_data: *mut PolyData,

    // Beam rendering
    beam_points: *mut Points,
    beam_lines: *mut CellArray,
    beam_poly_data: *mut PolyData,

    window: *mut RenderWindow,
    renderer: *mut Renderer,
    picker: *mut CellPicker,
    node_actor: *mut Actor,

    projection_mode: ProjectionMode,
    interaction_mode: InteractionMode,
    beam_selection: BeamSelection,
}

unsafe impl Send for VisualizationState {}
unsafe impl Sync for VisualizationState {}

impl VisualizationState {
    fn new(
        node_points: *mut Points,
        node_poly_data: *mut PolyData,
        beam_points: *mut Points,
        beam_lines: *mut CellArray,
        beam_poly_data: *mut PolyData,
        window: *mut RenderWindow,
        renderer: *mut Renderer,
        picker: *mut CellPicker,
        node_actor: *mut Actor
    ) -> Self {
        VisualizationState {
            nodes: Vec::new(),
            beams: Vec::new(),
            node_points,
            node_poly_data,
            beam_points,
            beam_lines,
            beam_poly_data,
            window,
            renderer,
            picker,
            node_actor,
            projection_mode: ProjectionMode::ConstantDepth,
            interaction_mode: InteractionMode::AddNode,
            beam_selection: BeamSelection::new(),
        }
    }

    fn toggle_mode(&mut self) {
        self.interaction_mode = match self.interaction_mode {
            InteractionMode::AddNode => {
                println!("\n🔄 Switched to ADD BEAM mode - click two nodes to create beam");
                InteractionMode::AddBeam
            }
            InteractionMode::AddBeam => {
                println!("\n🔄 Switched to ADD NODE mode - click to place nodes");
                self.beam_selection.clear();
                InteractionMode::AddNode
            }
        };
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
                    // Get near and far points along viewing ray
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

                    // Calculate normalized ray direction
                    let ray_dir = [
                        far_point[0] - near_point[0],
                        far_point[1] - near_point[1],
                        far_point[2] - near_point[2],
                    ];

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
                    // Ray-plane intersection with z=0
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

                    let ray_dir = [
                        far_point[0] - near_point[0],
                        far_point[1] - near_point[1],
                        far_point[2] - near_point[2],
                    ];

                    if ray_dir[2].abs() > 1e-10 {
                        let t = -near_point[2] / ray_dir[2];
                        [near_point[0] + t * ray_dir[0], near_point[1] + t * ray_dir[1], 0.0]
                    } else {
                        [near_point[0], near_point[1], 0.0]
                    }
                }
            }
        }
    }

    fn pick_node_at_screen_pos(&mut self, screen_x: i32, screen_y: i32) -> Option<usize> {
        unsafe {
            if self.picker.is_null() || self.renderer.is_null() {
                return None;
            }

            // Get the raw vtkRenderer* from our Renderer wrapper
            let vtk_renderer_ptr = (*self.renderer).as_mut_ptr();

            // Perform pick at screen coordinates
            // CellPicker works well with Glyph3D by picking the underlying point data
            let picked = (*self.picker).pick_with_ptr(
                screen_x as f64,
                screen_y as f64,
                0.0,
                vtk_renderer_ptr as *mut _
            );

            if !picked {
                return None;
            }

            // Get the picked 3D position in world coordinates
            let (pick_x, pick_y, pick_z) = (*self.picker).get_pick_position();

            // Find the closest node to the picked position
            // This works because CellPicker gives us the pick position on the glyph
            let mut closest_idx = None;
            let mut closest_dist_sq = f64::MAX;
            const THRESHOLD: f64 = 0.04; // 0.2² = 0.04 (squared distance threshold)

            for (idx, node_pos) in self.nodes.iter().enumerate() {
                // Early exit optimizations: check each axis independently
                let dx = node_pos[0] - pick_x;
                if dx * dx > THRESHOLD {
                    continue; // X distance alone exceeds threshold
                }

                let dy = node_pos[1] - pick_y;
                if dy * dy > THRESHOLD {
                    continue; // Y distance alone exceeds threshold
                }

                let dz = node_pos[2] - pick_z;
                if dz * dz > THRESHOLD {
                    continue; // Z distance alone exceeds threshold
                }

                // Only compute full distance if all axes are within threshold
                let dist_sq = dx * dx + dy * dy + dz * dz;

                // Use a reasonable threshold (0.2 units = twice the sphere radius)
                if dist_sq < THRESHOLD && dist_sq < closest_dist_sq {
                    closest_dist_sq = dist_sq;
                    closest_idx = Some(idx);
                }
            }

            closest_idx
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

        // Update VTK node structures
        unsafe {
            if !self.node_points.is_null() {
                (*self.node_points).insert_next_point(position[0], position[1], position[2]);
            }

            if !self.node_poly_data.is_null() {
                (*self.node_poly_data).modified();
            }

            if !self.window.is_null() {
                (*self.window).render();
            }
        }

        node_id
    }

    fn add_beam(&mut self, node1: usize, node2: usize) -> usize {
        if node1 >= self.nodes.len() || node2 >= self.nodes.len() {
            println!("❌ Invalid node IDs: {} or {}", node1, node2);
            return self.beams.len();
        }

        if node1 == node2 {
            println!("❌ Cannot create beam with same node twice");
            return self.beams.len();
        }

        let beam_id = self.beams.len();
        self.beams.push([node1, node2]);

        let pos1 = self.nodes[node1];
        let pos2 = self.nodes[node2];

        println!("✓ Added beam {} connecting nodes {} and {}", beam_id, node1, node2);

        // Update VTK beam structures
        unsafe {
            if !self.beam_points.is_null() && !self.beam_lines.is_null() {
                // Add both points to beam_points
                let pt1_id = (*self.beam_points).insert_next_point(
                    pos1[0],
                    pos1[1],
                    pos1[2]
                ) as i64;
                let pt2_id = (*self.beam_points).insert_next_point(
                    pos2[0],
                    pos2[1],
                    pos2[2]
                ) as i64;

                // Create line cell with both point IDs
                (*self.beam_lines).insert_next_cell(&[pt1_id, pt2_id]);

                if !self.beam_poly_data.is_null() {
                    (*self.beam_poly_data).modified();
                }

                if !self.window.is_null() {
                    (*self.window).render();
                }
            }
        }

        beam_id
    }

    fn handle_click(&mut self, screen_x: i32, screen_y: i32) {
        println!("\n=== Click detected at screen position ({}, {}) ===", screen_x, screen_y);

        match self.interaction_mode {
            InteractionMode::AddNode => {
                // Try to pick a node first
                if let Some(node_id) = self.pick_node_at_screen_pos(screen_x, screen_y) {
                    let node_pos = self.nodes[node_id];
                    println!(
                        "Clicked on existing node {} at ({:.2}, {:.2}, {:.2})",
                        node_id,
                        node_pos[0],
                        node_pos[1],
                        node_pos[2]
                    );
                    println!("→ (Node already exists, no action taken)");
                } else {
                    // No node picked - add new node at this location
                    let world_pos = self.screen_to_world(screen_x, screen_y);
                    println!(
                        "Converted to world coordinates: ({:.2}, {:.2}, {:.2}) [mode: {:?}]",
                        world_pos[0],
                        world_pos[1],
                        world_pos[2],
                        self.projection_mode
                    );
                    self.add_node(world_pos);
                }
            }
            InteractionMode::AddBeam => {
                // Use CellPicker to select nodes
                if let Some(node_id) = self.pick_node_at_screen_pos(screen_x, screen_y) {
                    let node_pos = self.nodes[node_id];
                    println!(
                        "Picked node {} at ({:.2}, {:.2}, {:.2})",
                        node_id,
                        node_pos[0],
                        node_pos[1],
                        node_pos[2]
                    );

                    if self.beam_selection.is_empty() {
                        // First node selected
                        self.beam_selection.set_first(node_id);
                        println!("→ First node selected. Click another node to create beam.");
                    } else {
                        // Second node selected - create beam
                        let first_node = self.beam_selection.get_first().unwrap();
                        if first_node != node_id {
                            self.add_beam(first_node, node_id);
                            self.beam_selection.clear();
                            println!(
                                "→ Beam created! Click two more nodes to create another beam."
                            );
                        } else {
                            println!("❌ Cannot create beam: same node selected twice");
                            self.beam_selection.clear();
                        }
                    }
                } else {
                    println!("❌ No node picked at this location");
                    println!("   Click directly on a node sphere to select it");
                    println!("   Or press 'm' to switch back to NODE mode to add new nodes");
                }
            }
        }
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn beam_count(&self) -> usize {
        self.beams.len()
    }

    fn get_nodes(&self) -> &Vec<[f64; 3]> {
        &self.nodes
    }

    fn get_beams(&self) -> &Vec<[usize; 2]> {
        &self.beams
    }
}

fn main() {
    println!("=== Interactive FEM Beam Structure Builder ===\n");
    println!("Instructions:");
    println!("  NODE MODE (default):");
    println!("    - Click in the 3D viewport to place nodes");
    println!("    - Nodes appear as blue spheres");
    println!("  BEAM MODE:");
    println!("    - Click two nodes consecutively to create a beam");
    println!("    - Beams appear as cyan tubes connecting nodes");
    println!("  CONTROLS:");
    println!("    - Press 'm' to toggle between NODE and BEAM modes");
    println!("    - Right-click and drag to rotate view");
    println!("    - Middle-click and drag to pan");
    println!("    - Scroll to zoom");
    println!("    - Press 'q' to exit\n");

    // Create VTK structures for nodes
    let mut node_points = Points::new();
    let mut node_poly_data = PolyData::new();
    node_poly_data.set_points(&mut node_points);

    // Create sphere for node visualization
    let mut sphere_source = SphereSource::new();
    sphere_source.set_radius(0.1);
    sphere_source.set_theta_resolution(16);
    sphere_source.set_phi_resolution(16);

    // Setup Glyph3D for efficient node rendering
    let mut node_glyph = Glyph3D::new();
    node_glyph.set_scale_mode_to_data_scaling_off();
    node_glyph.set_scale_factor(1.0);
    node_glyph.set_input_connection(node_poly_data.get_output_port());

    let sphere_port = SphereSource::get_output_port(&mut sphere_source);
    let sphere_ptr: *mut std::ffi::c_void = sphere_port.into();
    node_glyph.set_source_connection(sphere_ptr as *mut _);

    let mut node_mapper = PolyDataMapper::new();
    node_mapper.set_input_connection(unsafe {
        AlgorithmOutputPort::from_raw(node_glyph.get_output_port() as *mut std::ffi::c_void)
    });

    let mut node_actor = Actor::new();
    node_actor.set_mapper(&mut node_mapper);
    let mut node_property = node_actor.get_property();
    node_property.set_color(0.2, 0.5, 1.0); // Blue nodes

    // Create VTK structures for beams
    let mut beam_points = Points::new();
    let mut beam_lines = CellArray::new();
    let mut beam_poly_data = PolyData::new();
    beam_poly_data.set_points(&mut beam_points);
    beam_poly_data.set_lines(&mut beam_lines);

    // Use TubeFilter for 3D beam rendering
    let mut tube_filter = TubeFilter::new();
    println!("DEBUG: After TubeFilter::new()");

    let beam_output_port = beam_poly_data.get_output_port();
    println!("DEBUG: After beam_poly_data.get_output_port()");

    tube_filter.set_input_connection(beam_output_port as *mut std::ffi::c_void);
    println!("DEBUG: After tube_filter.set_input_connection()");

    tube_filter.set_radius(0.05);
    println!("DEBUG: After tube_filter.set_radius()");

    tube_filter.set_number_of_sides(12);
    println!("DEBUG: After tube_filter.set_number_of_sides()");

    let mut beam_mapper = PolyDataMapper::new();
    println!("DEBUG: After PolyDataMapper::new()");

    beam_mapper.set_input_connection(TubeFilter::get_output_port(&mut tube_filter));
    println!("DEBUG: After beam_mapper.set_input_connection()");

    let mut beam_actor = Actor::new();
    println!("DEBUG: After Actor::new()");

    beam_actor.set_mapper(&mut beam_mapper);
    println!("DEBUG: After beam_actor.set_mapper()");

    let mut beam_property = beam_actor.get_property();
    println!("DEBUG: After beam_actor.get_property()");

    beam_property.set_color(0.0, 1.0, 1.0); // Cyan beams
    println!("DEBUG: After beam_property.set_color()");

    // Setup renderer and window
    let mut renderer = Renderer::new();
    println!("DEBUG: After Renderer::new()");

    renderer.set_background(0.1, 0.1, 0.2);
    println!("DEBUG: After renderer.set_background()");

    renderer.add_actor(&mut node_actor);
    println!("DEBUG: After renderer.add_actor(node_actor)");

    renderer.add_actor(&mut beam_actor);
    println!("DEBUG: After renderer.add_actor(beam_actor)");

    // Setup camera for better initial view
    let mut camera = renderer.get_active_camera();
    println!("DEBUG: After renderer.get_active_camera()");

    camera.set_position(0.5, 0.5, 3.0);
    println!("DEBUG: After camera.set_position()");

    camera.set_focal_point(0.5, 0.5, 0.0);
    println!("DEBUG: After camera.set_focal_point()");

    camera.set_view_up(0.0, 1.0, 0.0);
    println!("DEBUG: After camera.set_view_up()");

    let mut window = RenderWindow::new();
    println!("DEBUG: After RenderWindow::new()");

    window.add_renderer(&mut renderer);
    println!("DEBUG: After window.add_renderer()");

    window.set_size(1920, 1080);
    println!("DEBUG: After window.set_size()");

    window.set_window_name("Interactive Beam Builder - 'm' to toggle mode, click to interact");
    println!("DEBUG: After window.set_window_name()");

    let mut interactor = RenderWindowInteractor::new();
    println!("DEBUG: After RenderWindowInteractor::new()");

    interactor.set_render_window(&mut window);
    println!("DEBUG: After interactor.set_render_window()");

    // Create CellPicker for accurate node selection
    let mut picker = CellPicker::new();
    picker.set_tolerance(0.025); // Picking tolerance - larger value = easier to pick
    println!("DEBUG: After picker configuration");

    // Create visualization state
    let vis_state = Arc::new(
        Mutex::new(
            VisualizationState::new(
                &mut node_points as *mut Points,
                &mut node_poly_data as *mut PolyData,
                &mut beam_points as *mut Points,
                &mut beam_lines as *mut CellArray,
                &mut beam_poly_data as *mut PolyData,
                &mut window as *mut RenderWindow,
                &mut renderer as *mut Renderer,
                &mut picker as *mut CellPicker,
                &mut node_actor as *mut Actor
            )
        )
    );
    println!("DEBUG: After VisualizationState::new()");

    // Add initial demonstration nodes in a grid
    {
        let mut state = vis_state.lock().unwrap();
        state.add_node([0.0, 0.0, 0.0]);
        state.add_node([1.0, 0.0, 0.0]);
        state.add_node([0.0, 1.0, 0.0]);
        state.add_node([1.0, 1.0, 0.0]);
        println!("\n✓ Added 4 initial nodes in a square pattern");
        println!("→ Try switching to BEAM mode (press 'm') and connecting them!\n");
    }

    // Setup custom interactor style with callbacks
    let mut style = InteractorStyleCustom::new();

    // Left click callback - handle both node and beam creation
    let vis_state_clone = Arc::clone(&vis_state);
    style.set_left_button_press_callback(move |x, y| {
        let mut state = vis_state_clone.lock().unwrap();
        state.handle_click(x, y);
    });

    // Key press callback for mode switching
    let vis_state_clone = Arc::clone(&vis_state);
    style.set_key_press_callback(move |key| {
        let mut state = vis_state_clone.lock().unwrap();

        match key.to_lowercase().as_str() {
            "m" => {
                state.toggle_mode();
            }
            _ => {}
        }
        true
    });

    interactor.set_interactor_style_custom(&mut style);

    println!("=== Starting Interactive Session ===");
    println!("Current mode: ADD NODE");
    println!("Window is now active!");
    println!("→ Click to add nodes, press 'm' to switch to BEAM mode.\n");

    window.render();
    interactor.start();

    // Print final structure summary
    let state = vis_state.lock().unwrap();
    println!("\n=== Final Structure Summary ===");
    println!("Total nodes: {}", state.node_count());
    println!("Total beams: {}", state.beam_count());

    println!("\nNode List:");
    for (i, node) in state.get_nodes().iter().enumerate() {
        println!("  Node {}: ({:.2}, {:.2}, {:.2})", i, node[0], node[1], node[2]);
    }

    println!("\nBeam Connectivity:");
    for (i, beam) in state.get_beams().iter().enumerate() {
        println!("  Beam {}: Node {} → Node {}", i, beam[0], beam[1]);
    }
}
