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
    /// # Example
    /// ```no_run
    /// # use vtk_rs::*;
    /// let mut torus = ParametricTorus::new();
    /// let mut source = ParametricFunctionSource::new();
    /// source.set_parametric_function(&mut torus);
    /// ```
    #[doc(alias = "SetParametricFunction")]
    pub fn set_parametric_function<T: super::ParametricFunction>(&mut self, func: &mut T) {
        unsafe {
            let vtk_func = func.as_parametric_function_ptr();
            // Cast from trait's ffi type to local ffi type (they're the same underlying C++ type)
            let local_ptr = vtk_func as *mut ffi::vtkParametricFunction;
            ffi::parametric_function_source_set_parametric_function(self.ptr.as_mut(), local_ptr)
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

    /// Create a fully configured actor from a parametric function.
    ///
    /// This is a convenience method that combines creating the source, setting resolution,
    /// creating a mapper, and creating an actor with visual properties.
    ///
    /// # Example
    /// ```no_run
    /// # use vtk_rs::*;
    /// let mut torus = ParametricTorus::new();
    /// torus.set_ring_radius(1.0);
    ///
    /// let (mut actor, mut source, mut mapper) = ParametricFunctionSource::create_actor(
    ///     &mut torus,
    ///     50,  // U resolution
    ///     50,  // V resolution
    ///     (1.0, 0.3, 0.3),  // RGB color
    ///     0.8,  // Opacity
    ///     (0.0, 0.0, 0.0)   // Position
    /// );
    /// ```
    pub fn create_actor<T: super::ParametricFunction>(
        parametric_func: &mut T,
        u_resolution: i32,
        v_resolution: i32,
        color: (f64, f64, f64),
        opacity: f64,
        position: (f64, f64, f64)
    ) -> (crate::Actor, Self, crate::PolyDataMapper) {
        let mut source = Self::new();
        source.set_parametric_function(parametric_func);
        source.set_u_resolution(u_resolution);
        source.set_v_resolution(v_resolution);

        let mut mapper = crate::PolyDataMapper::new();
        mapper.set_input_connection(source.get_output_port());

        let mut actor = crate::Actor::new();
        actor.set_mapper(&mut mapper);
        actor.get_property().set_color(color.0, color.1, color.2);
        actor.get_property().set_opacity(opacity);
        actor.set_position(position.0, position.1, position.2);

        (actor, source, mapper)
    }
}
