#include "vtk_plane.h"
#include "vtk_plane.rs.h"

vtkPlane* vtk_plane_new() {
    return vtkPlane::New();
}

void vtk_plane_delete(vtkPlane& plane) {
    plane.Delete();
}

void plane_set_origin(vtkPlane& plane, double x, double y, double z) {
    plane.SetOrigin(x, y, z);
}

void plane_get_origin(const vtkPlane& plane, double* x, double* y, double* z) {
    double origin[3];
    const_cast<vtkPlane&>(plane).GetOrigin(origin);
    *x = origin[0];
    *y = origin[1];
    *z = origin[2];
}

void plane_set_normal(vtkPlane& plane, double nx, double ny, double nz) {
    plane.SetNormal(nx, ny, nz);
}

void plane_get_normal(const vtkPlane& plane, double* nx, double* ny, double* nz) {
    double normal[3];
    const_cast<vtkPlane&>(plane).GetNormal(normal);
    *nx = normal[0];
    *ny = normal[1];
    *nz = normal[2];
}
