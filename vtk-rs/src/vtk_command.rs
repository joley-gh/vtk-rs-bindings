use core::pin::Pin;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_command.h");

        pub(crate) type RustCommand;
        pub(crate) type vtkObject;

        fn vtk_command_new() -> *mut RustCommand;
        unsafe fn vtk_command_delete(command: *mut RustCommand);
        fn vtk_command_set_callback(
            command: Pin<&mut RustCommand>,
            callback_fn: usize,
            user_data: usize
        );

        pub(crate) unsafe fn vtk_object_add_observer(
            obj: Pin<&mut vtkObject>,
            event: usize,
            command: Pin<&mut RustCommand>
        ) -> usize;
        pub(crate) unsafe fn vtk_object_remove_observer(obj: Pin<&mut vtkObject>, tag: usize);
    }
}

pub struct Command {
    ptr: *mut ffi::RustCommand,
}

impl Command {
    pub fn new() -> Self {
        let raw_ptr = ffi::vtk_command_new();
        if raw_ptr.is_null() {
            panic!("VTK command creation failed: vtk_command_new returned null pointer");
        }
        Self { ptr: raw_ptr }
    }

    /// Get a pinned mutable reference for FFI calls
    pub(crate) fn as_mut(&mut self) -> Pin<&mut ffi::RustCommand> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Get raw pointer for FFI calls (internal use)
    pub(crate) fn as_raw_ptr(&self) -> *mut ffi::RustCommand {
        self.ptr
    }

    /// Set a Rust callback function for this command.
    /// The callback will be invoked when the observed event occurs.
    ///
    /// # Safety
    /// The callback function must be a valid extern "C" function with signature:
    /// `extern "C" fn(caller: usize, event_id: usize, user_data: usize)`
    pub unsafe fn set_callback(
        &mut self,
        callback: extern "C" fn(usize, usize, usize),
        user_data: usize
    ) {
        ffi::vtk_command_set_callback(self.as_mut(), callback as usize, user_data);
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            ffi::vtk_command_delete(self.ptr);
        }
    }
}

// VTK Event constants (using usize to match VTK's unsigned long)
pub mod events {
    pub const NO_EVENT: usize = 0;
    pub const ANY_EVENT: usize = 1;
    pub const DELETE_EVENT: usize = 2;
    pub const START_EVENT: usize = 3;
    pub const END_EVENT: usize = 4;
    pub const RENDER_EVENT: usize = 5;
    pub const PROGRESS_EVENT: usize = 6;
    pub const PICK_EVENT: usize = 7;
    pub const START_PICK_EVENT: usize = 8;
    pub const END_PICK_EVENT: usize = 9;
    pub const ABORT_CHECK_EVENT: usize = 10;
    pub const EXIT_EVENT: usize = 11;
    pub const LEFT_BUTTON_PRESS_EVENT: usize = 12;
    pub const LEFT_BUTTON_RELEASE_EVENT: usize = 13;
    pub const MIDDLE_BUTTON_PRESS_EVENT: usize = 14;
    pub const MIDDLE_BUTTON_RELEASE_EVENT: usize = 15;
    pub const RIGHT_BUTTON_PRESS_EVENT: usize = 16;
    pub const RIGHT_BUTTON_RELEASE_EVENT: usize = 17;
    pub const ENTER_EVENT: usize = 18;
    pub const LEAVE_EVENT: usize = 19;
    pub const KEY_PRESS_EVENT: usize = 20;
    pub const KEY_RELEASE_EVENT: usize = 21;
    pub const CHAR_EVENT: usize = 22;
    pub const EXPOSE_EVENT: usize = 23;
    pub const CONFIGURE_EVENT: usize = 24;
    pub const TIMER_EVENT: usize = 25;
    pub const MOUSE_MOVE_EVENT: usize = 26;
    pub const MOUSE_WHEEL_FORWARD_EVENT: usize = 27;
    pub const MOUSE_WHEEL_BACKWARD_EVENT: usize = 28;
    pub const RESET_CAMERA_EVENT: usize = 29;
    pub const RESET_CAMERA_CLIPPING_RANGE_EVENT: usize = 30;
    pub const MODIFIED_EVENT: usize = 33;
    pub const WINDOW_IS_CURRENT_EVENT: usize = 34;
    pub const WINDOW_IS_DIRECT_EVENT: usize = 35;
    pub const WINDOW_SUPPORTSOPENG_EVENT: usize = 36;
    pub const WINDOW_FRAME_EVENT: usize = 37;
    pub const START_ANIMATION_CUE_EVENT: usize = 38;
    pub const ANIMATION_CUE_TICK_EVENT: usize = 39;
    pub const END_ANIMATION_CUE_EVENT: usize = 40;
    pub const VOLUME_MAPPER_RENDER_END_EVENT: usize = 41;
    pub const VOLUME_MAPPER_RENDER_PROGRESS_EVENT: usize = 42;
    pub const VOLUME_MAPPER_RENDER_START_EVENT: usize = 43;
    pub const VOLUME_MAPPER_COMPUTE_GRADIENTS_END_EVENT: usize = 44;
    pub const VOLUME_MAPPER_COMPUTE_GRADIENTS_PROGRESS_EVENT: usize = 45;
    pub const VOLUME_MAPPER_COMPUTE_GRADIENTS_START_EVENT: usize = 46;
    pub const START_INTERACTION_EVENT: usize = 47;
    pub const INTERACTION_EVENT: usize = 48;
    pub const END_INTERACTION_EVENT: usize = 49;
}

/// Helper trait for objects that support observers
pub trait Observable {
    /// Add an observer for a specific event
    ///
    /// # Safety
    /// The returned tag must be used to remove the observer later
    unsafe fn add_observer(&mut self, event: usize, command: &mut Command) -> usize;

    /// Remove an observer by tag
    unsafe fn remove_observer(&mut self, tag: usize);
}
