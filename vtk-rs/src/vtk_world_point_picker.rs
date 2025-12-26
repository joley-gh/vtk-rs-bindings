use core::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_world_point_picker.h");
        include!("vtk_renderer.h");

        type vtkWorldPointPicker;
        type vtkRenderer;

        fn vtk_world_point_picker_new() -> *mut vtkWorldPointPicker;
        fn vtk_world_point_picker_delete(picker: Pin<&mut vtkWorldPointPicker>);

        unsafe fn vtk_world_point_picker_pick(
            picker: Pin<&mut vtkWorldPointPicker>,
            x: f64,
            y: f64,
            z: f64,
            renderer: *mut vtkRenderer
        ) -> i32;

        fn vtk_world_point_picker_get_pick_position(
            picker: &vtkWorldPointPicker,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkWorldPointPicker.html",
    @name WorldPointPicker, ffi::vtkWorldPointPicker,
    @new ffi::vtk_world_point_picker_new,
    @delete ffi::vtk_world_point_picker_delete
);

unsafe impl Send for WorldPointPicker {}
unsafe impl Sync for WorldPointPicker {}

impl WorldPointPicker {
    /// Perform a pick operation at the specified display coordinates.
    /// Converts 2D screen coordinates (x, y) to 3D world coordinates.
    /// The z parameter is typically 0 for screen-space picking.
    ///
    /// Note: Unlike CellPicker, WorldPointPicker always succeeds and sets the
    /// pick position. The return value from VTK is always 0, so this function
    /// always returns true to indicate the position was computed.
    pub fn pick(&mut self, x: f64, y: f64, z: f64, renderer: &mut crate::Renderer) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::vtk_world_point_picker_pick(self.ptr.as_mut(), x, y, z, renderer_ptr);
            true // WorldPointPicker always succeeds
        }
    }

    /// Get the 3D world coordinates of the picked point.
    /// Returns (x, y, z) coordinates in world space.
    pub fn get_pick_position(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_world_point_picker_get_pick_position(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }
}
