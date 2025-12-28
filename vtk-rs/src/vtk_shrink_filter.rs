use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_shrink_filter.h");
        include!("vtk_algorithm_output.h");

        type vtkShrinkFilter;
        type vtkAlgorithmOutput;

        fn vtk_shrink_filter_new() -> *mut vtkShrinkFilter;
        fn vtk_shrink_filter_delete(f: Pin<&mut vtkShrinkFilter>);

        unsafe fn shrink_set_input_connection(
            f: Pin<&mut vtkShrinkFilter>,
            output: *mut vtkAlgorithmOutput
        );

        fn shrink_set_shrink_factor(f: Pin<&mut vtkShrinkFilter>, factor: f64);
        fn shrink_get_shrink_factor(f: Pin<&mut vtkShrinkFilter>) -> f64;

        unsafe fn shrink_get_output_port(f: Pin<&mut vtkShrinkFilter>) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/release/9.3/html/classvtkShrinkFilter.html",
    @name ShrinkFilter, ffi::vtkShrinkFilter,
    @new ffi::vtk_shrink_filter_new,
    @delete ffi::vtk_shrink_filter_delete
);

impl ShrinkFilter {
    #[doc(alias = "SetInputConnection")]
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::shrink_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    #[doc(alias = "SetShrinkFactor")]
    pub fn set_shrink_factor(&mut self, f: f64) {
        ffi::shrink_set_shrink_factor(self.ptr.as_mut(), f);
    }

    #[doc(alias = "GetShrinkFactor")]
    pub fn get_shrink_factor(&mut self) -> f64 {
        ffi::shrink_get_shrink_factor(self.ptr.as_mut())
    }

    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::shrink_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
