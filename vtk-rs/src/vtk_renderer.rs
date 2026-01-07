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

    /// Add a 2D actor (like ScalarBarActor or LegendBoxActor) to the renderer
    pub fn add_scalar_bar(&mut self, actor: &mut crate::ScalarBarActor) {
        unsafe {
            let actor_ptr = actor.as_raw_ptr() as *mut ffi::vtkActor2D;
            ffi::renderer_add_actor2d(self.ptr.as_mut(), actor_ptr);
        }
    }

    /// Add a legend box actor to the renderer
    pub fn add_legend_box(&mut self, actor: &mut crate::LegendBoxActor) {
        unsafe {
            let actor_ptr = actor.as_raw_ptr() as *mut ffi::vtkActor2D;
            ffi::renderer_add_actor2d(self.ptr.as_mut(), actor_ptr);
        }
    }

    /// Add a follower (billboard actor) to the renderer.
    /// vtkFollower inherits from vtkActor, so this is type-safe.
    pub fn add_follower(&mut self, follower: &mut crate::Follower) {
        unsafe {
            let actor_ptr = follower.as_raw_ptr() as *mut ffi::vtkActor;
            ffi::renderer_add_actor(self.ptr.as_mut(), actor_ptr);
        }
    }

    pub fn set_background(&mut self, r: f64, g: f64, b: f64) {
        ffi::renderer_set_background(self.ptr.as_mut(), r, g, b);
    }

    /// Gets the active camera for this renderer.
    /// Returns a non-owning wrapper - the camera is managed by the renderer.
    pub fn get_active_camera(&mut self) -> crate::CameraRef {
        let ptr = ffi::renderer_get_active_camera(self.ptr.as_mut());
        crate::CameraRef::from_raw_ptr(ptr as *mut _)
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

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkRenderer`](https://vtk.org/doc/nightly/html/classvtkRenderer.html)
#[allow(non_camel_case_types)]
pub trait vtkRenderer: private::Sealed {}
