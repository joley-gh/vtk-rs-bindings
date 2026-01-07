use core::pin::Pin;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_cell_picker.h");
        include!("vtk_renderer.h");

        type vtkCellPicker;
        type vtkRenderer;
        type vtkActor;
        type vtkDataSet;
        type vtkProp3D;

        fn cell_picker_new() -> *mut vtkCellPicker;
        fn cell_picker_delete(picker: Pin<&mut vtkCellPicker>);
        unsafe fn cell_picker_pick(
            picker: Pin<&mut vtkCellPicker>,
            x: f64,
            y: f64,
            z: f64,
            renderer: *mut vtkRenderer
        ) -> bool;
        fn cell_picker_get_cell_id(picker: Pin<&mut vtkCellPicker>) -> i32;
        fn cell_picker_get_dataset(picker: Pin<&mut vtkCellPicker>) -> *mut vtkDataSet;
        fn cell_picker_get_pick_position(
            picker: Pin<&mut vtkCellPicker>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn cell_picker_get_actor(picker: Pin<&mut vtkCellPicker>) -> *mut vtkActor;
        unsafe fn cell_picker_add_pick_list(picker: Pin<&mut vtkCellPicker>, prop: *mut vtkProp3D);
        unsafe fn cell_picker_set_pick_from_list(picker: Pin<&mut vtkCellPicker>, enabled: bool);
        unsafe fn cell_picker_set_tolerance(picker: Pin<&mut vtkCellPicker>, tolerance: f64);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCellPicker.html",
    @name CellPicker, ffi::vtkCellPicker,
    @new ffi::cell_picker_new,
    @delete ffi::cell_picker_delete
);

// VTK objects are internally reference counted and thread-safe
// We need to mark CellPicker as Send to use in callbacks
unsafe impl Send for CellPicker {}
unsafe impl Sync for CellPicker {}

impl CellPicker {
    /// Perform a pick on the given renderer at display coordinates (x, y).
    /// The z coordinate is typically 0.0.
    /// Returns true if something was picked, false otherwise.
    pub fn pick(&mut self, x: f64, y: f64, z: f64, renderer: &mut crate::Renderer) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::cell_picker_pick(self.ptr.as_mut(), x, y, z, renderer_ptr)
        }
    }

    /// Perform a pick using a raw renderer pointer.
    ///
    /// # Safety
    /// The renderer pointer must be valid and remain valid for the duration of the pick.
    pub unsafe fn pick_with_ptr(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        renderer_ptr: *mut ffi::vtkRenderer
    ) -> bool {
        ffi::cell_picker_pick(self.ptr.as_mut(), x, y, z, renderer_ptr)
    }

    /// Get the ID of the picked cell. Returns -1 if no cell was picked.
    pub fn get_cell_id(&mut self) -> i32 {
        ffi::cell_picker_get_cell_id(self.ptr.as_mut())
    }

    /// Get the picked position in world coordinates.
    pub fn get_pick_position(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::cell_picker_get_pick_position(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Get the actor that was picked. Returns null pointer if no actor was picked.
    /// Use is_null() to check if a valid actor was returned.
    pub fn get_actor(&mut self) -> *mut ffi::vtkActor {
        // call through an unsafe block in case the underlying FFI is considered unsafe
        unsafe {
            ffi::cell_picker_get_actor(self.ptr.as_mut())
        }
    }

    /// Return an `Option<ActorRef>` for the actor picked by the most recent
    /// `pick()`/`pick_with_ptr()` call. This keeps `pick()`'s signature stable
    /// (it still returns `bool`) while offering a safe way to obtain a typed
    /// non-owning reference to the picked actor.
    ///
    /// # Notes
    /// - This does not perform a pick itself; callers must call `pick()` or
    ///   `pick_with_ptr()` first.
    /// - `ActorRef` is non-owning and must not be stored beyond the callback
    ///   lifetime or used from other threads.
    pub fn get_actor_ref(&mut self) -> Option<crate::ActorRef> {
        let actor_ptr = unsafe { ffi::cell_picker_get_actor(self.ptr.as_mut()) };
        if actor_ptr.is_null() {
            None
        } else {
            let actor_ptr = actor_ptr as *mut crate::vtk_actor::ffi::vtkActor;
            crate::actor_ref::ActorRef::from_raw(actor_ptr)
        }
    }

    /// Add a prop3D (like an actor) to the pick list.
    ///
    /// # Safety
    /// The prop pointer must be valid and remain valid during picking.
    pub unsafe fn add_pick_list(&mut self, prop: *mut ffi::vtkProp3D) {
        ffi::cell_picker_add_pick_list(self.ptr.as_mut(), prop);
    }

    /// Set whether to pick only from the pick list.
    pub fn set_pick_from_list(&mut self, enabled: bool) {
        unsafe {
            ffi::cell_picker_set_pick_from_list(self.ptr.as_mut(), enabled);
        }
    }

    /// Set the tolerance for picking (how close the pick needs to be).
    /// Default is 0.025.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        unsafe {
            ffi::cell_picker_set_tolerance(self.ptr.as_mut(), tolerance);
        }
    }

    /// Check if a valid pick occurred.
    pub fn has_valid_pick(&mut self) -> bool {
        self.get_cell_id() >= 0
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkCellPicker`](https://vtk.org/doc/nightly/html/classvtkCellPicker.html)
#[allow(non_camel_case_types)]
pub trait vtkCellPicker: private::Sealed {}
