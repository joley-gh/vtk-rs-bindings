use crate::init_vtk;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_lookup_table.h");

        type vtkLookupTable;

        // Lifecycle
        fn lookup_table_new() -> *mut vtkLookupTable;
        fn lookup_table_delete(lut: Pin<&mut vtkLookupTable>);

        // Range configuration
        fn lookup_table_set_range(lut: Pin<&mut vtkLookupTable>, min: f64, max: f64);
        fn lookup_table_get_range(lut: Pin<&mut vtkLookupTable>, min: &mut f64, max: &mut f64);

        // Color range (hue in HSV color space)
        fn lookup_table_set_hue_range(lut: Pin<&mut vtkLookupTable>, min: f64, max: f64);
        fn lookup_table_get_hue_range(lut: Pin<&mut vtkLookupTable>, min: &mut f64, max: &mut f64);

        // Saturation range
        fn lookup_table_set_saturation_range(lut: Pin<&mut vtkLookupTable>, min: f64, max: f64);
        fn lookup_table_get_saturation_range(
            lut: Pin<&mut vtkLookupTable>,
            min: &mut f64,
            max: &mut f64
        );

        // Value range (brightness in HSV)
        fn lookup_table_set_value_range(lut: Pin<&mut vtkLookupTable>, min: f64, max: f64);
        fn lookup_table_get_value_range(
            lut: Pin<&mut vtkLookupTable>,
            min: &mut f64,
            max: &mut f64
        );

        // Alpha range (opacity)
        fn lookup_table_set_alpha_range(lut: Pin<&mut vtkLookupTable>, min: f64, max: f64);
        fn lookup_table_get_alpha_range(
            lut: Pin<&mut vtkLookupTable>,
            min: &mut f64,
            max: &mut f64
        );

        // Table configuration
        fn lookup_table_set_number_of_table_values(lut: Pin<&mut vtkLookupTable>, n: i64);
        fn lookup_table_get_number_of_table_values(lut: Pin<&mut vtkLookupTable>) -> i64;

        // Build the table
        fn lookup_table_build(lut: Pin<&mut vtkLookupTable>);

        // Get color for a value
        fn lookup_table_get_color(
            lut: Pin<&mut vtkLookupTable>,
            value: f64,
            r: &mut f64,
            g: &mut f64,
            b: &mut f64
        );
    }
}

use std::pin::Pin;

/// VTK Lookup Table for mapping scalar values to colors
///
/// Maps scalar data values to colors using HSV color space interpolation.
/// Essential for creating color legends and visualizing data distributions.
pub struct LookupTable {
    ptr: *mut ffi::vtkLookupTable,
}

impl LookupTable {
    /// Create a new lookup table
    pub fn new() -> Self {
        init_vtk();
        Self {
            ptr: ffi::lookup_table_new(),
        }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkLookupTable> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Get raw pointer for FFI interop
    pub(crate) fn as_raw_ptr(&mut self) -> *mut ffi::vtkLookupTable {
        self.ptr
    }

    /// Set the data value range to map
    pub fn set_range(&mut self, min: f64, max: f64) {
        ffi::lookup_table_set_range(self.as_mut(), min, max);
    }

    /// Get the data value range
    pub fn get_range(&mut self) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        ffi::lookup_table_get_range(self.as_mut(), &mut min, &mut max);
        (min, max)
    }

    /// Set the hue range in HSV color space [0, 1]
    ///
    /// Default: (0.0, 0.66667) which creates a blue-to-red rainbow
    /// - 0.0 = Red
    /// - 0.33 = Green
    /// - 0.66 = Blue
    /// - 1.0 = Red (wraps around)
    pub fn set_hue_range(&mut self, min: f64, max: f64) {
        ffi::lookup_table_set_hue_range(self.as_mut(), min, max);
    }

    /// Get the hue range
    pub fn get_hue_range(&mut self) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        ffi::lookup_table_get_hue_range(self.as_mut(), &mut min, &mut max);
        (min, max)
    }

    /// Set the saturation range in HSV color space [0, 1]
    ///
    /// Default: (1.0, 1.0) for fully saturated colors
    /// Lower values create more pastel colors
    pub fn set_saturation_range(&mut self, min: f64, max: f64) {
        ffi::lookup_table_set_saturation_range(self.as_mut(), min, max);
    }

    /// Get the saturation range
    pub fn get_saturation_range(&mut self) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        ffi::lookup_table_get_saturation_range(self.as_mut(), &mut min, &mut max);
        (min, max)
    }

    /// Set the value (brightness) range in HSV color space [0, 1]
    ///
    /// Default: (1.0, 1.0) for full brightness
    /// Lower values create darker colors
    pub fn set_value_range(&mut self, min: f64, max: f64) {
        ffi::lookup_table_set_value_range(self.as_mut(), min, max);
    }

    /// Get the value range
    pub fn get_value_range(&mut self) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        ffi::lookup_table_get_value_range(self.as_mut(), &mut min, &mut max);
        (min, max)
    }

    /// Set the alpha (opacity) range [0, 1]
    ///
    /// Default: (1.0, 1.0) for fully opaque colors
    pub fn set_alpha_range(&mut self, min: f64, max: f64) {
        ffi::lookup_table_set_alpha_range(self.as_mut(), min, max);
    }

    /// Get the alpha range
    pub fn get_alpha_range(&mut self) -> (f64, f64) {
        let mut min = 0.0;
        let mut max = 0.0;
        ffi::lookup_table_get_alpha_range(self.as_mut(), &mut min, &mut max);
        (min, max)
    }

    /// Set the number of discrete colors in the table
    ///
    /// Default: 256
    /// Lower values create a discrete/banded color map
    pub fn set_number_of_table_values(&mut self, n: i64) {
        ffi::lookup_table_set_number_of_table_values(self.as_mut(), n);
    }

    /// Get the number of table values
    pub fn get_number_of_table_values(&mut self) -> i64 {
        ffi::lookup_table_get_number_of_table_values(self.as_mut())
    }

    /// Build the lookup table
    ///
    /// Must be called after configuring ranges and before using the table
    pub fn build(&mut self) {
        ffi::lookup_table_build(self.as_mut());
    }

    /// Get the RGB color for a specific data value
    pub fn get_color(&mut self, value: f64) -> (f64, f64, f64) {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        ffi::lookup_table_get_color(self.as_mut(), value, &mut r, &mut g, &mut b);
        (r, g, b)
    }
}

impl Drop for LookupTable {
    fn drop(&mut self) {
        ffi::lookup_table_delete(self.as_mut());
    }
}

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}
