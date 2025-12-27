#include "cxx.h"
#include "vtk_actor.h"
#include "vtk_actor.rs.h"

#include <vtkActor.h>
#include <vtkMapper.h>

vtkActor* actor_new() {
    vtkActor* obj = vtkActor::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkActor");
    }
    return obj;
}

void actor_delete(vtkActor& actor) {
    actor.Delete();
}

void actor_set_mapper(vtkActor& actor, vtkMapper* mapper) {
    actor.SetMapper(mapper);
}

vtkProperty* actor_get_property(vtkActor& actor) {
    return actor.GetProperty();
}

// Position methods
void actor_set_position(vtkActor& actor, double x, double y, double z) {
    actor.SetPosition(x, y, z);
}

void actor_get_position(const vtkActor& actor, double& x, double& y, double& z) {
    double* pos = const_cast<vtkActor&>(actor).GetPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

void actor_add_position(vtkActor& actor, double x, double y, double z) {
    actor.AddPosition(x, y, z);
}

// Rotation methods
void actor_rotate_x(vtkActor& actor, double angle) {
    actor.RotateX(angle);
}

void actor_rotate_y(vtkActor& actor, double angle) {
    actor.RotateY(angle);
}

void actor_rotate_z(vtkActor& actor, double angle) {
    actor.RotateZ(angle);
}

void actor_set_orientation(vtkActor& actor, double x, double y, double z) {
    actor.SetOrientation(x, y, z);
}

void actor_get_orientation(const vtkActor& actor, double& x, double& y, double& z) {
    double* orient = const_cast<vtkActor&>(actor).GetOrientation();
    x = orient[0];
    y = orient[1];
    z = orient[2];
}

// Scale methods
void actor_set_scale(vtkActor& actor, double x, double y, double z) {
    actor.SetScale(x, y, z);
}

void actor_get_scale(const vtkActor& actor, double& x, double& y, double& z) {
    double* scale = const_cast<vtkActor&>(actor).GetScale();
    x = scale[0];
    y = scale[1];
    z = scale[2];
}

// Visibility methods
void actor_set_visibility(vtkActor& actor, bool visible) {
    actor.SetVisibility(visible);
}

bool actor_get_visibility(const vtkActor& actor) {
    return const_cast<vtkActor&>(actor).GetVisibility() != 0;
}

// Pickability methods
void actor_set_pickable(vtkActor& actor, bool pickable) {
    actor.SetPickable(pickable);
}

bool actor_get_pickable(const vtkActor& actor) {
    return const_cast<vtkActor&>(actor).GetPickable() != 0;
}
