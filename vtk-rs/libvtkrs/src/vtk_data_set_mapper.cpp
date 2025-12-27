#include "vtk_data_set_mapper.h"
#include "vtk_data_set_mapper.rs.h"

vtkDataSetMapper* vtk_data_set_mapper_new() {
    return vtkDataSetMapper::New();
}

void vtk_data_set_mapper_delete(vtkDataSetMapper& mapper) {
    mapper.Delete();
}

void data_set_mapper_set_input_connection(vtkDataSetMapper& mapper, vtkAlgorithmOutput* output) {
    if (output) {
        mapper.SetInputConnection(output);
    }
}

void data_set_mapper_set_input_data(vtkDataSetMapper& mapper, vtkDataSet* data_set) {
    mapper.SetInputData(data_set);
}
