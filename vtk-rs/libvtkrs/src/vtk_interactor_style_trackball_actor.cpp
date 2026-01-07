#include "cxx.h"
#include "vtk_interactor_style_trackball_actor.h"
#include <vtkRenderWindowInteractor.h>
#include <vtkInteractorStyleTrackballActor.h>
#include <vtkPropPicker.h>
#include <vtkActor.h>
#include <vtkRenderWindow.h>
#include <vtkRendererCollection.h>
#include <vtkRenderer.h>
#include <sstream>

extern "C" {
InteractorStyleTrackballActor* interactor_style_trackball_actor_new() {
    InteractorStyleTrackballActor* obj = InteractorStyleTrackballActor::New();
    if (!obj) throw std::runtime_error("Failed to create InteractorStyleTrackballActor");
    return obj;
}

void interactor_style_trackball_actor_delete(InteractorStyleTrackballActor* style) {
    if (style) style->Delete();
}

void interactor_style_trackball_actor_set_left_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetLeftButtonPressCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_left_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetLeftButtonReleaseCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_right_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetRightButtonPressCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_right_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetRightButtonReleaseCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_middle_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetMiddleButtonPressCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_middle_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetMiddleButtonReleaseCallbackId(callback_id);
}

void interactor_style_trackball_actor_set_mouse_move_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id) {
    if (!style) return;
    style->SetMouseMoveCallbackId(callback_id);
}
}

extern "C" {
int interactor_style_trackball_actor_isA(InteractorStyleTrackballActor* style, const char* name) {
    if (!style || !name) return 0;
    return style->IsA(name);
}

InteractorStyleTrackballActor* interactor_style_trackball_actor_new_instance(InteractorStyleTrackballActor* style) {
    if (!style) return nullptr;
    vtkInteractorStyleTrackballActor* inst = style->NewInstance();
    return static_cast<InteractorStyleTrackballActor*>(inst);
}

void interactor_style_trackball_actor_print_self(InteractorStyleTrackballActor* style, int indent) {
    if (!style) return;
    std::ostringstream oss;
    style->PrintSelf(oss, vtkIndent(indent));
    // No return; caller can retrieve or ignore output from logs
}

void interactor_style_trackball_actor_rotate(InteractorStyleTrackballActor* style) {
    if (!style) return;
    style->Rotate();
}

void interactor_style_trackball_actor_spin(InteractorStyleTrackballActor* style) {
    if (!style) return;
    style->Spin();
}

void interactor_style_trackball_actor_pan(InteractorStyleTrackballActor* style) {
    if (!style) return;
    style->Pan();
}

void interactor_style_trackball_actor_dolly(InteractorStyleTrackballActor* style) {
    if (!style) return;
    style->Dolly();
}

void interactor_style_trackball_actor_uniform_scale(InteractorStyleTrackballActor* style) {
    if (!style) return;
    style->UniformScale();
}
}

// Implementation
InteractorStyleTrackballActor* InteractorStyleTrackballActor::New() {
    return new InteractorStyleTrackballActor();
}

InteractorStyleTrackballActor::InteractorStyleTrackballActor() {
    this->mouse_move_callback_id = 0;
    this->left_press_callback_id = 0;
    this->left_release_callback_id = 0;
    this->right_press_callback_id = 0;
    this->right_release_callback_id = 0;
    this->middle_press_callback_id = 0;
    this->middle_release_callback_id = 0;
}

void InteractorStyleTrackballActor::SetLeftButtonPressCallbackId(int64_t callback_id) {
    this->left_press_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetLeftButtonReleaseCallbackId(int64_t callback_id) {
    this->left_release_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetRightButtonPressCallbackId(int64_t callback_id) {
    this->right_press_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetRightButtonReleaseCallbackId(int64_t callback_id) {
    this->right_release_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetMiddleButtonPressCallbackId(int64_t callback_id) {
    this->middle_press_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetMiddleButtonReleaseCallbackId(int64_t callback_id) {
    this->middle_release_callback_id = callback_id;
}

void InteractorStyleTrackballActor::SetMouseMoveCallbackId(int64_t callback_id) {
    this->mouse_move_callback_id = callback_id;
}

void InteractorStyleTrackballActor::OnLeftButtonDown() {
    if (this->left_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        // Try to pick a prop at the event position
        vtkPropPicker* picker = vtkPropPicker::New();
        picker->Pick(pos[0], pos[1], 0.0, this->Interactor->GetRenderWindow()->GetRenderers()->GetFirstRenderer());
        vtkActor* actor = vtkActor::SafeDownCast(picker->GetViewProp());
        vtk_rs_left_button_press_callback_with_actor(this->left_press_callback_id, pos[0], pos[1], actor);
        picker->Delete();
    }
    vtkInteractorStyleTrackballActor::OnLeftButtonDown();
}

void InteractorStyleTrackballActor::OnLeftButtonUp() {
    if (this->left_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtkPropPicker* picker = vtkPropPicker::New();
        picker->Pick(pos[0], pos[1], 0.0, this->Interactor->GetRenderWindow()->GetRenderers()->GetFirstRenderer());
        vtkActor* actor = vtkActor::SafeDownCast(picker->GetViewProp());
        vtk_rs_left_button_release_callback_with_actor(this->left_release_callback_id, pos[0], pos[1], actor);
        picker->Delete();
    }
    vtkInteractorStyleTrackballActor::OnLeftButtonUp();
}

void InteractorStyleTrackballActor::OnMouseMove() {
    if (this->mouse_move_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtkPropPicker* picker = vtkPropPicker::New();
        picker->Pick(pos[0], pos[1], 0.0, this->Interactor->GetRenderWindow()->GetRenderers()->GetFirstRenderer());
        vtkActor* actor = vtkActor::SafeDownCast(picker->GetViewProp());
        vtk_rs_mouse_move_callback_with_actor(this->mouse_move_callback_id, pos[0], pos[1], actor);
        picker->Delete();
    }
    vtkInteractorStyleTrackballActor::OnMouseMove();
}

void InteractorStyleTrackballActor::OnMiddleButtonDown() {
    if (this->middle_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_middle_button_press_callback(this->middle_press_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleTrackballActor::OnMiddleButtonDown();
}

void InteractorStyleTrackballActor::OnMiddleButtonUp() {
    if (this->middle_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_middle_button_release_callback(this->middle_release_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleTrackballActor::OnMiddleButtonUp();
}

void InteractorStyleTrackballActor::OnRightButtonDown() {
    if (this->right_press_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_right_button_press_callback(this->right_press_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleTrackballActor::OnRightButtonDown();
}

void InteractorStyleTrackballActor::OnRightButtonUp() {
    if (this->right_release_callback_id != 0 && this->Interactor) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_right_button_release_callback(this->right_release_callback_id, pos[0], pos[1]);
    }
    vtkInteractorStyleTrackballActor::OnRightButtonUp();
}
