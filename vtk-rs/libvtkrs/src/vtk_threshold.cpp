#include "vtk_threshold.h"
#include "vtk_threshold.rs.h"

vtkThreshold* vtk_threshold_new() {
    return vtkThreshold::New();
}

void vtk_threshold_delete(vtkThreshold& thr) {
    thr.Delete();
}

void threshold_set_input_connection(vtkThreshold& thr, vtkAlgorithmOutput* output) {
    thr.SetInputConnection(output);
}

void threshold_set_input_data(vtkThreshold& thr, vtkDataSet* data_set) {
    thr.SetInputData(data_set);
}

void threshold_set_lower_threshold(vtkThreshold& thr, double value) {
    thr.SetLowerThreshold(value);
}

void threshold_set_upper_threshold(vtkThreshold& thr, double value) {
    thr.SetUpperThreshold(value);
}

void threshold_set_threshold_between(vtkThreshold& thr, double lower, double upper) {
    thr.SetLowerThreshold(lower);
    thr.SetUpperThreshold(upper);
}

void threshold_set_component_mode(vtkThreshold& thr, int mode) {
    thr.SetComponentMode(mode);
}

void threshold_set_selected_component(vtkThreshold& thr, int comp) {
    thr.SetSelectedComponent(comp);
}

vtkAlgorithmOutput* threshold_get_output_port(vtkThreshold& thr) {
    return thr.GetOutputPort();
}
