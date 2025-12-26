#include "vtk_lookup_table.h"
#include "vtk_lookup_table.rs.h"
#include <stdexcept>

// Lifecycle
vtkLookupTable* lookup_table_new() {
    vtkLookupTable* lut = vtkLookupTable::New();
    if (!lut) {
        throw std::runtime_error("Failed to create vtkLookupTable");
    }
    return lut;
}

void lookup_table_delete(vtkLookupTable& lut) {
    lut.Delete();
}

// Range configuration
void lookup_table_set_range(vtkLookupTable& lut, double min, double max) {
    lut.SetRange(min, max);
}

void lookup_table_get_range(vtkLookupTable& lut, double& min, double& max) {
    double* range = lut.GetRange();
    min = range[0];
    max = range[1];
}

// Color range (hue in HSV color space)
void lookup_table_set_hue_range(vtkLookupTable& lut, double min, double max) {
    lut.SetHueRange(min, max);
}

void lookup_table_get_hue_range(vtkLookupTable& lut, double& min, double& max) {
    double* range = lut.GetHueRange();
    min = range[0];
    max = range[1];
}

// Saturation range
void lookup_table_set_saturation_range(vtkLookupTable& lut, double min, double max) {
    lut.SetSaturationRange(min, max);
}

void lookup_table_get_saturation_range(vtkLookupTable& lut, double& min, double& max) {
    double* range = lut.GetSaturationRange();
    min = range[0];
    max = range[1];
}

// Value range (brightness in HSV)
void lookup_table_set_value_range(vtkLookupTable& lut, double min, double max) {
    lut.SetValueRange(min, max);
}

void lookup_table_get_value_range(vtkLookupTable& lut, double& min, double& max) {
    double* range = lut.GetValueRange();
    min = range[0];
    max = range[1];
}

// Alpha range (opacity)
void lookup_table_set_alpha_range(vtkLookupTable& lut, double min, double max) {
    lut.SetAlphaRange(min, max);
}

void lookup_table_get_alpha_range(vtkLookupTable& lut, double& min, double& max) {
    double* range = lut.GetAlphaRange();
    min = range[0];
    max = range[1];
}

// Table configuration
void lookup_table_set_number_of_table_values(vtkLookupTable& lut, int64_t n) {
    lut.SetNumberOfTableValues(static_cast<vtkIdType>(n));
}

int64_t lookup_table_get_number_of_table_values(vtkLookupTable& lut) {
    return static_cast<int64_t>(lut.GetNumberOfTableValues());
}

// Build the table
void lookup_table_build(vtkLookupTable& lut) {
    lut.Build();
}

// Get color for a value
void lookup_table_get_color(vtkLookupTable& lut, double value, double& r, double& g, double& b) {
    double rgb[3];
    lut.GetColor(value, rgb);
    r = rgb[0];
    g = rgb[1];
    b = rgb[2];
}
