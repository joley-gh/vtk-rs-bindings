#pragma once
#ifndef VTK_SHRINK_FILTER_H
#define VTK_SHRINK_FILTER_H

#include <vtkShrinkFilter.h>
#include <vtkAlgorithmOutput.h>

// Create/Delete
vtkShrinkFilter* vtk_shrink_filter_new();
void vtk_shrink_filter_delete(vtkShrinkFilter& f);

// Input
void shrink_set_input_connection(vtkShrinkFilter& f, vtkAlgorithmOutput* output);

// Control
void shrink_set_shrink_factor(vtkShrinkFilter& f, double factor);
double shrink_get_shrink_factor(vtkShrinkFilter& f);

// Output
vtkAlgorithmOutput* shrink_get_output_port(vtkShrinkFilter& f);

#endif // VTK_SHRINK_FILTER_H
