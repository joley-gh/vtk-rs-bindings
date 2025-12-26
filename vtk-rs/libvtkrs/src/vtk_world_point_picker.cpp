#include "vtk_world_point_picker.h"
#include "vtk_world_point_picker.rs.h"

vtkWorldPointPicker* vtk_world_point_picker_new() {
    return vtkWorldPointPicker::New();
}

void vtk_world_point_picker_delete(vtkWorldPointPicker& picker) {
    picker.Delete();
}

int vtk_world_point_picker_pick(vtkWorldPointPicker& picker, double x, double y, double z, vtkRenderer* renderer) {
    // Note: vtkWorldPointPicker::Pick() always returns 0, but still sets the position correctly
    return picker.Pick(x, y, z, renderer);
}

void vtk_world_point_picker_get_pick_position(const vtkWorldPointPicker& picker, double& x, double& y, double& z) {
    double* pos = const_cast<vtkWorldPointPicker&>(picker).GetPickPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}
