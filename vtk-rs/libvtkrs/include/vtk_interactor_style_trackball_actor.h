#pragma once
#include "cxx.h"

#include <vtkInteractorStyleTrackballActor.h>
#include <cstdint>

// Extern C callbacks defined in Rust (trampolines)
class vtkActor; // forward declaration for actor-aware trampolines
extern "C" {
    void vtk_rs_left_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_left_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_right_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_right_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_middle_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_middle_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_mouse_move_callback(int64_t callback_id, int x, int y);

    // Actor-aware variants (actor pointer may be null)
    void vtk_rs_left_button_press_callback_with_actor(int64_t callback_id, int x, int y, vtkActor* actor);
    void vtk_rs_left_button_release_callback_with_actor(int64_t callback_id, int x, int y, vtkActor* actor);
    void vtk_rs_mouse_move_callback_with_actor(int64_t callback_id, int x, int y, vtkActor* actor);
}

class InteractorStyleTrackballActor : public vtkInteractorStyleTrackballActor {
public:
    static InteractorStyleTrackballActor* New();
    const char* GetClassName() { return "InteractorStyleTrackballActor"; }

    void SetLeftButtonPressCallbackId(int64_t callback_id);
    void SetLeftButtonReleaseCallbackId(int64_t callback_id);
    void SetRightButtonPressCallbackId(int64_t callback_id);
    void SetRightButtonReleaseCallbackId(int64_t callback_id);
    void SetMiddleButtonPressCallbackId(int64_t callback_id);
    void SetMiddleButtonReleaseCallbackId(int64_t callback_id);
    void SetMouseMoveCallbackId(int64_t callback_id);

    void OnMouseMove() override;
    void OnLeftButtonDown() override;
    void OnLeftButtonUp() override;
    void OnMiddleButtonDown() override;
    void OnMiddleButtonUp() override;
    void OnRightButtonDown() override;
    void OnRightButtonUp() override;

protected:
    InteractorStyleTrackballActor();
    ~InteractorStyleTrackballActor() override = default;

private:
    int64_t mouse_move_callback_id = 0;
    int64_t left_press_callback_id = 0;
    int64_t left_release_callback_id = 0;
    int64_t right_press_callback_id = 0;
    int64_t right_release_callback_id = 0;
    int64_t middle_press_callback_id = 0;
    int64_t middle_release_callback_id = 0;
};

// C-style wrapper functions
extern "C" {
    InteractorStyleTrackballActor* interactor_style_trackball_actor_new();
    void interactor_style_trackball_actor_delete(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_set_left_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_left_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_right_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_right_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_middle_button_press_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_middle_button_release_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    void interactor_style_trackball_actor_set_mouse_move_callback_id(InteractorStyleTrackballActor* style, int64_t callback_id);
    int interactor_style_trackball_actor_isA(InteractorStyleTrackballActor* style, const char* name);
    InteractorStyleTrackballActor* interactor_style_trackball_actor_new_instance(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_print_self(InteractorStyleTrackballActor* style, int indent);

    void interactor_style_trackball_actor_rotate(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_spin(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_pan(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_dolly(InteractorStyleTrackballActor* style);
    void interactor_style_trackball_actor_uniform_scale(InteractorStyleTrackballActor* style);
}
