use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_image_data.h");
        include!("vtk_algorithm_output.h");

        type vtkImageData;
        type vtkAlgorithmOutput;

        fn vtk_image_data_new() -> *mut vtkImageData;
        fn vtk_image_data_delete(image_data: Pin<&mut vtkImageData>);

        fn image_data_set_dimensions(image_data: Pin<&mut vtkImageData>, nx: i32, ny: i32, nz: i32);
        unsafe fn image_data_get_dimensions(
            image_data: &vtkImageData,
            nx: *mut i32,
            ny: *mut i32,
            nz: *mut i32
        );

        fn image_data_set_spacing(image_data: Pin<&mut vtkImageData>, dx: f64, dy: f64, dz: f64);
        unsafe fn image_data_get_spacing(
            image_data: &vtkImageData,
            dx: *mut f64,
            dy: *mut f64,
            dz: *mut f64
        );

        fn image_data_set_origin(image_data: Pin<&mut vtkImageData>, x: f64, y: f64, z: f64);
        unsafe fn image_data_get_origin(
            image_data: &vtkImageData,
            x: *mut f64,
            y: *mut f64,
            z: *mut f64
        );

        fn image_data_allocate_scalars(
            image_data: Pin<&mut vtkImageData>,
            vtk_type: i32,
            num_components: i32
        );

        fn image_data_set_scalar_component_from_double(
            image_data: Pin<&mut vtkImageData>,
            x: i32,
            y: i32,
            z: i32,
            component: i32,
            value: f64
        );

        fn image_data_get_scalar_component_as_double(
            image_data: &vtkImageData,
            x: i32,
            y: i32,
            z: i32,
            component: i32
        ) -> f64;

        fn image_data_get_number_of_points(image_data: &vtkImageData) -> i32;
        fn image_data_get_number_of_cells(image_data: &vtkImageData) -> i32;
        unsafe fn image_data_get_bounds(image_data: &vtkImageData, bounds: *mut f64);
    }
}

/// VTK scalar data types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VtkDataType {
    Float = 10,
    Double = 11,
    Int8 = 15,
    UInt8 = 3,
    Int16 = 4,
    UInt16 = 5,
    Int32 = 6,
    UInt32 = 7,
    Int64 = 16,
    UInt64 = 17,
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkImageData.html",
    @name ImageData, ffi::vtkImageData,
    @new ffi::vtk_image_data_new,
    @delete ffi::vtk_image_data_delete
);

impl ImageData {
    /// Set the dimensions (number of points in each direction)
    #[doc(alias = "SetDimensions")]
    pub fn set_dimensions(&mut self, nx: i32, ny: i32, nz: i32) {
        ffi::image_data_set_dimensions(self.ptr.as_mut(), nx, ny, nz);
    }

    /// Get the dimensions
    #[doc(alias = "GetDimensions")]
    pub fn get_dimensions(&self) -> (i32, i32, i32) {
        let mut nx = 0;
        let mut ny = 0;
        let mut nz = 0;
        unsafe {
            ffi::image_data_get_dimensions(self.ptr.as_ref().get_ref(), &mut nx, &mut ny, &mut nz);
        }
        (nx, ny, nz)
    }

    /// Set the spacing between voxels
    #[doc(alias = "SetSpacing")]
    pub fn set_spacing(&mut self, dx: f64, dy: f64, dz: f64) {
        ffi::image_data_set_spacing(self.ptr.as_mut(), dx, dy, dz);
    }

    /// Get the spacing
    #[doc(alias = "GetSpacing")]
    pub fn get_spacing(&self) -> (f64, f64, f64) {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        unsafe {
            ffi::image_data_get_spacing(self.ptr.as_ref().get_ref(), &mut dx, &mut dy, &mut dz);
        }
        (dx, dy, dz)
    }

    /// Set the origin (bottom-left-back corner in world coordinates)
    #[doc(alias = "SetOrigin")]
    pub fn set_origin(&mut self, x: f64, y: f64, z: f64) {
        ffi::image_data_set_origin(self.ptr.as_mut(), x, y, z);
    }

    /// Get the origin
    #[doc(alias = "GetOrigin")]
    pub fn get_origin(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            ffi::image_data_get_origin(self.ptr.as_ref().get_ref(), &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Allocate memory for scalar data
    #[doc(alias = "AllocateScalars")]
    pub fn allocate_scalars(&mut self, data_type: VtkDataType, num_components: i32) {
        ffi::image_data_allocate_scalars(self.ptr.as_mut(), data_type as i32, num_components);
    }

    /// Set a scalar value at specific voxel and component
    #[doc(alias = "SetScalarComponentFromDouble")]
    pub fn set_scalar_component(&mut self, x: i32, y: i32, z: i32, component: i32, value: f64) {
        ffi::image_data_set_scalar_component_from_double(
            self.ptr.as_mut(),
            x,
            y,
            z,
            component,
            value
        );
    }

    /// Get a scalar value at specific voxel and component
    #[doc(alias = "GetScalarComponentAsDouble")]
    pub fn get_scalar_component(&self, x: i32, y: i32, z: i32, component: i32) -> f64 {
        ffi::image_data_get_scalar_component_as_double(
            self.ptr.as_ref().get_ref(),
            x,
            y,
            z,
            component
        )
    }

    /// Get the number of points in the dataset
    #[doc(alias = "GetNumberOfPoints")]
    pub fn get_number_of_points(&self) -> i32 {
        ffi::image_data_get_number_of_points(self.ptr.as_ref().get_ref())
    }

    /// Get the number of cells in the dataset
    #[doc(alias = "GetNumberOfCells")]
    pub fn get_number_of_cells(&self) -> i32 {
        ffi::image_data_get_number_of_cells(self.ptr.as_ref().get_ref())
    }

    /// Get the bounding box
    #[doc(alias = "GetBounds")]
    pub fn get_bounds(&self) -> [f64; 6] {
        let mut bounds = [0.0; 6];
        unsafe {
            ffi::image_data_get_bounds(self.ptr.as_ref().get_ref(), bounds.as_mut_ptr());
        }
        bounds
    }
    
    /// Get raw pointer for VTK pipeline connections
    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkImageData {
        unsafe { Pin::get_unchecked_mut(self.ptr.as_mut()) as *mut _ }
    }
}
