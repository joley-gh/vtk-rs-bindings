#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_render_window.h");
        include!("vtk_renderer.h");
        include!("vtk_command.h");

        type vtkRenderWindow;
        type vtkRenderer;
        type vtkCommand;

        fn render_window_new() -> *mut vtkRenderWindow;
        fn render_window_delete(window: Pin<&mut vtkRenderWindow>);
        unsafe fn render_window_add_renderer(
            window: Pin<&mut vtkRenderWindow>,
            renderer: *mut vtkRenderer
        );
        fn render_window_set_size(window: Pin<&mut vtkRenderWindow>, width: i32, height: i32);
        fn render_window_set_window_name(window: Pin<&mut vtkRenderWindow>, name: &str);
        fn render_window_render(window: Pin<&mut vtkRenderWindow>);
        fn render_window_get_size(
            window: Pin<&mut vtkRenderWindow>,
            width: &mut i32,
            height: &mut i32
        );
        unsafe fn render_window_get_pixel_data(
            window: Pin<&mut vtkRenderWindow>,
            data: *mut u8,
            size: i32
        );
        unsafe fn render_window_set_pixel_data(
            window: Pin<&mut vtkRenderWindow>,
            data: *const u8,
            size: i32
        );
        unsafe fn render_window_add_observer(
            window: Pin<&mut vtkRenderWindow>,
            event: usize,
            command: *mut vtkCommand
        ) -> usize;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkRenderWindow.html",
    @name RenderWindow, ffi::vtkRenderWindow,
    @new ffi::render_window_new,
    @delete ffi::render_window_delete
);

impl RenderWindow {
    pub fn add_renderer(&mut self, renderer: &mut crate::Renderer) {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::render_window_add_renderer(self.ptr.as_mut(), renderer_ptr);
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        ffi::render_window_set_size(self.ptr.as_mut(), width, height);
    }

    pub fn set_window_name(&mut self, name: &str) {
        ffi::render_window_set_window_name(self.ptr.as_mut(), name);
    }

    pub fn render(&mut self) {
        ffi::render_window_render(self.ptr.as_mut());
    }

    pub fn get_size(&mut self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        ffi::render_window_get_size(self.ptr.as_mut(), &mut width, &mut height);
        (width, height)
    }

    /// Get RGBA pixel data from the render window
    pub fn get_pixel_data(&mut self) -> Vec<u8> {
        let (width, height) = self.get_size();
        let size = (width * height * 4) as usize;
        let mut data = vec![0u8; size];

        unsafe {
            ffi::render_window_get_pixel_data(self.ptr.as_mut(), data.as_mut_ptr(), size as i32);
        }

        data
    }

    /// Set RGBA pixel data to the render window (writes to front buffer and displays immediately)
    pub fn set_pixel_data(&mut self, data: &[u8]) {
        unsafe {
            ffi::render_window_set_pixel_data(self.ptr.as_mut(), data.as_ptr(), data.len() as i32);
        }
    }

    /// Add observer for window events (internal use)
    pub(crate) fn add_observer_raw(
        &mut self,
        event: usize,
        command: *mut ffi::vtkCommand
    ) -> usize {
        unsafe { ffi::render_window_add_observer(self.ptr.as_mut(), event, command) }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkRenderWindow`](https://vtk.org/doc/nightly/html/classvtkRenderWindow.html)
#[allow(non_camel_case_types)]
pub trait vtkRenderWindow: private::Sealed {}
