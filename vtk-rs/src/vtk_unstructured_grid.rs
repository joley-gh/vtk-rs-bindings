use std::pin::Pin;

// Direct extern "C" for get_point_data to avoid cxx bridge issues with opaque types
#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn unstructured_grid_get_point_data(
        grid: *mut std::ffi::c_void
    ) -> *mut crate::vtk_point_data::vtkPointData;
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_unstructured_grid.h");
        include!("vtk_points.h");

        type vtkUnstructuredGrid;
        type vtkPoints;

        fn vtk_unstructured_grid_new() -> *mut vtkUnstructuredGrid;
        fn vtk_unstructured_grid_delete(grid: Pin<&mut vtkUnstructuredGrid>);

        unsafe fn unstructured_grid_set_points(
            grid: Pin<&mut vtkUnstructuredGrid>,
            points: *mut vtkPoints
        );
        unsafe fn unstructured_grid_get_points(
            grid: Pin<&mut vtkUnstructuredGrid>
        ) -> *mut vtkPoints;

        fn unstructured_grid_allocate(grid: Pin<&mut vtkUnstructuredGrid>, num_cells: i32);

        unsafe fn unstructured_grid_insert_next_cell(
            grid: Pin<&mut vtkUnstructuredGrid>,
            cell_type: i32,
            num_points: i32,
            point_ids: *const i32
        );

        fn unstructured_grid_get_number_of_points(grid: &vtkUnstructuredGrid) -> i32;
        fn unstructured_grid_get_number_of_cells(grid: &vtkUnstructuredGrid) -> i32;
        unsafe fn unstructured_grid_get_bounds(grid: &vtkUnstructuredGrid, bounds: *mut f64);
    }
}

/// VTK cell type constants
/// See: https://vtk.org/doc/nightly/html/vtkCellType_8h.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VtkCellType {
    // Linear cells
    Vertex = 1,
    PolyVertex = 2,
    Line = 3,
    PolyLine = 4,
    Triangle = 5,
    TriangleStrip = 6,
    Polygon = 7,
    Quad = 9,
    Tetra = 10,
    Voxel = 11,
    Hexahedron = 12,
    Wedge = 13,
    Pyramid = 14,
    PentagonalPrism = 15,
    HexagonalPrism = 16,

    // Quadratic cells
    QuadraticEdge = 21,
    QuadraticTriangle = 22,
    QuadraticQuad = 23,
    QuadraticTetra = 24,
    QuadraticHexahedron = 25,
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkUnstructuredGrid.html",
    @name UnstructuredGrid, ffi::vtkUnstructuredGrid,
    @new ffi::vtk_unstructured_grid_new,
    @delete ffi::vtk_unstructured_grid_delete
);

impl UnstructuredGrid {
    /// Set the points for the grid
    #[doc(alias = "SetPoints")]
    pub fn set_points(&mut self, points: &mut crate::Points) {
        let vtk_points_ptr = points.as_mut_ptr() as *mut ffi::vtkPoints;
        unsafe {
            ffi::unstructured_grid_set_points(self.ptr.as_mut(), vtk_points_ptr);
        }
    }

    /// Allocate memory for a given number of cells
    #[doc(alias = "Allocate")]
    pub fn allocate(&mut self, num_cells: i32) {
        ffi::unstructured_grid_allocate(self.ptr.as_mut(), num_cells);
    }

    /// Insert the next cell into the grid
    #[doc(alias = "InsertNextCell")]
    pub fn insert_next_cell(&mut self, cell_type: VtkCellType, point_ids: &[i32]) {
        unsafe {
            ffi::unstructured_grid_insert_next_cell(
                self.ptr.as_mut(),
                cell_type as i32,
                point_ids.len() as i32,
                point_ids.as_ptr()
            );
        }
    }

    /// Get the number of points in the grid
    #[doc(alias = "GetNumberOfPoints")]
    pub fn get_number_of_points(&self) -> i32 {
        ffi::unstructured_grid_get_number_of_points(self.ptr.as_ref().get_ref())
    }

    /// Get the number of cells in the grid
    #[doc(alias = "GetNumberOfCells")]
    pub fn get_number_of_cells(&self) -> i32 {
        ffi::unstructured_grid_get_number_of_cells(self.ptr.as_ref().get_ref())
    }

    /// Get the bounding box of the grid
    #[doc(alias = "GetBounds")]
    pub fn get_bounds(&self) -> [f64; 6] {
        let mut bounds = [0.0; 6];
        unsafe {
            ffi::unstructured_grid_get_bounds(self.ptr.as_ref().get_ref(), bounds.as_mut_ptr());
        }
        bounds
    }

    /// Get raw pointer for VTK pipeline connections
    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkUnstructuredGrid {
        unsafe { Pin::get_unchecked_mut(self.ptr.as_mut()) as *mut _ }
    }

    /// Get point data for adding scalar/vector fields
    #[doc(alias = "GetPointData")]
    pub fn get_point_data(&mut self) -> crate::PointData {
        unsafe {
            let ptr = unstructured_grid_get_point_data(self.as_raw_ptr() as *mut std::ffi::c_void);
            crate::PointData::from_raw(ptr)
        }
    }
}
