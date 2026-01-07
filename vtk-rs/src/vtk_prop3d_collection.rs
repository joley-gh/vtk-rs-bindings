/// Non-owning view over a `vtkProp3DCollection` returned by `AreaPicker`.
pub struct Prop3DCollectionRef {
    ptr: *mut crate::vtk_area_picker::ffi::vtkProp3DCollection,
}

impl Prop3DCollectionRef {
    /// Wrap a raw pointer; returns `None` for null pointers.
    ///
    /// # Safety
    /// The pointer must be a valid `vtkProp3DCollection` owned/managed by VTK.
    /// The returned reference must not outlive the collection's owner (typically
    /// VTK objects) and must not be used from other threads.
    pub unsafe fn from_raw(
        ptr: *mut crate::vtk_area_picker::ffi::vtkProp3DCollection
    ) -> Option<Self> {
        if ptr.is_null() { None } else { Some(Self { ptr }) }
    }

    /// Number of items in the collection.
    pub fn len(&self) -> usize {
        unsafe { crate::vtk_area_picker::ffi::vtk_prop3d_collection_get_size(self.ptr) as usize }
    }

    /// Return actor refs for items in the collection that are actors.
    ///
    /// This consumes a mutable reference to self because we call into C++ helpers
    /// that take a raw pointer; we keep the API ergonomic and safe by returning
    /// `ActorRef` instances which are non-owning typed wrappers.
    pub fn get_actor_refs(&mut self) -> Vec<crate::ActorRef> {
        let mut out = Vec::new();
        let n = self.len();
        for i in 0..n as i32 {
            let prop = unsafe {
                crate::vtk_area_picker::ffi::vtk_prop3d_collection_get_item(self.ptr, i)
            };
            if prop.is_null() {
                continue;
            }
            let actor_ptr = unsafe { crate::vtk_area_picker::ffi::vtk_prop3d_to_actor(prop) };
            if actor_ptr.is_null() {
                continue;
            }
            // Convert to the crate's vtkActor FFI type and wrap
            let actor_ptr = actor_ptr as *mut crate::vtk_actor::ffi::vtkActor;
            if let Some(actor_ref) = crate::actor_ref::ActorRef::from_raw(actor_ptr) {
                out.push(actor_ref);
            }
        }
        out
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}
