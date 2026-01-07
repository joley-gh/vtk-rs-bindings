use crate::AlgorithmOutputPort;
use std::os::raw::c_int;

#[repr(C)]
pub struct vtkGlyph3D {
    _private: [u8; 0],
}

extern "C" {
    fn glyph_3d_new() -> *mut vtkGlyph3D;
    fn glyph_3d_delete(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_input_connection(glyph: *mut vtkGlyph3D, input: *mut AlgorithmOutputPort);
    fn glyph_3d_set_source_connection(glyph: *mut vtkGlyph3D, source: *mut AlgorithmOutputPort);
    fn glyph_3d_get_output_port(glyph: *mut vtkGlyph3D) -> *mut AlgorithmOutputPort;
    fn glyph_3d_set_scale_factor(glyph: *mut vtkGlyph3D, factor: f64);
    fn glyph_3d_get_scale_factor(glyph: *mut vtkGlyph3D) -> f64;
    fn glyph_3d_set_scale_mode_to_scale_by_scalar(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_scale_mode_to_scale_by_vector(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_scale_mode_to_scale_by_vector_components(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_scale_mode_to_data_scaling_off(glyph: *mut vtkGlyph3D);
    fn glyph_3d_get_scale_mode(glyph: *mut vtkGlyph3D) -> c_int;
    fn glyph_3d_set_scaling(glyph: *mut vtkGlyph3D, enable: c_int);
    fn glyph_3d_get_scaling(glyph: *mut vtkGlyph3D) -> c_int;
    fn glyph_3d_set_orient(glyph: *mut vtkGlyph3D, enable: c_int);
    fn glyph_3d_get_orient(glyph: *mut vtkGlyph3D) -> c_int;
    fn glyph_3d_set_clamping(glyph: *mut vtkGlyph3D, enable: c_int);
    fn glyph_3d_get_clamping(glyph: *mut vtkGlyph3D) -> c_int;
    fn glyph_3d_set_color_mode_to_color_by_scale(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_color_mode_to_color_by_scalar(glyph: *mut vtkGlyph3D);
    fn glyph_3d_set_color_mode_to_color_by_vector(glyph: *mut vtkGlyph3D);
    fn glyph_3d_get_color_mode(glyph: *mut vtkGlyph3D) -> c_int;
}

/// Scale mode constants for Glyph3D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleMode {
    /// Scale by scalar data values
    ScaleByScalar = 0,
    /// Scale by vector magnitude
    ScaleByVector = 1,
    /// Scale by vector components (X, Y, Z)
    ScaleByVectorComponents = 2,
    /// No data-based scaling, only use scale_factor
    DataScalingOff = 3,
}

/// Color mode constants for Glyph3D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Color by scale factor
    ColorByScale = 0,
    /// Color by scalar data
    ColorByScalar = 1,
    /// Color by vector magnitude
    ColorByVector = 2,
}

/// Glyph3D filter - places copies of a geometric object (glyph) at input points.
///
/// `Glyph3D` is a powerful filter for efficiently rendering multiple copies of a geometric
/// object at specified points. This is ideal for:
/// - Visualizing FEM nodes (place spheres at all node positions with one filter)
/// - Vector field visualization (arrows showing direction/magnitude)
/// - Particle systems
/// - Tensor visualization
///
/// # Workflow
/// 1. Input connection: Set the point dataset where glyphs will be placed
/// 2. Source connection: Set the geometric object to copy (e.g., SphereSource)
/// 3. Configure scaling, orientation, and coloring
/// 4. Connect output to mapper for rendering
///
/// # Example: Efficient Node Markers
/// ```no_run
/// use vtk_rs::*;
///
/// // Create point source with vertices
/// let mut point_source = SphereSource::new();
/// point_source.set_radius(1.0);
///
/// // Create a poly data from the source
/// let mut poly_data = PolyData::new();
/// // (In real usage, you'd set points and vertices here)
///
/// // Create sphere as glyph geometry
/// let mut sphere = SphereSource::new();
/// sphere.set_radius(0.1);
/// sphere.set_theta_resolution(8);
/// sphere.set_phi_resolution(8);
///
/// // Use Glyph3D to place spheres at all points
/// let mut glyph = Glyph3D::new();
/// glyph.set_input_connection(poly_data.get_output_port());
/// glyph.set_source_connection(sphere.get_output_port());
/// glyph.set_scale_mode_to_data_scaling_off();
/// glyph.set_scale_factor(1.0);
///
/// // Connect to mapper
/// let mut mapper = PolyDataMapper::new();
/// mapper.set_input_connection(glyph.get_output_port());
/// ```
///
/// # Scaling Options
/// - `DataScalingOff`: All glyphs same size (use `scale_factor` only)
/// - `ScaleByScalar`: Size based on scalar data at each point
/// - `ScaleByVector`: Size based on vector magnitude
/// - `ScaleByVectorComponents`: Independent X/Y/Z scaling from vector components
///
/// # Performance
/// Much more efficient than manually creating geometry for each point:
/// - 1 filter call instead of N object creations
/// - Shared geometry (one sphere definition, N instances)
/// - Optimized rendering pipeline
pub struct Glyph3D {
    inner: *mut vtkGlyph3D,
}

impl Glyph3D {
    /// Creates a new Glyph3D filter.
    ///
    /// Default configuration:
    /// - Scale mode: ScaleByScalar
    /// - Scale factor: 1.0
    /// - Scaling: enabled
    /// - Orient: enabled (align glyphs with vectors if available)
    /// - Clamping: disabled
    pub fn new() -> Self {
        let inner = unsafe { glyph_3d_new() };
        Glyph3D { inner }
    }

    /// Sets the input connection (points where glyphs will be placed).
    ///
    /// The input should be a dataset with points (PolyData, Points, etc.).
    /// A glyph will be placed at each point in the input.
    pub fn set_input_connection(&mut self, input: crate::AlgorithmOutputPort) {
        let ptr: *mut std::ffi::c_void = input.into();
        unsafe { glyph_3d_set_input_connection(self.inner, ptr as *mut AlgorithmOutputPort) }
    }

    /// Raw pointer version - internal use only.
    #[doc(hidden)]
    pub(crate) fn _set_input_connection_raw(&mut self, input: *mut AlgorithmOutputPort) {
        unsafe { glyph_3d_set_input_connection(self.inner, input) }
    }

    /// Sets the source connection (geometry to copy at each point).
    ///
    /// Common sources:
    /// - SphereSource: for node markers
    /// - ArrowSource: for vector visualization
    /// - CubeSource: for voxel rendering
    /// - Custom PolyData: any geometric shape
    ///
    /// # Example
    /// ```no_run
    /// # use vtk_rs::*;
    /// let mut sphere = SphereSource::new();
    /// let mut glyph = Glyph3D::new();
    /// glyph.set_source_connection(sphere.get_output_port());
    /// ```
    pub fn set_source_connection(&mut self, source: crate::AlgorithmOutputPort) {
        let ptr: *mut std::ffi::c_void = source.into();
        unsafe { glyph_3d_set_source_connection(self.inner, ptr as *mut AlgorithmOutputPort) }
    }

    /// Raw pointer version - internal use only.
    #[doc(hidden)]
    pub(crate) fn _set_source_connection_raw(&mut self, source: *mut AlgorithmOutputPort) {
        unsafe { glyph_3d_set_source_connection(self.inner, source) }
    }

    /// Gets the output port for connecting to a mapper.
    ///
    /// # Example
    /// ```no_run
    /// # use vtk_rs::*;
    /// let mut glyph = Glyph3D::new();
    /// let mut mapper = PolyDataMapper::new();
    /// mapper.set_input_connection(glyph.get_output_port());
    /// ```
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = glyph_3d_get_output_port(self.inner);
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }

    /// Raw pointer version - internal use only.
    #[doc(hidden)]
    pub(crate) fn _get_output_port_raw(&mut self) -> *mut AlgorithmOutputPort {
        unsafe { glyph_3d_get_output_port(self.inner) }
    }

    /// Sets the scale factor applied to all glyphs.
    ///
    /// This is a uniform multiplier:
    /// - `1.0`: Original size
    /// - `0.5`: Half size
    /// - `2.0`: Double size
    ///
    /// If data-based scaling is enabled, this acts as an additional multiplier
    /// on top of the data-driven scale.
    pub fn set_scale_factor(&mut self, factor: f64) {
        unsafe { glyph_3d_set_scale_factor(self.inner, factor) }
    }

    /// Gets the current scale factor.
    pub fn get_scale_factor(&self) -> f64 {
        unsafe { glyph_3d_get_scale_factor(self.inner as *mut _) }
    }

    /// Scale glyphs by scalar data at each point.
    ///
    /// The active scalar array in PointData determines glyph size.
    /// Useful for showing data magnitude (stress, temperature, etc.).
    pub fn set_scale_mode_to_scale_by_scalar(&mut self) {
        unsafe { glyph_3d_set_scale_mode_to_scale_by_scalar(self.inner) }
    }

    /// Scale glyphs by vector magnitude at each point.
    ///
    /// The active vector array in PointData determines glyph size.
    /// Useful for velocity/displacement fields.
    pub fn set_scale_mode_to_scale_by_vector(&mut self) {
        unsafe { glyph_3d_set_scale_mode_to_scale_by_vector(self.inner) }
    }

    /// Scale glyphs by individual vector components (X, Y, Z).
    ///
    /// Each glyph axis is scaled independently by the corresponding
    /// vector component. Creates ellipsoidal shapes.
    pub fn set_scale_mode_to_scale_by_vector_components(&mut self) {
        unsafe { glyph_3d_set_scale_mode_to_scale_by_vector_components(self.inner) }
    }

    /// Disable data-based scaling - all glyphs have uniform size.
    ///
    /// Only `scale_factor` affects size. This is the mode for simple
    /// node markers where all nodes should look identical.
    pub fn set_scale_mode_to_data_scaling_off(&mut self) {
        unsafe { glyph_3d_set_scale_mode_to_data_scaling_off(self.inner) }
    }

    /// Gets the current scale mode.
    pub fn get_scale_mode(&self) -> ScaleMode {
        let mode = unsafe { glyph_3d_get_scale_mode(self.inner as *mut _) };
        match mode {
            0 => ScaleMode::ScaleByScalar,
            1 => ScaleMode::ScaleByVector,
            2 => ScaleMode::ScaleByVectorComponents,
            3 => ScaleMode::DataScalingOff,
            _ => ScaleMode::DataScalingOff,
        }
    }

    /// Enables or disables scaling.
    ///
    /// When disabled, `scale_factor` and scale mode are ignored.
    /// All glyphs use the source geometry's original size.
    pub fn set_scaling(&mut self, enable: bool) {
        unsafe { glyph_3d_set_scaling(self.inner, enable as c_int) }
    }

    /// Returns whether scaling is enabled.
    pub fn get_scaling(&self) -> bool {
        unsafe { glyph_3d_get_scaling(self.inner as *mut _) != 0 }
    }

    /// Enables or disables glyph orientation.
    ///
    /// When enabled and vector data is available, glyphs are rotated
    /// to align with the vector direction. Useful for arrow glyphs
    /// showing vector fields.
    pub fn set_orient(&mut self, enable: bool) {
        unsafe { glyph_3d_set_orient(self.inner, enable as c_int) }
    }

    /// Returns whether orientation is enabled.
    pub fn get_orient(&self) -> bool {
        unsafe { glyph_3d_get_orient(self.inner as *mut _) != 0 }
    }

    /// Enables or disables clamping of scale factors.
    ///
    /// When enabled, prevents glyphs from becoming too small or too large
    /// based on data values. Use with SetClampRange() to set limits.
    pub fn set_clamping(&mut self, enable: bool) {
        unsafe { glyph_3d_set_clamping(self.inner, enable as c_int) }
    }

    /// Returns whether clamping is enabled.
    pub fn get_clamping(&self) -> bool {
        unsafe { glyph_3d_get_clamping(self.inner as *mut _) != 0 }
    }

    /// Color glyphs by their scale factor.
    pub fn set_color_mode_to_color_by_scale(&mut self) {
        unsafe { glyph_3d_set_color_mode_to_color_by_scale(self.inner) }
    }

    /// Color glyphs by scalar data at each point.
    ///
    /// The active scalar array in PointData determines glyph color
    /// via the mapper's lookup table.
    pub fn set_color_mode_to_color_by_scalar(&mut self) {
        unsafe { glyph_3d_set_color_mode_to_color_by_scalar(self.inner) }
    }

    /// Color glyphs by vector magnitude at each point.
    pub fn set_color_mode_to_color_by_vector(&mut self) {
        unsafe { glyph_3d_set_color_mode_to_color_by_vector(self.inner) }
    }

    /// Gets the current color mode.
    pub fn get_color_mode(&self) -> ColorMode {
        let mode = unsafe { glyph_3d_get_color_mode(self.inner as *mut _) };
        match mode {
            0 => ColorMode::ColorByScale,
            1 => ColorMode::ColorByScalar,
            2 => ColorMode::ColorByVector,
            _ => ColorMode::ColorByScalar,
        }
    }
}

impl Drop for Glyph3D {
    fn drop(&mut self) {
        unsafe { glyph_3d_delete(self.inner) }
    }
}

impl Default for Glyph3D {
    fn default() -> Self {
        Self::new()
    }
}
