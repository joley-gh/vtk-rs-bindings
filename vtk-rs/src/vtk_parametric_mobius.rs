#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_mobius.h");
        include!("vtk_parametric_function.h");

        type vtkParametricMobius;
        type vtkParametricFunction;

        fn vtk_parametric_mobius_new() -> *mut vtkParametricMobius;
        fn vtk_parametric_mobius_delete(ptr: Pin<&mut vtkParametricMobius>);
        fn parametric_mobius_set_radius(mobius: Pin<&mut vtkParametricMobius>, radius: f64);
        fn parametric_mobius_get_radius(mobius: &vtkParametricMobius) -> f64;
        unsafe fn parametric_mobius_as_parametric_function(
            mobius: Pin<&mut vtkParametricMobius>
        ) -> *mut vtkParametricFunction;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkParametricMobius.html",
    @name ParametricMobius, ffi::vtkParametricMobius,
    @new ffi::vtk_parametric_mobius_new,
    @delete ffi::vtk_parametric_mobius_delete,
    @inherit vtkObjectBase
);

impl ParametricMobius {
    /// Set the radius of the Mobius strip
    #[doc(alias = "SetRadius")]
    pub fn set_radius(&mut self, radius: f64) {
        ffi::parametric_mobius_set_radius(self.ptr.as_mut(), radius)
    }

    /// Get the radius of the Mobius strip
    #[doc(alias = "GetRadius")]
    pub fn get_radius(&self) -> f64 {
        ffi::parametric_mobius_get_radius(&self.ptr.as_ref())
    }
}

// Implement ParametricFunction trait for ParametricMobius
impl super::ParametricFunction for ParametricMobius {
    fn as_parametric_function_ptr(
        &mut self
    ) -> *mut crate::vtk_parametric_function::ffi::vtkParametricFunction {
        unsafe {
            ffi::parametric_mobius_as_parametric_function(
                self.ptr.as_mut()
            ) as *mut crate::vtk_parametric_function::ffi::vtkParametricFunction
        }
    }
}
