// Thin C helpers for vtkProp3DCollection used by Rust bindings
#pragma once

#include <vtkProp3DCollection.h>
#include <vtkProp3D.h>
#include <vtkActor.h>

extern "C" {
    int vtk_prop3d_collection_get_size(vtkProp3DCollection* col);
    vtkProp3D* vtk_prop3d_collection_get_item(vtkProp3DCollection* col, int index);
    vtkActor* vtk_prop3d_to_actor(vtkProp3D* prop);
}
