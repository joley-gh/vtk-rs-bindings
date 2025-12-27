#pragma once
#ifndef VTK_SMOOTH_POLY_DATA_FILTER_H
#define VTK_SMOOTH_POLY_DATA_FILTER_H

#include <vtkSmoothPolyDataFilter.h>
#include <vtkAlgorithmOutput.h>
#include <vtkPolyData.h>

// Create/Delete
vtkSmoothPolyDataFilter* vtk_smooth_poly_data_filter_new();
void vtk_smooth_poly_data_filter_delete(vtkSmoothPolyDataFilter& filter);

// Input
void smooth_poly_data_filter_set_input_connection(vtkSmoothPolyDataFilter& filter, vtkAlgorithmOutput* output);

// Parameters
void smooth_poly_data_filter_set_number_of_iterations(vtkSmoothPolyDataFilter& filter, int iterations);
int smooth_poly_data_filter_get_number_of_iterations(vtkSmoothPolyDataFilter& filter);

void smooth_poly_data_filter_set_relaxation_factor(vtkSmoothPolyDataFilter& filter, double factor);
double smooth_poly_data_filter_get_relaxation_factor(vtkSmoothPolyDataFilter& filter);

void smooth_poly_data_filter_set_feature_edge_smoothing(vtkSmoothPolyDataFilter& filter, bool on);
void smooth_poly_data_filter_set_boundary_smoothing(vtkSmoothPolyDataFilter& filter, bool on);

// Output
vtkAlgorithmOutput* smooth_poly_data_filter_get_output_port(vtkSmoothPolyDataFilter& filter);

#endif // VTK_SMOOTH_POLY_DATA_FILTER_H
