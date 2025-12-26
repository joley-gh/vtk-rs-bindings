#include "vtk_scalar_bar_actor.h"
#include "vtk_scalar_bar_actor.rs.h"
#include <stdexcept>
#include <string>

// Lifecycle
vtkScalarBarActor* scalar_bar_actor_new() {
    vtkScalarBarActor* actor = vtkScalarBarActor::New();
    if (!actor) {
        throw std::runtime_error("Failed to create vtkScalarBarActor");
    }
    return actor;
}

void scalar_bar_actor_delete(vtkScalarBarActor& actor) {
    actor.Delete();
}

// Lookup table
void scalar_bar_actor_set_lookup_table(vtkScalarBarActor& actor, vtkLookupTable& lut) {
    actor.SetLookupTable(&lut);
}

// Title
void scalar_bar_actor_set_title(vtkScalarBarActor& actor, rust::Str title) {
    actor.SetTitle(std::string(title).c_str());
}

rust::String scalar_bar_actor_get_title(vtkScalarBarActor& actor) {
    return rust::String(actor.GetTitle());
}

// Labels
void scalar_bar_actor_set_number_of_labels(vtkScalarBarActor& actor, int n) {
    actor.SetNumberOfLabels(n);
}

int scalar_bar_actor_get_number_of_labels(vtkScalarBarActor& actor) {
    return actor.GetNumberOfLabels();
}

// Position and size (normalized viewport coordinates [0,1])
void scalar_bar_actor_set_position(vtkScalarBarActor& actor, double x, double y) {
    actor.SetPosition(x, y);
}

void scalar_bar_actor_get_position(vtkScalarBarActor& actor, double& x, double& y) {
    double* pos = actor.GetPosition();
    x = pos[0];
    y = pos[1];
}

void scalar_bar_actor_set_width(vtkScalarBarActor& actor, double width) {
    actor.SetWidth(width);
}

double scalar_bar_actor_get_width(vtkScalarBarActor& actor) {
    return actor.GetWidth();
}

void scalar_bar_actor_set_height(vtkScalarBarActor& actor, double height) {
    actor.SetHeight(height);
}

double scalar_bar_actor_get_height(vtkScalarBarActor& actor) {
    return actor.GetHeight();
}

// Text properties (non-owning references)
vtkTextProperty* scalar_bar_actor_get_label_text_property(vtkScalarBarActor& actor) {
    return actor.GetLabelTextProperty();
}

vtkTextProperty* scalar_bar_actor_get_title_text_property(vtkScalarBarActor& actor) {
    return actor.GetTitleTextProperty();
}

// Orientation (0 = vertical, 1 = horizontal)
void scalar_bar_actor_set_orientation(vtkScalarBarActor& actor, int orientation) {
    actor.SetOrientation(orientation);
}

int scalar_bar_actor_get_orientation(vtkScalarBarActor& actor) {
    return actor.GetOrientation();
}

// Visibility
void scalar_bar_actor_set_visibility(vtkScalarBarActor& actor, bool visible) {
    actor.SetVisibility(visible ? 1 : 0);
}

bool scalar_bar_actor_get_visibility(vtkScalarBarActor& actor) {
    return actor.GetVisibility() != 0;
}
