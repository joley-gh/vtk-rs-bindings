#include "vtk_prop3d_collection.h"
#include <vtk_prop3d_collection.rs.h>
#include "cxx.h"

int vtk_prop3d_collection_get_size(vtkProp3DCollection* col) {
    if (!col) return 0;
    return col->GetNumberOfItems();
}

vtkProp3D* vtk_prop3d_collection_get_item(vtkProp3DCollection* col, int index) {
    if (!col || index < 0) return nullptr;
    vtkCollectionSimpleIterator it;
    col->InitTraversal(it);
    int i = 0;
    vtkProp3D* prop = nullptr;
    while ((prop = col->GetNextProp3D(it))) {
        if (i == index) {
            return prop;
        }
        ++i;
    }
    return nullptr;
}

vtkActor* vtk_prop3d_to_actor(vtkProp3D* prop) {
    if (!prop) return nullptr;
    return vtkActor::SafeDownCast(prop);
}
