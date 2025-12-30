#pragma once

#include <vtkInteractorStyleTrackballCamera.h>
#include <vtkRenderWindowInteractor.h>
#include <vtkRenderer.h>
#include <cstdint>

// Extern C callbacks defined in Rust
extern "C" {
    void vtk_rs_left_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_left_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_mouse_move_callback(int64_t callback_id, int x, int y);
    void vtk_rs_key_press_callback(int64_t callback_id, const char* key);
}

// Custom interactor style that allows Rust callbacks
class vtkInteractorStyleCustom : public vtkInteractorStyleTrackballCamera {
public:
    static vtkInteractorStyleCustom* New();
    
    // Type information (simplified, not using vtkTypeMacro to avoid issues)
    const char* GetClassName() { return "vtkInteractorStyleCustom"; }

    void SetLeftButtonPressCallbackId(int64_t callback_id);
    void SetLeftButtonReleaseCallbackId(int64_t callback_id);
    void SetMouseMoveCallbackId(int64_t callback_id);
    void SetKeyPressCallbackId(int64_t callback_id);
    
    // Enable/disable selection mode (disables camera controls on left drag)
    void SetSelectionMode(bool enabled);
    bool GetSelectionMode() const;
    
    // Rubber band state management
    void StartSelect();
    void EndSelect();
    
    // State accessors for Rust
    bool GetMoving() const { return moving; }
    void GetSelectionPositions(int* start_x, int* start_y, int* end_x, int* end_y) const {
        *start_x = start_position[0];
        *start_y = start_position[1];
        *end_x = end_position[0];
        *end_y = end_position[1];
    }

    void OnLeftButtonDown() override;
    void OnLeftButtonUp() override;
    void OnMouseMove() override;
    void OnKeyPress() override;

protected:
    vtkInteractorStyleCustom();
    ~vtkInteractorStyleCustom() override = default;

private:
    int64_t left_press_callback_id = 0;
    int64_t left_release_callback_id = 0;
    int64_t mouse_move_callback_id = 0;
    int64_t key_press_callback_id = 0;
    bool selection_mode = false;
    
    // Rubber band state
    int start_position[2] = {0, 0};
    int end_position[2] = {0, 0};
    bool moving = false;
};
// C-style wrapper functions for the Custom variant
extern "C" {
    vtkInteractorStyleCustom* interactor_style_custom_new();
    void interactor_style_custom_delete(vtkInteractorStyleCustom* style);
    void interactor_style_custom_set_left_button_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_left_button_release_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_mouse_move_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_key_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_selection_mode(
        vtkInteractorStyleCustom* style,
        bool enabled
    );
    bool interactor_style_custom_is_moving(
        vtkInteractorStyleCustom* style
    );
    void interactor_style_custom_get_selection_positions(
        vtkInteractorStyleCustom* style,
        int* start_x, int* start_y,
        int* end_x, int* end_y
    );
}
