#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_superquadric_source.h");
        include!("vtk_algorithm_output.h");

        type vtkSuperquadricSource;
        type vtkAlgorithmOutput;

        fn vtk_superquadric_source_new() -> *mut vtkSuperquadricSource;
        fn vtk_superquadric_source_delete(ptr: Pin<&mut vtkSuperquadricSource>);

        fn superquadric_source_set_center(
            source: Pin<&mut vtkSuperquadricSource>,
            x: f64,
            y: f64,
            z: f64
        );
        fn superquadric_source_get_center(
            source: Pin<&mut vtkSuperquadricSource>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn superquadric_source_set_scale(
            source: Pin<&mut vtkSuperquadricSource>,
            x: f64,
            y: f64,
            z: f64
        );
        fn superquadric_source_get_scale(
            source: Pin<&mut vtkSuperquadricSource>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn superquadric_source_set_size(source: Pin<&mut vtkSuperquadricSource>, size: f64);
        fn superquadric_source_get_size(source: Pin<&mut vtkSuperquadricSource>) -> f64;

        fn superquadric_source_set_theta_roundness(
            source: Pin<&mut vtkSuperquadricSource>,
            roundness: f64
        );
        fn superquadric_source_get_theta_roundness(source: Pin<&mut vtkSuperquadricSource>) -> f64;

        fn superquadric_source_set_phi_roundness(
            source: Pin<&mut vtkSuperquadricSource>,
            roundness: f64
        );
        fn superquadric_source_get_phi_roundness(source: Pin<&mut vtkSuperquadricSource>) -> f64;

        fn superquadric_source_set_thickness(
            source: Pin<&mut vtkSuperquadricSource>,
            thickness: f64
        );
        fn superquadric_source_get_thickness(source: Pin<&mut vtkSuperquadricSource>) -> f64;

        fn superquadric_source_set_toroidal(
            source: Pin<&mut vtkSuperquadricSource>,
            toroidal: bool
        );
        fn superquadric_source_get_toroidal(source: Pin<&mut vtkSuperquadricSource>) -> bool;
        fn superquadric_source_toroidal_on(source: Pin<&mut vtkSuperquadricSource>);
        fn superquadric_source_toroidal_off(source: Pin<&mut vtkSuperquadricSource>);

        fn superquadric_source_set_theta_resolution(
            source: Pin<&mut vtkSuperquadricSource>,
            resolution: i32
        );
        fn superquadric_source_get_theta_resolution(source: Pin<&mut vtkSuperquadricSource>) -> i32;

        fn superquadric_source_set_phi_resolution(
            source: Pin<&mut vtkSuperquadricSource>,
            resolution: i32
        );
        fn superquadric_source_get_phi_resolution(source: Pin<&mut vtkSuperquadricSource>) -> i32;

        unsafe fn superquadric_source_get_output_port(
            source: Pin<&mut vtkSuperquadricSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkSuperquadricSource.html",
    @name SuperquadricSource, ffi::vtkSuperquadricSource,
    @new ffi::vtk_superquadric_source_new,
    @delete ffi::vtk_superquadric_source_delete,
    @inherit vtkPolyDataAlgorithm
);

impl SuperquadricSource {
    /// Set the center of the superquadric
    #[doc(alias = "SetCenter")]
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::superquadric_source_set_center(self.ptr.as_mut(), x, y, z)
    }

    /// Get the center of the superquadric
    #[doc(alias = "GetCenter")]
    pub fn get_center(&mut self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::superquadric_source_get_center(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }

    /// Set the scale factors for X, Y, Z axes
    #[doc(alias = "SetScale")]
    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) {
        ffi::superquadric_source_set_scale(self.ptr.as_mut(), x, y, z)
    }

    /// Get the scale factors for X, Y, Z axes
    #[doc(alias = "GetScale")]
    pub fn get_scale(&mut self) -> [f64; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::superquadric_source_get_scale(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        [x, y, z]
    }

    /// Set the uniform size of the superquadric
    #[doc(alias = "SetSize")]
    pub fn set_size(&mut self, size: f64) {
        ffi::superquadric_source_set_size(self.ptr.as_mut(), size)
    }

    /// Get the uniform size of the superquadric
    #[doc(alias = "GetSize")]
    pub fn get_size(&mut self) -> f64 {
        ffi::superquadric_source_get_size(self.ptr.as_mut())
    }

    /// Set the roundness in the theta (east-west) direction
    ///
    /// Values:
    /// - 0.0 = rectangular
    /// - 1.0 = circular (default)
    /// - Values between 0 and 1 create smooth transitions
    #[doc(alias = "SetThetaRoundness")]
    pub fn set_theta_roundness(&mut self, roundness: f64) {
        ffi::superquadric_source_set_theta_roundness(self.ptr.as_mut(), roundness)
    }

    /// Get the roundness in the theta direction
    #[doc(alias = "GetThetaRoundness")]
    pub fn get_theta_roundness(&mut self) -> f64 {
        ffi::superquadric_source_get_theta_roundness(self.ptr.as_mut())
    }

    /// Set the roundness in the phi (north-south) direction
    ///
    /// Values:
    /// - 0.0 = rectangular
    /// - 1.0 = circular (default)
    /// - Values between 0 and 1 create smooth transitions
    #[doc(alias = "SetPhiRoundness")]
    pub fn set_phi_roundness(&mut self, roundness: f64) {
        ffi::superquadric_source_set_phi_roundness(self.ptr.as_mut(), roundness)
    }

    /// Get the roundness in the phi direction
    #[doc(alias = "GetPhiRoundness")]
    pub fn get_phi_roundness(&mut self) -> f64 {
        ffi::superquadric_source_get_phi_roundness(self.ptr.as_mut())
    }

    /// Set the thickness (only used when toroidal is true)
    ///
    /// Controls the "hole" size in toroidal mode
    #[doc(alias = "SetThickness")]
    pub fn set_thickness(&mut self, thickness: f64) {
        ffi::superquadric_source_set_thickness(self.ptr.as_mut(), thickness)
    }

    /// Get the thickness
    #[doc(alias = "GetThickness")]
    pub fn get_thickness(&mut self) -> f64 {
        ffi::superquadric_source_get_thickness(self.ptr.as_mut())
    }

    /// Set whether to create a toroidal (donut-like) superquadric
    ///
    /// - false = ellipsoid mode (default)
    /// - true = toroid mode (creates hole in center)
    #[doc(alias = "SetToroidal")]
    pub fn set_toroidal(&mut self, toroidal: bool) {
        ffi::superquadric_source_set_toroidal(self.ptr.as_mut(), toroidal)
    }

    /// Get whether toroidal mode is enabled
    #[doc(alias = "GetToroidal")]
    pub fn get_toroidal(&mut self) -> bool {
        ffi::superquadric_source_get_toroidal(self.ptr.as_mut())
    }

    /// Enable toroidal mode
    #[doc(alias = "ToroidalOn")]
    pub fn toroidal_on(&mut self) {
        ffi::superquadric_source_toroidal_on(self.ptr.as_mut())
    }

    /// Disable toroidal mode
    #[doc(alias = "ToroidalOff")]
    pub fn toroidal_off(&mut self) {
        ffi::superquadric_source_toroidal_off(self.ptr.as_mut())
    }

    /// Set the resolution in the theta (east-west) direction
    #[doc(alias = "SetThetaResolution")]
    pub fn set_theta_resolution(&mut self, resolution: i32) {
        ffi::superquadric_source_set_theta_resolution(self.ptr.as_mut(), resolution)
    }

    /// Get the resolution in the theta direction
    #[doc(alias = "GetThetaResolution")]
    pub fn get_theta_resolution(&mut self) -> i32 {
        ffi::superquadric_source_get_theta_resolution(self.ptr.as_mut())
    }

    /// Set the resolution in the phi (north-south) direction
    #[doc(alias = "SetPhiResolution")]
    pub fn set_phi_resolution(&mut self, resolution: i32) {
        ffi::superquadric_source_set_phi_resolution(self.ptr.as_mut(), resolution)
    }

    /// Get the resolution in the phi direction
    #[doc(alias = "GetPhiResolution")]
    pub fn get_phi_resolution(&mut self) -> i32 {
        ffi::superquadric_source_get_phi_resolution(self.ptr.as_mut())
    }

    /// Get the output port for connecting to a mapper
    #[doc(alias = "GetOutputPort")]
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::superquadric_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
