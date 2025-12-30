use std::sync::Mutex;
use std::collections::HashMap;

use crate::vtk_actor::ffi::vtkProperty;

// Ensure the vtkrs static library is linked
#[link(name = "vtkrs", kind = "static")]
extern "C" {}

// Global registry for callbacks
static CALLBACK_REGISTRY: Mutex<Option<CallbackRegistry>> = Mutex::new(None);

struct CallbackRegistry {
    next_id: i64,
    left_press_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    left_release_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    mouse_move_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    key_press_callbacks: HashMap<i64, Box<dyn Fn(&str) + Send>>,
    right_press_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    right_release_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    middle_press_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    middle_release_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
}

impl CallbackRegistry {
    fn new() -> Self {
        Self {
            next_id: 1,
            left_press_callbacks: HashMap::new(),
            left_release_callbacks: HashMap::new(),
            mouse_move_callbacks: HashMap::new(),
            key_press_callbacks: HashMap::new(),
            right_press_callbacks: HashMap::new(),
            right_release_callbacks: HashMap::new(),
            middle_press_callbacks: HashMap::new(),
            middle_release_callbacks: HashMap::new(),
        }
    }

    fn register_left_press<F>(&mut self, callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.left_press_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_left_release<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.left_release_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_mouse_move<F>(&mut self, callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.mouse_move_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_key_press<F>(&mut self, callback: F) -> i64 where F: Fn(&str) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.key_press_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_right_press<F>(&mut self, callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.right_press_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_right_release<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.right_release_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_middle_press<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.middle_press_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_middle_release<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.middle_release_callbacks.insert(id, Box::new(callback));
        id
    }
}

fn get_or_create_registry() -> &'static Mutex<Option<CallbackRegistry>> {
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(CallbackRegistry::new());
    }
    drop(registry);
    &CALLBACK_REGISTRY
}

// Direct extern "C" FFI (bypassing cxx bridge which was crashing)
#[repr(C)]
pub struct vtkInteractorStyleImage {
    _private: [u8; 0],
}

extern "C" {
    fn interactor_style_image_new() -> *mut vtkInteractorStyleImage;
    fn interactor_style_image_delete(style: *mut vtkInteractorStyleImage);

    fn interactor_style_image_set_left_button_press_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_left_button_release_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_right_button_press_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_right_button_release_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_middle_button_press_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_middle_button_release_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_mouse_move_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_set_key_press_callback_id(
        style: *mut vtkInteractorStyleImage,
        callback_id: i64
    );
    fn interactor_style_image_is_moving(style: *mut vtkInteractorStyleImage) -> bool;
    fn interactor_style_image_get_selection_positions(
        style: *mut vtkInteractorStyleImage,
        start_x: *mut i32,
        start_y: *mut i32,
        end_x: *mut i32,
        end_y: *mut i32
    );

    fn interactor_style_isA(style: *mut vtkInteractorStyleImage, name: &str) -> bool;

    fn interactor_style_print_self(style: *mut vtkInteractorStyleImage, indent: i32);
    fn interactor_style_on_char(style: *mut vtkInteractorStyleImage);
    fn interactor_style_window_level(style: *mut vtkInteractorStyleImage);
    fn interactor_style_pick(style: *mut vtkInteractorStyleImage);
    fn interactor_style_slice(style: *mut vtkInteractorStyleImage);
    fn interactor_style_start_window_level(style: *mut vtkInteractorStyleImage);
    fn interactor_style_end_window_level(style: *mut vtkInteractorStyleImage);
    fn interactor_style_start_pick(style: *mut vtkInteractorStyleImage);
    fn interactor_style_end_pick(style: *mut vtkInteractorStyleImage);
    fn interactor_style_start_slice(style: *mut vtkInteractorStyleImage);
    fn interactor_style_end_slice(style: *mut vtkInteractorStyleImage);
    fn interactor_style_set_image_orientation(
        style: *mut vtkInteractorStyleImage,
        right: &[f64; 3],
        up: &[f64; 3]
    );
    fn interactor_style_set_current_image_number(
        style: *mut vtkInteractorStyleImage,
        image_number: i32
    );
    fn interactor_style_get_current_image_number(style: *mut vtkInteractorStyleImage) -> i32;
    fn interactor_style_get_current_image_property(
        style: *mut vtkInteractorStyleImage
    ) -> *mut vtkProperty;
    fn interactor_style_get_window_level_start_position(
        style: *mut vtkInteractorStyleImage,
        x: *mut i32,
        y: *mut i32
    );
    fn interactor_style_get_window_level_current_position(
        style: *mut vtkInteractorStyleImage,
        x: *mut i32,
        y: *mut i32
    );

    fn interactor_style_set_interaction_mode(style: *mut vtkInteractorStyleImage, mode: i32);
    fn interactor_style_get_interaction_mode(style: *mut vtkInteractorStyleImage) -> i32;

    fn interactor_style_set_interaction_mode_to_image_2d(style: *mut vtkInteractorStyleImage);
    fn interactor_style_set_interaction_mode_to_image_3d(style: *mut vtkInteractorStyleImage);
    fn interactor_style_set_interaction_mode_to_image_slicing(style: *mut vtkInteractorStyleImage);
    fn interactor_style_set_x_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: &[f64; 3]
    );
    fn interactor_style_get_x_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: *mut [f64; 3]
    );
    fn interactor_style_set_x_view_up_vector(style: *mut vtkInteractorStyleImage, up: [f64; 3]);
    fn interactor_style_get_x_view_up_vector(
        style: *mut vtkInteractorStyleImage,
        up: *mut [f64; 3]
    );

    fn interactor_style_set_y_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: &[f64; 3]
    );
    fn interactor_style_get_y_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: *mut [f64; 3]
    );
    fn interactor_style_set_y_view_up_vector(style: *mut vtkInteractorStyleImage, up: &[f64; 3]);
    fn interactor_style_get_y_view_up_vector(
        style: *mut vtkInteractorStyleImage,
        up: *mut [f64; 3]
    );

    fn interactor_style_set_z_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: &[f64; 3]
    );
    fn interactor_style_get_z_view_right_vector(
        style: *mut vtkInteractorStyleImage,
        right: *mut [f64; 3]
    );
    fn interactor_style_set_z_view_up_vector(style: *mut vtkInteractorStyleImage, up: &[f64; 3]);
    fn interactor_style_get_z_view_up_vector(
        style: *mut vtkInteractorStyleImage,
        up: *mut [f64; 3]
    );
}

// Wrapper struct for safe Rust API
pub struct InteractorStyleImage {
    ptr: *mut vtkInteractorStyleImage,
}

impl InteractorStyleImage {
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { interactor_style_image_new() };
        if ptr.is_null() {
            panic!("Failed to create InteractorStyleImage");
        }
        Self { ptr }
    }

    pub fn as_mut_ptr(&mut self) -> *mut vtkInteractorStyleImage {
        self.ptr as *mut vtkInteractorStyleImage
    }

    /// Get a non-mut pointer to the underlying interactor (works from &self).
    pub fn as_ptr(&self) -> *mut vtkInteractorStyleImage {
        self.ptr as *mut vtkInteractorStyleImage
    }

    /// Set callback for left mouse button press events.
    /// The callback receives the (x, y) position of the click.
    pub fn set_left_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_left_press_callback(callback);
        unsafe {
            interactor_style_image_set_left_button_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for left mouse button release events.
    /// The callback receives the (x, y) position where the button was released.
    pub fn set_left_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_left_release_callback(callback);
        unsafe {
            interactor_style_image_set_left_button_release_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for right mouse button press events.
    /// The callback receives the (x, y) position of the click.
    pub fn set_right_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_right_press_callback(callback);
        unsafe {
            interactor_style_image_set_right_button_press_callback_id(self.ptr, callback_id);
        }
    }
    /// Set callback for right mouse button release events.
    /// The callback receives the (x, y) position where the button was released.
    pub fn set_right_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_right_release_callback(callback);
        unsafe {
            interactor_style_image_set_right_button_release_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for middle mouse button press events.
    /// The callback receives the (x, y) position of the click.
    pub fn set_middle_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_middle_press_callback(callback);
        unsafe {
            interactor_style_image_set_middle_button_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for middle mouse button release events.
    /// The callback receives the (x, y) position where the button was released.
    pub fn set_middle_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id =
            crate::vtk_interactor_style_custom::register_middle_release_callback(callback);
        unsafe {
            interactor_style_image_set_middle_button_release_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for mouse move events.
    /// The callback receives the current (x, y) position of the mouse.
    /// Note: This fires frequently during mouse movement.
    pub fn set_mouse_move_callback<F>(&mut self, callback: F) where F: Fn(i32, i32) + Send + 'static {
        let callback_id =
            crate::vtk_interactor_style_custom::register_mouse_move_callback(callback);
        unsafe {
            interactor_style_image_set_mouse_move_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for key press events.
    /// The callback receives the key symbol as a string (e.g., "m", "Escape", "F1").
    pub fn set_key_press_callback<F>(&mut self, callback: F)
        where F: Fn(&str) -> bool + Send + 'static
    {
        let callback_id = crate::vtk_interactor_style_custom::register_key_press_callback(callback);
        unsafe {
            interactor_style_image_set_key_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Check if currently in selection/moving state
    pub fn is_moving(&self) -> bool {
        unsafe { interactor_style_image_is_moving(self.ptr) }
    }

    /// Get current selection rectangle positions
    pub fn get_selection_positions(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut start_x = 0;
            let mut start_y = 0;
            let mut end_x = 0;
            let mut end_y = 0;
            interactor_style_image_get_selection_positions(
                self.ptr,
                &mut start_x,
                &mut start_y,
                &mut end_x,
                &mut end_y
            );
            (start_x, start_y, end_x, end_y)
        }
    }

    /// Check type name
    pub fn is_a(&self, name: &str) -> bool {
        unsafe { interactor_style_isA(self.as_ptr(), name) }
    }

    /// Print self with given indent
    pub fn print_self(&self, indent: i32) {
        unsafe { interactor_style_print_self(self.as_ptr(), indent) }
    }

    /// Character event
    pub fn on_char(&self) {
        unsafe { interactor_style_on_char(self.as_ptr()) }
    }

    /// Window/level interaction helper
    pub fn window_level(&self) {
        unsafe { interactor_style_window_level(self.as_ptr()) }
    }

    /// Pick helper
    pub fn pick(&self) {
        unsafe { interactor_style_pick(self.as_ptr()) }
    }

    /// Slice helper
    pub fn slice(&self) {
        unsafe { interactor_style_slice(self.as_ptr()) }
    }

    pub fn start_window_level(&self) {
        unsafe { interactor_style_start_window_level(self.as_ptr()) }
    }

    pub fn end_window_level(&self) {
        unsafe { interactor_style_end_window_level(self.as_ptr()) }
    }

    pub fn start_pick(&self) {
        unsafe { interactor_style_start_pick(self.as_ptr()) }
    }

    pub fn end_pick(&self) {
        unsafe { interactor_style_end_pick(self.as_ptr()) }
    }

    pub fn start_slice(&self) {
        unsafe { interactor_style_start_slice(self.as_ptr()) }
    }

    pub fn end_slice(&self) {
        unsafe { interactor_style_end_slice(self.as_ptr()) }
    }

    /// Set orientation vectors (right, up)
    pub fn set_image_orientation(&mut self, right: &[f64; 3], up: &[f64; 3]) {
        unsafe { interactor_style_set_image_orientation(self.as_ptr(), right, up) }
    }

    /// Set/get current image index
    pub fn set_current_image_number(&mut self, image_number: i32) {
        unsafe { interactor_style_set_current_image_number(self.as_ptr(), image_number) }
    }

    pub fn get_current_image_number(&self) -> i32 {
        unsafe { interactor_style_get_current_image_number(self.as_ptr()) }
    }

    /// Return raw pointer to the current image property (may be null)
    pub fn get_current_image_property(&self) -> *mut vtkProperty {
        unsafe { interactor_style_get_current_image_property(self.as_ptr()) }
    }

    /// Window/level positions
    pub fn get_window_level_start_position(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            interactor_style_get_window_level_start_position(self.as_ptr(), &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_window_level_current_position(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            interactor_style_get_window_level_current_position(self.as_ptr(), &mut x, &mut y);
            (x, y)
        }
    }

    /// Interaction mode helpers
    pub fn set_interaction_mode(&mut self, mode: i32) {
        unsafe { interactor_style_set_interaction_mode(self.as_ptr(), mode) }
    }

    pub fn get_interaction_mode(&self) -> i32 {
        unsafe { interactor_style_get_interaction_mode(self.as_ptr()) }
    }

    pub fn set_interaction_mode_to_image_2d(&mut self) {
        unsafe { interactor_style_set_interaction_mode_to_image_2d(self.as_ptr()) }
    }

    pub fn set_interaction_mode_to_image_3d(&mut self) {
        unsafe { interactor_style_set_interaction_mode_to_image_3d(self.as_ptr()) }
    }

    pub fn set_interaction_mode_to_image_slicing(&mut self) {
        unsafe { interactor_style_set_interaction_mode_to_image_slicing(self.as_ptr()) }
    }

    /// View vectors (X)
    pub fn set_x_view_right_vector(&mut self, right: &[f64; 3]) {
        unsafe { interactor_style_set_x_view_right_vector(self.as_ptr(), right) }
    }

    pub fn get_x_view_right_vector(&self) -> [f64; 3] {
        unsafe {
            let mut right = [0.0_f64; 3];
            interactor_style_get_x_view_right_vector(self.as_ptr(), &mut right);
            right
        }
    }

    pub fn set_x_view_up_vector(&mut self, up: [f64; 3]) {
        unsafe { interactor_style_set_x_view_up_vector(self.as_ptr(), up) }
    }

    pub fn get_x_view_up_vector(&self) -> [f64; 3] {
        unsafe {
            let mut up = [0.0_f64; 3];
            interactor_style_get_x_view_up_vector(self.as_ptr(), &mut up);
            up
        }
    }

    /// View vectors (Y)
    pub fn set_y_view_right_vector(&mut self, right: &[f64; 3]) {
        unsafe { interactor_style_set_y_view_right_vector(self.as_ptr(), right) }
    }

    pub fn get_y_view_right_vector(&self) -> [f64; 3] {
        unsafe {
            let mut right = [0.0_f64; 3];
            interactor_style_get_y_view_right_vector(self.as_ptr(), &mut right);
            right
        }
    }

    pub fn set_y_view_up_vector(&mut self, up: &[f64; 3]) {
        unsafe { interactor_style_set_y_view_up_vector(self.as_ptr(), up) }
    }

    pub fn get_y_view_up_vector(&self) -> [f64; 3] {
        unsafe {
            let mut up = [0.0_f64; 3];
            interactor_style_get_y_view_up_vector(self.as_ptr(), &mut up);
            up
        }
    }

    /// View vectors (Z)
    pub fn set_z_view_right_vector(&mut self, right: &[f64; 3]) {
        unsafe { interactor_style_set_z_view_right_vector(self.as_ptr(), right) }
    }

    pub fn get_z_view_right_vector(&self) -> [f64; 3] {
        unsafe {
            let mut right = [0.0_f64; 3];
            interactor_style_get_z_view_right_vector(self.as_ptr(), &mut right);
            right
        }
    }

    pub fn set_z_view_up_vector(&mut self, up: &[f64; 3]) {
        unsafe { interactor_style_set_z_view_up_vector(self.as_ptr(), up) }
    }

    pub fn get_z_view_up_vector(&self) -> [f64; 3] {
        unsafe {
            let mut up = [0.0_f64; 3];
            interactor_style_get_z_view_up_vector(self.as_ptr(), &mut up);
            up
        }
    }
}

impl Drop for InteractorStyleImage {
    fn drop(&mut self) {
        unsafe {
            interactor_style_image_delete(self.ptr);
        }
    }
}
