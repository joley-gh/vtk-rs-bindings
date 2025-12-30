#include "cxx.h"
#include "vtk_interactor_style_image.h"
#include "vtk_interactor_style_image.rs.h"
#include <sstream>

#include <vtkInteractorStyleImage.h>
#include <vtkRenderWindowInteractor.h>

// Provide C ABI wrappers that match the Rust extern declarations and
// implement the callback-enabled image interactor.

// Create/destroy image interactor
extern "C" {
InteractorStyleImage* interactor_style_image_new() {
        InteractorStyleImage* obj = InteractorStyleImage::New();
        if (!obj) {
            throw std::runtime_error("Failed to create InteractorStyleImage");
        }
        return obj;
    }

    void interactor_style_image_delete(InteractorStyleImage* style) {
        if (style) {
            style->Delete();
        }
    }

    bool interactor_style_isA(InteractorStyleImage* style, const char* name) {
        if (!style || !name) return false;
        return style->IsA(name) != 0;
    }

    void interactor_style_print_self(InteractorStyleImage* style, int indent) {
        if (!style) return;
        std::ostringstream oss;
        style->PrintSelf(oss, vtkIndent(indent));
        // Intentionally not returning the string; caller can print or ignore.
    }

    void interactor_style_on_char(InteractorStyleImage* style) {
        if (!style) return;
        style->OnChar();
    }

    void interactor_style_window_level(InteractorStyleImage* style) {
        if (!style) return;
        style->WindowLevel();
    }

    void interactor_style_pick(InteractorStyleImage* style) {
        if (!style) return;
        style->Pick();
    }

    void interactor_style_slice(InteractorStyleImage* style) {
        if (!style) return;
        style->Slice();
    }

    void interactor_style_start_window_level(InteractorStyleImage* style) {
        if (!style) return;
        style->StartWindowLevel();
    }

    void interactor_style_end_window_level(InteractorStyleImage* style) {
        if (!style) return;
        style->EndWindowLevel();
    }

    void interactor_style_start_pick(InteractorStyleImage* style) {
        if (!style) return;
        style->StartPick();
    }

    void interactor_style_end_pick(InteractorStyleImage* style) {
        if (!style) return;
        style->EndPick();
    }

    void interactor_style_start_slice(InteractorStyleImage* style) {
        if (!style) return;
        style->StartSlice();
    }

    void interactor_style_end_slice(InteractorStyleImage* style) {
        if (!style) return;
        style->EndSlice();
    }

    void interactor_style_set_image_orientation(
        InteractorStyleImage* style,
        const double* right,
        const double* up
    ) {
        if (!style || !right || !up) return;
        double r[3] = { right[0], right[1], right[2] };
        double u[3] = { up[0], up[1], up[2] };
        style->SetImageOrientation(r, u);
    }

    void interactor_style_set_current_image_number(InteractorStyleImage* style, int image_num) {
        if (!style) return;
        style->SetCurrentImageNumber(image_num);
    }

    int interactor_style_get_current_image_number(InteractorStyleImage* style) {
        if (!style) return -1;
        return style->GetCurrentImageNumber();
    }

    vtkImageProperty* interactor_style_get_current_image_property(InteractorStyleImage* style) {
        if (!style) return nullptr;
        return style->GetCurrentImageProperty();
    }

    void interactor_style_get_window_level_start_position(InteractorStyleImage* style, int* x, int* y) {
        if (!style || !x || !y) return;
        int pos[2];
        style->GetWindowLevelStartPosition(pos);
        *x = pos[0];
        *y = pos[1];
    }

    void interactor_style_get_window_level_current_position(InteractorStyleImage* style, int* x, int* y) {
        if (!style || !x || !y) return;
        int pos[2];
        style->GetWindowLevelCurrentPosition(pos);
        *x = pos[0];
        *y = pos[1];
    }

    void interactor_style_set_interaction_mode(InteractorStyleImage* style, int mode) {
        if (!style) return;
        style->SetInteractionMode(mode);
    }

    int interactor_style_get_interaction_mode(InteractorStyleImage* style) {
        if (!style) return -1;
        return style->GetInteractionMode();
    }

    void interactor_style_set_interaction_mode_to_image_2d(InteractorStyleImage* style) {
        if (!style) return;
        style->SetInteractionModeToImage2D();
    }

    void interactor_style_set_interaction_mode_to_image_3d(InteractorStyleImage* style) {
        if (!style) return;
        style->SetInteractionModeToImage3D();
    }

    void interactor_style_set_interaction_mode_to_image_slicing(InteractorStyleImage* style) {
        if (!style) return;
        style->SetInteractionModeToImageSlicing();
    }

    void interactor_style_set_x_view_right_vector(InteractorStyleImage* style, const double* right) {
        if (!style || !right) return;
        double r[3] = { right[0], right[1], right[2] };
        style->SetXViewRightVector(r);
    }

    void interactor_style_get_x_view_right_vector(InteractorStyleImage* style, double* right_out) {
        if (!style || !right_out) return;
        style->GetXViewRightVector(right_out);
    }

    void interactor_style_set_x_view_up_vector(InteractorStyleImage* style, const double* up) {
        if (!style || !up) return;
        double u[3] = { up[0], up[1], up[2] };
        style->SetXViewUpVector(u);
    }

    void interactor_style_get_x_view_up_vector(InteractorStyleImage* style, double* up_out) {
        if (!style || !up_out) return;
        style->GetXViewUpVector(up_out);
    }

    void interactor_style_set_y_view_right_vector(InteractorStyleImage* style, const double* right) {
        if (!style || !right) return;
        double r[3] = { right[0], right[1], right[2] };
        style->SetYViewRightVector(r);
    }

    void interactor_style_get_y_view_right_vector(InteractorStyleImage* style, double* right_out) {
        if (!style || !right_out) return;
        style->GetYViewRightVector(right_out);
    }

    void interactor_style_set_y_view_up_vector(InteractorStyleImage* style, const double* up) {
        if (!style || !up) return;
        double u[3] = { up[0], up[1], up[2] };
        style->SetYViewUpVector(u);
    }

    void interactor_style_get_y_view_up_vector(InteractorStyleImage* style, double* up_out) {
        if (!style || !up_out) return;
        style->GetYViewUpVector(up_out);
    }

    void interactor_style_set_z_view_right_vector(InteractorStyleImage* style, const double* right) {
        if (!style || !right) return;
        double r[3] = { right[0], right[1], right[2] };
        style->SetZViewRightVector(r);
    }

    void interactor_style_get_z_view_right_vector(InteractorStyleImage* style, double* right_out) {
        if (!style || !right_out) return;
        style->GetZViewRightVector(right_out);
    }

    void interactor_style_set_z_view_up_vector(InteractorStyleImage* style, const double* up) {
        if (!style || !up) return;
        double u[3] = { up[0], up[1], up[2] };
        style->SetZViewUpVector(u);
    }

    void interactor_style_get_z_view_up_vector(InteractorStyleImage* style, double* up_out) {
        if (!style || !up_out) return;
        style->GetZViewUpVector(up_out);
    }

    void interactor_style_image_set_left_button_press_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetLeftButtonPressCallbackId(callback_id);
    }

    void interactor_style_image_set_left_button_release_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetLeftButtonReleaseCallbackId(callback_id);
    }

    void interactor_style_image_set_right_button_press_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetRightButtonPressCallbackId(callback_id);
    }

    void interactor_style_image_set_right_button_release_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetRightButtonReleaseCallbackId(callback_id);
    }

    void interactor_style_image_set_middle_button_press_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetMiddleButtonPressCallbackId(callback_id);
    }

    void interactor_style_image_set_middle_button_release_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetMiddleButtonReleaseCallbackId(callback_id);
    }

    void interactor_style_image_set_mouse_move_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetMouseMoveCallbackId(callback_id);
    }

    void interactor_style_image_set_key_press_callback_id(
        InteractorStyleImage* style,
        int64_t callback_id
    ) {
        if (!style) return;
        style->SetKeyPressCallbackId(callback_id);
    }

    void interactor_style_image_set_selection_mode(
        InteractorStyleImage* style,
        bool enabled
    ) {
        if (!style) return;
        style->SetSelectionMode(enabled);
    }

    bool interactor_style_image_is_moving(InteractorStyleImage* style) {
        if (!style) return false;
        return style->GetMoving();
    }

    void interactor_style_image_get_selection_positions(
        InteractorStyleImage* style,
        int* start_x,
        int* start_y,
        int* end_x,
        int* end_y
    ) {
        if (!style || !start_x || !start_y || !end_x || !end_y) return;
        style->GetSelectionPositions(start_x, start_y, end_x, end_y);
    }

}

// --- vtkInteractorStyleImage implementation ---
// Define the methods declared in the header so the image interactor
// supports Rust callback IDs and selection handling.

// Direct instantiation
InteractorStyleImage* InteractorStyleImage::New() {
    return new InteractorStyleImage();
}

InteractorStyleImage::InteractorStyleImage() {
    this->mouse_move_callback_id = 0;
    this->left_press_callback_id = 0;
    this->left_release_callback_id = 0;
    this->right_press_callback_id = 0;
    this->right_release_callback_id = 0;
    this->middle_press_callback_id = 0;
    this->middle_release_callback_id = 0;
    this->key_press_callback_id = 0;
    this->start_position[0] = this->start_position[1] = 0;
    this->end_position[0] = this->end_position[1] = 0;
    this->moving = false;
}

void InteractorStyleImage::SetLeftButtonPressCallbackId(int64_t callback_id) {
    this->left_press_callback_id = callback_id;
}

void InteractorStyleImage::SetLeftButtonReleaseCallbackId(int64_t callback_id) {
    this->left_release_callback_id = callback_id;
}

void InteractorStyleImage::SetRightButtonPressCallbackId(int64_t callback_id) {
    this->right_press_callback_id = callback_id;
}

void InteractorStyleImage::SetRightButtonReleaseCallbackId(int64_t callback_id) {
    this->right_release_callback_id = callback_id;
}

void InteractorStyleImage::SetMiddleButtonPressCallbackId(int64_t callback_id) {
    this->middle_press_callback_id = callback_id;
}

void InteractorStyleImage::SetMiddleButtonReleaseCallbackId(int64_t callback_id) {
    this->middle_release_callback_id = callback_id;
}

void InteractorStyleImage::SetMouseMoveCallbackId(int64_t callback_id) {
    this->mouse_move_callback_id = callback_id;
}

void InteractorStyleImage::SetKeyPressCallbackId(int64_t callback_id) {
    this->key_press_callback_id = callback_id;
}

void InteractorStyleImage::SetSelectionMode(bool enabled) {
    // store selection mode in moving flag semantics
    this->moving = enabled ? this->moving : false;
}

bool InteractorStyleImage::GetSelectionMode() const {
    return this->moving; // conservative
}

void InteractorStyleImage::StartSelect() {
    if (!this->Interactor) return;
    int* pos = this->Interactor->GetEventPosition();
    this->start_position[0] = pos[0];
    this->start_position[1] = pos[1];
    this->end_position[0] = pos[0];
    this->end_position[1] = pos[1];
    this->moving = true;
}

void InteractorStyleImage::EndSelect() {
    this->moving = false;
}

void InteractorStyleImage::OnLeftButtonDown() {
    if (this->moving) {
        StartSelect();
    }
    if (this->left_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_press_callback(this->left_press_callback_id, pos[0], pos[1]);
    }
    if (!this->moving) {
        vtkInteractorStyleImage::OnLeftButtonDown();
    }
}

void InteractorStyleImage::OnLeftButtonUp() {
    if (this->moving) {
        EndSelect();
    }
    if (this->left_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_release_callback(this->left_release_callback_id, pos[0], pos[1]);
    }
    if (!this->moving) {
        vtkInteractorStyleImage::OnLeftButtonUp();
    }
}

void InteractorStyleImage::OnMouseMove() {
    if (this->moving && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        this->end_position[0] = pos[0];
        this->end_position[1] = pos[1];
    }
    if (this->mouse_move_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_mouse_move_callback(this->mouse_move_callback_id, pos[0], pos[1]);
    }
    if (!this->moving) {
        vtkInteractorStyleImage::OnMouseMove();
    }
}

void InteractorStyleImage::OnMiddleButtonDown() {
    if (this->middle_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_middle_button_press_callback(this->middle_press_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleImage::OnMiddleButtonDown();
}

void InteractorStyleImage::OnMiddleButtonUp() {
    if (this->middle_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_middle_button_release_callback(this->middle_release_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleImage::OnMiddleButtonUp();
}

void InteractorStyleImage::OnRightButtonDown() {
    if (this->right_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_right_button_press_callback(this->right_press_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleImage::OnRightButtonDown();
}

void InteractorStyleImage::OnRightButtonUp() {
    if (this->right_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_right_button_release_callback(this->right_release_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleImage::OnRightButtonUp();
}

void InteractorStyleImage::OnChar() {
    // Forward printable character events to Rust key press callback as well.
    bool consumed = false;
    if (this->key_press_callback_id != 0 && this->Interactor) {
        vtkRenderWindowInteractor* rwi = this->Interactor;
        std::string key = rwi->GetKeySym();
        int rv = vtk_rs_key_press_callback(this->key_press_callback_id, key.c_str());
        consumed = (rv != 0);
    }

    // Call base class handler only if not consumed by Rust callback
    if (!consumed) {
        vtkInteractorStyleImage::OnChar();
    }

    // Ensure the render window repaints after character-driven visual changes
    if (this->Interactor) {
        this->Interactor->Render();
    }
}

void InteractorStyleImage::OnKeyPress() {
    bool consumed = false;
    if (this->key_press_callback_id != 0 && this->Interactor) {
        vtkRenderWindowInteractor* rwi = this->Interactor;
        std::string key = rwi->GetKeySym();
        int rv = vtk_rs_key_press_callback(this->key_press_callback_id, key.c_str());
        consumed = (rv != 0);
    }

    if (!consumed) {
        vtkInteractorStyleImage::OnKeyPress();
    }

    // Force a repaint after keypress-driven modifications
    if (this->Interactor) {
        this->Interactor->Render();
    }
}
