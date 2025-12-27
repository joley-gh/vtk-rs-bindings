#include "vtk_smooth_poly_data_filter.h"
#include "vtk_smooth_poly_data_filter.rs.h"

vtkSmoothPolyDataFilter* vtk_smooth_poly_data_filter_new() {
    return vtkSmoothPolyDataFilter::New();
}

void vtk_smooth_poly_data_filter_delete(vtkSmoothPolyDataFilter& filter) {
    filter.Delete();
}

void smooth_poly_data_filter_set_input_connection(vtkSmoothPolyDataFilter& filter, vtkAlgorithmOutput* output) {
    filter.SetInputConnection(output);
}

void smooth_poly_data_filter_set_number_of_iterations(vtkSmoothPolyDataFilter& filter, int iterations) {
    filter.SetNumberOfIterations(iterations);
}

int smooth_poly_data_filter_get_number_of_iterations(vtkSmoothPolyDataFilter& filter) {
    return filter.GetNumberOfIterations();
}

void smooth_poly_data_filter_set_relaxation_factor(vtkSmoothPolyDataFilter& filter, double factor) {
    filter.SetRelaxationFactor(factor);
}

double smooth_poly_data_filter_get_relaxation_factor(vtkSmoothPolyDataFilter& filter) {
    return filter.GetRelaxationFactor();
}

void smooth_poly_data_filter_set_feature_edge_smoothing(vtkSmoothPolyDataFilter& filter, bool on) {
    filter.SetFeatureEdgeSmoothing(on);
}

void smooth_poly_data_filter_set_boundary_smoothing(vtkSmoothPolyDataFilter& filter, bool on) {
    filter.SetBoundarySmoothing(on);
}

vtkAlgorithmOutput* smooth_poly_data_filter_get_output_port(vtkSmoothPolyDataFilter& filter) {
    return filter.GetOutputPort();
}
