use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_threshold.h");
        include!("vtk_algorithm_output.h");

        type vtkThreshold;
        type vtkAlgorithmOutput;
        type vtkDataSet;

        fn vtk_threshold_new() -> *mut vtkThreshold;
        fn vtk_threshold_delete(thr: Pin<&mut vtkThreshold>);

        unsafe fn threshold_set_input_connection(
            thr: Pin<&mut vtkThreshold>,
            output: *mut vtkAlgorithmOutput
        );
        unsafe fn threshold_set_input_data(thr: Pin<&mut vtkThreshold>, data_set: *mut vtkDataSet);

        fn threshold_set_lower_threshold(thr: Pin<&mut vtkThreshold>, value: f64);
        fn threshold_set_upper_threshold(thr: Pin<&mut vtkThreshold>, value: f64);
        fn threshold_set_threshold_between(thr: Pin<&mut vtkThreshold>, lower: f64, upper: f64);

        fn threshold_set_component_mode(thr: Pin<&mut vtkThreshold>, mode: i32);
        fn threshold_set_selected_component(thr: Pin<&mut vtkThreshold>, comp: i32);

        unsafe fn threshold_get_output_port(thr: Pin<&mut vtkThreshold>) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/release/9.3/html/classvtkThreshold.html",
    @name Threshold, ffi::vtkThreshold,
    @new ffi::vtk_threshold_new,
    @delete ffi::vtk_threshold_delete
);

impl Threshold {
    #[doc(alias = "SetInputConnection")]
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::threshold_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    #[doc(alias = "SetInputData")]
    pub fn set_input_data(&mut self, data: &mut crate::ImageData) {
        unsafe {
            let ptr = data.as_raw_ptr() as *mut ffi::vtkDataSet;
            ffi::threshold_set_input_data(self.ptr.as_mut(), ptr);
        }
    }

    #[doc(alias = "SetLowerThreshold")]
    pub fn set_lower_threshold(&mut self, v: f64) {
        ffi::threshold_set_lower_threshold(self.ptr.as_mut(), v);
    }

    #[doc(alias = "SetUpperThreshold")]
    pub fn set_upper_threshold(&mut self, v: f64) {
        ffi::threshold_set_upper_threshold(self.ptr.as_mut(), v);
    }

    #[doc(alias = "ThresholdBetween")]
    pub fn threshold_between(&mut self, lower: f64, upper: f64) {
        ffi::threshold_set_threshold_between(self.ptr.as_mut(), lower, upper);
    }

    #[doc(alias = "SetComponentMode")]
    pub fn set_component_mode(&mut self, mode: i32) {
        ffi::threshold_set_component_mode(self.ptr.as_mut(), mode);
    }

    #[doc(alias = "SetSelectedComponent")]
    pub fn set_selected_component(&mut self, comp: i32) {
        ffi::threshold_set_selected_component(self.ptr.as_mut(), comp);
    }

    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::threshold_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
