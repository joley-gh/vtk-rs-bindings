#include "cxx.h"
#include "vtk_renderer.h"
#include "vtk_renderer.rs.h"

#include <vtkRenderer.h>
#include <vtkActor.h>
#include <vtkActor2D.h>
#include <vtkCamera.h>

vtkRenderer* renderer_new() {
    vtkRenderer* obj = vtkRenderer::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkRenderer");
    }
    return obj;
}

void renderer_delete(vtkRenderer& renderer) {
    renderer.Delete();
}

void renderer_add_actor(vtkRenderer& renderer, vtkActor* actor) {
    renderer.AddActor(actor);
}

void renderer_add_actor2d(vtkRenderer& renderer, vtkActor2D* actor) {
    renderer.AddActor2D(actor);
}

void renderer_set_background(vtkRenderer& renderer, double r, double g, double b) {
    renderer.SetBackground(r, g, b);
}

vtkCamera* renderer_get_active_camera(vtkRenderer& renderer) {
    return renderer.GetActiveCamera();
}

void renderer_reset_camera(vtkRenderer& renderer) {
    renderer.ResetCamera();
}

// Coordinate conversion
void renderer_set_display_point(vtkRenderer& renderer, double x, double y, double z) {
    renderer.SetDisplayPoint(x, y, z);
}

void renderer_display_to_world(vtkRenderer& renderer) {
    renderer.DisplayToWorld();
}

void renderer_get_world_point(vtkRenderer& renderer, double& x, double& y, double& z, double& w) {
    double* point = renderer.GetWorldPoint();
    x = point[0];
    y = point[1];
    z = point[2];
    w = point[3];
}

void renderer_set_world_point(vtkRenderer& renderer, double x, double y, double z, double w) {
    renderer.SetWorldPoint(x, y, z, w);
}

void renderer_world_to_display(vtkRenderer& renderer) {
    renderer.WorldToDisplay();
}

void renderer_get_display_point(vtkRenderer& renderer, double& x, double& y, double& z) {
    double* point = renderer.GetDisplayPoint();
    x = point[0];
    y = point[1];
    z = point[2];
}
