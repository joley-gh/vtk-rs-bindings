#include "vtk_text_actor.h"
#include "vtk_text_actor.rs.h"

// vtkTextActor implementation
vtkTextActor* text_actor_new() {
    return vtkTextActor::New();
}

void text_actor_delete(vtkTextActor& actor) {
    actor.Delete();
}

void text_actor_set_input(vtkTextActor& actor, rust::String text) {
    std::string cpp_text(text.begin(), text.end());
    actor.SetInput(cpp_text.c_str());
}

rust::String text_actor_get_input(vtkTextActor& actor) {
    const char* text = actor.GetInput();
    return rust::String(text ? text : "");
}

void text_actor_set_position(vtkTextActor& actor, double x, double y) {
    actor.SetPosition(x, y);
}

void text_actor_get_position(vtkTextActor& actor, double& x, double& y) {
    double* pos = actor.GetPosition();
    x = pos[0];
    y = pos[1];
}

void text_actor_set_position2(vtkTextActor& actor, double x, double y) {
    actor.SetPosition2(x, y);
}

void text_actor_get_position2(vtkTextActor& actor, double& x, double& y) {
    double* pos = actor.GetPosition2();
    x = pos[0];
    y = pos[1];
}

vtkTextProperty* text_actor_get_text_property(vtkTextActor& actor) {
    return actor.GetTextProperty();
}

void text_actor_set_visibility(vtkTextActor& actor, bool visible) {
    actor.SetVisibility(visible ? 1 : 0);
}

bool text_actor_get_visibility(vtkTextActor& actor) {
    return actor.GetVisibility() != 0;
}

// vtkTextProperty implementation
void text_property_set_font_size(vtkTextProperty& prop, int size) {
    prop.SetFontSize(size);
}

int text_property_get_font_size(vtkTextProperty& prop) {
    return prop.GetFontSize();
}

void text_property_set_color(vtkTextProperty& prop, double r, double g, double b) {
    prop.SetColor(r, g, b);
}

void text_property_get_color(vtkTextProperty& prop, double& r, double& g, double& b) {
    double* color = prop.GetColor();
    r = color[0];
    g = color[1];
    b = color[2];
}

void text_property_set_opacity(vtkTextProperty& prop, double opacity) {
    prop.SetOpacity(opacity);
}

double text_property_get_opacity(vtkTextProperty& prop) {
    return prop.GetOpacity();
}

void text_property_set_bold(vtkTextProperty& prop, bool bold) {
    prop.SetBold(bold ? 1 : 0);
}

bool text_property_get_bold(vtkTextProperty& prop) {
    return prop.GetBold() != 0;
}

void text_property_set_italic(vtkTextProperty& prop, bool italic) {
    prop.SetItalic(italic ? 1 : 0);
}

bool text_property_get_italic(vtkTextProperty& prop) {
    return prop.GetItalic() != 0;
}

void text_property_set_font_family_to_arial(vtkTextProperty& prop) {
    prop.SetFontFamilyToArial();
}

void text_property_set_font_family_to_courier(vtkTextProperty& prop) {
    prop.SetFontFamilyToCourier();
}

void text_property_set_font_family_to_times(vtkTextProperty& prop) {
    prop.SetFontFamilyToTimes();
}

// Text alignment (justification)
void text_property_set_justification_to_left(vtkTextProperty& prop) {
    prop.SetJustificationToLeft();
}

void text_property_set_justification_to_centered(vtkTextProperty& prop) {
    prop.SetJustificationToCentered();
}

void text_property_set_justification_to_right(vtkTextProperty& prop) {
    prop.SetJustificationToRight();
}

void text_property_set_vertical_justification_to_bottom(vtkTextProperty& prop) {
    prop.SetVerticalJustificationToBottom();
}

void text_property_set_vertical_justification_to_centered(vtkTextProperty& prop) {
    prop.SetVerticalJustificationToCentered();
}

void text_property_set_vertical_justification_to_top(vtkTextProperty& prop) {
    prop.SetVerticalJustificationToTop();
}
