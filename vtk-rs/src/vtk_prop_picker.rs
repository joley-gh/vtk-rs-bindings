use core::pin::Pin;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_prop_picker.h");
        include!("vtk_renderer.h");

        type vtkPropPicker;
        type vtkRenderer;
        type vtkProp;
        type vtkActor;

        fn prop_picker_new() -> *mut vtkPropPicker;
        fn prop_picker_delete(picker: Pin<&mut vtkPropPicker>);
        unsafe fn prop_picker_pick(
            picker: Pin<&mut vtkPropPicker>,
            x: f64,
            y: f64,
            z: f64,
            renderer: *mut vtkRenderer
        ) -> bool;
        fn prop_picker_get_pick_position(
            picker: Pin<&mut vtkPropPicker>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        unsafe fn prop_picker_get_view_prop(picker: Pin<&mut vtkPropPicker>) -> *mut vtkProp;
        unsafe fn prop_picker_get_actor(picker: Pin<&mut vtkPropPicker>) -> *mut vtkActor;
        unsafe fn prop_picker_add_pick_list(picker: Pin<&mut vtkPropPicker>, prop: *mut vtkProp);
        fn prop_picker_set_pick_from_list(picker: Pin<&mut vtkPropPicker>, enabled: bool);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPropPicker.html",
    @name PropPicker, ffi::vtkPropPicker,
    @new ffi::prop_picker_new,
    @delete ffi::prop_picker_delete
);

unsafe impl Send for PropPicker {}
unsafe impl Sync for PropPicker {}

impl PropPicker {
    /// Perform a pick on the given renderer at display coordinates (x, y).
    /// Returns true if something was picked.
    pub fn pick(&mut self, x: f64, y: f64, z: f64, renderer: &mut crate::Renderer) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::prop_picker_pick(self.ptr.as_mut(), x, y, z, renderer_ptr)
        }
    }

    /// Convenience method that takes a raw renderer pointer (for use in callbacks)
    ///
    /// # Safety
    /// The renderer pointer must be valid and remain valid for the duration of the pick.
    pub unsafe fn pick_with_ptr(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        renderer: *mut ffi::vtkRenderer
    ) -> bool {
        ffi::prop_picker_pick(self.ptr.as_mut(), x, y, z, renderer)
    }

    /// Get the picked position in world coordinates.
    pub fn get_pick_position(&mut self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::prop_picker_get_pick_position(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Get the picked prop (actor, volume, etc.)
    pub unsafe fn get_view_prop(&mut self) -> *mut ffi::vtkProp {
        unsafe { ffi::prop_picker_get_view_prop(self.ptr.as_mut()) }
    }

    /// Get the picked actor (if the picked prop is an actor)
    pub unsafe fn get_actor(&mut self) -> *mut ffi::vtkActor {
        unsafe { ffi::prop_picker_get_actor(self.ptr.as_mut()) }
    }

    /// Add a prop to the pick list (restricts picking to only these props)
    pub unsafe fn add_pick_list(&mut self, prop: *mut ffi::vtkProp) {
        ffi::prop_picker_add_pick_list(self.ptr.as_mut(), prop);
    }

    /// Enable/disable picking from the pick list only
    pub fn set_pick_from_list(&mut self, enabled: bool) {
        ffi::prop_picker_set_pick_from_list(self.ptr.as_mut(), enabled);
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
        let actor_ptr = unsafe { ffi::prop_picker_get_actor(self.ptr.as_mut()) };
        if actor_ptr.is_null() {
            None
        } else {
            let actor_ptr = actor_ptr as *mut crate::vtk_actor::ffi::vtkActor;
            crate::actor_ref::ActorRef::from_raw(actor_ptr)
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkPropPicker`](https://vtk.org/doc/nightly/html/classvtkPropPicker.html)
#[allow(non_camel_case_types)]
pub trait vtkPropPicker: private::Sealed {}
