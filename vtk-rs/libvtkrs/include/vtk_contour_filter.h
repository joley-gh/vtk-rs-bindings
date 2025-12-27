#pragma once
#ifndef VTK_CONTOUR_FILTER_H
#define VTK_CONTOUR_FILTER_H

#include <vtkContourFilter.h>
#include <vtkDataSet.h>

// Create/Delete
vtkContourFilter* vtk_contour_filter_new();
void vtk_contour_filter_delete(vtkContourFilter& filter);

// Input
void contour_filter_set_input_connection(vtkContourFilter& filter, vtkAlgorithmOutput* output);
void contour_filter_set_input_data(vtkContourFilter& filter, vtkDataSet* data_set);

// Contour values
void contour_filter_set_value(vtkContourFilter& filter, int i, double value);
void contour_filter_generate_values(vtkContourFilter& filter, int num_contours, double range_min, double range_max);

// Output
vtkAlgorithmOutput* contour_filter_get_output_port(vtkContourFilter& filter);

#endif // VTK_CONTOUR_FILTER_H
