#pragma once
#include "cxx.h"

#include <vtkLookupTable.h>
#include <vtkNew.h>

// Lifecycle
vtkLookupTable* lookup_table_new();
void lookup_table_delete(vtkLookupTable& lut);

// Range configuration
void lookup_table_set_range(vtkLookupTable& lut, double min, double max);
void lookup_table_get_range(vtkLookupTable& lut, double& min, double& max);

// Color range (hue in HSV color space)
void lookup_table_set_hue_range(vtkLookupTable& lut, double min, double max);
void lookup_table_get_hue_range(vtkLookupTable& lut, double& min, double& max);

// Saturation range
void lookup_table_set_saturation_range(vtkLookupTable& lut, double min, double max);
void lookup_table_get_saturation_range(vtkLookupTable& lut, double& min, double& max);

// Value range (brightness in HSV)
void lookup_table_set_value_range(vtkLookupTable& lut, double min, double max);
void lookup_table_get_value_range(vtkLookupTable& lut, double& min, double& max);

// Alpha range (opacity)
void lookup_table_set_alpha_range(vtkLookupTable& lut, double min, double max);
void lookup_table_get_alpha_range(vtkLookupTable& lut, double& min, double& max);

// Table configuration
void lookup_table_set_number_of_table_values(vtkLookupTable& lut, int64_t n);
int64_t lookup_table_get_number_of_table_values(vtkLookupTable& lut);

// Build the table
void lookup_table_build(vtkLookupTable& lut);

// Get color for a value
void lookup_table_get_color(vtkLookupTable& lut, double value, double& r, double& g, double& b);
