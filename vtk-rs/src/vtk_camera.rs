#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_camera.h");

        type vtkCamera;

        fn camera_new() -> *mut vtkCamera;
        fn camera_delete(camera: Pin<&mut vtkCamera>);

        // Position and orientation
        fn camera_set_position(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_position(camera: Pin<&mut vtkCamera>, x: &mut f64, y: &mut f64, z: &mut f64);
        fn camera_set_focal_point(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_focal_point(
            camera: Pin<&mut vtkCamera>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn camera_set_view_up(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_view_up(camera: Pin<&mut vtkCamera>, x: &mut f64, y: &mut f64, z: &mut f64);

        // Camera movements
        fn camera_azimuth(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_elevation(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_roll(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_zoom(camera: Pin<&mut vtkCamera>, factor: f64);
        fn camera_dolly(camera: Pin<&mut vtkCamera>, factor: f64);

        // Clipping planes
        fn camera_set_clipping_range(camera: Pin<&mut vtkCamera>, near: f64, far: f64);
        fn camera_get_clipping_range(camera: Pin<&mut vtkCamera>, near: &mut f64, far: &mut f64);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCamera.html",
    @name Camera, ffi::vtkCamera,
    @new ffi::camera_new,
    @delete ffi::camera_delete
);

impl Camera {
    /// Set the position of the camera in world coordinates.
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_position(self.ptr.as_mut(), x, y, z);
    }

    /// Get the position of the camera in world coordinates.
    pub fn get_position(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_position(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Set the focal point of the camera in world coordinates.
    /// This is the point the camera is looking at.
    pub fn set_focal_point(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_focal_point(self.ptr.as_mut(), x, y, z);
    }

    /// Get the focal point of the camera in world coordinates.
    pub fn get_focal_point(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_focal_point(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Set the view up direction for the camera.
    /// This defines which direction is "up" for the camera.
    pub fn set_view_up(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_view_up(self.ptr.as_mut(), x, y, z);
    }

    /// Get the view up direction for the camera.
    pub fn get_view_up(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_view_up(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Rotate the camera about the view up vector centered at the focal point.
    /// Angle is in degrees.
    pub fn azimuth(&mut self, angle: f64) {
        ffi::camera_azimuth(self.ptr.as_mut(), angle);
    }

    /// Rotate the camera about the cross product of the view plane normal
    /// and the view up vector, centered at the focal point.
    /// Angle is in degrees.
    pub fn elevation(&mut self, angle: f64) {
        ffi::camera_elevation(self.ptr.as_mut(), angle);
    }

    /// Rotate the camera about the direction of projection.
    /// Angle is in degrees.
    pub fn roll(&mut self, angle: f64) {
        ffi::camera_roll(self.ptr.as_mut(), angle);
    }

    /// Change the camera's viewing angle (field of view).
    /// A factor > 1.0 zooms in, < 1.0 zooms out.
    pub fn zoom(&mut self, factor: f64) {
        ffi::camera_zoom(self.ptr.as_mut(), factor);
    }

    /// Move the camera toward (factor > 1.0) or away from (factor < 1.0) the focal point.
    pub fn dolly(&mut self, factor: f64) {
        ffi::camera_dolly(self.ptr.as_mut(), factor);
    }

    /// Set the near and far clipping planes.
    pub fn set_clipping_range(&mut self, near: f64, far: f64) {
        ffi::camera_set_clipping_range(self.ptr.as_mut(), near, far);
    }

    /// Get the near and far clipping planes.
    pub fn get_clipping_range(&mut self) -> (f64, f64) {
        let (mut near, mut far) = (0.0, 0.0);
        ffi::camera_get_clipping_range(self.ptr.as_mut(), &mut near, &mut far);
        (near, far)
    }

    /// Get raw pointer for FFI (internal use)
    pub(crate) fn as_raw_ptr(&mut self) -> *mut ffi::vtkCamera {
        self.as_mut_ptr()
    }

    /// Get raw pointer as vtkObject for observer support
    pub(crate) fn as_vtk_object_ptr(&mut self) -> *mut crate::vtk_command::ffi::vtkObject {
        self.as_mut_ptr() as *mut crate::vtk_command::ffi::vtkObject
    }
}

impl crate::vtk_command::Observable for Camera {
    unsafe fn add_observer(&mut self, event: usize, command: &mut crate::Command) -> usize {
        use core::pin::Pin;
        let obj_ptr = self.as_vtk_object_ptr();
        let obj_ref = Pin::new_unchecked(&mut *obj_ptr);
        crate::vtk_command::ffi::vtk_object_add_observer(obj_ref, event, command.as_mut())
    }

    unsafe fn remove_observer(&mut self, tag: usize) {
        use core::pin::Pin;
        let obj_ptr = self.as_vtk_object_ptr();
        let obj_ref = Pin::new_unchecked(&mut *obj_ptr);
        crate::vtk_command::ffi::vtk_object_remove_observer(obj_ref, tag);
    }
}

/// A non-owning reference to a Camera managed by a Renderer.
/// This does not delete the camera when dropped.
pub struct CameraRef {
    ptr: *mut ffi::vtkCamera,
}

impl CameraRef {
    /// Create a CameraRef from a raw vtkCamera pointer.
    /// This is a non-owning reference - the camera must remain valid for the lifetime of this ref.
    pub fn from_raw_ptr(ptr: *mut ffi::vtkCamera) -> Self {
        Self { ptr }
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::vtkCamera {
        self.ptr
    }

    /// Set the position of the camera in world coordinates.
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_set_position(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the position of the camera in world coordinates.
    pub fn get_position(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_get_position(
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
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_set_focal_point(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the focal point of the camera in world coordinates.
    pub fn get_focal_point(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_get_focal_point(
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
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_set_view_up(std::pin::Pin::new_unchecked(camera_ref), x, y, z);
        }
    }

    /// Get the view up vector of the camera.
    pub fn get_view_up(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_get_view_up(
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
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_azimuth(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Rotate the camera about the cross product of the view plane normal and view up vector.
    pub fn elevation(&mut self, degrees: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_elevation(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Rotate the camera about the view plane normal.
    pub fn roll(&mut self, degrees: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_roll(std::pin::Pin::new_unchecked(camera_ref), degrees);
        }
    }

    /// Change the view angle by a factor (zoom in/out by changing field of view).
    pub fn zoom(&mut self, factor: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_zoom(std::pin::Pin::new_unchecked(camera_ref), factor);
        }
    }

    /// Move the camera closer/farther from the focal point (zoom by moving camera).
    pub fn dolly(&mut self, factor: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_dolly(std::pin::Pin::new_unchecked(camera_ref), factor);
        }
    }

    /// Set the near and far clipping planes.
    pub fn set_clipping_range(&mut self, near: f64, far: f64) {
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_set_clipping_range(std::pin::Pin::new_unchecked(camera_ref), near, far);
        }
    }

    /// Get the near and far clipping planes.
    pub fn get_clipping_range(&mut self) -> (f64, f64) {
        let (mut near, mut far) = (0.0, 0.0);
        unsafe {
            let camera_ref = &mut *self.ptr;
            ffi::camera_get_clipping_range(
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

/// [`vtkCamera`](https://vtk.org/doc/nightly/html/classvtkCamera.html)
///
/// A virtual camera for 3D rendering.
#[allow(non_camel_case_types)]
pub trait vtkCamera: private::Sealed {}
