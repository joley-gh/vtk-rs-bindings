#pragma once
#include "cxx.h"

#include <vtkScalarBarActor.h>
#include <vtkLookupTable.h>
#include <vtkTextProperty.h>
#include <vtkNew.h>

// Lifecycle
vtkScalarBarActor* scalar_bar_actor_new();
void scalar_bar_actor_delete(vtkScalarBarActor& actor);

// Lookup table
void scalar_bar_actor_set_lookup_table(vtkScalarBarActor& actor, vtkLookupTable& lut);

// Title
void scalar_bar_actor_set_title(vtkScalarBarActor& actor, rust::Str title);
rust::String scalar_bar_actor_get_title(vtkScalarBarActor& actor);

// Labels
void scalar_bar_actor_set_number_of_labels(vtkScalarBarActor& actor, int n);
int scalar_bar_actor_get_number_of_labels(vtkScalarBarActor& actor);

// Position and size (normalized viewport coordinates [0,1])
void scalar_bar_actor_set_position(vtkScalarBarActor& actor, double x, double y);
void scalar_bar_actor_get_position(vtkScalarBarActor& actor, double& x, double& y);

void scalar_bar_actor_set_width(vtkScalarBarActor& actor, double width);
double scalar_bar_actor_get_width(vtkScalarBarActor& actor);

void scalar_bar_actor_set_height(vtkScalarBarActor& actor, double height);
double scalar_bar_actor_get_height(vtkScalarBarActor& actor);

// Text properties (non-owning references)
vtkTextProperty* scalar_bar_actor_get_label_text_property(vtkScalarBarActor& actor);
vtkTextProperty* scalar_bar_actor_get_title_text_property(vtkScalarBarActor& actor);

// Orientation
void scalar_bar_actor_set_orientation(vtkScalarBarActor& actor, int orientation);
int scalar_bar_actor_get_orientation(vtkScalarBarActor& actor);

// Visibility
void scalar_bar_actor_set_visibility(vtkScalarBarActor& actor, bool visible);
bool scalar_bar_actor_get_visibility(vtkScalarBarActor& actor);
