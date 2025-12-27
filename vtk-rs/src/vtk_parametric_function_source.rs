#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_function_source.h");
        include!("vtk_algorithm_output.h");
        include!("vtk_parametric_function.h");

        type vtkParametricFunctionSource;
        type vtkAlgorithmOutput;
        type vtkParametricFunction;

        fn vtk_parametric_function_source_new() -> *mut vtkParametricFunctionSource;
        fn vtk_parametric_function_source_delete(ptr: Pin<&mut vtkParametricFunctionSource>);
        unsafe fn parametric_function_source_set_parametric_function(
            source: Pin<&mut vtkParametricFunctionSource>,
            func: *mut vtkParametricFunction
        );
        fn parametric_function_source_set_u_resolution(
            source: Pin<&mut vtkParametricFunctionSource>,
            resolution: i32
        );
        fn parametric_function_source_get_u_resolution(source: &vtkParametricFunctionSource) -> i32;
        fn parametric_function_source_set_v_resolution(
            source: Pin<&mut vtkParametricFunctionSource>,
            resolution: i32
        );
        fn parametric_function_source_get_v_resolution(source: &vtkParametricFunctionSource) -> i32;
        fn parametric_function_source_set_w_resolution(
            source: Pin<&mut vtkParametricFunctionSource>,
            resolution: i32
        );
        fn parametric_function_source_get_w_resolution(source: &vtkParametricFunctionSource) -> i32;
        unsafe fn parametric_function_source_get_output_port(
            source: Pin<&mut vtkParametricFunctionSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkParametricFunctionSource.html",
    @name ParametricFunctionSource, ffi::vtkParametricFunctionSource,
    @new ffi::vtk_parametric_function_source_new,
    @delete ffi::vtk_parametric_function_source_delete,
    @inherit vtkPolyDataAlgorithm
);

impl ParametricFunctionSource {
    /// Set the parametric function to use (e.g., torus, klein bottle, mobius strip)
    ///
    /// # Safety
    /// The func pointer must be a valid vtkParametricFunction pointer from any parametric function type
    #[doc(alias = "SetParametricFunction")]
    pub fn set_parametric_function(&mut self, func: *mut std::ffi::c_void) {
        unsafe {
            // Cast void pointer to vtkParametricFunction for FFI boundary
            let vtk_func = func as *mut ffi::vtkParametricFunction;
            ffi::parametric_function_source_set_parametric_function(self.ptr.as_mut(), vtk_func)
        }
    }

    /// Set the number of subdivisions in the u parametric direction
    #[doc(alias = "SetUResolution")]
    pub fn set_u_resolution(&mut self, resolution: i32) {
        ffi::parametric_function_source_set_u_resolution(self.ptr.as_mut(), resolution)
    }

    /// Get the number of subdivisions in the u parametric direction
    #[doc(alias = "GetUResolution")]
    pub fn get_u_resolution(&self) -> i32 {
        ffi::parametric_function_source_get_u_resolution(&self.ptr.as_ref())
    }

    /// Set the number of subdivisions in the v parametric direction
    #[doc(alias = "SetVResolution")]
    pub fn set_v_resolution(&mut self, resolution: i32) {
        ffi::parametric_function_source_set_v_resolution(self.ptr.as_mut(), resolution)
    }

    /// Get the number of subdivisions in the v parametric direction
    #[doc(alias = "GetVResolution")]
    pub fn get_v_resolution(&self) -> i32 {
        ffi::parametric_function_source_get_v_resolution(&self.ptr.as_ref())
    }

    /// Set the number of subdivisions in the w parametric direction (for 3D parametric)
    #[doc(alias = "SetWResolution")]
    pub fn set_w_resolution(&mut self, resolution: i32) {
        ffi::parametric_function_source_set_w_resolution(self.ptr.as_mut(), resolution)
    }

    /// Get the number of subdivisions in the w parametric direction
    #[doc(alias = "GetWResolution")]
    pub fn get_w_resolution(&self) -> i32 {
        ffi::parametric_function_source_get_w_resolution(&self.ptr.as_ref())
    }

    /// Get the output port for connecting to a mapper
    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::parametric_function_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
