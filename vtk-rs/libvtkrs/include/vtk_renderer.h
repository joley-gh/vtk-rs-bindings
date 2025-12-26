#pragma once

#include <vtkRenderer.h>
#include <vtkActor.h>
#include <vtkActor2D.h>
#include <vtkCamera.h>

vtkRenderer* renderer_new();
void renderer_delete(vtkRenderer& renderer);
void renderer_add_actor(vtkRenderer& renderer, vtkActor* actor);
void renderer_add_actor2d(vtkRenderer& renderer, vtkActor2D* actor);
void renderer_set_background(vtkRenderer& renderer, double r, double g, double b);
vtkCamera* renderer_get_active_camera(vtkRenderer& renderer);
void renderer_reset_camera(vtkRenderer& renderer);

// Coordinate conversion
void renderer_set_display_point(vtkRenderer& renderer, double x, double y, double z);
void renderer_display_to_world(vtkRenderer& renderer);
void renderer_get_world_point(vtkRenderer& renderer, double& x, double& y, double& z, double& w);
void renderer_set_world_point(vtkRenderer& renderer, double x, double y, double z, double w);
void renderer_world_to_display(vtkRenderer& renderer);
void renderer_get_display_point(vtkRenderer& renderer, double& x, double& y, double& z);
