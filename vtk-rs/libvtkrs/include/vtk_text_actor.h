#pragma once

#include <vtkTextActor.h>
#include <vtkTextProperty.h>
#include <cxx.h>

// vtkTextActor - 2D text overlay in screen space
vtkTextActor* text_actor_new();
void text_actor_delete(vtkTextActor& actor);

void text_actor_set_input(vtkTextActor& actor, rust::String text);
rust::String text_actor_get_input(vtkTextActor& actor);

void text_actor_set_position(vtkTextActor& actor, double x, double y);
void text_actor_get_position(vtkTextActor& actor, double& x, double& y);

void text_actor_set_position2(vtkTextActor& actor, double x, double y);
void text_actor_get_position2(vtkTextActor& actor, double& x, double& y);

vtkTextProperty* text_actor_get_text_property(vtkTextActor& actor);

void text_actor_set_visibility(vtkTextActor& actor, bool visible);
bool text_actor_get_visibility(vtkTextActor& actor);

// vtkTextProperty - Font and color properties
void text_property_set_font_size(vtkTextProperty& prop, int size);
int text_property_get_font_size(vtkTextProperty& prop);

void text_property_set_color(vtkTextProperty& prop, double r, double g, double b);
void text_property_get_color(vtkTextProperty& prop, double& r, double& g, double& b);

void text_property_set_opacity(vtkTextProperty& prop, double opacity);
double text_property_get_opacity(vtkTextProperty& prop);

void text_property_set_bold(vtkTextProperty& prop, bool bold);
bool text_property_get_bold(vtkTextProperty& prop);

void text_property_set_italic(vtkTextProperty& prop, bool italic);
bool text_property_get_italic(vtkTextProperty& prop);

void text_property_set_font_family_to_arial(vtkTextProperty& prop);
void text_property_set_font_family_to_courier(vtkTextProperty& prop);
void text_property_set_font_family_to_times(vtkTextProperty& prop);

// Text alignment (justification)
void text_property_set_justification_to_left(vtkTextProperty& prop);
void text_property_set_justification_to_centered(vtkTextProperty& prop);
void text_property_set_justification_to_right(vtkTextProperty& prop);

void text_property_set_vertical_justification_to_bottom(vtkTextProperty& prop);
void text_property_set_vertical_justification_to_centered(vtkTextProperty& prop);
void text_property_set_vertical_justification_to_top(vtkTextProperty& prop);

