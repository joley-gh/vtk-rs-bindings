#pragma once
#ifndef VTK_THRESHOLD_H
#define VTK_THRESHOLD_H

#include <vtkThreshold.h>
#include <vtkAlgorithmOutput.h>
#include <vtkDataSet.h>

// Create/Delete
vtkThreshold* vtk_threshold_new();
void vtk_threshold_delete(vtkThreshold& thr);

// Input
void threshold_set_input_connection(vtkThreshold& thr, vtkAlgorithmOutput* output);
void threshold_set_input_data(vtkThreshold& thr, vtkDataSet* data_set);

// Threshold control
void threshold_set_lower_threshold(vtkThreshold& thr, double value);
void threshold_set_upper_threshold(vtkThreshold& thr, double value);
void threshold_set_threshold_between(vtkThreshold& thr, double lower, double upper);

void threshold_set_component_mode(vtkThreshold& thr, int mode);
void threshold_set_selected_component(vtkThreshold& thr, int comp);

// Output
vtkAlgorithmOutput* threshold_get_output_port(vtkThreshold& thr);

#endif // VTK_THRESHOLD_H
