#include "cxx.h"
#include "vtk_unstructured_grid.h"
#include "vtk_unstructured_grid.rs.h"

#include <vtkUnstructuredGrid.h>
#include <vtkPoints.h>
#include <vector>

// Create/Delete
vtkUnstructuredGrid* vtk_unstructured_grid_new() {
    return vtkUnstructuredGrid::New();
}

void vtk_unstructured_grid_delete(vtkUnstructuredGrid& grid) {
    grid.Delete();
}

// Points
void unstructured_grid_set_points(vtkUnstructuredGrid& grid, vtkPoints* points) {
    grid.SetPoints(points);
}

vtkPoints* unstructured_grid_get_points(vtkUnstructuredGrid& grid) {
    return grid.GetPoints();
}

// Cell allocation
void unstructured_grid_allocate(vtkUnstructuredGrid& grid, int num_cells) {
    grid.Allocate(num_cells);
}

// Cell insertion
void unstructured_grid_insert_next_cell(
    vtkUnstructuredGrid& grid,
    int cell_type,
    int num_points,
    const int* point_ids
) {
    // Convert int array to vtkIdType array
    std::vector<vtkIdType> ids(num_points);
    for (int i = 0; i < num_points; ++i) {
        ids[i] = static_cast<vtkIdType>(point_ids[i]);
    }
    grid.InsertNextCell(cell_type, static_cast<vtkIdType>(num_points), ids.data());
}

// Metadata
int unstructured_grid_get_number_of_points(const vtkUnstructuredGrid& grid) {
    return const_cast<vtkUnstructuredGrid&>(grid).GetNumberOfPoints();
}

int unstructured_grid_get_number_of_cells(const vtkUnstructuredGrid& grid) {
    return const_cast<vtkUnstructuredGrid&>(grid).GetNumberOfCells();
}

void unstructured_grid_get_bounds(const vtkUnstructuredGrid& grid, double* bounds) {
    const_cast<vtkUnstructuredGrid&>(grid).GetBounds(bounds);
}

// Point data access
extern "C" vtkPointData* unstructured_grid_get_point_data(void* grid_ptr) {
    vtkUnstructuredGrid* grid = static_cast<vtkUnstructuredGrid*>(grid_ptr);
    return grid->GetPointData();
}
