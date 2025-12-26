#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_renderer.h");
        include!("vtk_actor.h");

        type vtkRenderer;
        type vtkActor;
        type vtkActor2D;
        type vtkCamera;

        fn renderer_new() -> *mut vtkRenderer;
        fn renderer_delete(renderer: Pin<&mut vtkRenderer>);
        unsafe fn renderer_add_actor(renderer: Pin<&mut vtkRenderer>, actor: *mut vtkActor);
        unsafe fn renderer_add_actor2d(renderer: Pin<&mut vtkRenderer>, actor: *mut vtkActor2D);
        fn renderer_set_background(renderer: Pin<&mut vtkRenderer>, r: f64, g: f64, b: f64);
        fn renderer_get_active_camera(renderer: Pin<&mut vtkRenderer>) -> *mut vtkCamera;
        fn renderer_reset_camera(renderer: Pin<&mut vtkRenderer>);

        // Coordinate conversion
        fn renderer_set_display_point(renderer: Pin<&mut vtkRenderer>, x: f64, y: f64, z: f64);
        fn renderer_display_to_world(renderer: Pin<&mut vtkRenderer>);
        fn renderer_get_world_point(
            renderer: Pin<&mut vtkRenderer>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64,
            w: &mut f64
        );
        fn renderer_set_world_point(
            renderer: Pin<&mut vtkRenderer>,
            x: f64,
            y: f64,
            z: f64,
            w: f64
        );
        fn renderer_world_to_display(renderer: Pin<&mut vtkRenderer>);
        fn renderer_get_display_point(
            renderer: Pin<&mut vtkRenderer>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkRenderer.html",
    @name Renderer, ffi::vtkRenderer,
    @new ffi::renderer_new,
    @delete ffi::renderer_delete
);

// VTK objects are internally reference counted and thread-safe
// We need to mark Renderer as Send to use in callbacks
unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

impl Renderer {
    pub fn add_actor(&mut self, actor: &mut crate::Actor) {
        unsafe {
            let actor_ptr = actor.as_mut_ptr() as *mut ffi::vtkActor;
            ffi::renderer_add_actor(self.ptr.as_mut(), actor_ptr);
        }
    }

    /// Adds an actor using a raw pointer. Useful for actor subtypes like CubeAxesActor.
    ///
    /// # Safety
    /// The pointer must be a valid vtkActor pointer or subclass.
    pub unsafe fn add_actor_raw(&mut self, actor_ptr: *mut std::ffi::c_void) {
        let actor_ptr = actor_ptr as *mut ffi::vtkActor;
        ffi::renderer_add_actor(self.ptr.as_mut(), actor_ptr);
    }

    /// Add a 2D actor (like ScalarBarActor or TextActor) to the renderer
    pub fn add_actor2d(&mut self, actor: &mut crate::ScalarBarActor) {
        unsafe {
            let actor_ptr = actor.as_raw_ptr() as *mut ffi::vtkActor2D;
            ffi::renderer_add_actor2d(self.ptr.as_mut(), actor_ptr);
        }
    }

    pub fn set_background(&mut self, r: f64, g: f64, b: f64) {
        ffi::renderer_set_background(self.ptr.as_mut(), r, g, b);
    }

    /// Gets the active camera for this renderer.
    /// Returns a non-owning wrapper - the camera is managed by the renderer.
    pub fn get_active_camera(&mut self) -> CameraRef {
        let ptr = ffi::renderer_get_active_camera(self.ptr.as_mut());
        CameraRef { ptr }
    }

    /// Resets the camera to automatically fit all visible actors in the scene.
    /// This adjusts the camera position and clipping planes.
    pub fn reset_camera(&mut self) {
        ffi::renderer_reset_camera(self.ptr.as_mut());
    }

    /// Convert display coordinates to world coordinates.
    /// First call set_display_point, then display_to_world, then get_world_point.
    pub fn set_display_point(&mut self, x: f64, y: f64, z: f64) {
        ffi::renderer_set_display_point(self.ptr.as_mut(), x, y, z);
    }

    /// Perform the conversion from display to world coordinates.
    /// Call this after set_display_point and before get_world_point.
    pub fn display_to_world(&mut self) {
        ffi::renderer_display_to_world(self.ptr.as_mut());
    }

    /// Get the world point coordinates after calling display_to_world.
    /// Returns (x, y, z, w) where w is the homogeneous coordinate.
    pub fn get_world_point(&mut self) -> (f64, f64, f64, f64) {
        let (mut x, mut y, mut z, mut w) = (0.0, 0.0, 0.0, 0.0);
        ffi::renderer_get_world_point(self.ptr.as_mut(), &mut x, &mut y, &mut z, &mut w);
        (x, y, z, w)
    }

    /// Convert world coordinates to display coordinates.
    /// First call set_world_point, then world_to_display, then get_display_point.
    pub fn set_world_point(&mut self, x: f64, y: f64, z: f64, w: f64) {
        ffi::renderer_set_world_point(self.ptr.as_mut(), x, y, z, w);
    }

    /// Perform the conversion from world to display coordinates.
    /// Call this after set_world_point and before get_display_point.
    pub fn world_to_display(&mut self) {
        ffi::renderer_world_to_display(self.ptr.as_mut());
    }

    /// Get the display point coordinates after calling world_to_display.
    /// Returns (x, y, z) in display coordinates.
    pub fn get_display_point(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::renderer_get_display_point(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }
}

/// A non-owning reference to a Camera managed by a Renderer.
/// This does not delete the camera when dropped.
pub struct CameraRef {
    ptr: *mut ffi::vtkCamera,
}

impl CameraRef {
    pub fn as_mut_ptr(&mut self) -> *mut ffi::vtkCamera {
        self.ptr
    }

    /// Set the position of the camera in world coordinates.
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_set_position(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the position of the camera in world coordinates.
    pub fn get_position(&mut self) -> (f64, f64, f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_get_position(
                std::pin::Pin::new_unchecked(camera_ref),
                &mut x,
                &mut y,
                &mut z
            );
        }
        (x, y, z)
    }

    /// Set the focal point of the camera in world coordinates.
    pub fn set_focal_point(&mut self, x: f64, y: f64, z: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_set_focal_point(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the focal point of the camera in world coordinates.
    pub fn get_focal_point(&mut self) -> (f64, f64, f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_get_focal_point(
                std::pin::Pin::new_unchecked(camera_ref),
                &mut x,
                &mut y,
                &mut z
            );
        }
        (x, y, z)
    }

    /// Set the view up vector of the camera.
    pub fn set_view_up(&mut self, x: f64, y: f64, z: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_set_view_up(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the view up vector of the camera.
    pub fn get_view_up(&mut self) -> (f64, f64, f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_get_view_up(
                std::pin::Pin::new_unchecked(camera_ref),
                &mut x,
                &mut y,
                &mut z
            );
        }
        (x, y, z)
    }

    /// Rotate the camera about the view up vector centered at the focal point.
    pub fn azimuth(&mut self, degrees: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_azimuth(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Rotate the camera about the cross product of the view plane normal and view up vector.
    pub fn elevation(&mut self, degrees: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_elevation(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Rotate the camera about the view plane normal.
    pub fn roll(&mut self, degrees: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_roll(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Change the view angle by a factor (zoom in/out by changing field of view).
    pub fn zoom(&mut self, factor: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_zoom(std::pin::Pin::new_unchecked(camera_ref), factor);
        }
    }

    /// Move the camera closer/farther from the focal point (zoom by moving camera).
    pub fn dolly(&mut self, factor: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_dolly(std::pin::Pin::new_unchecked(camera_ref), factor);
        }
    }

    /// Set the near and far clipping planes.
    pub fn set_clipping_range(&mut self, near: f64, far: f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_set_clipping_range(
                std::pin::Pin::new_unchecked(camera_ref),
                near,
                far
            );
        }
    }

    /// Get the near and far clipping planes.
    pub fn get_clipping_range(&mut self) -> (f64, f64) {
        use crate::vtk_camera::ffi as camera_ffi;
        let (mut near, mut far) = (0.0, 0.0);
        unsafe {
            let camera_ref = &mut *(self.ptr as *mut camera_ffi::vtkCamera);
            camera_ffi::camera_get_clipping_range(
                std::pin::Pin::new_unchecked(camera_ref),
                &mut near,
                &mut far
            );
        }
        (near, far)
    }
}

impl crate::vtk_command::Observable for CameraRef {
    unsafe fn add_observer(&mut self, event: usize, command: &mut crate::Command) -> usize {
        use core::pin::Pin;
        let obj_ptr = self.ptr as *mut crate::vtk_command::ffi::vtkObject;
        let obj_ref = Pin::new_unchecked(&mut *obj_ptr);
        crate::vtk_command::ffi::vtk_object_add_observer(obj_ref, event, command.as_mut())
    }

    unsafe fn remove_observer(&mut self, tag: usize) {
        use core::pin::Pin;
        let obj_ptr = self.ptr as *mut crate::vtk_command::ffi::vtkObject;
        let obj_ref = Pin::new_unchecked(&mut *obj_ptr);
        crate::vtk_command::ffi::vtk_object_remove_observer(obj_ref, tag);
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkRenderer`](https://vtk.org/doc/nightly/html/classvtkRenderer.html)
#[allow(non_camel_case_types)]
pub trait vtkRenderer: private::Sealed {}
