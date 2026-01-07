use core::pin::Pin;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_area_picker.h");
        include!("vtk_renderer.h");
        include!("vtk_prop3d_collection.h");

        type vtkAreaPicker;
        type vtkRenderer;
        type vtkProp3DCollection;
        type vtkProp3D;
        type vtkActor;

        fn vtk_area_picker_new() -> *mut vtkAreaPicker;
        fn vtk_area_picker_delete(picker: Pin<&mut vtkAreaPicker>);

        unsafe fn vtk_area_picker_area_pick(
            picker: Pin<&mut vtkAreaPicker>,
            x0: f64,
            y0: f64,
            x1: f64,
            y1: f64,
            renderer: *mut vtkRenderer
        ) -> i32;

        fn vtk_area_picker_get_prop3ds(picker: Pin<&mut vtkAreaPicker>) -> *mut vtkProp3DCollection;
        unsafe fn vtk_prop3d_collection_get_size(col: *mut vtkProp3DCollection) -> i32;
        unsafe fn vtk_prop3d_collection_get_item(
            col: *mut vtkProp3DCollection,
            index: i32
        ) -> *mut vtkProp3D;
        unsafe fn vtk_prop3d_to_actor(prop: *mut vtkProp3D) -> *mut vtkActor;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAreaPicker.html",
    @name AreaPicker, ffi::vtkAreaPicker,
    @new ffi::vtk_area_picker_new,
    @delete ffi::vtk_area_picker_delete
);

unsafe impl Send for AreaPicker {}
unsafe impl Sync for AreaPicker {}

impl AreaPicker {
    /// Perform area pick operation within a rectangular region.
    ///
    /// # Arguments
    /// * `x0, y0` - Bottom-left corner of selection rectangle (display coordinates)
    /// * `x1, y1` - Top-right corner of selection rectangle (display coordinates)
    /// * `renderer` - The renderer to pick from
    ///
    /// Returns true if any props were picked, false otherwise.
    pub fn area_pick(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        renderer: &mut crate::Renderer
    ) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            let result = ffi::vtk_area_picker_area_pick(
                Pin::new_unchecked(&mut *self.as_mut_ptr()),
                x0,
                y0,
                x1,
                y1,
                renderer_ptr
            );
            result != 0
        }
    }

    /// Get the collection of Prop3D objects that were picked.
    /// Returns a safe non-owning `Prop3DCollectionRef` wrapper or `None` when
    /// no props were picked.
    pub fn get_prop3ds(&mut self) -> Option<crate::Prop3DCollectionRef> {
        let ptr = unsafe {
            ffi::vtk_area_picker_get_prop3ds(Pin::new_unchecked(&mut *self.as_mut_ptr()))
        };
        // Safety: the pointer is owned by VTK; we only create a non-owning view
        unsafe {
            crate::vtk_prop3d_collection::Prop3DCollectionRef::from_raw(ptr)
        }
    }

    /// Convenience: return a `Vec<ActorRef>` for any actors found in the
    /// picked prop collection.
    pub fn get_actor_refs(&mut self) -> Vec<crate::ActorRef> {
        if let Some(mut col) = self.get_prop3ds() { col.get_actor_refs() } else { Vec::new() }
    }
}
