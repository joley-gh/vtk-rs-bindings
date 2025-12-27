#pragma once
#ifndef VTK_PLANE_H
#define VTK_PLANE_H

#include <vtkPlane.h>

// Create/Delete
vtkPlane* vtk_plane_new();
void vtk_plane_delete(vtkPlane& plane);

// Origin
void plane_set_origin(vtkPlane& plane, double x, double y, double z);
void plane_get_origin(const vtkPlane& plane, double* x, double* y, double* z);

// Normal
void plane_set_normal(vtkPlane& plane, double nx, double ny, double nz);
void plane_get_normal(const vtkPlane& plane, double* nx, double* ny, double* nz);

#endif // VTK_PLANE_H
