// Safe Rust bindings for InteractorStyleTrackballActor C glue

use std::sync::Mutex;

// Ensure the vtkrs static library is linked
#[link(name = "vtkrs", kind = "static")]
extern "C" {}

// Opaque C type for FFI
#[repr(C)]
pub struct InteractorStyleTrackballActorOpaque {
    _private: [u8; 0],
}

extern "C" {
    fn interactor_style_trackball_actor_new() -> *mut InteractorStyleTrackballActorOpaque;
    fn interactor_style_trackball_actor_delete(style: *mut InteractorStyleTrackballActorOpaque);

    fn interactor_style_trackball_actor_set_left_button_press_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_left_button_release_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_right_button_press_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_right_button_release_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_middle_button_press_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_middle_button_release_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );
    fn interactor_style_trackball_actor_set_mouse_move_callback_id(
        style: *mut InteractorStyleTrackballActorOpaque,
        callback_id: i64
    );

    fn interactor_style_trackball_actor_isA(
        style: *mut InteractorStyleTrackballActorOpaque,
        name: *const std::os::raw::c_char
    ) -> i32;
    fn interactor_style_trackball_actor_new_instance(
        style: *mut InteractorStyleTrackballActorOpaque
    ) -> *mut InteractorStyleTrackballActorOpaque;
    fn interactor_style_trackball_actor_print_self(
        style: *mut InteractorStyleTrackballActorOpaque,
        indent: i32
    );

    fn interactor_style_trackball_actor_rotate(style: *mut InteractorStyleTrackballActorOpaque);
    fn interactor_style_trackball_actor_spin(style: *mut InteractorStyleTrackballActorOpaque);
    fn interactor_style_trackball_actor_pan(style: *mut InteractorStyleTrackballActorOpaque);
    fn interactor_style_trackball_actor_dolly(style: *mut InteractorStyleTrackballActorOpaque);
    fn interactor_style_trackball_actor_uniform_scale(
        style: *mut InteractorStyleTrackballActorOpaque
    );
}

// Use the central callback registry helpers
use crate::vtk_interactor_style_custom as custom;

/// Safe wrapper around the C interactor style
pub struct InteractorStyleTrackballActor {
    ptr: *mut InteractorStyleTrackballActorOpaque,
}

impl InteractorStyleTrackballActor {
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { interactor_style_trackball_actor_new() };
        if ptr.is_null() {
            panic!("Failed to create InteractorStyleTrackballActor");
        }
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut InteractorStyleTrackballActorOpaque {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut InteractorStyleTrackballActorOpaque {
        self.ptr
    }

    pub fn set_left_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_left_press_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_left_button_press_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_left_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_left_release_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_left_button_release_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_right_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_right_press_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_right_button_press_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_right_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_right_release_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_right_button_release_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_middle_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_middle_press_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_middle_button_press_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_middle_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        let callback_id = custom::register_middle_release_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_middle_button_release_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    /// Set middle-button-release callback that receives an optional non-owning `ActorRef`.
    /// The `ActorRef` will be `None` when the click did not hit an actor.
    pub fn set_middle_button_release_callback_with_actor<F>(&mut self, callback: F)
        where F: Fn(i32, i32, Option<crate::ActorRef>) + Send + 'static
    {
        // Register a raw callback that converts the raw vtkActor* into our ActorRef
        let raw_callback = move |x: i32, y: i32, actor_ptr: *mut crate::vtk_actor::ffi::vtkActor| {
            let actor_ref = crate::actor_ref::ActorRef::from_raw(actor_ptr);
            callback(x, y, actor_ref);
        };

        let callback_id = custom::register_middle_release_callback_with_actor(raw_callback);
        unsafe {
            interactor_style_trackball_actor_set_middle_button_release_callback_id(
                self.ptr,
                callback_id
            )
        }
    }

    pub fn set_mouse_move_callback<F>(&mut self, callback: F) where F: Fn(i32, i32) + Send + 'static {
        let callback_id = custom::register_mouse_move_callback(callback);
        unsafe {
            interactor_style_trackball_actor_set_mouse_move_callback_id(self.ptr, callback_id)
        }
    }

    pub fn is_a(&self, name: &str) -> bool {
        let c_name = std::ffi::CString::new(name).unwrap_or_default();
        unsafe { interactor_style_trackball_actor_isA(self.ptr, c_name.as_ptr()) != 0 }
    }

    pub fn new_instance(&self) -> Option<InteractorStyleTrackballActor> {
        let ptr = unsafe { interactor_style_trackball_actor_new_instance(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(InteractorStyleTrackballActor { ptr })
        }
    }

    pub fn print_self(&self, indent: i32) {
        unsafe { interactor_style_trackball_actor_print_self(self.ptr, indent) }
    }

    pub fn rotate(&mut self) {
        unsafe { interactor_style_trackball_actor_rotate(self.ptr) }
    }
    pub fn spin(&mut self) {
        unsafe { interactor_style_trackball_actor_spin(self.ptr) }
    }
    pub fn pan(&mut self) {
        unsafe { interactor_style_trackball_actor_pan(self.ptr) }
    }
    pub fn dolly(&mut self) {
        unsafe { interactor_style_trackball_actor_dolly(self.ptr) }
    }
    pub fn uniform_scale(&mut self) {
        unsafe { interactor_style_trackball_actor_uniform_scale(self.ptr) }
    }
}

impl Drop for InteractorStyleTrackballActor {
    fn drop(&mut self) {
        unsafe { interactor_style_trackball_actor_delete(self.ptr) }
    }
}
