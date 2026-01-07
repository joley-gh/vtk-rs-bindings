#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_render_window_interactor.h");
        include!("vtk_render_window.h");

        type vtkRenderWindowInteractor;
        type vtkRenderWindow;
        type vtkInteractorStyle;

        fn render_window_interactor_new() -> *mut vtkRenderWindowInteractor;
        fn render_window_interactor_delete(interactor: Pin<&mut vtkRenderWindowInteractor>);
        unsafe fn render_window_interactor_set_render_window(
            interactor: Pin<&mut vtkRenderWindowInteractor>,
            window: *mut vtkRenderWindow
        );
        unsafe fn render_window_interactor_set_interactor_style(
            interactor: Pin<&mut vtkRenderWindowInteractor>,
            style: *mut vtkInteractorStyle
        );
        fn render_window_interactor_initialize(interactor: Pin<&mut vtkRenderWindowInteractor>);
        fn render_window_interactor_start(interactor: Pin<&mut vtkRenderWindowInteractor>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkRenderWindowInteractor.html",
    @name RenderWindowInteractor, ffi::vtkRenderWindowInteractor,
    @new ffi::render_window_interactor_new,
    @delete ffi::render_window_interactor_delete
);

impl RenderWindowInteractor {
    pub fn set_render_window(&mut self, window: &mut crate::RenderWindow) {
        unsafe {
            let window_ptr = window.as_mut_ptr() as *mut ffi::vtkRenderWindow;
            ffi::render_window_interactor_set_render_window(self.ptr.as_mut(), window_ptr);
        }
    }

    /// Sets the interactor style for mouse/keyboard interaction.
    /// For trackball-style camera controls (rotation, zoom, pan), use InteractorStyleTrackballCamera.
    pub fn set_interactor_style(&mut self, style: &mut crate::InteractorStyleTrackballCamera) {
        unsafe {
            let style_ptr = style.as_mut_ptr() as *mut ffi::vtkInteractorStyle;
            ffi::render_window_interactor_set_interactor_style(self.ptr.as_mut(), style_ptr);
        }
    }

    /// Sets a custom interactor style with event callbacks.
    pub fn set_interactor_style_custom(&mut self, style: &mut crate::InteractorStyleCustom) {
        unsafe {
            let style_ptr = style.as_mut_ptr() as *mut ffi::vtkInteractorStyle;
            ffi::render_window_interactor_set_interactor_style(self.ptr.as_mut(), style_ptr);
        }
    }

    /// Sets the interactor style for image interaction (pan, zoom).
    /// For image-specific interactions, use InteractorStyleImage.
    pub fn set_interactor_style_image(&mut self, style: &mut crate::InteractorStyleImage) {
        unsafe {
            let style_ptr = style.as_mut_ptr() as *mut ffi::vtkInteractorStyle;
            ffi::render_window_interactor_set_interactor_style(self.ptr.as_mut(), style_ptr);
        }
    }

    /// Sets the interactor style for actor trackball interaction.
    pub fn set_interactor_style_trackball_actor(
        &mut self,
        style: &mut crate::InteractorStyleTrackballActor
    ) {
        unsafe {
            let style_ptr = style.as_mut_ptr() as *mut ffi::vtkInteractorStyle;
            ffi::render_window_interactor_set_interactor_style(self.ptr.as_mut(), style_ptr);
        }
    }

    pub fn initialize(&mut self) {
        ffi::render_window_interactor_initialize(self.ptr.as_mut());
    }

    pub fn start(&mut self) {
        ffi::render_window_interactor_start(self.ptr.as_mut());
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkRenderWindowInteractor`](https://vtk.org/doc/nightly/html/classvtkRenderWindowInteractor.html)
#[allow(non_camel_case_types)]
pub trait vtkRenderWindowInteractor: private::Sealed {}
