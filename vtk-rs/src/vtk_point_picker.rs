use core::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_point_picker.h");
        include!("vtk_renderer.h");

        type vtkPointPicker;
        type vtkRenderer;

        fn vtk_point_picker_new() -> *mut vtkPointPicker;
        fn vtk_point_picker_delete(picker: Pin<&mut vtkPointPicker>);

        unsafe fn vtk_point_picker_pick(
            picker: Pin<&mut vtkPointPicker>,
            x: f64,
            y: f64,
            z: f64,
            renderer: *mut vtkRenderer
        ) -> i32;

        fn vtk_point_picker_get_point_id(picker: Pin<&mut vtkPointPicker>) -> i32;

        fn vtk_point_picker_get_pick_position(
            picker: Pin<&mut vtkPointPicker>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPointPicker.html",
    @name PointPicker, ffi::vtkPointPicker,
    @new ffi::vtk_point_picker_new,
    @delete ffi::vtk_point_picker_delete
);

unsafe impl Send for PointPicker {}
unsafe impl Sync for PointPicker {}

impl PointPicker {
    /// Perform a pick operation at the specified display coordinates.
    /// Returns true if a point was successfully picked, false otherwise.
    ///
    /// Unlike WorldPointPicker, PointPicker only succeeds when it hits actual
    /// geometry and can identify a specific point in the mesh.
    pub fn pick(&mut self, x: f64, y: f64, z: f64, renderer: &mut crate::Renderer) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::vtk_point_picker_pick(self.ptr.as_mut(), x, y, z, renderer_ptr) != 0
        }
    }

    /// Get the ID of the picked point in the dataset.
    /// Returns -1 if no point was picked.
    /// Valid after a successful pick() call.
    pub fn get_point_id(&mut self) -> i32 {
        ffi::vtk_point_picker_get_point_id(self.ptr.as_mut())
    }

    /// Get the 3D world coordinates of the picked point.
    /// Returns (x, y, z) coordinates in world space.
    /// Valid after a successful pick() call.
    pub fn get_pick_position(&mut self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_point_picker_get_pick_position(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }
}
