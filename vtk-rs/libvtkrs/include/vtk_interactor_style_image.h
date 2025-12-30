#pragma once
#include "cxx.h"

#include <vtkInteractorStyleImage.h>
#include <vtkActor.h>
#include <vtkNew.h>

// Lifecycle
vtkInteractorStyleImage* vtk_interactor_style_image_new();
void vtk_interactor_style_image_delete(vtkInteractorStyleImage* style);

void vtk_isA(vtkInteractorStyleImage* style, rust::Str name, bool& result);
void vtk_print_Self(vtkInteractorStyleImage* style, rust::Str filename, int indent);
void vtk_on_char(vtkInteractorStyleImage* style); // Override the "fly-to" (f keypress for images)

void vtk_window_level(vtkInteractorStyleImage* style);
void vtk_pick(vtkInteractorStyleImage* style);
void vtk_slice(vtkInteractorStyleImage* style);
void vtk_start_window_level(vtkInteractorStyleImage* style);
void vtk_end_window_level(vtkInteractorStyleImage* style);
void vtk_start_pick(vtkInteractorStyleImage* style);
void vtk_end_pick(vtkInteractorStyleImage* style);
void vtk_start_slice(vtkInteractorStyleImage* style);
void vtk_end_slice(vtkInteractorStyleImage* style);
void vtk_set_image_orientation(
    vtkInteractorStyleImage* style,
    double right_vector_x, double right_vector_y, double right_vector_z,
    double up_vector_x, double up_vector_y, double up_vector_z
);
void vtk_set_current_image_number(vtkInteractorStyleImage* style, int image_num);
void vtk_get_current_image_number(vtkInteractorStyleImage* style, int& image_num);

// how do we get the property?
void vtk_get_current_image_property(vtkInteractorStyleImage* style, vtkImageProperty*& property);
void vtk_get_window_level_start_position(vtkInteractorStyleImage* style, int& x, int& y);
void vtk_get_window_level_current_position(vtkInteractorStyleImage* style, int& x, int& y);

void vtk_set_interaction_mode(vtkInteractorStyleImage* style, int mode);
void vtk_get_interaction_mode(vtkInteractorStyleImage* style, int& mode);

void vtk_set_interaction_mode_to_image_2d(vtkInteractorStyleImage* style);
void vtk_set_interaction_mode_to_image_3d(vtkInteractorStyleImage* style);
void vtk_set_interaction_mode_to_image_slicing(vtkInteractorStyleImage* style);

void vtk_set_x_view_right_vector(vtkInteractorStyleImage* style, double x, double y, double z);
void vtk_get_x_view_right_vector(vtkInteractorStyleImage* style, double& x, double& y, double& z);
void vtk_set_x_view_up_vector(vtkInteractorStyleImage* style, double x, double y, double z);
void vtk_get_x_view_up_vector(vtkInteractorStyleImage* style, double& x, double& y, double& z);
void vtk_set_y_view_right_vector(vtkInteractorStyleImage* style, double x, double y, double z);
void vtk_get_y_view_right_vector(vtkInteractorStyleImage* style, double& x, double& y, double& z);

// --- Custom Image Interactor (callback-enabled) ---
// This class provides Rust callback hooks for image-style interaction.
#include <cstdint>

// Extern C callbacks defined in Rust (trampolines)
extern "C" {
    void vtk_rs_left_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_left_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_mouse_move_callback(int64_t callback_id, int x, int y);
    void vtk_rs_right_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_right_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_middle_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_middle_button_release_callback(int64_t callback_id, int x, int y);
    int vtk_rs_key_press_callback(int64_t callback_id, const char* key);
}

class InteractorStyleImage : public vtkInteractorStyleImage {
public:
    static InteractorStyleImage* New();
    const char* GetClassName() { return "InteractorStyleImage"; }

    void SetLeftButtonPressCallbackId(int64_t callback_id);
    void SetLeftButtonReleaseCallbackId(int64_t callback_id);
    void SetRightButtonPressCallbackId(int64_t callback_id);
    void SetRightButtonReleaseCallbackId(int64_t callback_id);
    void SetMiddleButtonPressCallbackId(int64_t callback_id);
    void SetMiddleButtonReleaseCallbackId(int64_t callback_id);
    void SetMouseMoveCallbackId(int64_t callback_id);
    void SetKeyPressCallbackId(int64_t callback_id);

    void SetSelectionMode(bool enabled);
    bool GetSelectionMode() const;

    void StartSelect();
    void EndSelect();

    bool GetMoving() const { return moving; }
    void GetSelectionPositions(int* start_x, int* start_y, int* end_x, int* end_y) const {
        *start_x = start_position[0];
        *start_y = start_position[1];
        *end_x = end_position[0];
        *end_y = end_position[1];
    }

    void OnMouseMove() override;
    void OnLeftButtonDown() override;
    void OnLeftButtonUp() override;
    void OnMiddleButtonDown() override;
    void OnMiddleButtonUp() override;
    void OnRightButtonDown() override;
    void OnRightButtonUp() override;
    void OnChar() override;
    void OnKeyPress() override;

protected:
    InteractorStyleImage();
    ~InteractorStyleImage() override = default;

private:
    int64_t mouse_move_callback_id = 0;
    int64_t left_press_callback_id = 0;
    int64_t left_release_callback_id = 0;
    int64_t right_press_callback_id = 0;
    int64_t right_release_callback_id = 0;
    int64_t middle_press_callback_id = 0;
    int64_t middle_release_callback_id = 0;
    int64_t key_press_callback_id = 0;

    int start_position[2] = {0, 0};
    int end_position[2] = {0, 0};
    bool moving = false;
};

// C-style wrapper functions for the image variant
extern "C" {
    InteractorStyleImage* interactor_style_image_new();
    void interactor_style_image_delete(InteractorStyleImage* style);
    void interactor_style_image_set_left_button_press_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_left_button_release_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_right_button_press_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_right_button_release_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_middle_button_press_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_middle_button_release_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_mouse_move_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_key_press_callback_id(InteractorStyleImage* style, int64_t callback_id);
    void interactor_style_image_set_selection_mode(InteractorStyleImage* style, bool enabled);
    bool interactor_style_image_is_moving(InteractorStyleImage* style);
    void interactor_style_image_get_selection_positions(InteractorStyleImage* style, int* start_x, int* start_y, int* end_x, int* end_y);
}
void vtk_set_y_view_up_vector(InteractorStyleImage* style, double x, double y, double z);
void vtk_get_y_view_up_vector(InteractorStyleImage* style, double& x, double& y, double& z);
void vtk_set_z_view_right_vector(InteractorStyleImage* style, double x, double y, double z);
void vtk_get_z_view_right_vector(InteractorStyleImage* style, double& x, double& y, double& z);
void vtk_set_z_view_up_vector(InteractorStyleImage* style, double x, double y, double z);
void vtk_get_z_view_up_vector(InteractorStyleImage* style, double& x, double& y, double& z);