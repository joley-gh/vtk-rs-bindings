#include "cxx.h"
#include "vtk_image_data.h"
#include "vtk_image_data.rs.h"

#include <vtkImageData.h>

// Create/Delete
vtkImageData* vtk_image_data_new() {
    return vtkImageData::New();
}

void vtk_image_data_delete(vtkImageData& image_data) {
    image_data.Delete();
}

// Dimensions
void image_data_set_dimensions(vtkImageData& image_data, int nx, int ny, int nz) {
    image_data.SetDimensions(nx, ny, nz);
}

void image_data_get_dimensions(const vtkImageData& image_data, int* nx, int* ny, int* nz) {
    // VTK GetDimensions() is not const, so we need to cast
    int* dims = const_cast<vtkImageData&>(image_data).GetDimensions();
    *nx = dims[0];
    *ny = dims[1];
    *nz = dims[2];
}

// Spacing
void image_data_set_spacing(vtkImageData& image_data, double dx, double dy, double dz) {
    image_data.SetSpacing(dx, dy, dz);
}

void image_data_get_spacing(const vtkImageData& image_data, double* dx, double* dy, double* dz) {
    // VTK GetSpacing() is not const, so we need to cast
    double* spacing = const_cast<vtkImageData&>(image_data).GetSpacing();
    *dx = spacing[0];
    *dy = spacing[1];
    *dz = spacing[2];
}

// Origin
void image_data_set_origin(vtkImageData& image_data, double x, double y, double z) {
    image_data.SetOrigin(x, y, z);
}

void image_data_get_origin(const vtkImageData& image_data, double* x, double* y, double* z) {
    // VTK GetOrigin() is not const, so we need to cast
    double* origin = const_cast<vtkImageData&>(image_data).GetOrigin();
    *x = origin[0];
    *y = origin[1];
    *z = origin[2];
}

// Scalar allocation
void image_data_allocate_scalars(vtkImageData& image_data, int vtk_type, int num_components) {
    image_data.AllocateScalars(vtk_type, num_components);
}

// Scalar pointer access
void* image_data_get_scalar_pointer(vtkImageData& image_data) {
    return image_data.GetScalarPointer();
}

void* image_data_get_scalar_pointer_xyz(vtkImageData& image_data, int x, int y, int z) {
    return image_data.GetScalarPointer(x, y, z);
}

// Scalar value access
void image_data_set_scalar_component_from_double(
    vtkImageData& image_data, 
    int x, int y, int z, 
    int component, 
    double value
) {
    image_data.SetScalarComponentFromDouble(x, y, z, component, value);
}

double image_data_get_scalar_component_as_double(
    const vtkImageData& image_data,
    int x, int y, int z,
    int component
) {
    // VTK GetScalarComponentAsDouble() is not const
    return const_cast<vtkImageData&>(image_data).GetScalarComponentAsDouble(x, y, z, component);
}

// Number of points and cells
int image_data_get_number_of_points(const vtkImageData& image_data) {
    // VTK GetNumberOfPoints() is not const
    return const_cast<vtkImageData&>(image_data).GetNumberOfPoints();
}

int image_data_get_number_of_cells(const vtkImageData& image_data) {
    // VTK GetNumberOfCells() is not const
    return const_cast<vtkImageData&>(image_data).GetNumberOfCells();
}

// Bounds
void image_data_get_bounds(const vtkImageData& image_data, double bounds[6]) {
    // VTK GetBounds() is not const
    const_cast<vtkImageData&>(image_data).GetBounds(bounds);
}

vtkAlgorithmOutput* image_data_get_output_port(vtkImageData& image_data) {
    // ImageData is a data object, not an algorithm, so we need to use TrivialProducer
    // However, for simplicity in filters, we can create a producer on-the-fly
    // Actually, VTK filters that take SetInputData() don't need GetOutputPort()
    // But for consistency with algorithm sources, we return nullptr
    // The contour filter should use SetInputData instead
    return nullptr;  // ImageData doesn't have output ports - it's a data object
}
