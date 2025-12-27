use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_plane.h");

        type vtkPlane;

        fn vtk_plane_new() -> *mut vtkPlane;
        fn vtk_plane_delete(plane: Pin<&mut vtkPlane>);

        fn plane_set_origin(plane: Pin<&mut vtkPlane>, x: f64, y: f64, z: f64);
        unsafe fn plane_get_origin(plane: &vtkPlane, x: *mut f64, y: *mut f64, z: *mut f64);

        fn plane_set_normal(plane: Pin<&mut vtkPlane>, nx: f64, ny: f64, nz: f64);
        unsafe fn plane_get_normal(plane: &vtkPlane, nx: *mut f64, ny: *mut f64, nz: *mut f64);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPlane.html",
    @name Plane, ffi::vtkPlane,
    @new ffi::vtk_plane_new,
    @delete ffi::vtk_plane_delete
);

impl Plane {
    /// Set the plane origin
    #[doc(alias = "SetOrigin")]
    pub fn set_origin(&mut self, x: f64, y: f64, z: f64) {
        ffi::plane_set_origin(self.ptr.as_mut(), x, y, z);
    }

    /// Get the plane origin
    #[doc(alias = "GetOrigin")]
    pub fn get_origin(&self) -> [f64; 3] {
        let mut origin = [0.0; 3];
        unsafe {
            ffi::plane_get_origin(
                self.ptr.as_ref().get_ref(),
                &mut origin[0],
                &mut origin[1],
                &mut origin[2]
            );
        }
        origin
    }

    /// Set the plane normal vector
    #[doc(alias = "SetNormal")]
    pub fn set_normal(&mut self, nx: f64, ny: f64, nz: f64) {
        ffi::plane_set_normal(self.ptr.as_mut(), nx, ny, nz);
    }

    /// Get the plane normal vector
    #[doc(alias = "GetNormal")]
    pub fn get_normal(&self) -> [f64; 3] {
        let mut normal = [0.0; 3];
        unsafe {
            ffi::plane_get_normal(
                self.ptr.as_ref().get_ref(),
                &mut normal[0],
                &mut normal[1],
                &mut normal[2]
            );
        }
        normal
    }

    /// Get raw pointer for VTK pipeline connections
    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkPlane {
        unsafe { Pin::get_unchecked_mut(self.ptr.as_mut()) as *mut _ }
    }
}
