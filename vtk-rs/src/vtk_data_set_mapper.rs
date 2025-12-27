use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_data_set_mapper.h");
        include!("vtk_mapper.h");

        type vtkDataSetMapper;
        type vtkMapper;
        type vtkDataSet;

        fn vtk_data_set_mapper_new() -> *mut vtkDataSetMapper;
        fn vtk_data_set_mapper_delete(mapper: Pin<&mut vtkDataSetMapper>);

        unsafe fn data_set_mapper_set_input_data(
            mapper: Pin<&mut vtkDataSetMapper>,
            data_set: *mut vtkDataSet
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkDataSetMapper.html",
    @name DataSetMapper, ffi::vtkDataSetMapper,
    @new ffi::vtk_data_set_mapper_new,
    @delete ffi::vtk_data_set_mapper_delete,
    @inherit vtkMapper
);

impl DataSetMapper {
    /// Sets the input data from any VTK dataset (PolyData, UnstructuredGrid, ImageData, etc.)
    #[doc(alias = "SetInputData")]
    pub fn set_input_data(&mut self, data_set: &mut crate::UnstructuredGrid) {
        let ptr = data_set.as_raw_ptr() as *mut ffi::vtkDataSet;
        unsafe {
            ffi::data_set_mapper_set_input_data(self.ptr.as_mut(), ptr);
        }
    }

    /// Get raw pointer for VTK pipeline connections
    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkDataSetMapper {
        unsafe { Pin::get_unchecked_mut(self.ptr.as_mut()) as *mut _ }
    }

    /// Get mapper base pointer for Actor
    pub fn as_mapper_ptr(&mut self) -> *mut ffi::vtkMapper {
        self.as_raw_ptr() as *mut ffi::vtkMapper
    }
}
