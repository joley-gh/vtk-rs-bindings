use crate::init_vtk;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_scalar_bar_actor.h");

        type vtkScalarBarActor;
        type vtkLookupTable = crate::vtk_lookup_table::ffi::vtkLookupTable;
        type vtkTextProperty = crate::vtk_text_actor::ffi::vtkTextProperty;

        // Lifecycle
        fn scalar_bar_actor_new() -> *mut vtkScalarBarActor;
        fn scalar_bar_actor_delete(actor: Pin<&mut vtkScalarBarActor>);

        // Lookup table
        fn scalar_bar_actor_set_lookup_table(
            actor: Pin<&mut vtkScalarBarActor>,
            lut: Pin<&mut vtkLookupTable>
        );

        // Title
        fn scalar_bar_actor_set_title(actor: Pin<&mut vtkScalarBarActor>, title: &str);
        fn scalar_bar_actor_get_title(actor: Pin<&mut vtkScalarBarActor>) -> String;

        // Labels
        fn scalar_bar_actor_set_number_of_labels(actor: Pin<&mut vtkScalarBarActor>, n: i32);
        fn scalar_bar_actor_get_number_of_labels(actor: Pin<&mut vtkScalarBarActor>) -> i32;

        // Position and size (normalized viewport coordinates [0,1])
        fn scalar_bar_actor_set_position(actor: Pin<&mut vtkScalarBarActor>, x: f64, y: f64);
        fn scalar_bar_actor_get_position(
            actor: Pin<&mut vtkScalarBarActor>,
            x: &mut f64,
            y: &mut f64
        );

        fn scalar_bar_actor_set_width(actor: Pin<&mut vtkScalarBarActor>, width: f64);
        fn scalar_bar_actor_get_width(actor: Pin<&mut vtkScalarBarActor>) -> f64;

        fn scalar_bar_actor_set_height(actor: Pin<&mut vtkScalarBarActor>, height: f64);
        fn scalar_bar_actor_get_height(actor: Pin<&mut vtkScalarBarActor>) -> f64;

        // Text properties (non-owning references)
        fn scalar_bar_actor_get_label_text_property(
            actor: Pin<&mut vtkScalarBarActor>
        ) -> *mut vtkTextProperty;
        fn scalar_bar_actor_get_title_text_property(
            actor: Pin<&mut vtkScalarBarActor>
        ) -> *mut vtkTextProperty;

        // Orientation
        fn scalar_bar_actor_set_orientation(actor: Pin<&mut vtkScalarBarActor>, orientation: i32);
        fn scalar_bar_actor_get_orientation(actor: Pin<&mut vtkScalarBarActor>) -> i32;

        // Visibility
        fn scalar_bar_actor_set_visibility(actor: Pin<&mut vtkScalarBarActor>, visible: bool);
        fn scalar_bar_actor_get_visibility(actor: Pin<&mut vtkScalarBarActor>) -> bool;
    }
}

use std::pin::Pin;
use crate::vtk_text_actor::TextPropertyRef;

/// Orientation for the scalar bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarBarOrientation {
    /// Vertical orientation (default)
    Vertical = 0,
    /// Horizontal orientation
    Horizontal = 1,
}

/// VTK Scalar Bar Actor - Color legend for data visualization
///
/// Displays a color bar showing the mapping between data values and colors.
/// Essential for scientific visualizations where quantitative color mapping is used.
pub struct ScalarBarActor {
    ptr: *mut ffi::vtkScalarBarActor,
}

impl ScalarBarActor {
    /// Create a new scalar bar actor
    pub fn new() -> Self {
        init_vtk();
        Self {
            ptr: ffi::scalar_bar_actor_new(),
        }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkScalarBarActor> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Get raw pointer for renderer integration
    pub(crate) fn as_raw_ptr(&mut self) -> *mut ffi::vtkScalarBarActor {
        self.ptr
    }

    /// Set the lookup table that defines the color mapping
    pub fn set_lookup_table(&mut self, lut: &mut crate::LookupTable) {
        let lut_ptr = lut.as_raw_ptr();
        unsafe {
            ffi::scalar_bar_actor_set_lookup_table(
                self.as_mut(),
                Pin::new_unchecked(&mut *lut_ptr)
            );
        }
    }

    /// Set the title displayed above the color bar
    pub fn set_title(&mut self, title: &str) {
        ffi::scalar_bar_actor_set_title(self.as_mut(), title);
    }

    /// Get the current title
    pub fn get_title(&mut self) -> String {
        ffi::scalar_bar_actor_get_title(self.as_mut())
    }

    /// Set the number of numeric labels along the color bar
    ///
    /// Default: 5
    pub fn set_number_of_labels(&mut self, n: i32) {
        ffi::scalar_bar_actor_set_number_of_labels(self.as_mut(), n);
    }

    /// Get the number of labels
    pub fn get_number_of_labels(&mut self) -> i32 {
        ffi::scalar_bar_actor_get_number_of_labels(self.as_mut())
    }

    /// Set position in normalized viewport coordinates [0, 1]
    ///
    /// (0, 0) = bottom-left
    /// (1, 1) = top-right
    pub fn set_position(&mut self, x: f64, y: f64) {
        ffi::scalar_bar_actor_set_position(self.as_mut(), x, y);
    }

    /// Get the current position
    pub fn get_position(&mut self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::scalar_bar_actor_get_position(self.as_mut(), &mut x, &mut y);
        (x, y)
    }

    /// Set width in normalized viewport coordinates [0, 1]
    ///
    /// Default: 0.05 for vertical, 0.8 for horizontal
    pub fn set_width(&mut self, width: f64) {
        ffi::scalar_bar_actor_set_width(self.as_mut(), width);
    }

    /// Get the current width
    pub fn get_width(&mut self) -> f64 {
        ffi::scalar_bar_actor_get_width(self.as_mut())
    }

    /// Set height in normalized viewport coordinates [0, 1]
    ///
    /// Default: 0.75 for vertical, 0.17 for horizontal
    pub fn set_height(&mut self, height: f64) {
        ffi::scalar_bar_actor_set_height(self.as_mut(), height);
    }

    /// Get the current height
    pub fn get_height(&mut self) -> f64 {
        ffi::scalar_bar_actor_get_height(self.as_mut())
    }

    /// Get the text property for numeric labels
    ///
    /// Use this to customize font, size, color, etc.
    pub fn get_label_text_property(&mut self) -> TextPropertyRef {
        let ptr = ffi::scalar_bar_actor_get_label_text_property(self.as_mut());
        TextPropertyRef::from_raw_ptr(ptr)
    }

    /// Get the text property for the title
    ///
    /// Use this to customize font, size, color, etc.
    pub fn get_title_text_property(&mut self) -> TextPropertyRef {
        let ptr = ffi::scalar_bar_actor_get_title_text_property(self.as_mut());
        TextPropertyRef::from_raw_ptr(ptr)
    }

    /// Set the orientation (vertical or horizontal)
    pub fn set_orientation(&mut self, orientation: ScalarBarOrientation) {
        ffi::scalar_bar_actor_set_orientation(self.as_mut(), orientation as i32);
    }

    /// Get the current orientation
    pub fn get_orientation(&mut self) -> ScalarBarOrientation {
        let val = ffi::scalar_bar_actor_get_orientation(self.as_mut());
        if val == 0 {
            ScalarBarOrientation::Vertical
        } else {
            ScalarBarOrientation::Horizontal
        }
    }

    /// Set visibility of the scalar bar
    pub fn set_visibility(&mut self, visible: bool) {
        ffi::scalar_bar_actor_set_visibility(self.as_mut(), visible);
    }

    /// Get the current visibility
    pub fn get_visibility(&mut self) -> bool {
        ffi::scalar_bar_actor_get_visibility(self.as_mut())
    }
}

impl Drop for ScalarBarActor {
    fn drop(&mut self) {
        ffi::scalar_bar_actor_delete(self.as_mut());
    }
}

impl Default for ScalarBarActor {
    fn default() -> Self {
        Self::new()
    }
}
