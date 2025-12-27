use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_smooth_poly_data_filter.h");
        include!("vtk_algorithm_output.h");

        type vtkSmoothPolyDataFilter;
        type vtkAlgorithmOutput;

        fn vtk_smooth_poly_data_filter_new() -> *mut vtkSmoothPolyDataFilter;
        fn vtk_smooth_poly_data_filter_delete(filter: Pin<&mut vtkSmoothPolyDataFilter>);

        unsafe fn smooth_poly_data_filter_set_input_connection(
            filter: Pin<&mut vtkSmoothPolyDataFilter>,
            output: *mut vtkAlgorithmOutput
        );

        fn smooth_poly_data_filter_set_number_of_iterations(
            filter: Pin<&mut vtkSmoothPolyDataFilter>,
            iterations: i32
        );
        fn smooth_poly_data_filter_get_number_of_iterations(
            filter: Pin<&mut vtkSmoothPolyDataFilter>
        ) -> i32;

        fn smooth_poly_data_filter_set_relaxation_factor(
            filter: Pin<&mut vtkSmoothPolyDataFilter>,
            factor: f64
        );
        fn smooth_poly_data_filter_get_relaxation_factor(
            filter: Pin<&mut vtkSmoothPolyDataFilter>
        ) -> f64;

        fn smooth_poly_data_filter_set_feature_edge_smoothing(
            filter: Pin<&mut vtkSmoothPolyDataFilter>,
            on: bool
        );
        fn smooth_poly_data_filter_set_boundary_smoothing(
            filter: Pin<&mut vtkSmoothPolyDataFilter>,
            on: bool
        );

        unsafe fn smooth_poly_data_filter_get_output_port(
            filter: Pin<&mut vtkSmoothPolyDataFilter>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkSmoothPolyDataFilter.html",
    @name SmoothPolyDataFilter, ffi::vtkSmoothPolyDataFilter,
    @new ffi::vtk_smooth_poly_data_filter_new,
    @delete ffi::vtk_smooth_poly_data_filter_delete
);

impl SmoothPolyDataFilter {
    #[doc(alias = "SetInputConnection")]
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::smooth_poly_data_filter_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    #[doc(alias = "SetNumberOfIterations")]
    pub fn set_number_of_iterations(&mut self, n: i32) {
        ffi::smooth_poly_data_filter_set_number_of_iterations(self.ptr.as_mut(), n);
    }

    #[doc(alias = "GetNumberOfIterations")]
    pub fn get_number_of_iterations(&mut self) -> i32 {
        ffi::smooth_poly_data_filter_get_number_of_iterations(self.ptr.as_mut())
    }

    #[doc(alias = "SetRelaxationFactor")]
    pub fn set_relaxation_factor(&mut self, f: f64) {
        ffi::smooth_poly_data_filter_set_relaxation_factor(self.ptr.as_mut(), f);
    }

    #[doc(alias = "GetRelaxationFactor")]
    pub fn get_relaxation_factor(&mut self) -> f64 {
        ffi::smooth_poly_data_filter_get_relaxation_factor(self.ptr.as_mut())
    }

    #[doc(alias = "SetFeatureEdgeSmoothing")]
    pub fn set_feature_edge_smoothing(&mut self, on: bool) {
        ffi::smooth_poly_data_filter_set_feature_edge_smoothing(self.ptr.as_mut(), on);
    }

    #[doc(alias = "SetBoundarySmoothing")]
    pub fn set_boundary_smoothing(&mut self, on: bool) {
        ffi::smooth_poly_data_filter_set_boundary_smoothing(self.ptr.as_mut(), on);
    }

    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::smooth_poly_data_filter_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
