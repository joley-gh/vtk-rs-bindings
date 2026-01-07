use std::sync::Mutex;
use std::collections::HashMap;

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
    // Actor-aware callbacks receive the actor pointer (nullable) as third arg
    left_press_actor_callbacks: HashMap<
        i64,
        Box<dyn Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send>
    >,
    left_release_actor_callbacks: HashMap<
        i64,
        Box<dyn Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send>
    >,
    mouse_move_actor_callbacks: HashMap<
        i64,
        Box<dyn Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send>
    >,
    middle_press_actor_callbacks: HashMap<
        i64,
        Box<dyn Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send>
    >,
    middle_release_actor_callbacks: HashMap<
        i64,
        Box<dyn Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send>
    >,
    key_press_callbacks: HashMap<i64, Box<dyn (Fn(&str) -> bool) + Send>>,
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
            left_press_actor_callbacks: HashMap::new(),
            left_release_actor_callbacks: HashMap::new(),
            mouse_move_actor_callbacks: HashMap::new(),
            middle_press_actor_callbacks: HashMap::new(),
            middle_release_actor_callbacks: HashMap::new(),
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

    fn register_left_press_with_actor<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.left_press_actor_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_left_release_with_actor<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.left_release_actor_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_mouse_move_with_actor<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.mouse_move_actor_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_middle_press_with_actor<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.middle_press_actor_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_middle_release_with_actor<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.middle_release_actor_callbacks.insert(id, Box::new(callback));
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

    fn register_key_press<F>(&mut self, callback: F) -> i64
        where F: Fn(&str) -> bool + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.key_press_callbacks.insert(id, Box::new(callback));
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

// Public registration helpers so other modules (e.g. `vtk_interactor_style_image`) can
// register callbacks into the single global registry used by the C++ trampolines.
pub fn register_left_press_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_left_press(callback)
}

pub fn register_left_release_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_left_release(callback)
}

pub fn register_right_press_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_right_press(callback)
}

pub fn register_right_release_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_right_release(callback)
}

pub fn register_middle_press_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_middle_press(callback)
}

pub fn register_middle_release_callback<F>(callback: F) -> i64
    where F: Fn(i32, i32) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_middle_release(callback)
}

pub fn register_mouse_move_callback<F>(callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_mouse_move(callback)
}

pub fn register_left_press_callback_with_actor<F>(callback: F) -> i64
    where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_left_press_with_actor(callback)
}

pub fn register_left_release_callback_with_actor<F>(callback: F) -> i64
    where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_left_release_with_actor(callback)
}

pub fn register_mouse_move_callback_with_actor<F>(callback: F) -> i64
    where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_mouse_move_with_actor(callback)
}

pub fn register_middle_press_callback_with_actor<F>(callback: F) -> i64
    where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_middle_press_with_actor(callback)
}

pub fn register_middle_release_callback_with_actor<F>(callback: F) -> i64
    where F: Fn(i32, i32, *mut crate::vtk_actor::ffi::vtkActor) + Send + 'static
{
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_middle_release_with_actor(callback)
}

pub fn register_key_press_callback<F>(callback: F) -> i64 where F: Fn(&str) -> bool + Send + 'static {
    get_or_create_registry();
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    registry.as_mut().unwrap().register_key_press(callback)
}

// C callback trampolines - these are called from C++
#[no_mangle]
pub extern "C" fn vtk_rs_left_button_press_callback(callback_id: i64, x: i32, y: i32) {
    // Add defensive check and error handling
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_press_callbacks.get(&callback_id) {
                // Use catch_unwind to prevent panics from crossing FFI boundary
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_left_button_release_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_release_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_mouse_move_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.mouse_move_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_left_button_press_callback_with_actor(
    callback_id: i64,
    x: i32,
    y: i32,
    actor: *mut crate::vtk_actor::ffi::vtkActor
) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_press_actor_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y, actor);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_left_button_release_callback_with_actor(
    callback_id: i64,
    x: i32,
    y: i32,
    actor: *mut crate::vtk_actor::ffi::vtkActor
) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_release_actor_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y, actor);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_mouse_move_callback_with_actor(
    callback_id: i64,
    x: i32,
    y: i32,
    actor: *mut crate::vtk_actor::ffi::vtkActor
) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.mouse_move_actor_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y, actor);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_middle_button_press_callback_with_actor(
    callback_id: i64,
    x: i32,
    y: i32,
    actor: *mut crate::vtk_actor::ffi::vtkActor
) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.middle_press_actor_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y, actor);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_middle_button_release_callback_with_actor(
    callback_id: i64,
    x: i32,
    y: i32,
    actor: *mut crate::vtk_actor::ffi::vtkActor
) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.middle_release_actor_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y, actor);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_right_button_press_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.right_press_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_right_button_release_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.right_release_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_middle_button_press_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.middle_press_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_middle_button_release_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.middle_release_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_key_press_callback(
    callback_id: i64,
    key: *const std::os::raw::c_char
) -> i32 {
    if callback_id == 0 || key.is_null() {
        return 0;
    }

    // Convert C string to Rust string
    let key_str = unsafe { std::ffi::CStr::from_ptr(key).to_str().unwrap_or("") };

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.key_press_callbacks.get(&callback_id) {
                let res = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| { callback(key_str) })
                );

                if let Ok(handled) = res {
                    return if handled { 1 } else { 0 };
                }
            }
        }
    }

    0
}

// Direct extern "C" FFI (bypassing cxx bridge which was crashing)
// Opaque C type for FFI
#[repr(C)]
pub struct vtkInteractorStyleCustom {
    _private: [u8; 0],
}

extern "C" {
    fn interactor_style_custom_new() -> *mut vtkInteractorStyleCustom;
    fn interactor_style_custom_delete(style: *mut vtkInteractorStyleCustom);
    fn interactor_style_custom_set_left_button_press_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_left_button_release_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_mouse_move_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_key_press_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_selection_mode(
        style: *mut vtkInteractorStyleCustom,
        enabled: bool
    );
    fn interactor_style_custom_is_moving(style: *mut vtkInteractorStyleCustom) -> bool;
    fn interactor_style_custom_get_selection_positions(
        style: *mut vtkInteractorStyleCustom,
        start_x: *mut i32,
        start_y: *mut i32,
        end_x: *mut i32,
        end_y: *mut i32
    );
}

// Wrapper struct for safe Rust API
pub struct InteractorStyleCustom {
    ptr: *mut vtkInteractorStyleCustom,
}

impl InteractorStyleCustom {
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { interactor_style_custom_new() };
        if ptr.is_null() {
            panic!("Failed to create InteractorStyleCustom");
        }
        Self { ptr }
    }

    pub fn as_mut_ptr(&mut self) -> *mut vtkInteractorStyleCustom {
        self.ptr
    }

    /// Set callback for left mouse button press events.
    /// The callback receives the (x, y) position of the click.
    pub fn set_left_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_left_press(callback)
        };

        unsafe {
            interactor_style_custom_set_left_button_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for left mouse button release events.
    /// The callback receives the (x, y) position where the button was released.
    pub fn set_left_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_left_release(callback)
        };

        unsafe {
            interactor_style_custom_set_left_button_release_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for mouse move events.
    /// The callback receives the current (x, y) position of the mouse.
    /// Note: This fires frequently during mouse movement.
    pub fn set_mouse_move_callback<F>(&mut self, callback: F) where F: Fn(i32, i32) + Send + 'static {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_mouse_move(callback)
        };

        unsafe {
            interactor_style_custom_set_mouse_move_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for key press events.
    /// The callback receives the key symbol as a string (e.g., "m", "Escape", "F1").
    /// Return `true` from the callback to indicate the event was handled and the
    /// default VTK behavior should be suppressed; return `false` to allow the
    /// usual VTK handling to proceed.
    pub fn set_key_press_callback<F>(&mut self, callback: F)
        where F: Fn(&str) -> bool + Send + 'static
    {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_key_press(callback)
        };

        unsafe {
            interactor_style_custom_set_key_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Enable or disable selection mode.
    /// When selection mode is enabled, left-click-drag will not rotate the camera,
    /// allowing for area selection operations.
    pub fn set_selection_mode(&mut self, enabled: bool) {
        unsafe {
            interactor_style_custom_set_selection_mode(self.ptr, enabled);
        }
    }

    /// Check if currently in selection/moving state
    pub fn is_moving(&self) -> bool {
        unsafe { interactor_style_custom_is_moving(self.ptr) }
    }

    /// Get current selection rectangle positions
    pub fn get_selection_positions(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut start_x = 0;
            let mut start_y = 0;
            let mut end_x = 0;
            let mut end_y = 0;
            interactor_style_custom_get_selection_positions(
                self.ptr,
                &mut start_x,
                &mut start_y,
                &mut end_x,
                &mut end_y
            );
            (start_x, start_y, end_x, end_y)
        }
    }
}

/// Draw a rubber band selection rectangle on the render window.
/// This is application logic that uses VTK render window pixel manipulation.
///
/// This is the unoptimized version that re-renders on every call.
/// For smooth, flicker-free drawing during mouse drag, use `draw_rubber_band_rectangle_cached` instead.
pub fn draw_rubber_band_rectangle(
    render_window: &mut crate::RenderWindow,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    color: (u8, u8, u8),
    thickness: usize
) {
    // First render the scene to get current frame
    render_window.render();

    let size = render_window.get_size();
    let width = size.0;
    let height = size.1;

    // Calculate bounds
    let min_x = start_x
        .min(end_x)
        .max(0)
        .min(width - 1);
    let max_x = start_x
        .max(end_x)
        .max(0)
        .min(width - 1);
    let min_y = start_y
        .min(end_y)
        .max(0)
        .min(height - 1);
    let max_y = start_y
        .max(end_y)
        .max(0)
        .min(height - 1);

    // Get pixel data from window
    let mut pixels = render_window.get_pixel_data();

    // Draw horizontal lines (top and bottom)
    for x in min_x..=max_x {
        for offset in 0..thickness as i32 {
            // Top line
            let y_top = min_y + offset;
            if y_top >= 0 && y_top < height {
                let idx = (4 * (y_top * width + x)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }

            // Bottom line
            let y_bottom = max_y - offset;
            if y_bottom >= 0 && y_bottom < height {
                let idx = (4 * (y_bottom * width + x)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    // Draw vertical lines (left and right)
    for y in min_y..=max_y {
        for offset in 0..thickness as i32 {
            // Left line
            let x_left = min_x + offset;
            if x_left >= 0 && x_left < width {
                let idx = (4 * (y * width + x_left)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }

            // Right line
            let x_right = max_x - offset;
            if x_right >= 0 && x_right < width {
                let idx = (4 * (y * width + x_right)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    // Set pixels back to front buffer for immediate display
    render_window.set_pixel_data(&pixels);

    // Print coordinates for console feedback
    eprint!("\rSelection: ({},{}) to ({},{})", start_x, start_y, end_x, end_y);
    use std::io::Write;
    let _ = std::io::stderr().flush();
}

/// Optimized rubber band drawing using a cached base frame (no re-rendering during drag).
/// This provides smooth, flicker-free rectangle drawing by avoiding full 3D scene re-renders.
///
/// Use this when drawing repeatedly during mouse drag for best performance.
/// The base_frame should be captured once at the start of the drag operation.
pub fn draw_rubber_band_rectangle_cached(
    render_window: &mut crate::RenderWindow,
    base_frame: &[u8],
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    color: (u8, u8, u8),
    thickness: usize
) {
    let size = render_window.get_size();
    let width = size.0;
    let height = size.1;

    // Clone the base frame
    let mut pixels = base_frame.to_vec();

    // Calculate bounds
    let min_x = start_x
        .min(end_x)
        .max(0)
        .min(width - 1);
    let max_x = start_x
        .max(end_x)
        .max(0)
        .min(width - 1);
    let min_y = start_y
        .min(end_y)
        .max(0)
        .min(height - 1);
    let max_y = start_y
        .max(end_y)
        .max(0)
        .min(height - 1);

    // Draw horizontal lines (top and bottom)
    for x in min_x..=max_x {
        for offset in 0..thickness as i32 {
            // Top line
            let y_top = min_y + offset;
            if y_top >= 0 && y_top < height {
                let idx = (4 * (y_top * width + x)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }

            // Bottom line
            let y_bottom = max_y - offset;
            if y_bottom >= 0 && y_bottom < height {
                let idx = (4 * (y_bottom * width + x)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    // Draw vertical lines (left and right)
    for y in min_y..=max_y {
        for offset in 0..thickness as i32 {
            // Left line
            let x_left = min_x + offset;
            if x_left >= 0 && x_left < width {
                let idx = (4 * (y * width + x_left)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }

            // Right line
            let x_right = max_x - offset;
            if x_right >= 0 && x_right < width {
                let idx = (4 * (y * width + x_right)) as usize;
                if idx + 3 < pixels.len() {
                    pixels[idx] = color.0;
                    pixels[idx + 1] = color.1;
                    pixels[idx + 2] = color.2;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }

    // Set pixels directly without re-rendering
    render_window.set_pixel_data(&pixels);

    // Print coordinates for console feedback
    eprint!("\rSelection: ({},{}) to ({},{})", start_x, start_y, end_x, end_y);
    use std::io::Write;
    let _ = std::io::stderr().flush();
}

impl Drop for InteractorStyleCustom {
    fn drop(&mut self) {
        unsafe {
            interactor_style_custom_delete(self.ptr);
        }
    }
}
