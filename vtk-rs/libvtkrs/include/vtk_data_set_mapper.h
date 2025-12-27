#pragma once
#ifndef VTK_DATA_SET_MAPPER_H
#define VTK_DATA_SET_MAPPER_H

#include <vtkDataSetMapper.h>
#include <vtkDataSet.h>
#include <vtkAlgorithmOutput.h>

// Create/Delete
vtkDataSetMapper* vtk_data_set_mapper_new();
void vtk_data_set_mapper_delete(vtkDataSetMapper& mapper);

// Input connection
void data_set_mapper_set_input_connection(vtkDataSetMapper& mapper, vtkAlgorithmOutput* output);
void data_set_mapper_set_input_data(vtkDataSetMapper& mapper, vtkDataSet* data_set);

#endif // VTK_DATA_SET_MAPPER_H
