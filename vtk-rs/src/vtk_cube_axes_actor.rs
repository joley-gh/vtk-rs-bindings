#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_cube_axes_actor.h");

        type vtkCubeAxesActor;
        type vtkCamera;

        fn cube_axes_actor_new() -> *mut vtkCubeAxesActor;
        fn cube_axes_actor_delete(actor: Pin<&mut vtkCubeAxesActor>);
        fn cube_axes_actor_set_bounds(
            actor: Pin<&mut vtkCubeAxesActor>,
            x_min: f64,
            x_max: f64,
            y_min: f64,
            y_max: f64,
            z_min: f64,
            z_max: f64
        );
        unsafe fn cube_axes_actor_set_camera(
            actor: Pin<&mut vtkCubeAxesActor>,
            camera: *mut vtkCamera
        );
        fn cube_axes_actor_set_x_label(actor: Pin<&mut vtkCubeAxesActor>, label: &str);
        fn cube_axes_actor_set_y_label(actor: Pin<&mut vtkCubeAxesActor>, label: &str);
        fn cube_axes_actor_set_z_label(actor: Pin<&mut vtkCubeAxesActor>, label: &str);
        fn cube_axes_actor_draw_x_gridlines_on(actor: Pin<&mut vtkCubeAxesActor>);
        fn cube_axes_actor_draw_y_gridlines_on(actor: Pin<&mut vtkCubeAxesActor>);
        fn cube_axes_actor_draw_z_gridlines_on(actor: Pin<&mut vtkCubeAxesActor>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCubeAxesActor.html",
    @name CubeAxesActor, ffi::vtkCubeAxesActor,
    @new ffi::cube_axes_actor_new,
    @delete ffi::cube_axes_actor_delete
);

impl CubeAxesActor {
    /// Sets the bounds for the cube axes (min/max for each axis).
    pub fn set_bounds(
        &mut self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64
    ) {
        ffi::cube_axes_actor_set_bounds(
            self.ptr.as_mut(),
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max
        );
    }

    /// Sets the camera (required for proper orientation of labels).
    pub fn set_camera(&mut self, camera: &mut crate::CameraRef) {
        unsafe {
            let camera_ptr = camera.as_mut_ptr() as *mut ffi::vtkCamera;
            ffi::cube_axes_actor_set_camera(self.ptr.as_mut(), camera_ptr);
        }
    }

    /// Sets the X axis label.
    pub fn set_x_label(&mut self, label: &str) {
        ffi::cube_axes_actor_set_x_label(self.ptr.as_mut(), label);
    }

    /// Sets the Y axis label.
    pub fn set_y_label(&mut self, label: &str) {
        ffi::cube_axes_actor_set_y_label(self.ptr.as_mut(), label);
    }

    /// Sets the Z axis label.
    pub fn set_z_label(&mut self, label: &str) {
        ffi::cube_axes_actor_set_z_label(self.ptr.as_mut(), label);
    }

    /// Enables gridlines along the X axis.
    pub fn draw_x_gridlines_on(&mut self) {
        ffi::cube_axes_actor_draw_x_gridlines_on(self.ptr.as_mut());
    }

    /// Enables gridlines along the Y axis.
    pub fn draw_y_gridlines_on(&mut self) {
        ffi::cube_axes_actor_draw_y_gridlines_on(self.ptr.as_mut());
    }

    /// Enables gridlines along the Z axis.
    pub fn draw_z_gridlines_on(&mut self) {
        ffi::cube_axes_actor_draw_z_gridlines_on(self.ptr.as_mut());
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkCubeAxesActor`](https://vtk.org/doc/nightly/html/classvtkCubeAxesActor.html)
///
/// Creates a bounding box with labeled axes and optional gridlines.
#[allow(non_camel_case_types)]
pub trait vtkCubeAxesActor: private::Sealed {}

// Implement AddableToRenderer for CubeAxesActor (3D actor - inherits from vtkActor)
impl crate::AddableToRenderer for CubeAxesActor {
    fn add_to_renderer_internal(&mut self, renderer: &mut crate::Renderer) {
        let actor_ptr = self.as_mut_ptr() as *mut crate::vtk_renderer::ffi::vtkActor;
        renderer._add_actor_3d(actor_ptr);
    }
}
