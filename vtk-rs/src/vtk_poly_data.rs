// Direct extern "C" bindings (no cxx bridge)
use crate::{ Points, CellArray, PointData, CellData };

#[repr(C)]
pub struct vtkPolyData {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn poly_data_new() -> *mut vtkPolyData;
    fn poly_data_delete(poly_data: *mut vtkPolyData);
    fn poly_data_set_points(poly_data: *mut vtkPolyData, points: *mut crate::vtk_points::vtkPoints);
    fn poly_data_get_points(poly_data: *mut vtkPolyData) -> *mut crate::vtk_points::vtkPoints;
    fn poly_data_get_number_of_points(poly_data: *mut vtkPolyData) -> i64;
    fn poly_data_set_lines(
        poly_data: *mut vtkPolyData,
        lines: *mut crate::vtk_cell_array::vtkCellArray
    );
    fn poly_data_get_lines(poly_data: *mut vtkPolyData) -> *mut crate::vtk_cell_array::vtkCellArray;
    fn poly_data_get_number_of_lines(poly_data: *mut vtkPolyData) -> i64;
    fn poly_data_get_number_of_cells(poly_data: *mut vtkPolyData) -> i64;
    fn poly_data_get_bounds(poly_data: *mut vtkPolyData, bounds: *mut [f64; 6]);
    fn poly_data_allocate(poly_data: *mut vtkPolyData, num_verts: i64, connectivity_size: i64);
    fn poly_data_modified(poly_data: *mut vtkPolyData);
    fn poly_data_compute_bounds(poly_data: *mut vtkPolyData);
    fn poly_data_get_point_data(
        poly_data: *mut vtkPolyData
    ) -> *mut crate::vtk_point_data::vtkPointData;
    fn poly_data_get_cell_data(
        poly_data: *mut vtkPolyData
    ) -> *mut crate::vtk_cell_data::vtkCellData;
    fn poly_data_get_producer_port(poly_data: *mut vtkPolyData) -> *mut crate::AlgorithmOutputPort;
}

/// Safe wrapper for vtkPolyData
///
/// PolyData is the fundamental data structure for representing geometry in VTK.
/// For FEM beam structures, it combines:
/// - Points: Node locations (x, y, z coordinates)
/// - Lines: Beam element connectivity (which nodes connect to form beams)
/// - Data Arrays: Properties like deformations, displacements, stress, etc.
pub struct PolyData {
    ptr: *mut vtkPolyData,
}

impl PolyData {
    /// Create a new empty PolyData
    pub fn new() -> Self {
        let ptr = unsafe { poly_data_new() };
        assert!(!ptr.is_null(), "Failed to create vtkPolyData");
        Self { ptr }
    }

    /// Set the points (node positions) for this PolyData
    ///
    /// # Example
    /// ```
    /// # use vtk_rs::{Points, PolyData};
    /// let mut points = Points::new();
    /// points.insert_next_point(0.0, 0.0, 0.0);
    /// points.insert_next_point(1.0, 0.0, 0.0);
    ///
    /// let mut poly_data = PolyData::new();
    /// poly_data.set_points(&points);
    /// ```
    pub fn set_points(&mut self, points: &Points) {
        unsafe {
            poly_data_set_points(self.ptr, points.as_ptr());
        }
    }

    /// Get the number of points in this PolyData
    pub fn get_number_of_points(&self) -> i64 {
        unsafe { poly_data_get_number_of_points(self.ptr) }
    }

    /// Set the lines (beam elements) for this PolyData
    ///
    /// Each line connects two points to form a beam element.
    ///
    /// # Example
    /// ```
    /// # use vtk_rs::{CellArray, PolyData};
    /// let mut cells = CellArray::new();
    /// cells.insert_next_cell(&[0, 1]); // Beam from point 0 to point 1
    /// cells.insert_next_cell(&[1, 2]); // Beam from point 1 to point 2
    ///
    /// let mut poly_data = PolyData::new();
    /// poly_data.set_lines(&cells);
    /// ```
    pub fn set_lines(&mut self, lines: &CellArray) {
        unsafe {
            poly_data_set_lines(self.ptr, lines.as_ptr());
        }
    }

    /// Get the number of line cells (beam elements) in this PolyData
    pub fn get_number_of_lines(&self) -> i64 {
        unsafe { poly_data_get_number_of_lines(self.ptr) }
    }

    /// Get the total number of cells (all types) in this PolyData
    pub fn get_number_of_cells(&self) -> i64 {
        unsafe { poly_data_get_number_of_cells(self.ptr) }
    }

    /// Get the spatial bounds of this PolyData
    ///
    /// Returns (xmin, xmax, ymin, ymax, zmin, zmax)
    pub fn get_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let mut bounds = [0.0; 6];
        unsafe {
            poly_data_get_bounds(self.ptr, &mut bounds);
        }
        (bounds[0], bounds[1], bounds[2], bounds[3], bounds[4], bounds[5])
    }

    /// Allocate memory for cells
    ///
    /// Pre-allocates memory to improve performance when adding many cells
    pub fn allocate(&mut self, num_verts: i64, connectivity_size: i64) {
        unsafe {
            poly_data_allocate(self.ptr, num_verts, connectivity_size);
        }
    }

    /// Mark the data as modified to trigger updates in the visualization pipeline
    pub fn modified(&mut self) {
        unsafe {
            poly_data_modified(self.ptr);
        }
    }

    /// Manually compute the spatial bounds
    pub fn compute_bounds(&mut self) {
        unsafe {
            poly_data_compute_bounds(self.ptr);
        }
    }

    /// Get the raw pointer (for internal use with other VTK functions)
    pub fn as_ptr(&self) -> *mut vtkPolyData {
        self.ptr
    }

    /// Get the point data (attributes associated with points/nodes)
    ///
    /// Use this to attach displacement, stress, temperature, or other node-based data
    pub fn get_point_data(&self) -> PointData {
        unsafe {
            let ptr = poly_data_get_point_data(self.ptr);
            PointData::from_raw(ptr)
        }
    }

    /// Get the cell data (attributes associated with cells/elements)
    ///
    /// Use this to attach material IDs, element stresses, or other element-based data
    pub fn get_cell_data(&self) -> CellData {
        unsafe {
            let ptr = poly_data_get_cell_data(self.ptr);
            CellData::from_raw(ptr)
        }
    }

    /// Get the output port for connecting to filters (like Glyph3D)
    ///
    /// This creates a vtkTrivialProducer to wrap the PolyData as an algorithm output,
    /// allowing it to be connected to filters that expect algorithm output ports.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = poly_data_get_producer_port(self.ptr);
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }

    /// Raw pointer version - internal use only.
    #[doc(hidden)]
    pub(crate) fn _get_output_port_raw(&mut self) -> *mut crate::AlgorithmOutputPort {
        unsafe { poly_data_get_producer_port(self.ptr) }
    }

    /// Create a simple beam structure example
    ///
    /// Creates a PolyData with points and line cells representing a beam structure
    pub fn from_beam_structure(points: &Points, cells: &CellArray) -> Self {
        let mut poly_data = Self::new();
        poly_data.set_points(points);
        poly_data.set_lines(cells);
        poly_data.modified();
        poly_data
    }
}

impl Default for PolyData {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PolyData {
    fn drop(&mut self) {
        unsafe {
            poly_data_delete(self.ptr);
        }
    }
}

unsafe impl Send for PolyData {}
unsafe impl Sync for PolyData {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_data_creation() {
        let poly_data = PolyData::new();
        assert_eq!(poly_data.get_number_of_points(), 0);
        assert_eq!(poly_data.get_number_of_cells(), 0);
    }

    #[test]
    fn test_poly_data_with_points() {
        let mut points = Points::new();
        points.insert_next_point(0.0, 0.0, 0.0);
        points.insert_next_point(1.0, 0.0, 0.0);
        points.insert_next_point(1.0, 1.0, 0.0);

        let mut poly_data = PolyData::new();
        poly_data.set_points(&points);

        assert_eq!(poly_data.get_number_of_points(), 3);
    }

    #[test]
    fn test_poly_data_beam_structure() {
        let mut points = Points::new();
        points.insert_next_point(0.0, 0.0, 0.0);
        points.insert_next_point(1.0, 0.0, 0.0);
        points.insert_next_point(2.0, 0.0, 0.0);

        let mut cells = CellArray::new();
        cells.insert_next_cell(&[0, 1]);
        cells.insert_next_cell(&[1, 2]);

        let poly_data = PolyData::from_beam_structure(&points, &cells);

        assert_eq!(poly_data.get_number_of_points(), 3);
        assert_eq!(poly_data.get_number_of_lines(), 2);
        assert_eq!(poly_data.get_number_of_cells(), 2);
    }

    #[test]
    fn test_poly_data_bounds() {
        let mut points = Points::new();
        points.insert_next_point(-1.0, -2.0, -3.0);
        points.insert_next_point(4.0, 5.0, 6.0);

        let mut poly_data = PolyData::new();
        poly_data.set_points(&points);
        poly_data.compute_bounds();

        let (xmin, xmax, ymin, ymax, zmin, zmax) = poly_data.get_bounds();
        assert_eq!(xmin, -1.0);
        assert_eq!(xmax, 4.0);
        assert_eq!(ymin, -2.0);
        assert_eq!(ymax, 5.0);
        assert_eq!(zmin, -3.0);
        assert_eq!(zmax, 6.0);
    }
}
