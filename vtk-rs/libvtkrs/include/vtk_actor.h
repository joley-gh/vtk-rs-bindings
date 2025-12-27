#pragma once

#include <vtkActor.h>
#include <vtkMapper.h>
#include <vtkProperty.h>

vtkActor* actor_new();
void actor_delete(vtkActor& actor);
void actor_set_mapper(vtkActor& actor, vtkMapper* mapper);
vtkProperty* actor_get_property(vtkActor& actor);

// Position methods
void actor_set_position(vtkActor& actor, double x, double y, double z);
void actor_get_position(const vtkActor& actor, double& x, double& y, double& z);
void actor_add_position(vtkActor& actor, double x, double y, double z);

// Rotation methods
void actor_rotate_x(vtkActor& actor, double angle);
void actor_rotate_y(vtkActor& actor, double angle);
void actor_rotate_z(vtkActor& actor, double angle);
void actor_set_orientation(vtkActor& actor, double x, double y, double z);
void actor_get_orientation(const vtkActor& actor, double& x, double& y, double& z);

// Scale methods
void actor_set_scale(vtkActor& actor, double x, double y, double z);
void actor_get_scale(const vtkActor& actor, double& x, double& y, double& z);

// Visibility methods
void actor_set_visibility(vtkActor& actor, bool visible);
bool actor_get_visibility(const vtkActor& actor);

// Pickability methods
void actor_set_pickable(vtkActor& actor, bool pickable);
bool actor_get_pickable(const vtkActor& actor);
