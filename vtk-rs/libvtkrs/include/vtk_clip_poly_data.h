#pragma once
#ifndef VTK_CLIP_POLY_DATA_H
#define VTK_CLIP_POLY_DATA_H

#include <vtkClipPolyData.h>
#include <vtkPlane.h>

// Create/Delete
vtkClipPolyData* vtk_clip_poly_data_new();
void vtk_clip_poly_data_delete(vtkClipPolyData& clipper);

// Input
void clip_poly_data_set_input_connection(vtkClipPolyData& clipper, vtkAlgorithmOutput* output);

// Clip function (implicit function)
void clip_poly_data_set_clip_function(vtkClipPolyData& clipper, vtkPlane* plane);

// Clip value
void clip_poly_data_set_value(vtkClipPolyData& clipper, double value);

// Output
vtkAlgorithmOutput* clip_poly_data_get_output_port(vtkClipPolyData& clipper);

#endif // VTK_CLIP_POLY_DATA_H
