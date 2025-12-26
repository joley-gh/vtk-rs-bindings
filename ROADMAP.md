# VTK-RS Development Roadmap

Focus: Building a comprehensive VTK wrapper for Rust with core primitives, interaction tools, and visualization capabilities.

---

## Priority 1: Core Geometric Primitives 🎯 CURRENT FOCUS

Essential VTK source objects for building 3D visualizations.

### P1.1 Cone Source - `vtk_cone_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_radius(r)` / `get_radius()`
- [x] `set_height(h)` / `get_height()`
- [x] `set_resolution(n)` / `get_resolution()`
- [x] `set_direction(x, y, z)` / `get_direction()`
- [x] `set_center(x, y, z)` / `get_center()`
- [x] `set_capping(bool)` / `get_capping()`
- [x] `set_angle(degrees)` / `get_angle()`
- [x] `get_output_port()`
- [x] Example: `cone_demo.rs` - 6 different cone configurations with trackball camera

### P1.2 Cylinder Source - `vtk_cylinder_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_radius(r)` / `get_radius()`
- [x] `set_height(h)` / `get_height()`
- [x] `set_resolution(n)` / `get_resolution()`
- [x] `set_center(x, y, z)` / `get_center()`
- [x] `set_capping(bool)` / `get_capping()`
- [x] `get_output_port()`
- [x] Example: `cylinder_demo.rs` - 6 different cylinder configurations (pipes, columns, tubes)

### P1.3 Cube Source - `vtk_cube_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_x_length(len)` / `get_x_length()` / `set_y_length(len)` / `get_y_length()` / `set_z_length(len)` / `get_z_length()`
- [x] `set_center(x, y, z)` / `get_center()`
- [x] `set_bounds(xmin, xmax, ymin, ymax, zmin, zmax)` / `get_bounds()`
- [x] `get_output_port()`
- [x] Example: `cube_demo.rs` - 6 different box shapes (unit cube, tall box, platform, beam, sheet)

### P1.4 Plane Source - `vtk_plane_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_origin(x, y, z)` / `get_origin()`
- [x] `set_point1(x, y, z)` / `get_point1()`
- [x] `set_point2(x, y, z)` / `get_point2()`
- [x] `set_x_resolution(n)` / `get_x_resolution()` / `set_y_resolution(n)` / `get_y_resolution()`
- [x] `set_center(x, y, z)` / `set_normal(x, y, z)`
- [x] `push(distance)` - move along normal
- [x] `get_output_port()`
- [x] Example: `plane_demo.rs` - 6 planes (ground XY, wall XZ, side YZ, high-res mesh, rotated, pushed)

### P1.5 Disk Source - `vtk_disk_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_inner_radius(r)` / `get_inner_radius()`
- [x] `set_outer_radius(r)` / `get_outer_radius()`
- [x] `set_radial_resolution(n)` / `get_radial_resolution()` / `set_circumferential_resolution(n)` / `get_circumferential_resolution()`
- [x] `set_center(x, y, z)` / `get_center()` / `set_normal(x, y, z)` / `get_normal()`
- [x] `get_output_port()`
- [x] Example: `disk_demo.rs` - 6 disks (full disk, washer, low-res, high-res, thick washer, rotated)

### P1.6 Arrow Source - `vtk_arrow_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_arrow_origin_to_default()` / `set_arrow_origin_to_center()` / `get_arrow_origin_as_string()`
- [x] `set_tip_length(len)` / `get_tip_length()` / `set_tip_radius(r)` / `get_tip_radius()` / `set_tip_resolution(n)` / `get_tip_resolution()`
- [x] `set_shaft_radius(r)` / `get_shaft_radius()` / `set_shaft_resolution(n)` / `get_shaft_resolution()`
- [x] `set_invert(bool)` / `invert_on()` / `invert_off()` / `get_invert()`
- [x] `get_output_port()`
- [x] Example: `arrow_demo.rs` - 6 arrows (default, thin, fat, long tip, short tip, inverted)

### P1.7 Regular Polygon Source - `vtk_regular_polygon_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_number_of_sides(n)` / `get_number_of_sides()`
- [x] `set_radius(r)` / `get_radius()`
- [x] `set_center(x, y, z)` / `get_center()`
- [x] `set_normal(x, y, z)` / `get_normal()`
- [x] `set_generate_polygon(bool)` / `generate_polygon_on()` / `generate_polygon_off()` / `get_generate_polygon()`
- [x] `set_generate_polyline(bool)` / `generate_polyline_on()` / `generate_polyline_off()` / `get_generate_polyline()`
- [x] `set_output_points_precision(n)` / `get_output_points_precision()`
- [x] `get_output_port()`
- [x] Example: `regular_polygon_demo.rs` - triangle, square, pentagon, hexagon, octagon, decagon (filled and outline)

### P1.8 Comprehensive Primitives Example ✅ COMPLETE
- [x] Create `examples/geometric_primitives.rs`
- [x] Show all 7 basic shapes in one scene (3x3 grid layout)
- [x] Demonstrate parameter variations for each shape
- [x] Color-code by shape type (red cone, green cylinder, blue cube, etc.)
- [x] Include different representations (solid and wireframe)

---

## ✅ PRIORITY 1 COMPLETE! 🎉

All 7 geometric primitives implemented with comprehensive demos and a showcase example.

---

## Priority 2: Interaction Widgets & Selection

Advanced user interaction capabilities for object selection and manipulation.

### P2.1 Rubber Band Picker - `vtk_interactor_style_rubber_band_pick.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_interactor(interactor)` - attach to render window interactor
- [x] Wrapper for vtkInteractorStyleRubberBandPick
- [x] Note: Not used in final implementation - InteractorStyleCustom with selection_mode provides better control
- [x] Pattern available for future use if native VTK rubber band needed

### P2.2 Area Picker - `vtk_area_picker.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `area_pick(x1, y1, x2, y2, renderer)` → bool
- [x] `get_prop3ds()` → *mut vtkProp3DCollection (raw pointer, needs wrapper)
- [x] Pick all objects within a 2D rectangular region
- [x] **Critical requirement**: Must include `.rs.h` headers for cxx bridge (e.g., `#include "vtk_area_picker.rs.h"`)
- [x] Visual rubber band feedback via optimized frame caching
- [x] Architecture: C++ for VTK glue, Rust for application logic (drawing, caching, console output)
- [x] RenderWindow pixel manipulation: `get_pixel_data()` / `set_pixel_data()`
- [x] InteractorStyleCustom: selection mode, state accessors (`is_moving()`, `get_selection_positions()`)
- [x] Rubber band functions: `draw_rubber_band_rectangle()` (simple), `draw_rubber_band_rectangle_cached()` (optimized)
- [x] Example: `interactive_area_picker.rs` - drag to select with smooth visual feedback, inline coordinate updates

### P2.3 Point Picker - `vtk_point_picker.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `pick(x, y, z, renderer)` → bool
- [x] `get_point_id()` → returns vertex ID (-1 if no pick)
- [x] `get_pick_position()` → (x, y, z)
- [x] Pick individual points (vertices) from mesh geometry
- [x] Unlike WorldPointPicker, returns meaningful success/failure
- [x] Note: SphereSource.get_output_port() has trait/method collision - use explicit call: `SphereSource::get_output_port(&mut sphere)`
- [x] Example: `interactive_point_picker.rs` - click-to-pick vertices with red marker visualization

### P2.4 World Point Picker - `vtk_world_point_picker.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `pick(x, y, z, renderer)` → bool (always returns true)
- [x] `get_pick_position()` → (x, y, z)
- [x] Convert screen coordinates to 3D world coordinates
- [x] Essential for accurate 3D placement
- [x] Note: VTK's WorldPointPicker.Pick() always returns 0 but still sets position correctly
- [x] Example: `world_point_picker_demo.rs` - demonstrates programmatic coordinate conversion with reference objects
- [x] Example: `interactive_world_point_picker.rs` - click-to-place spheres using event callbacks

### P2.5 Interactive Selection Example ✅ COMPLETE
- [x] Create `examples/interactive_area_picker.rs`
- [x] Drag to create selection rectangle with visual rubber band feedback
- [x] Display selection coordinates in real-time (inline console updates)
- [x] Highlight selection with smooth, flicker-free yellow rectangle
- [x] Coordinate conversion handling (screen to VTK coordinates)
- [x] Optimized frame caching for performance
- [x] Clean architecture demonstration: C++ glue / Rust application logic

---

## ✅ PRIORITY 2 COMPLETE! 🎉

All interaction widgets and selection tools implemented with comprehensive examples. Key achievements:
- **Architecture Pattern Established**: C++ for VTK glue, Rust for application logic
- **Optimized Visual Feedback**: Frame caching for flicker-free rubber band drawing
- **Complete Picker Suite**: WorldPointPicker, PointPicker, AreaPicker with wrappers
- **Interactive Examples**: Click-to-pick, click-to-place, drag-to-select demonstrations

---

## Priority 3: Advanced Sources & Utilities

Mathematical surfaces and specialized geometric sources.

### P3.1 Parametric Function Source - `vtk_parametric_function_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_parametric_function(func)` - accepts void pointer for cross-module compatibility
- [x] `set_u_resolution(n)` / `set_v_resolution(n)` / `set_w_resolution(n)`
- [x] `output_port()` - renamed to avoid trait method shadowing
- [x] **Parametric Torus** - `vtk_parametric_torus.rs`
  - [x] `set_ring_radius(r)` / `get_ring_radius()` - major radius
  - [x] `set_cross_section_radius(r)` / `get_cross_section_radius()` - minor radius
  - [x] `as_parametric_function()` → void pointer
- [x] **Parametric Klein Bottle** - `vtk_parametric_klein.rs`
  - [x] Non-orientable surface (no distinct inside/outside)
  - [x] `as_parametric_function()` → void pointer
- [x] **Parametric Mobius Strip** - `vtk_parametric_mobius.rs`
  - [x] One-sided surface topology
  - [x] `set_radius(r)` / `get_radius()`
  - [x] `as_parametric_function()` → void pointer
- [x] Example: `parametric_surfaces.rs` - demonstrates all three surfaces with helper function pattern
- [x] **FFI Pattern**: Void pointer casting for cross-module type compatibility
- [x] **Architecture Note**: Each parametric function inherits from vtkObjectBase (not vtkParametricFunction due to macro limitations)

#### Future Parametric Functions (Optional Expansion)
- [ ] **vtkParametricEllipsoid** - Configurable ellipsoid (stretched sphere)
  - [ ] `set_x_radius(r)` / `set_y_radius(r)` / `set_z_radius(r)`
  - [ ] Useful for planetary bodies, stretched shapes
- [ ] **vtkParametricBoy** - Boy's surface (non-orientable, immersion of projective plane)
  - [ ] Mathematical topology demonstration
- [ ] **vtkParametricConicSpiral** - Conical spiral pattern
  - [ ] `set_a(val)` / `set_b(val)` / `set_c(val)` / `set_n(val)`
  - [ ] Useful for spiral staircases, springs, decorative elements
- [ ] **vtkParametricSuperEllipsoid** - Superellipsoid with power parameters
  - [ ] `set_n1(val)` / `set_n2(val)` - control roundness
  - [ ] Morphs between sphere, cube, and other shapes
- [ ] **vtkParametricRoman** - Roman surface (Steiner surface)
  - [ ] Another non-orientable surface example
  - [ ] `set_radius(r)`

### P3.2 Superquadric Source - `vtk_superquadric_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_theta_roundness(r)` / `get_theta_roundness()` - east-west curvature (0.0=cube, 1.0=sphere)
- [x] `set_phi_roundness(r)` / `get_phi_roundness()` - north-south curvature (0.0=cube, 1.0=sphere)
- [x] `set_thickness(t)` / `get_thickness()` - hole size in toroidal mode
- [x] `set_size(s)` / `get_size()` - overall scale
- [x] `set_scale(x, y, z)` / `get_scale()` - non-uniform scaling
- [x] `set_center(x, y, z)` / `get_center()` - position in 3D space
- [x] `set_toroidal(bool)` / `get_toroidal()` - switch between ellipsoid (solid) and toroidal (donut) modes
- [x] `set_theta_resolution(n)` / `set_phi_resolution(n)` - mesh detail
- [x] `output_port()` - renamed to avoid trait shadowing
- [x] **FFI Pattern**: References (&) not pointers (*), no const getters (VTK limitation)
- [x] **Toroidal Mode**: ALWAYS creates hole (thickness controls size, not existence)
- [x] Example: `superquadric_demo.rs` - 20 shapes in 4 rows demonstrating morphing patterns

### P3.3 Text Source - `vtk_text_source.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_text(text: &str)`
- [x] `set_backing(bool)` - add background plane
- [x] `get_output_port()`
- [x] Extruded text geometry for 3D labels
- [x] Example: `text_demo.rs` - demonstrates extruded 3D text with backing plane

### P3.4 Billboard Text - `vtk_follower.rs`, `vtk_vector_text.rs`, `vtk_command.rs` ✅ COMPLETE
- [x] **vtkFollower** - Billboard actor that always faces camera
  - [x] `new()` / `delete()`
  - [x] `set_camera()` / `set_camera_ref()` - attach to camera
  - [x] `set_mapper()` - set text geometry mapper
  - [x] `set_position()` / `get_position()` - 3D placement
  - [x] `set_scale()` / `get_scale()` - dynamic sizing
  - [x] `as_raw_ptr()` - for renderer integration
- [x] **vtkVectorText** - Lightweight 3D stroke text
  - [x] `new()` / `delete()`
  - [x] `set_text()` / `get_text()` - string handling via rust::String
  - [x] `output_port()` - pipeline connection
- [x] **vtkCommand/Observer** - Event callback system
  - [x] RustCommand class with vtkObjectFactory integration
  - [x] `Command::new()` - create observer
  - [x] `set_callback()` - register extern "C" callback
  - [x] Observable trait for Camera and CameraRef
  - [x] `add_observer()` / `remove_observer()` - attach to VTK objects
  - [x] Event constants (MODIFIED_EVENT, MOUSE_MOVE_EVENT, etc.)
  - [x] Type-safe FFI using usize for unsigned long compatibility
- [x] Example: `billboard_text_demo.rs` - 5x5 grid of FEM node labels with zoom-invariant scaling

---

## ✅ PRIORITY 3 COMPLETE! 🎉

All advanced sources and billboard text features implemented:
- **Parametric Surfaces**: Torus, Klein bottle, Möbius strip with mathematical precision
- **Superquadrics**: Morphing shapes from cubes to spheres with toroidal mode
- **3D Text**: Both extruded (TextSource) and stroke (VectorText) rendering
- **Billboard System**: Camera-facing labels with dynamic zoom-invariant scaling
- **Observer Pattern**: Full VTK event system integration for reactive updates

---

## Priority 4: Actors & Annotations

Text rendering and visual annotations for labeling and legends.

### P4.1 Text Actor - `vtk_text_actor.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_input(text: &str)`
- [x] `set_position(x, y)` - display coords (pixels)
- [x] `set_position_normalized(fx, fy, width, height)` - normalized [0,1] with auto-resize
- [x] `get_text_property()` → TextPropertyRef
- [x] `set_visibility(bool)`
- [x] Screen-space text rendering for HUD elements
- [x] **TextProperty**: font size, family (Arial/Courier/Times), bold, italic, color, opacity
- [x] **Text Alignment**: horizontal justification (left/center/right), vertical justification (bottom/center/top)
- [x] **Auto-Resize System**: TextActorResizeManager with VTK observer pattern
- [x] **FFI Pattern**: Extern "C" callbacks with user_data, Box<TextActor> for stable memory
- [x] **Architecture**: Coordinate conversion in Rust, C++ glue for VTK API only
- [x] Example: `text_actor_demo.rs` - 6 labels with different positions and alignments
- [x] Commit: cbd9818

### P4.2 Scalar Bar Actor - `vtk_scalar_bar_actor.rs` ✅ COMPLETE
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_lookup_table(lut)`
- [x] `set_title(text: &str)`
- [x] `set_number_of_labels(n)`
- [x] `set_position(x, y)` / `set_width(w)` / `set_height(h)`
- [x] `get_label_text_property()` / `get_title_text_property()`
- [x] `set_orientation()` / `set_visibility()`
- [x] Color bar legends for scientific visualization
- [x] **LookupTable** - `vtk_lookup_table.rs`
  - [x] `new()` / `delete()`
  - [x] `set_range(min, max)` - data value range
  - [x] `set_hue_range(min, max)` / `set_saturation_range()` / `set_value_range()` - HSV color control
  - [x] `set_alpha_range(min, max)` - opacity control
  - [x] `set_number_of_table_values(n)` - discrete/continuous gradient
  - [x] `build()` - generate color table
  - [x] `get_color(value)` → (r, g, b) - query colors
- [x] **TextPropertyRef** - Shared with TextActor for font/color configuration
- [x] **Renderer.add_actor2d()** - Added support for 2D actors
- [x] Example: `scalar_bar_demo.rs` - 5×5 grid of spheres colored by temperature with color legend
- [x] Architecture: C++ glue layer, Rust business logic
- [x] FFI Pattern: Type aliases for cross-module vtkLookupTable and vtkTextProperty references

### P4.3 Legend Box Actor - `vtk_legend_box_actor.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_number_of_entries(n)`
- [ ] `set_entry(i, actor, label)`
- [ ] `set_position(x, y)`
- [ ] Multi-item legends for scene annotations

---

## Priority 5: Data Structures & Filters

Advanced data handling for volumetric and unstructured data.

### P5.1 Image Data - `vtk_image_data.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_dimensions(nx, ny, nz)`
- [ ] `set_spacing(dx, dy, dz)` / `set_origin(x, y, z)`
- [ ] `allocate_scalars(type, components)`
- [ ] `get_scalar_pointer()` - access voxel data
- [ ] Structured grid data for volumetric visualization

### P5.2 Unstructured Grid - `vtk_unstructured_grid.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_points(points)`
- [ ] `allocate(ncells)`
- [ ] `insert_next_cell(type, point_ids)`
- [ ] FEM meshes, arbitrary topology

### P5.3 Contour Filter - `vtk_contour_filter.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input_connection(port)`
- [ ] `set_value(i, value)` - isosurface value
- [ ] `generate_values(n, range_min, range_max)`
- [ ] `get_output_port()`
- [ ] Extract surfaces at constant values

### P5.4 Clip Poly Data - `vtk_clip_poly_data.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input_connection(port)`
- [ ] `set_clip_function(plane)` - implicit function
- [ ] `set_value(val)` - clip threshold
- [ ] `get_output_port()`
- [ ] Cut meshes with planes for cross-sections

---

## Sprint 1: Core Visualization Control ✓ COMPLETED

### 1.1 Camera Control - `vtk_camera.rs` ✓
- [x] `set_position(x, y, z)` / `get_position()`
- [x] `set_focal_point(x, y, z)` / `get_focal_point()`
- [x] `set_view_up(x, y, z)` / `get_view_up()`
- [x] `azimuth(angle)` / `elevation(angle)` / `roll(angle)`
- [x] `zoom(factor)` / `dolly(factor)`
- [x] `set_clipping_range(near, far)` / `get_clipping_range()`
- [x] Implemented for both `Camera` and `CameraRef`
- [x] Created animated example: `examples/camera_control.rs`

### 1.2 Property (NEW) - `vtk_property.rs` ✓ COMPLETED
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_color(r, g, b)` / `get_color()`
- [x] `set_opacity(alpha)` / `get_opacity()`
- [x] `set_line_width(width)` / `set_point_size(size)`
- [x] `set_representation(POINTS/WIREFRAME/SURFACE)` - enum RepresentationType
- [x] `set_edge_visibility(bool)` / `set_edge_color(r, g, b)`
- [x] `set_interpolation(FLAT/GOURAUD/PHONG)` - enum InterpolationType
- [x] `set_ambient(value)` / `set_diffuse(value)` / `set_specular(value)`
- [x] `set_specular_power(value)`

### 1.3 Actor Enhancements - `vtk_actor.rs` ✓ COMPLETED
- [x] `get_property()` → PropertyRef (non-owning reference)
- [x] PropertyRef with all essential property methods
- [ ] `set_position(x, y, z)` / `get_position()` - DEFERRED
- [ ] `set_scale(x, y, z)` / `set_scale_uniform(s)` - DEFERRED
- [ ] `set_orientation(x, y, z)` / `rotate_xyz(x, y, z)` - DEFERRED
- [ ] `set_visibility(bool)` / `get_visibility()` - DEFERRED
- [ ] `set_pickable(bool)` / `get_pickable()` - DEFERRED

### 1.4 Example Update ✓ COMPLETED
- [x] Created `examples/property_demo.rs` with animated demonstrations
- [x] Shows color changes, transparency, wireframe mode
- [x] Shows different representations (points, wireframe, surface)
- [x] Shows edge visibility and line width animation

**Sprint 1 Summary:**
- ✅ Full camera control with 12 methods
- ✅ Complete property system with visual appearance control
- ✅ Actor property access via get_property()
- ✅ Two animated examples: camera_control and property_demo

---

## Sprint 2A: Basic Picking Infrastructure ✓ COMPLETED

### 2A.1 Coordinate Conversion - `vtk_renderer.rs` ✓
- [x] `set_display_point(x, y, z)` / `display_to_world()`
- [x] `set_world_point(x, y, z, w)` / `world_to_display()`
- [x] `get_display_point()` / `get_world_point()`

### 2A.2 Cell Picker - `vtk_cell_picker.rs` ✓
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `pick(x, y, z, renderer)` → bool
- [x] `get_cell_id()` / `get_dataset()`
- [x] `get_pick_position()` → (x, y, z)
- [x] `get_actor()`
- [x] `has_valid_pick()` helper method

### 2A.3 Point Picker - `vtk_point_picker.rs` - DEFERRED
- [ ] Not needed immediately for measurement tools
- [ ] Can be added later if required

### 2A.4 Prop Picker - `vtk_prop_picker.rs` - DEFERRED
- [ ] Not needed immediately for measurement tools
- [ ] Can be added later if required

### 2A.5 Basic Picking Example ✓
- [x] Create `examples/picking_demo.rs`
- [x] Demonstrates picking at specific coordinates
- [x] Shows coordinate conversion (display ↔ world)
- [x] Display picked cell ID and position

**Sprint 2A Summary:**
- ✅ Full coordinate conversion in Renderer
- ✅ CellPicker for picking geometry
- ✅ Picking demonstration example
- ✅ Foundation ready for measurement tools

---

## Sprint 2B: Event Handling & LineSource ✓ EVENT HANDLING COMPLETE

### 2B.1 Event Handling - ✓ COMPLETED
- [x] `InteractorStyleCustom` C++ class created
- [x] Callback infrastructure with extern "C" trampolines  
- [x] `set_left_button_press_callback(callback)` 
- [x] `set_left_button_release_callback(callback)`
- [x] `set_mouse_move_callback(callback)`
- [x] Full callback integration via direct FFI (bypassed cxx bridge)
- [x] Event examples: `event_test.rs` and `interactive_picking.rs`

**Solution:**
- Custom VTK interactor style inheriting from TrackballCamera
- Direct `extern "C"` FFI instead of cxx bridge (which was causing segfaults)
- Global callback registry with Mutex for thread-safe callback storage
- Callback trampolines: C++ → extern "C" → Rust closure
- Static initializer confirms proper linking

**Working features:**
- ✅ Mouse click events with (x, y) display coordinates
- ✅ Mouse move events (trackable but optional)
- ✅ Button release events
- ✅ Full trackball camera interaction preserved
- ✅ Thread-safe callback registration

### 2B.2 Line Source - `vtk_line_source.rs` ✓ COMPLETED
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_point1(x, y, z)` / `get_point1()`
- [x] `set_point2(x, y, z)` / `get_point2()`
- [x] `set_resolution(resolution)` / `get_resolution()`
- [x] `get_output_port()` → AlgorithmOutputPort
- [x] `get_length()` - calculate distance
- [x] Created example: `examples/line_source_demo.rs`
- [x] Uses direct extern "C" FFI (like InteractorStyleCustom)

### 2B.3 PolyData Enhanced - `vtk_poly_data.rs`
- [ ] `get_number_of_points()` / `get_number_of_cells()`
- [ ] `get_bounds()` → (xmin, xmax, ymin, ymax, zmin, zmax)

### 2B.4 Points - `vtk_points.rs` ✓ COMPLETED
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `insert_next_point(x, y, z)` → point_id
- [x] `set_point(id, x, y, z)`
- [x] `get_point(id)` → (x, y, z)
- [x] `get_number_of_points()`
- [x] `reset()` - clear all points
- [x] `set_number_of_points(n)` - pre-allocate
- [x] `resize(n)` - resize array
- [x] Iterator support for point traversal
- [x] Created example: `examples/points_demo.rs`
- [x] Uses direct extern "C" FFI

---

## Sprint 2C: Interactive Measurement Tool

### 2C.1 Interactor Style Base - `vtk_interactor_style.rs`
- [ ] Create base class bindings
- [ ] Virtual method hooks for custom styles
- [ ] `set_current_renderer(renderer)`
- [ ] `get_interactor()`

### 2C.2 Custom Measurement Example
- [ ] Create `examples/measurement_tool.rs`
- [ ] Click to place first point
- [ ] Rubber-band line follows cursor
- [ ] Click to place second point
- [ ] Display distance calculation
- [ ] Continue measuring multiple segments

### 2C.3 Distance Calculation Utility
- [ ] Add `src/utils/geometry.rs`
- [ ] `distance_3d(p1, p2)` → f64
- [ ] `distance_2d(p1, p2)` → f64

---

## Sprint 3: Annotations & Text

### 3.1 Text Property - `vtk_text_property.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_font_size(size)`
- [ ] `set_color(r, g, b)` / `set_opacity(alpha)`
- [ ] `set_font_family(ARIAL/COURIER/TIMES)` - enum
- [ ] `set_bold(bool)` / `set_italic(bool)` / `set_shadow(bool)`
- [ ] `set_justification(LEFT/CENTER/RIGHT)` - enum

### 3.2 Text Actor - `vtk_text_actor.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input(text: &str)`
- [ ] `set_position(x, y)` - normalized viewport coords [0,1]
- [ ] `get_text_property()` → TextPropertyRef
- [ ] `set_visibility(bool)`

### 3.3 Caption Actor 2D - `vtk_caption_actor2d.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_caption(text: &str)`
- [ ] `set_attachment_point(x, y, z)` - 3D world coords
- [ ] `set_position(x, y)` - 2D screen coords
- [ ] `get_caption_text_property()`
- [ ] `set_leader(bool)` / `set_three_dimensional_leader(bool)`
- [ ] `set_arrow(bool)` / `set_border(bool)`

### 3.4 Enhanced Measurement Example
- [ ] Update `examples/measurement_tool.rs`
- [ ] Add text labels showing distance
- [ ] Add CaptionActor2D for annotations

---

## Sprint 4: Followers & Additional Sources

### 4.1 Follower - `vtk_follower.rs`
- [ ] Create new module and C++ bindings
- [ ] Extends Actor functionality
- [ ] `new()` / `delete()`
- [ ] `set_camera(camera)`
- [ ] `set_mapper(mapper)`
- [ ] Always faces camera (billboard effect)

### 4.2 Vector Text - `vtk_vector_text.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_text(text: &str)`
- [ ] `get_output()` → PolyData
- [ ] Use with PolyDataMapper + Follower

### 4.3 Geometric Sources
- [ ] `vtk_cube_source.rs` - Boxes
- [ ] `vtk_cylinder_source.rs` - Cylinders
- [ ] `vtk_cone_source.rs` - Cones
- [ ] `vtk_arrow_source.rs` - Arrows (great for vectors)
- [ ] `vtk_plane_source.rs` - Planes

### 4.4 Examples
- [ ] Create `examples/geometric_primitives.rs`
- [ ] Show all basic shapes
- [ ] Demonstrate Follower with text labels

---

## Sprint 5: Transform Support

### 5.1 Transform - `vtk_transform.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `identity()` / `post_multiply()` / `pre_multiply()`
- [ ] `translate(x, y, z)` / `rotate_x/y/z(angle)`
- [ ] `scale(x, y, z)`
- [ ] `set_matrix(mat4x4)` / `get_matrix()`
- [ ] `inverse()` / `concatenate(transform)`
- [ ] `transform_point(pt)` / `transform_vector(vec)`

### 5.2 Transform Filter - `vtk_transform_poly_data_filter.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input_connection(port)`
- [ ] `set_transform(transform)`
- [ ] `get_output()`

### 5.3 Example
- [ ] Create `examples/transforms.rs`
- [ ] Rotate, scale, translate objects
- [ ] Animated transformations

---

## Sprint 6: Advanced Widgets

### 6.1 Distance Widget - `vtk_distance_widget.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_interactor(interactor)`
- [ ] `set_enabled(bool)`
- [ ] `create_default_representation()`
- [ ] Event observers for interaction

### 6.2 Distance Representation - `vtk_distance_representation.rs`
- [ ] Create new module and C++ bindings
- [ ] `set_label_format(format: &str)`
- [ ] `get_distance()` → f64
- [ ] Handle rendering and interaction

### 6.3 Seed Widget - `vtk_seed_widget.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_interactor(interactor)`
- [ ] `set_enabled(bool)`
- [ ] `get_seed_representation()`
- [ ] Place multiple points interactively

### 6.4 Contour Widget - `vtk_contour_widget.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] Interactive polyline/polygon drawing
- [ ] Click to add vertices, draggable control points
- [ ] `initialize()` / `set_enabled(bool)`

---

## Sprint 7: Lighting

### 7.1 Light - `vtk_light.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_position(x, y, z)` / `set_focal_point(x, y, z)`
- [ ] `set_color(r, g, b)` / `set_intensity(val)`
- [ ] `set_cone_angle(angle)` - for spotlights
- [ ] `set_positional(bool)` - vs directional
- [ ] `switch_on()` / `switch_off()`

### 7.2 Renderer Light Support
- [ ] `add_light(light)` in vtk_renderer.rs
- [ ] `remove_light(light)`
- [ ] `get_lights()` - collection

### 7.3 Example
- [ ] Create `examples/lighting.rs`
- [ ] Custom light setup
- [ ] Spotlight effects

---

## Sprint 8: Area Selection & Advanced Picking

### 8.1 Area Picker - `vtk_area_picker.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `area_pick(x1, y1, x2, y2, renderer)` → bool
- [ ] `get_prop3ds()` - collection of picked actors

### 8.2 Rubber Band Style - `vtk_interactor_style_rubber_band_pick.rs`
- [ ] Create new module and C++ bindings
- [ ] Mouse drag for rectangle selection
- [ ] Visual feedback of selection area
- [ ] Integration with AreaPicker

### 8.3 Hardware Selector - `vtk_hardware_selector.rs`
- [ ] Create new module and C++ bindings
- [ ] GPU-based picking (faster)
- [ ] `set_area(x1, y1, x2, y2)`
- [ ] `set_renderer(renderer)`
- [ ] `select()` / `generate_selection()`

### 8.4 Example
- [ ] Create `examples/area_selection.rs`
- [ ] Rubber-band rectangle selection
- [ ] Highlight selected objects

---

## Future Considerations

### Data Processing Filters
- `vtk_triangle_filter.rs` - Triangulate meshes
- `vtk_clean_poly_data.rs` - Remove duplicates
- `vtk_append_poly_data.rs` - Combine datasets
- `vtk_decimate_pro.rs` - Reduce polygon count
- `vtk_smooth_poly_data_filter.rs` - Smooth surfaces

### Advanced Annotations
- `vtk_scalar_bar_actor.rs` - Color legend/colorbar
- `vtk_corner_annotation.rs` - Text in screen corners
- `vtk_axis_actor.rs` - Custom axis rendering

### Angle Measurement
- `vtk_angle_widget.rs` - Three-point angle measurement
- `vtk_angle_representation.rs`

### Manual Geometry Construction
- Enhanced `vtk_poly_data.rs`:
  - `set_points(points)` / `get_points()`
  - `set_polys(cells)` / `get_polys()`
  - `insert_next_cell()`
  - `build_links()` / `build_cells()`

### Lookup Tables & Color Mapping
- `vtk_lookup_table.rs` - Color mapping
- `vtk_color_transfer_function.rs` - Advanced colormapping

---

## Notes

- File I/O (readers/writers) deferred - not critical for current goals
- Focus on interactive visualization and measurement
- Maintain consistent API patterns across all modules
- Generate C++ headers using `generate_headers.sh` after each module
- Update `linker-args.txt` if new VTK modules are required
- Write examples demonstrating each new feature

---

## APPENDIX: Application-Specific Roadmaps

The following sections contain roadmaps for specific applications built using vtk-rs.
These are **excluded from core wrapper development** and serve as reference for application developers.

---

# FEM BEAM STRUCTURE PRE/POST-PROCESSOR (Application Example)

> **Note**: This is an application-specific roadmap, not part of the core VTK wrapper.
> Move to a separate repository when developing this application.

## Application Goal
Build a specialized FEM pre-processor and post-processor for beam structures with:
- Interactive node placement and beam element definition
- Visual property assignment (material, cross-section, boundary conditions)
- Result visualization (deformation, stress, forces, moments)
- Measurement and annotation tools

---

## Sprint F1: Core Beam Structure Data Model 🎯 PRIORITY

### F1.1 Points Module - `vtk_points.rs` ✅ COMPLETED
- [x] Direct extern "C" FFI bindings (avoiding cxx::bridge)
- [x] `new()` / `delete()`
- [x] `insert_next_point(x, y, z)` → point_id
- [x] `set_point(id, x, y, z)` / `get_point(id)` → (x, y, z)
- [x] `get_number_of_points()`
- [x] `reset()` - clear all points
- [x] `resize()` / `set_number_of_points()`
- [x] Iterator implementation (PointsIterator)
- [x] Default and Drop traits
- [x] Example: `points_demo.rs` - helix pattern

### F1.2 CellArray - `vtk_cell_array.rs` ✅ COMPLETED
- [x] Created module with C++ bindings
- [x] Direct extern "C" FFI pattern
- [x] `new()` / `delete()`
- [x] `insert_next_cell(point_ids: &[i64])` → cell_id
- [x] `get_number_of_cells()`
- [x] `get_number_of_connectivity_ids()`
- [x] `get_cell(cell_id)` → Option<Vec<i64>>
- [x] `reset()` / `initialize()` - clear cells
- [x] Iterator implementation (CellArrayIterator)
- [x] Default and Drop traits
- [x] Example: `cell_array_demo.rs` - 3D truss structure with 12 beams
- [x] Supports lines (2 points) and triangles (3+ points)

### F1.3 Enhanced PolyData - `vtk_poly_data.rs` ✅ COMPLETED
- [x] Converted from cxx::bridge to direct extern "C" FFI
- [x] `set_points(points: &Points)` - assign node positions
- [x] `get_number_of_points()` → i64
- [x] `set_lines(lines: &CellArray)` - assign beam elements
- [x] `get_number_of_lines()` → i64
- [x] `get_number_of_cells()` → i64 (total cells)
- [x] `get_bounds()` → (xmin, xmax, ymin, ymax, zmin, zmax)
- [x] `allocate(num_verts, connectivity_size)` - pre-allocate memory
- [x] `modified()` - trigger pipeline update
- [x] `compute_bounds()` - calculate spatial extent
- [x] `from_beam_structure()` - convenience constructor
- [x] Example: `polydata_beam.rs` - 3D truss (8 nodes, 12 beams)
- [x] Full integration with Points and CellArray modules

### F1.4 Data Arrays ✅ COMPLETED
- [x] Created DoubleArray and IntArray modules
- [x] Direct extern "C" FFI pattern
- [x] `vtk_double_array.rs` - for scalars and vectors
  - [x] `new()` / `delete()` / `new_scalar()` / `new_vector()`
  - [x] `set_number_of_components(n)` / `get_number_of_components()`
  - [x] `set_number_of_tuples(n)` / `get_number_of_tuples()`
  - [x] `get_number_of_values()` - total values (tuples × components)
  - [x] `insert_next_value(val)` - for 1-component arrays
  - [x] `insert_next_tuple1/2/3()` - for N-component tuples
  - [x] `set_value(id, val)` / `get_value(id)`
  - [x] `set_tuple1/2/3(id, ...)` / `get_tuple(id)`
  - [x] `set_name(name: &str)` / `get_name()` → Option<String>
  - [x] `initialize()` / `squeeze()` - memory management
- [x] `vtk_int_array.rs` - for IDs, flags, element types
  - [x] `new()` / `delete()` / `new_id_array()`
  - [x] Same API as DoubleArray but for i32 values
  - [x] Special handling for GetTuple using GetComponent
- [x] Example: `data_array_demo.rs` - FEM cantilever beam
  - [x] Scalar: Displacement magnitude (0, 2.5, 10 mm)
  - [x] Vector: Displacement vectors (dx, dy, dz)
  - [x] Scalar: Von Mises stress (150, 75, 10 MPa)
  - [x] Integer: Material IDs (Steel = 1)
  - [x] Integer: Boundary condition flags (7 = fixed all)
  - [x] Data modification demonstration

### F1.5 Point/Cell Data - `vtk_point_data.rs`, `vtk_cell_data.rs` ✓ COMPLETED
- [x] Bindings for PolyData attribute management
- [x] `add_array(array: &DataArray)`
- [x] `remove_array(name: &str)`
- [x] `get_array(name: &str)` → DataArray
- [x] `get_number_of_arrays()`
- [x] `set_scalars(array)` / `get_scalars()`
- [x] `set_vectors(array)` / `get_vectors()`
- [x] `set_active_scalars(name)` / `set_active_vectors(name)` - visualization control
- [x] Enhanced PolyData with `get_point_data()` / `get_cell_data()`
- [x] Example: `fem_beam_data.rs` - Cantilever beam with data
  - [x] 4 nodes, 3 beam elements
  - [x] PointData: DisplacementMagnitude (0, 2.5, 7.5, 15 mm)
  - [x] PointData: DisplacementVector (dx, dy, dz tuples)
  - [x] PointData: BoundaryCondition flags (7=fixed, 0=free)
  - [x] CellData: MaterialID (all Steel=1)
  - [x] CellData: ElementStress (180, 120, 60 MPa)
  - [x] CellData: CrossSectionType (all Rectangle=1)
  - [x] Active scalars set for color mapping

**Implementation Notes:**
- Direct `extern "C"` FFI pattern matching other modules
- PointData/CellData not owned by Rust (owned by PolyData)
- No Drop trait needed - parent PolyData manages lifetime
- add_array handles both DoubleArray and IntArray via separate methods
- set_active_scalars/vectors enable color mapping in visualization
- Complete FEM workflow: geometry + node properties + element properties

### F1.6 Example: Beam Structure Visualization ✓ COMPLETED
- [x] Create `examples/fem_beam_visualization.rs`
- [x] Define nodes (Points) - 4-node cantilever beam
- [x] Create beam elements (Lines in CellArray)  
- [x] Store element properties (stress) in CellData
- [x] Store node properties (displacement) in PointData
- [x] Visualize beams with LineSource (color-coded by stress)
- [x] Visualize nodes with SphereSource (blue spheres)
- [x] Interactive 3D rendering with trackball camera

**Implementation Notes:**
- Used LineSource for each beam segment (3 separate lines)
- Color-coded beams by stress: Red (180 MPa) → Orange (120 MPa) → Green (60 MPa)
- SphereSource for node markers (radius 0.08)
- Thick lines (width 6.0) for better visibility
- Dark blue background for contrast
- Complete FEM visualization workflow: geometry + data + rendering

---

## Sprint F2: Interactive Node & Element Creation

### F2.1 Interactive Node Placement ✅ COMPLETED
- [x] Extend InteractorStyleCustom with "add node" mode
- [x] Click in 3D viewport to place nodes
- [x] Camera-aware coordinate projection (constant depth from camera)
- [x] Ray-plane intersection for world coordinates
- [x] Node state management with Arc/Mutex for thread-safe updates
- [x] Raw pointer pattern for VTK object access from callbacks
- [x] Live visualization updates: insert_next_point() + modified() + render()
- [x] Visual feedback with Glyph3D spheres
- [x] Example: `interactive_node_placement.rs`

**Implementation Notes:**
- Uses InteractorStyleCustom callback infrastructure
- VisualizationState with raw pointers (*mut Points, *mut PolyData, *mut RenderWindow, *mut Renderer)
- unsafe impl Send + Sync for thread-safe callback access
- Arc/Mutex pattern for shared state management
- **Constant Depth Projection** (default):
  - Projects at fixed distance (5.0 units) along viewing ray from camera
  - Most stable method - nodes stay at consistent depth regardless of camera rotation/zoom
  - Normalized ray direction for precise placement
  - Works perfectly for 3D structure building
- **Z-Plane Projection** (alternative):
  - Ray-plane intersection with z=0 world plane
  - Good for 2D-like structures on a fixed plane
- screen_to_world() method:
  - Uses VTK's display_to_world() coordinate transformation
  - Gets near/far points to construct viewing ray
  - Projects to fixed depth or intersects with plane
- add_node() method with live updates:
  - Adds to Rust Vec for tracking
  - (*points).insert_next_point() adds to VTK structure
  - (*poly_data).modified() notifies pipeline
  - (*window).render() triggers immediate redraw
- Glyph3D efficiently renders all node markers as blue spheres
- Real-time visualization: each click immediately shows new sphere
- Console output shows: screen position → world coordinates → node added
- User tested extensively: placed 80+ nodes with camera rotation
- Production-ready for FEM model building

### F2.2 Interactive Beam Element Creation ✅ COMPLETED
- [x] "Add beam" mode: click two nodes to create beam
- [x] Keyboard-based mode switching with 'm' key
- [x] Store beam connectivity in CellArray
- [x] Render beams as tubes (TubeFilter)
- [x] Example: `interactive_beam_builder.rs`

**Implementation Notes:**
- Extends F2.1 foundation with beam connectivity management
- **Key Press Support Added to InteractorStyleCustom**:
  - Extended C++ vtkInteractorStyleCustom with OnKeyPress() override
  - Added vtk_rs_key_press_callback() trampoline for Rust callbacks
  - Key symbols passed as C strings (e.g., "m", "Escape", "F1", "space")
  - set_key_press_callback() method in Rust API
  - Enables keyboard-driven interaction modes
- BeamSelection state machine: None → FirstNodeSelected → BeamCreated → None
- Mode switching via 'm' key:
  - Press 'm' to toggle between NODE and BEAM modes
  - Clear mode provides explicit control over interaction state
  - Console feedback confirms mode switches
- Dual PolyData structure:
  - node_poly_data with Points for node positions (rendered via Glyph3D)
  - beam_poly_data with Points + Lines CellArray for beam connectivity (rendered via TubeFilter)
- find_nearest_node() helper with distance threshold (0.5 units) for node picking
- Beam rendering pipeline: PolyData → TubeFilter (radius 0.05, 12 sides) → PolyDataMapper → Actor
- Cyan tubes (0.0, 1.0, 1.0) for beams, blue spheres (0.2, 0.5, 1.0) for nodes
- Real-time beam visualization: insert_next_cell() + modified() + render()
- Console feedback for all actions:
  - Node placement with world coordinates
  - Mode switches (NODE ↔ BEAM) via keyboard
  - Node selection for beam creation
  - Beam creation confirmation with node IDs
  - Error messages for invalid operations (same node twice, no node found)
- Production-ready for FEM model building workflow

### F2.3 Tube Filter - `vtk_tube_filter.rs` ✓ COMPLETED
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_input_connection(port)`
- [x] `set_radius(radius)` / `get_radius()`
- [x] `set_number_of_sides(n)` - circular resolution
- [x] `set_radius_factor(factor)` - for variable radius
- [x] `set_capping(bool)` / `get_capping()` - close tube ends
- [x] `get_output_port()`
- [x] Example: `fem_beam_tubes.rs` demonstrating 3D beam rendering

**Implementation Notes:**
- Direct extern "C" FFI pattern (consistent with all other modules)
- C++ bindings: `vtk_tube_filter.h/cpp` with null pointer checks
- Rust wrapper: Complete TubeFilter struct with Drop trait
- Methods: set_input_connection, get_output_port, set_radius, set_number_of_sides, set_capping
- Default values: radius 0.5, sides 3 (typical: 6-16 for beams, 20+ for smooth cylinders)
- fem_beam_tubes.rs: 3D tube rendering with stress-based coloring
  - Radius 0.05, 12 sides, capping enabled
  - Overlaid initial (gray lines) and deformed (colored tubes) geometry
  - Stress gradient: Red (180 MPa) → Orange (120 MPa) → Green (60 MPa)
- Visualization pipeline: LineSource → TubeFilter → PolyDataMapper → Actor
- Professional-quality FEM visualization suitable for engineering reports

### F2.4 Glyph for Nodes - `vtk_glyph3d.rs` ✓ COMPLETED
- [x] Create new module and C++ bindings
- [x] `new()` / `delete()`
- [x] `set_input_connection(port)` - points to glyph at
- [x] `set_source_connection(port)` - geometry to place (sphere, cube)
- [x] `set_scale_factor(factor)`
- [x] `set_scale_mode_to_data_scaling_off()`
- [x] `get_output_port()`
- [x] Additional methods: set_scaling, set_orient, set_clamping, color modes
- [x] Example: `fem_beam_glyph.rs` demonstrating efficient node visualization

**Implementation Notes:**
- Direct extern "C" FFI pattern with comprehensive API
- C++ bindings: vtk_glyph_3d.h/cpp with null pointer checks
- Rust wrapper: Full Glyph3D struct with ScaleMode/ColorMode enums
- Methods: set_input_connection, set_source_connection, get_output_port
- Scale modes: ScaleByScalar, ScaleByVector, ScaleByVectorComponents, DataScalingOff
- Color modes: ColorByScale, ColorByScalar, ColorByVector
- Additional controls: scaling on/off, orient on/off, clamping on/off
- Extended PolyData with get_output_port() using vtkTrivialProducer
- fem_beam_glyph.rs: Places spheres at 4 nodes with single filter
  - Much more efficient than creating 4 separate SphereSource objects
  - Scales to thousands of nodes efficiently
  - Overlaid tubes (deformed) + lines (initial) + glyph spheres (nodes)
- Visualization pipeline: Points → PolyData → Glyph3D (source=SphereSource) → Mapper
- Perfect foundation for interactive node placement with visual feedback

### F2.5 Sphere Source - Already exists!
- [x] Use existing `SphereSource` for node representation
- [ ] Create small spheres at each node position

### F2.6 Example: Interactive Beam Model Builder
- [ ] Create `examples/beam_model_builder.rs`
- [ ] Click to add nodes
- [ ] Click two nodes to create beam
- [ ] Display node IDs
- [ ] Display beam IDs
- [ ] Save/load model data

---

## Sprint F3: Property Assignment & Visualization

### F3.1 Property Assignment UI
- [ ] Select node → assign boundary condition
- [ ] Select beam → assign material properties
- [ ] Select beam → assign cross-section properties
- [ ] Store in PointData/CellData arrays

### F3.2 Lookup Table - `vtk_lookup_table.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_range(min, max)` - data range
- [ ] `set_hue_range(min, max)` - color range [0,1]
- [ ] `set_value_range(min, max)` - brightness
- [ ] `set_number_of_table_values(n)`
- [ ] `build()` - generate color table
- [ ] `get_color(value)` → (r, g, b)

### F3.3 Mapper Color Mapping
- [ ] Extend PolyDataMapper with color mapping
- [ ] `set_lookup_table(lut)`
- [ ] `set_scalar_range(min, max)`
- [ ] `set_scalar_mode_to_use_point_data()` / `use_cell_data()`
- [ ] `set_scalar_visibility(bool)`

### F3.4 Scalar Bar Actor - `vtk_scalar_bar_actor.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_lookup_table(lut)`
- [ ] `set_title(text: &str)`
- [ ] `set_number_of_labels(n)`
- [ ] `set_position(x, y)` / `set_width(w)` / `set_height(h)`
- [ ] `get_label_text_property()` / `get_title_text_property()`

### F3.5 Example: Property Visualization
- [ ] Create `examples/fem_properties_viz.rs`
- [ ] Color nodes by boundary condition type
- [ ] Color beams by material property (E, A, I)
- [ ] Add scalar bar legend
- [ ] Interactive property editing

---

## Sprint F4: Result Post-Processing

### F4.1 Deformation Visualization
- [ ] Load FEM results (displacements at nodes)
- [ ] Store displacements in PointData as vectors
- [ ] Apply WarpVector filter to show deformed shape

### F4.2 Warp Vector Filter - `vtk_warp_vector.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input_connection(port)`
- [ ] `set_scale_factor(scale)` - amplification factor
- [ ] `get_output_port()`

### F4.3 Stress/Force Visualization
- [ ] Store element forces/moments in CellData
- [ ] Store nodal stresses in PointData
- [ ] Color-code by stress level using LookupTable
- [ ] Min/max stress labels

### F4.4 Arrow Glyph for Forces - `vtk_arrow_source.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_tip_resolution(n)` / `set_shaft_resolution(n)`
- [ ] `set_tip_radius(r)` / `set_shaft_radius(r)`
- [ ] `set_tip_length(len)`
- [ ] `get_output_port()`

### F4.5 Glyph for Vector Fields
- [ ] Use Glyph3D with ArrowSource
- [ ] Display force vectors at nodes
- [ ] Display moment vectors at nodes
- [ ] Scale arrows by magnitude

### F4.6 Example: FEM Result Visualization
- [ ] Create `examples/fem_results_viz.rs`
- [ ] Load displacement results
- [ ] Show original + deformed shape
- [ ] Color by stress
- [ ] Display force arrows
- [ ] Animate deformation

---

## Sprint F5: Advanced Beam Visualization

### F5.1 Variable Radius Tubes
- [ ] Extend TubeFilter to vary radius along beam
- [ ] Visualize stress with tube thickness
- [ ] Visualize bending moment with varying diameter

### F5.2 Extrusion for Cross-Sections
- [ ] Show actual beam cross-section shapes
- [ ] I-beams, rectangular, circular, etc.

### F5.3 Linear Extrusion - `vtk_linear_extrusion_filter.rs`
- [ ] Create new module and C++ bindings
- [ ] `new()` / `delete()`
- [ ] `set_input_connection(port)` - 2D profile
- [ ] `set_extrusion_type(NORMAL/POINT/VECTOR)`
- [ ] `set_scale_factor(scale)`
- [ ] `set_vector(x, y, z)` - extrusion direction
- [ ] `get_output_port()`

### F5.4 Example: Cross-Section Visualization
- [ ] Create `examples/beam_cross_sections.rs`
- [ ] Define cross-section profiles (I, H, rectangular, circular)
- [ ] Extrude along beam paths
- [ ] Show realistic 3D beam structures

---

## Sprint F6: Model Verification & Checks

### F6.1 Connectivity Checks
- [ ] Find disconnected nodes
- [ ] Find duplicate beams
- [ ] Verify node-to-beam references

### F6.2 Geometry Validation
- [ ] Check for zero-length beams
- [ ] Check for overlapping beams
- [ ] Validate boundary conditions (at least one fixed node)

### F6.3 Visual Highlighting
- [ ] Highlight problematic elements in red
- [ ] Show warnings with text annotations
- [ ] Blink or pulse effect for errors

### F6.4 Example: Model Validation
- [ ] Create `examples/fem_model_validation.rs`
- [ ] Run checks on beam model
- [ ] Display results visually
- [ ] Generate validation report

---

## Sprint F7: File I/O for FEM Data

### F7.1 Custom FEM Format Writer
- [ ] Export nodes (ID, x, y, z)
- [ ] Export elements (ID, node1, node2, material_id, section_id)
- [ ] Export properties (materials, cross-sections)
- [ ] Export boundary conditions
- [ ] JSON or custom text format

### F7.2 Custom FEM Format Reader
- [ ] Parse node definitions
- [ ] Parse element definitions
- [ ] Parse properties
- [ ] Parse boundary conditions
- [ ] Rebuild VTK representation

### F7.3 Results File I/O
- [ ] Export FEM input file for solver
- [ ] Import solver results (displacements, forces, stresses)
- [ ] Map results back to VTK data arrays

### F7.4 VTK Format Support (Optional)
- [ ] Write PolyData to .vtp file
- [ ] Read PolyData from .vtp file
- [ ] Preserve all custom data arrays

---

## Sprint F8: Advanced Selection & Editing

### F8.1 Box Selection
- [ ] Select multiple nodes/beams by box
- [ ] Use AreaPicker or rubber-band style
- [ ] Bulk property assignment

### F8.2 Query/Filter Selection
- [ ] Select by property value (e.g., all steel beams)
- [ ] Select by boundary condition type
- [ ] Select by stress range

### F8.3 Transform Tools
- [ ] Move selected nodes
- [ ] Rotate selected sub-structure
- [ ] Mirror geometry
- [ ] Copy/paste elements

### F8.4 Undo/Redo System
- [ ] Command pattern for all modifications
- [ ] Undo stack
- [ ] Redo stack

---

## Sprint F9: Analysis Integration

### F9.1 External Solver Interface
- [ ] Write input file for external FEM solver
- [ ] Execute solver as subprocess
- [ ] Monitor solve progress
- [ ] Read results automatically

### F9.2 Simple Built-in Solver (Optional)
- [ ] 2D truss solver
- [ ] 3D beam solver (Timoshenko or Euler-Bernoulli)
- [ ] Direct stiffness method
- [ ] Linear static analysis only

### F9.3 Result Comparison
- [ ] Load multiple result sets
- [ ] Compare deformations
- [ ] Compare stresses
- [ ] Show differences

---

## Sprint F10: Reports & Documentation

### F10.1 Screenshot/Image Export
- [ ] Render to image file (PNG, JPEG)
- [ ] High-resolution rendering
- [ ] White background for reports

### F10.2 Automatic Report Generation
- [ ] Model summary (# nodes, # elements)
- [ ] Property table
- [ ] Result summary (max displacement, max stress)
- [ ] Include screenshots
- [ ] Export to HTML/PDF

### F10.3 Annotations for Reports
- [ ] Title blocks
- [ ] Scale indicators
- [ ] North arrow / coordinate system indicator
- [ ] Dimension lines with measurements

---

## Implementation Priority for Beam FEM

**Phase 1: Foundation (Start Here)**
1. Sprint F1.1-F1.3: Points, CellArray, enhanced PolyData
2. Sprint F2.3: TubeFilter for beam visualization
3. Sprint F2.4-F2.5: Glyph3D + SphereSource for nodes
4. Example: Basic static beam model display

**Phase 2: Interactivity**
5. Sprint F2.1-F2.2: Interactive node/beam creation
6. Sprint F2.6: Beam model builder example
7. Sprint F3.1-F3.5: Property assignment and visualization

**Phase 3: Results**
8. Sprint F4.1-F4.3: Deformation and stress visualization
9. Sprint F4.4-F4.5: Force/moment arrows
10. Sprint F4.6: Results visualization example

**Phase 4: Production**
11. Sprint F7: File I/O
12. Sprint F8: Advanced selection
13. Sprint F10: Reports

**Phase 5: Advanced (Optional)**
14. Sprint F5: Advanced beam viz with cross-sections
15. Sprint F6: Model validation
16. Sprint F9: Solver integration

