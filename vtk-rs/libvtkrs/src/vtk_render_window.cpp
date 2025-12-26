#include "cxx.h"
#include "vtk_render_window.h"
#include "vtk_render_window.rs.h"

#include <vtkRenderWindow.h>
#include <vtkRenderer.h>
#include <vtkUnsignedCharArray.h>
#include <string>

vtkRenderWindow* render_window_new() {
    vtkRenderWindow* obj = vtkRenderWindow::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkRenderWindow");
    }
    return obj;
}

void render_window_delete(vtkRenderWindow& window) {
    window.Delete();
}

void render_window_add_renderer(vtkRenderWindow& window, vtkRenderer* renderer) {
    window.AddRenderer(renderer);
}

void render_window_set_size(vtkRenderWindow& window, int width, int height) {
    window.SetSize(width, height);
}

void render_window_set_window_name(vtkRenderWindow& window, rust::Str name) {
    window.SetWindowName(std::string(name).c_str());
}

void render_window_render(vtkRenderWindow& window) {
    window.Render();
}

void render_window_get_size(vtkRenderWindow& window, int& width, int& height) {
    int* size = window.GetSize();
    width = size[0];
    height = size[1];
}

void render_window_get_pixel_data(vtkRenderWindow& window, unsigned char* data, int size) {
    int* win_size = window.GetSize();
    vtkUnsignedCharArray* pixels = vtkUnsignedCharArray::New();
    window.GetRGBACharPixelData(0, 0, win_size[0] - 1, win_size[1] - 1, 1, pixels);
    
    int pixel_count = win_size[0] * win_size[1] * 4;
    if (size >= pixel_count) {
        for (int i = 0; i < pixel_count; i++) {
            data[i] = pixels->GetValue(i);
        }
    }
    pixels->Delete();
}

void render_window_set_pixel_data(vtkRenderWindow& window, const unsigned char* data, int size) {
    int* win_size = window.GetSize();
    int pixel_count = win_size[0] * win_size[1] * 4;
    
    if (size >= pixel_count) {
        vtkUnsignedCharArray* pixels = vtkUnsignedCharArray::New();
        pixels->SetNumberOfValues(pixel_count);
        
        for (int i = 0; i < pixel_count; i++) {
            pixels->SetValue(i, data[i]);
        }
        
        // Write to front buffer (0) for immediate display
        window.SetRGBACharPixelData(0, 0, win_size[0] - 1, win_size[1] - 1, pixels, 0);
        window.Frame(); // Force display update
        pixels->Delete();
    }
}

size_t render_window_add_observer(vtkRenderWindow& window, size_t event, vtkCommand* command) {
    return window.AddObserver(static_cast<unsigned long>(event), command);
}
