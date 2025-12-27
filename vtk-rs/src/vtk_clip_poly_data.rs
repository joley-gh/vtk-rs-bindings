use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_clip_poly_data.h");
        include!("vtk_algorithm_output.h");
        include!("vtk_plane.h");

        type vtkClipPolyData;
        type vtkAlgorithmOutput;
        type vtkPlane;

        fn vtk_clip_poly_data_new() -> *mut vtkClipPolyData;
        fn vtk_clip_poly_data_delete(clipper: Pin<&mut vtkClipPolyData>);

        unsafe fn clip_poly_data_set_input_connection(
            clipper: Pin<&mut vtkClipPolyData>,
            output: *mut vtkAlgorithmOutput
        );
        unsafe fn clip_poly_data_set_clip_function(
            clipper: Pin<&mut vtkClipPolyData>,
            plane: *mut vtkPlane
        );
        fn clip_poly_data_set_value(clipper: Pin<&mut vtkClipPolyData>, value: f64);
        unsafe fn clip_poly_data_get_output_port(
            clipper: Pin<&mut vtkClipPolyData>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkClipPolyData.html",
    @name ClipPolyData, ffi::vtkClipPolyData,
    @new ffi::vtk_clip_poly_data_new,
    @delete ffi::vtk_clip_poly_data_delete
);

impl ClipPolyData {
    /// Sets the input connection from any VTK algorithm output port
    #[doc(alias = "SetInputConnection")]
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::clip_poly_data_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    /// Set the implicit function to use for clipping (e.g., a Plane)
    #[doc(alias = "SetClipFunction")]
    pub fn set_clip_function(&mut self, plane: &mut crate::Plane) {
        unsafe {
            let plane_ptr = plane.as_raw_ptr() as *mut ffi::vtkPlane;
            ffi::clip_poly_data_set_clip_function(self.ptr.as_mut(), plane_ptr);
        }
    }

    /// Set the clipping value (distance from implicit function)
    #[doc(alias = "SetValue")]
    pub fn set_value(&mut self, value: f64) {
        ffi::clip_poly_data_set_value(self.ptr.as_mut(), value);
    }

    /// Get the output port for connecting to mappers
    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::clip_poly_data_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
