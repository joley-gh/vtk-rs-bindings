#include "vtk_warp_vector.h"
#include "vtk_warp_vector.rs.h"
#include <vtkAlgorithmOutput.h>
#include <vtkDataSet.h>

extern "C" {
    vtkWarpVector* vtk_warp_vector_new() {
        return vtkWarpVector::New();
    }
    
    void vtk_warp_vector_delete(vtkWarpVector& warp) {
        warp.Delete();
    }
    
    void warp_vector_set_input_connection(vtkWarpVector& warp, vtkAlgorithmOutput* port) {
        if (port) {
            warp.SetInputConnection(port);
        }
    }
    
    void warp_vector_set_input_data(vtkWarpVector& warp, vtkDataSet* dataset) {
        if (dataset) {
            warp.SetInputData(dataset);
        }
    }
    
    void warp_vector_set_scale_factor(vtkWarpVector& warp, double scale) {
        warp.SetScaleFactor(scale);
    }
    
    double warp_vector_get_scale_factor(vtkWarpVector& warp) {
        return warp.GetScaleFactor();
    }
    
    vtkAlgorithmOutput* warp_vector_get_output_port(vtkWarpVector& warp) {
        return warp.GetOutputPort();
    }
}
