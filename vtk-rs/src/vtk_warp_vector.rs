use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_warp_vector.h");
        include!("vtk_algorithm_output.h");

        type vtkWarpVector;
        type vtkAlgorithmOutput;
        type vtkDataSet;

        fn vtk_warp_vector_new() -> *mut vtkWarpVector;
        fn vtk_warp_vector_delete(warp: Pin<&mut vtkWarpVector>);

        unsafe fn warp_vector_set_input_connection(
            warp: Pin<&mut vtkWarpVector>,
            port: *mut vtkAlgorithmOutput
        );
        unsafe fn warp_vector_set_input_data(
            warp: Pin<&mut vtkWarpVector>,
            dataset: *mut vtkDataSet
        );

        fn warp_vector_set_scale_factor(warp: Pin<&mut vtkWarpVector>, scale: f64);
        fn warp_vector_get_scale_factor(warp: Pin<&mut vtkWarpVector>) -> f64;

        unsafe fn warp_vector_get_output_port(
            warp: Pin<&mut vtkWarpVector>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkWarpVector.html",
    @name WarpVector, ffi::vtkWarpVector,
    @new ffi::vtk_warp_vector_new,
    @delete ffi::vtk_warp_vector_delete
);

impl WarpVector {
    /// Set input from algorithm output port
    pub fn set_input_connection(&mut self, port: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = port.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::warp_vector_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    /// Set input data directly
    pub fn set_input_data(&mut self, dataset: &mut crate::UnstructuredGrid) {
        unsafe {
            let ptr = dataset.as_raw_ptr() as *mut ffi::vtkDataSet;
            ffi::warp_vector_set_input_data(self.ptr.as_mut(), ptr);
        }
    }

    /// Set scale factor for displacement magnitude
    /// scale > 1.0: amplify deformation
    /// scale < 1.0: reduce deformation
    /// scale = 1.0: actual displacement
    pub fn set_scale_factor(&mut self, scale: f64) {
        unsafe {
            ffi::warp_vector_set_scale_factor(self.ptr.as_mut(), scale);
        }
    }

    /// Get current scale factor
    pub fn get_scale_factor(&mut self) -> f64 {
        unsafe { ffi::warp_vector_get_scale_factor(self.ptr.as_mut()) }
    }

    /// Get output port for pipeline connection
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::warp_vector_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
