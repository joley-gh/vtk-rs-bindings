use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_follower.h");

        type vtkFollower;

        fn vtk_follower_new() -> *mut vtkFollower;
        fn vtk_follower_delete(ptr: Pin<&mut vtkFollower>);

        unsafe fn follower_set_camera(follower: Pin<&mut vtkFollower>, camera: usize);
        unsafe fn follower_set_mapper(follower: Pin<&mut vtkFollower>, mapper: usize);

        fn follower_set_position(follower: Pin<&mut vtkFollower>, x: f64, y: f64, z: f64);
        fn follower_get_position(
            follower: Pin<&mut vtkFollower>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn follower_set_scale(follower: Pin<&mut vtkFollower>, x: f64, y: f64, z: f64);
        fn follower_get_scale(
            follower: Pin<&mut vtkFollower>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
    }
}

/// A Follower is an actor that always faces the camera.
/// Perfect for text labels that need to follow 3D points (like FEM node IDs).
pub struct Follower {
    ptr: *mut ffi::vtkFollower,
}

impl Follower {
    pub fn new() -> Self {
        let ptr = ffi::vtk_follower_new();
        Self { ptr }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkFollower> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    pub fn set_camera(&mut self, camera: &mut crate::Camera) {
        unsafe {
            ffi::follower_set_camera(self.as_mut(), camera.as_raw_ptr() as usize);
        }
    }

    pub fn set_camera_ref(&mut self, camera: &mut crate::CameraRef) {
        unsafe {
            ffi::follower_set_camera(self.as_mut(), camera.as_mut_ptr() as usize);
        }
    }

    pub fn set_mapper(&mut self, mapper: &mut crate::PolyDataMapper) {
        unsafe {
            ffi::follower_set_mapper(self.as_mut(), mapper.as_raw_ptr() as usize);
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        ffi::follower_set_position(self.as_mut(), x, y, z);
    }

    pub fn get_position(&mut self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::follower_get_position(self.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }

    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) {
        ffi::follower_set_scale(self.as_mut(), x, y, z);
    }

    pub fn get_scale(&mut self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::follower_get_scale(self.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }

    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkFollower {
        self.ptr
    }

    /// Convert this Follower to an Actor reference for adding to renderer.
    /// Since vtkFollower inherits from vtkActor, this is safe.
    pub fn as_actor(&mut self) -> &mut crate::Actor {
        unsafe {
            // vtkFollower* is compatible with vtkActor* due to inheritance
            &mut *(self.ptr as *mut crate::Actor)
        }
    }
}

impl Drop for Follower {
    fn drop(&mut self) {
        ffi::vtk_follower_delete(self.as_mut());
    }
}

/// A non-owning, Send-safe reference to a Follower.
/// Useful for capturing followers in closures for callbacks/observers.
///
/// # Safety
/// The follower must remain valid for the lifetime of this reference.
/// This type is Send to allow use in callbacks, but the user must ensure
/// the follower is not accessed from multiple threads simultaneously.
pub struct FollowerRef {
    ptr: *mut ffi::vtkFollower,
}

unsafe impl Send for FollowerRef {}

impl FollowerRef {
    /// Create a FollowerRef from a mutable reference to a Follower.
    /// The follower must remain valid for the lifetime of this reference.
    pub fn from_follower(follower: &mut Follower) -> Self {
        Self { ptr: follower.ptr }
    }

    fn as_mut(&self) -> Pin<&mut ffi::vtkFollower> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Convenience method to set position without dereferencing.
    pub fn set_position(&self, x: f64, y: f64, z: f64) {
        ffi::follower_set_position(self.as_mut(), x, y, z)
    }

    /// Convenience method to get position without dereferencing.
    pub fn get_position(&self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::follower_get_position(self.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }

    /// Convenience method to set scale without dereferencing.
    pub fn set_scale(&self, x: f64, y: f64, z: f64) {
        ffi::follower_set_scale(self.as_mut(), x, y, z)
    }

    /// Convenience method to get scale without dereferencing.
    pub fn get_scale(&self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::follower_get_scale(self.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }
}

// Implement AddableToRenderer for Follower (3D actor - inherits from vtkActor)
impl crate::AddableToRenderer for Follower {
    fn add_to_renderer_internal(&mut self, renderer: &mut crate::Renderer) {
        let actor_ptr = self.as_raw_ptr() as *mut crate::vtk_renderer::ffi::vtkActor;
        renderer._add_actor_3d(actor_ptr);
    }
}
