#pragma once

#include <vtkImageData.h>
#include <vtkAlgorithmOutput.h>

// Create/Delete
vtkImageData* vtk_image_data_new();
void vtk_image_data_delete(vtkImageData& image_data);

// Dimensions
void image_data_set_dimensions(vtkImageData& image_data, int nx, int ny, int nz);
void image_data_get_dimensions(const vtkImageData& image_data, int* nx, int* ny, int* nz);

// Spacing
void image_data_set_spacing(vtkImageData& image_data, double dx, double dy, double dz);
void image_data_get_spacing(const vtkImageData& image_data, double* dx, double* dy, double* dz);

// Origin
void image_data_set_origin(vtkImageData& image_data, double x, double y, double z);
void image_data_get_origin(const vtkImageData& image_data, double* x, double* y, double* z);

// Scalar allocation
void image_data_allocate_scalars(vtkImageData& image_data, int vtk_type, int num_components);

// Scalar pointer access (for direct memory manipulation)
void* image_data_get_scalar_pointer(vtkImageData& image_data);
void* image_data_get_scalar_pointer_xyz(vtkImageData& image_data, int x, int y, int z);

// Scalar value access
void image_data_set_scalar_component_from_double(
    vtkImageData& image_data, 
    int x, int y, int z, 
    int component, 
    double value
);

double image_data_get_scalar_component_as_double(
    const vtkImageData& image_data,
    int x, int y, int z,
    int component
);

// Number of points and cells
int image_data_get_number_of_points(const vtkImageData& image_data);
int image_data_get_number_of_cells(const vtkImageData& image_data);

// Bounds  
void image_data_get_bounds(const vtkImageData& image_data, double bounds[6]);

// Algorithm output port
vtkAlgorithmOutput* image_data_get_output_port(vtkImageData& image_data);