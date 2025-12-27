#include "vtk_clip_poly_data.h"
#include "vtk_clip_poly_data.rs.h"

vtkClipPolyData* vtk_clip_poly_data_new() {
    return vtkClipPolyData::New();
}

void vtk_clip_poly_data_delete(vtkClipPolyData& clipper) {
    clipper.Delete();
}

void clip_poly_data_set_input_connection(vtkClipPolyData& clipper, vtkAlgorithmOutput* output) {
    clipper.SetInputConnection(output);
}

void clip_poly_data_set_clip_function(vtkClipPolyData& clipper, vtkPlane* plane) {
    clipper.SetClipFunction(plane);
}

void clip_poly_data_set_value(vtkClipPolyData& clipper, double value) {
    clipper.SetValue(value);
}

vtkAlgorithmOutput* clip_poly_data_get_output_port(vtkClipPolyData& clipper) {
    return clipper.GetOutputPort();
}
