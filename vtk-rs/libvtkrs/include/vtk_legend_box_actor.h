#pragma once
#include "cxx.h"

#include <vtkLegendBoxActor.h>
#include <vtkActor.h>
#include <vtkNew.h>

// Lifecycle
vtkLegendBoxActor* legend_box_actor_new();
void legend_box_actor_delete(vtkLegendBoxActor& actor);

// Entries
void legend_box_actor_set_number_of_entries(vtkLegendBoxActor& actor, int n);
int legend_box_actor_get_number_of_entries(vtkLegendBoxActor& actor);

void legend_box_actor_set_entry_string(vtkLegendBoxActor& actor, int index, rust::Str label);
void legend_box_actor_set_entry_color(vtkLegendBoxActor& actor, int index, double r, double g, double b);

// Position and size (normalized viewport coordinates [0,1])
void legend_box_actor_set_position(vtkLegendBoxActor& actor, double x, double y);
void legend_box_actor_get_position(vtkLegendBoxActor& actor, double& x, double& y);

void legend_box_actor_set_position2(vtkLegendBoxActor& actor, double x, double y);
void legend_box_actor_get_position2(vtkLegendBoxActor& actor, double& x, double& y);

// Border and background
void legend_box_actor_set_border(vtkLegendBoxActor& actor, bool border);
bool legend_box_actor_get_border(vtkLegendBoxActor& actor);

void legend_box_actor_set_box(vtkLegendBoxActor& actor, bool box);
bool legend_box_actor_get_box(vtkLegendBoxActor& actor);

// Padding
void legend_box_actor_set_padding(vtkLegendBoxActor& actor, int padding);
int legend_box_actor_get_padding(vtkLegendBoxActor& actor);

// Visibility
void legend_box_actor_set_visibility(vtkLegendBoxActor& actor, bool visible);
bool legend_box_actor_get_visibility(vtkLegendBoxActor& actor);
