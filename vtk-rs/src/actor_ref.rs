use std::ptr::NonNull;

/// Non-owning handle to a `vtkActor` returned to callbacks.
/// This does not take ownership; callers must not drop the underlying object.
pub struct ActorRef {
    ptr: NonNull<crate::vtk_actor::ffi::vtkActor>,
}

impl ActorRef {
    /// Create an ActorRef from a raw pointer. Returns None if the pointer is null.
    pub fn from_raw(ptr: *mut crate::vtk_actor::ffi::vtkActor) -> Option<Self> {
        NonNull::new(ptr).map(|nn| ActorRef { ptr: nn })
    }

    /// Get position as (x,y,z)
    pub fn position(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        // SAFETY: caller guarantees pointer is valid for the duration of the callback
        unsafe {
            crate::vtk_actor::ffi::actor_get_position(self.ptr.as_ref(), &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Set position of the actor.
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            crate::vtk_actor::ffi::actor_set_position(
                std::pin::Pin::new_unchecked(&mut *(self.ptr.as_ptr() as *mut _)),
                x,
                y,
                z
            );
        }
    }

    /// Get orientation (pitch, yaw, roll)
    pub fn orientation(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            crate::vtk_actor::ffi::actor_get_orientation(self.ptr.as_ref(), &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Set orientation
    pub fn set_orientation(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            crate::vtk_actor::ffi::actor_set_orientation(
                std::pin::Pin::new_unchecked(&mut *(self.ptr.as_ptr() as *mut _)),
                x,
                y,
                z
            );
        }
    }
}
