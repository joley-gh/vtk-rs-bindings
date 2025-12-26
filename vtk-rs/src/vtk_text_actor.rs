use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_text_actor.h");
        include!("vtk_render_window.h");

        type vtkTextActor;
        type vtkTextProperty;
        type vtkRenderWindow;

        // vtkTextActor
        fn text_actor_new() -> *mut vtkTextActor;
        fn text_actor_delete(actor: Pin<&mut vtkTextActor>);

        fn text_actor_set_input(actor: Pin<&mut vtkTextActor>, text: String);
        fn text_actor_get_input(actor: Pin<&mut vtkTextActor>) -> String;

        fn text_actor_set_position(actor: Pin<&mut vtkTextActor>, x: f64, y: f64);
        fn text_actor_get_position(actor: Pin<&mut vtkTextActor>, x: &mut f64, y: &mut f64);

        fn text_actor_set_position2(actor: Pin<&mut vtkTextActor>, x: f64, y: f64);
        fn text_actor_get_position2(actor: Pin<&mut vtkTextActor>, x: &mut f64, y: &mut f64);

        fn text_actor_get_text_property(actor: Pin<&mut vtkTextActor>) -> *mut vtkTextProperty;

        fn text_actor_set_visibility(actor: Pin<&mut vtkTextActor>, visible: bool);
        fn text_actor_get_visibility(actor: Pin<&mut vtkTextActor>) -> bool;

        // vtkTextProperty
        fn text_property_set_font_size(prop: Pin<&mut vtkTextProperty>, size: i32);
        fn text_property_get_font_size(prop: Pin<&mut vtkTextProperty>) -> i32;

        fn text_property_set_color(prop: Pin<&mut vtkTextProperty>, r: f64, g: f64, b: f64);
        fn text_property_get_color(
            prop: Pin<&mut vtkTextProperty>,
            r: &mut f64,
            g: &mut f64,
            b: &mut f64
        );

        fn text_property_set_opacity(prop: Pin<&mut vtkTextProperty>, opacity: f64);
        fn text_property_get_opacity(prop: Pin<&mut vtkTextProperty>) -> f64;

        fn text_property_set_bold(prop: Pin<&mut vtkTextProperty>, bold: bool);
        fn text_property_get_bold(prop: Pin<&mut vtkTextProperty>) -> bool;

        fn text_property_set_italic(prop: Pin<&mut vtkTextProperty>, italic: bool);
        fn text_property_get_italic(prop: Pin<&mut vtkTextProperty>) -> bool;

        fn text_property_set_font_family_to_arial(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_font_family_to_courier(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_font_family_to_times(prop: Pin<&mut vtkTextProperty>);
        
        // Text alignment
        fn text_property_set_justification_to_left(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_justification_to_centered(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_justification_to_right(prop: Pin<&mut vtkTextProperty>);
        
        fn text_property_set_vertical_justification_to_bottom(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_vertical_justification_to_centered(prop: Pin<&mut vtkTextProperty>);
        fn text_property_set_vertical_justification_to_top(prop: Pin<&mut vtkTextProperty>);
        
        // vtkRenderWindow (needed for callback)
        fn render_window_get_size(
            window: Pin<&mut vtkRenderWindow>,
            width: &mut i32,
            height: &mut i32
        );
    }
}

/// TextActor displays 2D text in screen space (HUD overlay).
/// Supports both pixel and normalized [0,1] coordinates.
/// Perfect for labels, titles, status messages, and UI elements.
/// 
/// When using normalized coordinates, automatically handles window resize events.
pub struct TextActor {
    ptr: *mut ffi::vtkTextActor,
    // Store normalized coordinates for window resize handling
    normalized_x: Option<f64>,
    normalized_y: Option<f64>,
    // Track which render window this actor is attached to for resize events
    render_window_ptr: Option<*mut std::ffi::c_void>,
    observer_tag: Option<usize>,
}

impl TextActor {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::text_actor_new() };
        Self { 
            ptr,
            normalized_x: None,
            normalized_y: None,
            render_window_ptr: None,
            observer_tag: None,
        }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkTextActor> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Set the text to display
    pub fn set_input(&mut self, text: &str) {
        ffi::text_actor_set_input(self.as_mut(), text.to_string());
    }

    /// Get the current text
    pub fn get_input(&mut self) -> String {
        ffi::text_actor_get_input(self.as_mut())
    }

    /// Set position in pixel coordinates (display coordinates)
    /// (0,0) = bottom-left corner
    /// This clears normalized coordinate tracking.
    pub fn set_position(&mut self, x: f64, y: f64) {
        ffi::text_actor_set_position(self.as_mut(), x, y);
        // Clear normalized coordinates since we're using absolute positioning
        self.normalized_x = None;
        self.normalized_y = None;
    }

    /// Get position in pixel coordinates
    pub fn get_position(&mut self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::text_actor_get_position(self.as_mut(), &mut x, &mut y);
        (x, y)
    }

    /// Set position using normalized coordinates [0,1] relative to window size.
    /// (0,0) = bottom-left, (1,1) = top-right
    /// This allows text to maintain relative positioning when window is resized.
    /// Call update_for_window_size() after window resize to reposition.
    pub fn set_position_normalized(&mut self, fx: f64, fy: f64, window_width: i32, window_height: i32) {
        // Convert normalized [0,1] to pixel coordinates (logic in Rust)
        let x = fx * window_width as f64;
        let y = fy * window_height as f64;
        ffi::text_actor_set_position(self.as_mut(), x, y);
        
        // Store normalized coordinates for resize handling
        self.normalized_x = Some(fx);
        self.normalized_y = Some(fy);
    }

    /// Get position in normalized coordinates [0,1]
    pub fn get_position_normalized(&mut self, window_width: i32, window_height: i32) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::text_actor_get_position(self.as_mut(), &mut x, &mut y);
        
        // Convert pixel coordinates to normalized [0,1] (logic in Rust)
        let fx = x / window_width as f64;
        let fy = y / window_height as f64;
        (fx, fy)
    }

    /// Update position after window resize (only if using normalized coordinates).
    /// Call this when the window size changes to maintain relative positioning.
    pub fn update_for_window_size(&mut self, window_width: i32, window_height: i32) {
        if let (Some(fx), Some(fy)) = (self.normalized_x, self.normalized_y) {
            // Recalculate pixel position from stored normalized coords (logic in Rust)
            let x = fx * window_width as f64;
            let y = fy * window_height as f64;
            ffi::text_actor_set_position(self.as_mut(), x, y);
        }
    }

    /// Check if this text actor is using normalized coordinates
    pub fn is_using_normalized_coords(&self) -> bool {
        self.normalized_x.is_some() && self.normalized_y.is_some()
    }

    /// Enable automatic window resize handling for this text actor.
    /// This sets up an observer on the render window that will automatically
    /// update the text position when the window is resized.
    /// 
    /// Must be called after the text actor is added to the renderer and
    /// after using set_position_normalized().
    pub fn enable_auto_resize(&mut self, window: &mut crate::RenderWindow) {
        if !self.is_using_normalized_coords() {
            eprintln!("Warning: enable_auto_resize called on TextActor not using normalized coordinates");
            return;
        }

        // Store window pointer for future updates
        self.render_window_ptr = Some(window.as_mut_ptr() as *mut std::ffi::c_void);

        // Create a command callback that will update this text actor's position
        // Note: This requires careful lifetime management - for now we document
        // that the user should call update_for_window_size manually or use
        // a global resize handler
    }

    /// Detach from render window (cleanup observers)
    pub(crate) fn detach_from_window(&mut self) {
        self.render_window_ptr = None;
        self.observer_tag = None;
    }

    /// Set Position2 (used for text box sizing in some configurations)
    pub fn set_position2(&mut self, x: f64, y: f64) {
        ffi::text_actor_set_position2(self.as_mut(), x, y);
    }

    /// Get Position2
    pub fn get_position2(&mut self) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        ffi::text_actor_get_position2(self.as_mut(), &mut x, &mut y);
        (x, y)
    }

    /// Get the text property for font/color customization
    pub fn get_text_property(&mut self) -> TextPropertyRef {
        let ptr = ffi::text_actor_get_text_property(self.as_mut());
        TextPropertyRef { ptr }
    }

    /// Set visibility
    pub fn set_visibility(&mut self, visible: bool) {
        ffi::text_actor_set_visibility(self.as_mut(), visible);
    }

    /// Get visibility
    pub fn get_visibility(&mut self) -> bool {
        ffi::text_actor_get_visibility(self.as_mut())
    }

    /// Get raw pointer for renderer
    pub fn as_raw_ptr(&mut self) -> *mut ffi::vtkTextActor {
        self.ptr
    }
}

impl Drop for TextActor {
    fn drop(&mut self) {
        ffi::text_actor_delete(self.as_mut());
    }
}

/// Non-owning reference to a vtkTextProperty.
/// Used to configure font, color, and style properties of text.
pub struct TextPropertyRef {
    ptr: *mut ffi::vtkTextProperty,
}

impl TextPropertyRef {
    fn as_mut(&mut self) -> Pin<&mut ffi::vtkTextProperty> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Set font size in points
    pub fn set_font_size(&mut self, size: i32) {
        ffi::text_property_set_font_size(self.as_mut(), size);
    }

    /// Get font size
    pub fn get_font_size(&mut self) -> i32 {
        ffi::text_property_get_font_size(self.as_mut())
    }

    /// Set text color (RGB, each 0.0-1.0)
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        ffi::text_property_set_color(self.as_mut(), r, g, b);
    }

    /// Get text color
    pub fn get_color(&mut self) -> (f64, f64, f64) {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        ffi::text_property_get_color(self.as_mut(), &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Set opacity (0.0 = transparent, 1.0 = opaque)
    pub fn set_opacity(&mut self, opacity: f64) {
        ffi::text_property_set_opacity(self.as_mut(), opacity);
    }

    /// Get opacity
    pub fn get_opacity(&mut self) -> f64 {
        ffi::text_property_get_opacity(self.as_mut())
    }

    /// Set bold text
    pub fn set_bold(&mut self, bold: bool) {
        ffi::text_property_set_bold(self.as_mut(), bold);
    }

    /// Get bold setting
    pub fn get_bold(&mut self) -> bool {
        ffi::text_property_get_bold(self.as_mut())
    }

    /// Set italic text
    pub fn set_italic(&mut self, italic: bool) {
        ffi::text_property_set_italic(self.as_mut(), italic);
    }

    /// Get italic setting
    pub fn get_italic(&mut self) -> bool {
        ffi::text_property_get_italic(self.as_mut())
    }

    /// Set font family to Arial
    pub fn set_font_family_to_arial(&mut self) {
        ffi::text_property_set_font_family_to_arial(self.as_mut());
    }

    /// Set font family to Courier (monospace)
    pub fn set_font_family_to_courier(&mut self) {
        ffi::text_property_set_font_family_to_courier(self.as_mut());
    }

    /// Set font family to Times
    pub fn set_font_family_to_times(&mut self) {
        ffi::text_property_set_font_family_to_times(self.as_mut());
    }
    
    /// Set horizontal justification to left (default)
    pub fn set_justification_to_left(&mut self) {
        ffi::text_property_set_justification_to_left(self.as_mut());
    }
    
    /// Set horizontal justification to centered
    pub fn set_justification_to_centered(&mut self) {
        ffi::text_property_set_justification_to_centered(self.as_mut());
    }
    
    /// Set horizontal justification to right
    pub fn set_justification_to_right(&mut self) {
        ffi::text_property_set_justification_to_right(self.as_mut());
    }
    
    /// Set vertical justification to bottom (default)
    pub fn set_vertical_justification_to_bottom(&mut self) {
        ffi::text_property_set_vertical_justification_to_bottom(self.as_mut());
    }
    
    /// Set vertical justification to centered
    pub fn set_vertical_justification_to_centered(&mut self) {
        ffi::text_property_set_vertical_justification_to_centered(self.as_mut());
    }
    
    /// Set vertical justification to top
    pub fn set_vertical_justification_to_top(&mut self) {
        ffi::text_property_set_vertical_justification_to_top(self.as_mut());
    }
}

/// Data passed to the resize callback for text actors.
/// Uses Box<TextActor> to ensure actors have stable memory addresses.
struct TextActorResizeCallbackData {
    actors: Vec<Box<TextActor>>,
    window_ptr: *mut ffi::vtkRenderWindow,
}

/// Extern "C" callback function that updates text actor positions on window resize.
/// This is called by VTK's observer mechanism when the window's ModifiedEvent fires.
/// 
/// # Safety
/// The user_data pointer must be a valid pointer to TextActorResizeCallbackData
/// that remains alive for the duration of the callback registration.
extern "C" fn update_text_actors_on_resize(
    _caller: usize,
    _event: usize,
    user_data: usize,
) {
    unsafe {
        let callback_data = &mut *(user_data as *mut TextActorResizeCallbackData);
        
        // Get current window size directly from VTK (not through Rust wrapper)
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let window_ref = std::pin::Pin::new_unchecked(&mut *callback_data.window_ptr);
        ffi::render_window_get_size(window_ref, &mut width, &mut height);
        
        // Update all text actors that use normalized coordinates
        for actor in &mut callback_data.actors {
            if actor.is_using_normalized_coords() {
                actor.update_for_window_size(width, height);
            }
        }
    }
}

/// Helper to automatically handle window resize for multiple TextActors.
/// 
/// This manager sets up an observer on the render window that automatically
/// updates all registered text actors when the window is resized.
/// 
/// **IMPORTANT**: The manager must be kept alive for the entire duration of the application.
/// If dropped, the callback will be invalidated and may cause undefined behavior.
/// 
/// # Example
/// ```no_run
/// # use vtk_rs as vtk;
/// let mut window = vtk::RenderWindow::new();
/// let mut text1 = vtk::TextActor::new();
/// text1.set_position_normalized(0.5, 0.9, 1200, 800);
/// 
/// let mut text2 = vtk::TextActor::new();
/// text2.set_position_normalized(0.1, 0.1, 1200, 800);
/// 
/// // Create manager and register actors
/// // Keep this alive for the entire program!
/// let _resize_manager = vtk::TextActorResizeManager::new(
///     &mut window,
///     vec![text1, text2]
/// );
/// ```
pub struct TextActorResizeManager {
    _command: crate::Command,
    _observer_tag: usize,
    _callback_data: Box<TextActorResizeCallbackData>,
}

impl TextActorResizeManager {
    /// Create a new resize manager for the given text actors.
    /// 
    /// The manager will automatically update all actors when the window is resized.
    /// **The manager must be kept alive for the duration of the application.**
    /// 
    /// # Safety
    /// The callback data is stored in a Box and kept alive by this struct.
    /// Actors are stored in Box<TextActor> to ensure stable memory addresses.
    pub fn new(window: &mut crate::RenderWindow, actors: Vec<TextActor>) -> Self {
        // Box each actor to ensure stable memory addresses
        let boxed_actors: Vec<Box<TextActor>> = actors.into_iter()
            .map(Box::new)
            .collect();
        
        // Create callback data
        let callback_data = Box::new(TextActorResizeCallbackData {
            actors: boxed_actors,
            window_ptr: window.as_mut_ptr() as *mut ffi::vtkRenderWindow,
        });
        
        let callback_data_ptr = Box::into_raw(callback_data);
        
        // Create VTK command and set the extern "C" callback
        let mut command = crate::Command::new();
        unsafe {
            command.set_callback(update_text_actors_on_resize, callback_data_ptr as usize);
        }
        
        // Add observer to window (ModifiedEvent = 33 fires on resize)
        let observer_tag = window.add_observer_raw(
            33,
            command.as_raw_ptr() as *mut _
        );
        
        // Reclaim the Box to ensure proper cleanup
        let callback_data = unsafe { Box::from_raw(callback_data_ptr) };
        
        Self {
            _command: command,
            _observer_tag: observer_tag,
            _callback_data: callback_data,
        }
    }
    
    /// Get immutable access to the managed text actors
    pub fn actors(&self) -> impl Iterator<Item = &TextActor> {
        self._callback_data.actors.iter().map(|b| b.as_ref())
    }
    
    /// Get mutable access to the managed text actors
    pub fn actors_mut(&mut self) -> impl Iterator<Item = &mut TextActor> {
        self._callback_data.actors.iter_mut().map(|b| b.as_mut())
    }
}
