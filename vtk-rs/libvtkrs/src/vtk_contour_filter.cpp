#include "vtk_contour_filter.h"
#include "vtk_contour_filter.rs.h"

vtkContourFilter* vtk_contour_filter_new() {
    return vtkContourFilter::New();
}

void vtk_contour_filter_delete(vtkContourFilter& filter) {
    filter.Delete();
}

void contour_filter_set_input_connection(vtkContourFilter& filter, vtkAlgorithmOutput* output) {
    filter.SetInputConnection(output);
}

void contour_filter_set_input_data(vtkContourFilter& filter, vtkDataSet* data_set) {
    filter.SetInputData(data_set);
}

void contour_filter_set_value(vtkContourFilter& filter, int i, double value) {
    filter.SetValue(i, value);
}

void contour_filter_generate_values(vtkContourFilter& filter, int num_contours, double range_min, double range_max) {
    filter.GenerateValues(num_contours, range_min, range_max);
}

vtkAlgorithmOutput* contour_filter_get_output_port(vtkContourFilter& filter) {
    return filter.GetOutputPort();
}
