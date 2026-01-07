#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_klein.h");
        include!("vtk_parametric_function.h");

        type vtkParametricKlein;
        type vtkParametricFunction;

        fn vtk_parametric_klein_new() -> *mut vtkParametricKlein;
        fn vtk_parametric_klein_delete(ptr: Pin<&mut vtkParametricKlein>);
        unsafe fn parametric_klein_as_parametric_function(
            klein: Pin<&mut vtkParametricKlein>
        ) -> *mut vtkParametricFunction;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkParametricKlein.html",
    @name ParametricKlein, ffi::vtkParametricKlein,
    @new ffi::vtk_parametric_klein_new,
    @delete ffi::vtk_parametric_klein_delete,
    @inherit vtkObjectBase
);

// Implement ParametricFunction trait for ParametricKlein
impl super::ParametricFunction for ParametricKlein {
    fn as_parametric_function_ptr(
        &mut self
    ) -> *mut crate::vtk_parametric_function::ffi::vtkParametricFunction {
        unsafe {
            ffi::parametric_klein_as_parametric_function(
                self.ptr.as_mut()
            ) as *mut crate::vtk_parametric_function::ffi::vtkParametricFunction
        }
    }
}
