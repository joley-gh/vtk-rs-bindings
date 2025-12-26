#pragma once

#include <vtkRenderWindow.h>
#include <vtkRenderer.h>
#include <vtkCommand.h>
#include "cxx.h"

vtkRenderWindow* render_window_new();
void render_window_delete(vtkRenderWindow& window);
void render_window_add_renderer(vtkRenderWindow& window, vtkRenderer* renderer);
void render_window_set_size(vtkRenderWindow& window, int width, int height);
void render_window_set_window_name(vtkRenderWindow& window, rust::Str name);
void render_window_render(vtkRenderWindow& window);
void render_window_get_size(vtkRenderWindow& window, int& width, int& height);
void render_window_get_pixel_data(vtkRenderWindow& window, unsigned char* data, int size);
void render_window_set_pixel_data(vtkRenderWindow& window, const unsigned char* data, int size);

// Observer support - use size_t which maps to usize in Rust
size_t render_window_add_observer(vtkRenderWindow& window, size_t event, vtkCommand* command);
