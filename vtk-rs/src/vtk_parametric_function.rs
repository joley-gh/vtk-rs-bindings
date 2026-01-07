#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_function.h");

        type vtkParametricFunction;

        // No new/delete here - these are abstract base class references
        // Specific parametric functions will have their own new/delete
    }
}

/// Trait for types that represent VTK parametric functions.
/// All parametric function types (Torus, Klein, Mobius, etc.) implement this trait.
pub trait ParametricFunction {
    #[doc(hidden)]
    fn as_parametric_function_ptr(&mut self) -> *mut ffi::vtkParametricFunction;
}

// This is just the base type for parametric functions
// Individual parametric functions (torus, klein, mobius) will have their own wrappers
