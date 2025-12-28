#include "vtk_shrink_filter.h"
#include "vtk_shrink_filter.rs.h"

vtkShrinkFilter* vtk_shrink_filter_new() {
    return vtkShrinkFilter::New();
}

void vtk_shrink_filter_delete(vtkShrinkFilter& f) {
    f.Delete();
}

void shrink_set_input_connection(vtkShrinkFilter& f, vtkAlgorithmOutput* output) {
    f.SetInputConnection(output);
}

void shrink_set_shrink_factor(vtkShrinkFilter& f, double factor) {
    f.SetShrinkFactor(factor);
}

double shrink_get_shrink_factor(vtkShrinkFilter& f) {
    return f.GetShrinkFactor();
}

vtkAlgorithmOutput* shrink_get_output_port(vtkShrinkFilter& f) {
    return f.GetOutputPort();
}
