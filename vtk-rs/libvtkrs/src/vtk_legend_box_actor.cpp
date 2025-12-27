#include "vtk_legend_box_actor.h"
#include "vtk_legend_box_actor.rs.h"
#include <stdexcept>
#include <string>

// Lifecycle
vtkLegendBoxActor* legend_box_actor_new() {
    vtkLegendBoxActor* actor = vtkLegendBoxActor::New();
    if (!actor) {
        throw std::runtime_error("Failed to create vtkLegendBoxActor");
    }
    return actor;
}

void legend_box_actor_delete(vtkLegendBoxActor& actor) {
    actor.Delete();
}

// Entries
void legend_box_actor_set_number_of_entries(vtkLegendBoxActor& actor, int n) {
    actor.SetNumberOfEntries(n);
}

int legend_box_actor_get_number_of_entries(vtkLegendBoxActor& actor) {
    return actor.GetNumberOfEntries();
}

void legend_box_actor_set_entry_string(vtkLegendBoxActor& actor, int index, rust::Str label) {
    actor.SetEntryString(index, std::string(label).c_str());
}

void legend_box_actor_set_entry_color(vtkLegendBoxActor& actor, int index, double r, double g, double b) {
    actor.SetEntryColor(index, r, g, b);
}

// Position and size (normalized viewport coordinates [0,1])
void legend_box_actor_set_position(vtkLegendBoxActor& actor, double x, double y) {
    actor.SetPosition(x, y);
}

void legend_box_actor_get_position(vtkLegendBoxActor& actor, double& x, double& y) {
    double* pos = actor.GetPosition();
    x = pos[0];
    y = pos[1];
}

void legend_box_actor_set_position2(vtkLegendBoxActor& actor, double x, double y) {
    actor.SetPosition2(x, y);
}

void legend_box_actor_get_position2(vtkLegendBoxActor& actor, double& x, double& y) {
    double* pos = actor.GetPosition2();
    x = pos[0];
    y = pos[1];
}

// Border and background
void legend_box_actor_set_border(vtkLegendBoxActor& actor, bool border) {
    actor.SetBorder(border ? 1 : 0);
}

bool legend_box_actor_get_border(vtkLegendBoxActor& actor) {
    return actor.GetBorder() != 0;
}

void legend_box_actor_set_box(vtkLegendBoxActor& actor, bool box) {
    actor.SetBox(box ? 1 : 0);
}

bool legend_box_actor_get_box(vtkLegendBoxActor& actor) {
    return actor.GetBox() != 0;
}

// Padding
void legend_box_actor_set_padding(vtkLegendBoxActor& actor, int padding) {
    actor.SetPadding(padding);
}

int legend_box_actor_get_padding(vtkLegendBoxActor& actor) {
    return actor.GetPadding();
}

// Visibility
void legend_box_actor_set_visibility(vtkLegendBoxActor& actor, bool visible) {
    actor.SetVisibility(visible ? 1 : 0);
}

bool legend_box_actor_get_visibility(vtkLegendBoxActor& actor) {
    return actor.GetVisibility() != 0;
}
