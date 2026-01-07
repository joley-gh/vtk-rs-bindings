use crate::init_vtk;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_legend_box_actor.h");

        type vtkLegendBoxActor;
        type vtkActor = crate::vtk_actor::ffi::vtkActor;

        // Lifecycle
        fn legend_box_actor_new() -> *mut vtkLegendBoxActor;
        fn legend_box_actor_delete(actor: Pin<&mut vtkLegendBoxActor>);

        // Entries
        fn legend_box_actor_set_number_of_entries(actor: Pin<&mut vtkLegendBoxActor>, n: i32);
        fn legend_box_actor_get_number_of_entries(actor: Pin<&mut vtkLegendBoxActor>) -> i32;

        fn legend_box_actor_set_entry_string(
            actor: Pin<&mut vtkLegendBoxActor>,
            index: i32,
            label: &str
        );
        fn legend_box_actor_set_entry_color(
            actor: Pin<&mut vtkLegendBoxActor>,
            index: i32,
            r: f64,
            g: f64,
            b: f64
        );

        // Position and size (normalized viewport coordinates [0,1])
        fn legend_box_actor_set_position(actor: Pin<&mut vtkLegendBoxActor>, x: f64, y: f64);
        fn legend_box_actor_get_position(
            actor: Pin<&mut vtkLegendBoxActor>,
            x: &mut f64,
            y: &mut f64
        );

        fn legend_box_actor_set_position2(actor: Pin<&mut vtkLegendBoxActor>, x: f64, y: f64);
        fn legend_box_actor_get_position2(
            actor: Pin<&mut vtkLegendBoxActor>,
            x: &mut f64,
            y: &mut f64
        );

        // Border and background
        fn legend_box_actor_set_border(actor: Pin<&mut vtkLegendBoxActor>, border: bool);
        fn legend_box_actor_get_border(actor: Pin<&mut vtkLegendBoxActor>) -> bool;

        fn legend_box_actor_set_box(actor: Pin<&mut vtkLegendBoxActor>, box_: bool);
        fn legend_box_actor_get_box(actor: Pin<&mut vtkLegendBoxActor>) -> bool;

        // Padding
        fn legend_box_actor_set_padding(actor: Pin<&mut vtkLegendBoxActor>, padding: i32);
        fn legend_box_actor_get_padding(actor: Pin<&mut vtkLegendBoxActor>) -> i32;

        // Visibility
        fn legend_box_actor_set_visibility(actor: Pin<&mut vtkLegendBoxActor>, visible: bool);
        fn legend_box_actor_get_visibility(actor: Pin<&mut vtkLegendBoxActor>) -> bool;
    }
}

use std::pin::Pin;

/// VTK Legend Box Actor - Multi-item legend for annotating scenes
///
/// Displays a legend box with multiple entries, each showing a color/symbol and label.
/// Useful for identifying different objects or data series in a visualization.
pub struct LegendBoxActor {
    ptr: *mut ffi::vtkLegendBoxActor,
}

impl LegendBoxActor {
    /// Create a new legend box actor
    pub fn new() -> Self {
        init_vtk();
        Self {
            ptr: ffi::legend_box_actor_new(),
        }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkLegendBoxActor> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Get raw pointer for renderer integration
    pub(crate) fn as_raw_ptr(&mut self) -> *mut ffi::vtkLegendBoxActor {
        self.ptr
    }

    /// Set the number of legend entries
    pub fn set_number_of_entries(&mut self, n: i32) {
        ffi::legend_box_actor_set_number_of_entries(self.as_mut(), n);
    }

    /// Get the number of legend entries
    pub fn get_number_of_entries(&mut self) -> i32 {
        ffi::legend_box_actor_get_number_of_entries(self.as_mut())
    }

    /// Set a legend entry with label and color
    ///
    /// # Arguments
    /// * `index` - Entry index (0-based)
    /// * `label` - Text label for this entry
    /// * `r`, `g`, `b` - RGB color values [0.0, 1.0]
    pub fn set_entry(&mut self, index: i32, label: &str, r: f64, g: f64, b: f64) {
        ffi::legend_box_actor_set_entry_string(self.as_mut(), index, label);
        ffi::legend_box_actor_set_entry_color(self.as_mut(), index, r, g, b);
    }

    /// Set only the text label for an entry
    pub fn set_entry_string(&mut self, index: i32, label: &str) {
        ffi::legend_box_actor_set_entry_string(self.as_mut(), index, label);
    }

    /// Set only the color for an entry
    pub fn set_entry_color(&mut self, index: i32, r: f64, g: f64, b: f64) {
        ffi::legend_box_actor_set_entry_color(self.as_mut(), index, r, g, b);
    }

    /// Set position in normalized viewport coordinates [0, 1]
    ///
    /// (0, 0) = bottom-left
    /// (1, 1) = top-right
    pub fn set_position(&mut self, x: f64, y: f64) {
        ffi::legend_box_actor_set_position(self.as_mut(), x, y);
    }

    /// Get the current position
    pub fn get_position(&mut self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::legend_box_actor_get_position(self.as_mut(), &mut x, &mut y);
        (x, y)
    }

    /// Set size in normalized viewport coordinates [0, 1]
    ///
    /// Position2 represents width and height relative to position
    pub fn set_position2(&mut self, x: f64, y: f64) {
        ffi::legend_box_actor_set_position2(self.as_mut(), x, y);
    }

    /// Get the current size
    pub fn get_position2(&mut self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::legend_box_actor_get_position2(self.as_mut(), &mut x, &mut y);
        (x, y)
    }

    /// Set whether to draw a border around the legend
    pub fn set_border(&mut self, border: bool) {
        ffi::legend_box_actor_set_border(self.as_mut(), border);
    }

    /// Get the current border setting
    pub fn get_border(&mut self) -> bool {
        ffi::legend_box_actor_get_border(self.as_mut())
    }

    /// Set whether to draw a background box
    pub fn set_box(&mut self, box_: bool) {
        ffi::legend_box_actor_set_box(self.as_mut(), box_);
    }

    /// Get the current box setting
    pub fn get_box(&mut self) -> bool {
        ffi::legend_box_actor_get_box(self.as_mut())
    }

    /// Set padding around legend entries in pixels
    pub fn set_padding(&mut self, padding: i32) {
        ffi::legend_box_actor_set_padding(self.as_mut(), padding);
    }

    /// Get the current padding
    pub fn get_padding(&mut self) -> i32 {
        ffi::legend_box_actor_get_padding(self.as_mut())
    }

    /// Set visibility of the legend
    pub fn set_visibility(&mut self, visible: bool) {
        ffi::legend_box_actor_set_visibility(self.as_mut(), visible);
    }

    /// Get the current visibility
    pub fn get_visibility(&mut self) -> bool {
        ffi::legend_box_actor_get_visibility(self.as_mut())
    }
}

impl Drop for LegendBoxActor {
    fn drop(&mut self) {
        ffi::legend_box_actor_delete(self.as_mut());
    }
}

impl Default for LegendBoxActor {
    fn default() -> Self {
        Self::new()
    }
}

// Implement AddableToRenderer for LegendBoxActor (2D actor - vtkActor2D)
impl crate::AddableToRenderer for LegendBoxActor {
    fn add_to_renderer_internal(&mut self, renderer: &mut crate::Renderer) {
        let actor_ptr = self.as_raw_ptr() as *mut crate::vtk_renderer::ffi::vtkActor2D;
        renderer._add_actor_2d(actor_ptr);
    }
}
