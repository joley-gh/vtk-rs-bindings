#pragma once

#include <vtkNew.h>
#include <vtkWorldPointPicker.h>
#include <vtkRenderer.h>

vtkWorldPointPicker* vtk_world_point_picker_new();
void vtk_world_point_picker_delete(vtkWorldPointPicker& picker);

int vtk_world_point_picker_pick(vtkWorldPointPicker& picker, double x, double y, double z, vtkRenderer* renderer);
void vtk_world_point_picker_get_pick_position(const vtkWorldPointPicker& picker, double& x, double& y, double& z);
