use std::sync::Mutex;
use std::collections::HashMap;

// Global registry for camera observer callbacks
static CAMERA_OBSERVER_REGISTRY: Mutex<Option<CameraObserverRegistry>> = Mutex::new(None);

struct CameraObserverRegistry {
    next_id: i64,
    // Callback takes camera pointer as usize to avoid FFI type visibility issues
    modified_callbacks: HashMap<i64, Box<dyn Fn(usize) + Send>>,
}

impl CameraObserverRegistry {
    fn new() -> Self {
        Self {
            next_id: 1,
            modified_callbacks: HashMap::new(),
        }
    }

    fn register_modified<F>(&mut self, callback: F) -> i64 where F: Fn(usize) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.modified_callbacks.insert(id, Box::new(callback));
        id
    }
}

fn get_or_create_registry() -> &'static Mutex<Option<CameraObserverRegistry>> {
    &CAMERA_OBSERVER_REGISTRY
}

// Trampoline function called from C++
#[no_mangle]
pub extern "C" fn camera_observer_modified_trampoline(
    caller: usize,
    _event_id: usize,
    callback_id: usize
) {
    let result = std::panic::catch_unwind(|| {
        let registry_guard = get_or_create_registry().lock().unwrap();
        if let Some(ref registry) = *registry_guard {
            if let Some(callback) = registry.modified_callbacks.get(&(callback_id as i64)) {
                callback(caller);
            }
        }
    });
    if result.is_err() {
        eprintln!("Panic in camera observer callback!");
    }
}

/// Extension trait to add closure-based observer registration to CameraRef
pub trait CameraObserverExt {
    /// Register a closure to be called when the camera is modified.
    /// Returns a tuple of (observer_tag, Command) that must be kept alive.
    ///
    /// The closure receives a CameraRef which can be used to query camera state.
    ///
    /// # Example
    /// ```no_run
    /// use vtk_rs::CameraObserverExt;
    ///
    /// let mut camera = renderer.get_active_camera();
    /// let (_tag, _command) = camera.on_modified(|camera_ref| {
    ///     let (x, y, z) = camera_ref.get_position();
    ///     println!("Camera moved to: ({}, {}, {})", x, y, z);
    /// });
    /// // Keep _command alive for the duration of the observer
    /// ```
    fn on_modified<F>(&mut self, callback: F) -> (usize, crate::Command)
        where F: Fn(&mut crate::CameraRef) + Send + 'static;
}

impl CameraObserverExt for crate::CameraRef {
    fn on_modified<F>(&mut self, callback: F) -> (usize, crate::Command)
        where F: Fn(&mut crate::CameraRef) + Send + 'static
    {
        // Wrap the user callback to convert usize pointer to CameraRef
        let wrapped_callback = move |camera_ptr_usize: usize| {
            // SAFETY: The camera pointer comes from VTK and is valid for the observer lifetime
            unsafe {
                let camera_ptr = camera_ptr_usize as *mut _;
                let mut camera_ref = crate::CameraRef::from_raw_ptr(camera_ptr);
                callback(&mut camera_ref);
            }
        };

        // Register in the global registry
        let callback_id = {
            let mut registry_guard = get_or_create_registry().lock().unwrap();
            let registry = registry_guard.get_or_insert_with(CameraObserverRegistry::new);
            registry.register_modified(wrapped_callback)
        };

        // Create command and set the trampoline
        let mut command = crate::Command::new();
        unsafe {
            command.set_callback(camera_observer_modified_trampoline, callback_id as usize);
        }

        // Add observer and return both tag and command (command must be kept alive!)
        let tag = unsafe {
            use crate::vtk_command::Observable;
            self.add_observer(crate::vtk_command::events::MODIFIED_EVENT, &mut command)
        };

        (tag, command)
    }
}

/// Extension trait for owned Camera (less common, usually work with CameraRef from renderer)
impl CameraObserverExt for crate::Camera {
    fn on_modified<F>(&mut self, callback: F) -> (usize, crate::Command)
        where F: Fn(&mut crate::CameraRef) + Send + 'static
    {
        // Convert camera pointer to usize
        let camera_ptr_usize = self.as_mut_ptr() as usize;

        let wrapped_callback = move |_caller_ptr: usize| {
            // SAFETY: The camera pointer is captured and valid for the observer lifetime
            unsafe {
                let camera_ptr = camera_ptr_usize as *mut _;
                let mut camera_ref = crate::CameraRef::from_raw_ptr(camera_ptr);
                callback(&mut camera_ref);
            }
        };

        let callback_id = {
            let mut registry_guard = get_or_create_registry().lock().unwrap();
            let registry = registry_guard.get_or_insert_with(CameraObserverRegistry::new);
            registry.register_modified(wrapped_callback)
        };

        let mut command = crate::Command::new();
        unsafe {
            command.set_callback(camera_observer_modified_trampoline, callback_id as usize);
        }

        let tag = unsafe {
            use crate::vtk_command::Observable;
            self.add_observer(crate::vtk_command::events::MODIFIED_EVENT, &mut command)
        };

        (tag, command)
    }
}
