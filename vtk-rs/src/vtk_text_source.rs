use std::pin::Pin;

use crate::algorithm_output_port::AlgorithmOutputPort;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_text_source.h");

        type vtkTextSource;

        fn vtk_text_source_new() -> *mut vtkTextSource;
        fn vtk_text_source_delete(ptr: Pin<&mut vtkTextSource>);

        fn text_source_set_text(source: Pin<&mut vtkTextSource>, text: String);
        fn text_source_get_text(source: Pin<&mut vtkTextSource>) -> String;

        fn text_source_set_backing(source: Pin<&mut vtkTextSource>, backing: bool);
        fn text_source_get_backing(source: Pin<&mut vtkTextSource>) -> bool;

        fn text_source_get_output_port(source: Pin<&mut vtkTextSource>) -> *mut vtkAlgorithmOutput;

        type vtkAlgorithmOutput;
    }
}

pub struct TextSource {
    ptr: *mut ffi::vtkTextSource,
}

impl TextSource {
    pub fn new() -> Self {
        let ptr = ffi::vtk_text_source_new();
        Self { ptr }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkTextSource> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    pub fn set_text(&mut self, text: &str) {
        ffi::text_source_set_text(self.as_mut(), text.to_string());
    }

    pub fn get_text(&mut self) -> String {
        ffi::text_source_get_text(self.as_mut())
    }

    pub fn set_backing(&mut self, backing: bool) {
        ffi::text_source_set_backing(self.as_mut(), backing);
    }

    pub fn get_backing(&mut self) -> bool {
        ffi::text_source_get_backing(self.as_mut())
    }

    pub fn get_output_port(&mut self) -> AlgorithmOutputPort {
        let ptr = ffi::text_source_get_output_port(self.as_mut());
        unsafe { crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void) }
    }
}

impl Drop for TextSource {
    fn drop(&mut self) {
        ffi::vtk_text_source_delete(self.as_mut());
    }
}
