use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_contour_filter.h");
        include!("vtk_algorithm_output.h");

        type vtkContourFilter;
        type vtkAlgorithmOutput;
        type vtkDataSet;

        fn vtk_contour_filter_new() -> *mut vtkContourFilter;
        fn vtk_contour_filter_delete(filter: Pin<&mut vtkContourFilter>);

        unsafe fn contour_filter_set_input_connection(
            filter: Pin<&mut vtkContourFilter>,
            output: *mut vtkAlgorithmOutput
        );
        unsafe fn contour_filter_set_input_data(
            filter: Pin<&mut vtkContourFilter>,
            data_set: *mut vtkDataSet
        );
        fn contour_filter_set_value(filter: Pin<&mut vtkContourFilter>, i: i32, value: f64);
        fn contour_filter_generate_values(
            filter: Pin<&mut vtkContourFilter>,
            num_contours: i32,
            range_min: f64,
            range_max: f64
        );
        unsafe fn contour_filter_get_output_port(
            filter: Pin<&mut vtkContourFilter>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkContourFilter.html",
    @name ContourFilter, ffi::vtkContourFilter,
    @new ffi::vtk_contour_filter_new,
    @delete ffi::vtk_contour_filter_delete
);

impl ContourFilter {
    /// Sets the input connection from any VTK algorithm output port
    #[doc(alias = "SetInputConnection")]
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::contour_filter_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }
    
    /// Sets the input data directly (for data objects like ImageData)
    #[doc(alias = "SetInputData")]
    pub fn set_input_data(&mut self, data: &mut crate::ImageData) {
        unsafe {
            let ptr = data.as_raw_ptr() as *mut ffi::vtkDataSet;
            ffi::contour_filter_set_input_data(self.ptr.as_mut(), ptr);
        }
    }

    /// Set a particular contour value at index i
    #[doc(alias = "SetValue")]
    pub fn set_value(&mut self, i: i32, value: f64) {
        ffi::contour_filter_set_value(self.ptr.as_mut(), i, value);
    }

    /// Generate n equally spaced contours between range_min and range_max
    #[doc(alias = "GenerateValues")]
    pub fn generate_values(&mut self, num_contours: i32, range_min: f64, range_max: f64) {
        ffi::contour_filter_generate_values(self.ptr.as_mut(), num_contours, range_min, range_max);
    }

    /// Get the output port for connecting to mappers
    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::contour_filter_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
