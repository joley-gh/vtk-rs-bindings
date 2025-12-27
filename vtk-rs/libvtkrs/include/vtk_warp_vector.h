#ifndef VTK_WARP_VECTOR_H
#define VTK_WARP_VECTOR_H

#include <vtkWarpVector.h>
#include <vtkDataSet.h>
#include <vtkAlgorithmOutput.h>

extern "C" {
    vtkWarpVector* vtk_warp_vector_new();
    void vtk_warp_vector_delete(vtkWarpVector& warp);
    
    void warp_vector_set_input_connection(vtkWarpVector& warp, vtkAlgorithmOutput* port);
    void warp_vector_set_input_data(vtkWarpVector& warp, vtkDataSet* dataset);
    
    void warp_vector_set_scale_factor(vtkWarpVector& warp, double scale);
    double warp_vector_get_scale_factor(vtkWarpVector& warp);
    
    vtkAlgorithmOutput* warp_vector_get_output_port(vtkWarpVector& warp);
}

#endif // VTK_WARP_VECTOR_H
