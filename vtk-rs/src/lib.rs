//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

#![cfg_attr(docsrs, feature(doc_cfg))]

// Internal Tools
mod macros;

use macros::*;

// Exposed API
mod algorithm_output_port;
mod vtk_abstract_mapper;
mod vtk_abstract_mapper_3d;
mod vtk_actor;
mod vtk_algorithm;
mod vtk_algorithm_output;
mod vtk_axes_actor;
mod vtk_camera;
pub use vtk_camera::ffi as camera_ffi;
mod vtk_cell_array;
mod vtk_cell_data;
mod vtk_cell_picker;
mod vtk_command;
mod vtk_cone_source;
mod vtk_cylinder_source;
mod vtk_cube_source;
mod vtk_plane_source;
mod vtk_disk_source;
mod vtk_arrow_source;
mod vtk_regular_polygon_source;
mod vtk_cube_axes_actor;
mod vtk_data_object;
mod vtk_prop_picker;
mod vtk_double_array;
mod vtk_executive;
mod vtk_information;
mod vtk_information_vector;
mod vtk_int_array;
mod vtk_interactor_style_custom;
mod vtk_interactor_style_rubber_band_pick;
mod vtk_interactor_style_trackball_camera;
mod vtk_line_source;
mod vtk_mapper;
mod vtk_named_colors;
mod vtk_object;
mod vtk_object_base;
mod vtk_orientation_marker_widget;
mod vtk_points;
mod vtk_point_data;
mod vtk_poly_data;
mod vtk_poly_data_algorithm;
mod vtk_poly_data_mapper;
mod vtk_property;
mod vtk_render_window;
mod vtk_render_window_interactor;
mod vtk_renderer;
mod vtk_sphere;
mod vtk_sphere_source;
mod vtk_tube_filter;
mod vtk_glyph_3d;
mod vtk_world_point_picker;
mod vtk_point_picker;
mod vtk_area_picker;
mod vtk_parametric_function;
mod vtk_parametric_function_source;
mod vtk_parametric_torus;
mod vtk_parametric_klein;
mod vtk_parametric_mobius;
mod vtk_superquadric_source;
mod vtk_text_source;
mod vtk_follower;
mod vtk_vector_text;
mod vtk_text_actor;
mod vtk_lookup_table;
mod vtk_scalar_bar_actor;
mod vtk_legend_box_actor;
mod vtk_image_data;
mod vtk_unstructured_grid;
mod vtk_data_set_mapper;
mod vtk_contour_filter;
mod vtk_smooth_poly_data_filter;
mod vtk_threshold;
mod vtk_plane;
mod vtk_clip_poly_data;
mod vtk_warp_vector;

// VTK Initialization
// This function must be called before using any VTK objects
extern "C" {
    fn vtk_force_init();
}

/// Initialize VTK modules. This is called automatically when using VTK objects.
#[doc(hidden)]
pub fn init_vtk() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        unsafe {
            vtk_force_init();
        }
    });
}

pub use algorithm_output_port::*;
pub use vtk_abstract_mapper::*;
pub use vtk_abstract_mapper_3d::*;
pub use vtk_actor::*;
pub use vtk_algorithm::*;
pub use vtk_algorithm_output::*;
pub use vtk_axes_actor::*;
pub use vtk_camera::*;
pub use vtk_cell_array::*;
pub use vtk_cell_data::*;
pub use vtk_cell_picker::*;
pub use vtk_command::*;
pub use vtk_cone_source::*;
pub use vtk_cylinder_source::*;
pub use vtk_cube_source::*;
pub use vtk_disk_source::*;
pub use vtk_plane_source::*;
pub use vtk_arrow_source::*;
pub use vtk_regular_polygon_source::*;
pub use vtk_cube_axes_actor::*;
pub use vtk_data_object::*;
pub use vtk_prop_picker::*;
pub use vtk_double_array::*;
pub use vtk_executive::*;
pub use vtk_information::*;
pub use vtk_information_vector::*;
pub use vtk_int_array::*;
pub use vtk_interactor_style_custom::*;
pub use vtk_interactor_style_rubber_band_pick::*;
pub use vtk_interactor_style_trackball_camera::*;

// Re-export interactor_style_custom submodule for rubber band drawing functions
pub mod interactor_style_custom {
    pub use super::vtk_interactor_style_custom::draw_rubber_band_rectangle;
    pub use super::vtk_interactor_style_custom::draw_rubber_band_rectangle_cached;
}
pub use vtk_line_source::*;
pub use vtk_mapper::*;
pub use vtk_named_colors::*;
pub use vtk_object::*;
pub use vtk_object_base::*;
pub use vtk_orientation_marker_widget::*;
pub use vtk_points::*;
pub use vtk_point_data::*;
pub use vtk_poly_data::*;
pub use vtk_poly_data_algorithm::*;
pub use vtk_poly_data_mapper::*;
pub use vtk_property::*;
pub use vtk_render_window::*;
pub use vtk_render_window_interactor::*;
pub use vtk_renderer::*;
pub use vtk_sphere::*;
pub use vtk_sphere_source::*;
pub use vtk_tube_filter::*;
pub use vtk_glyph_3d::*;
pub use vtk_world_point_picker::*;
pub use vtk_point_picker::*;
pub use vtk_area_picker::*;
// pub use vtk_parametric_function::*;  // No public types to export from base class
pub use vtk_parametric_function_source::*;
pub use vtk_parametric_torus::*;
pub use vtk_parametric_klein::*;
pub use vtk_parametric_mobius::*;
pub use vtk_superquadric_source::*;
pub use vtk_text_source::*;
pub use vtk_follower::*;
pub use vtk_vector_text::*;
pub use vtk_text_actor::*;
pub use vtk_lookup_table::*;
pub use vtk_scalar_bar_actor::*;
pub use vtk_legend_box_actor::*;
pub use vtk_image_data::*;
pub use vtk_unstructured_grid::*;
pub use vtk_data_set_mapper::*;
pub use vtk_contour_filter::*;
pub use vtk_smooth_poly_data_filter::*;
pub use vtk_threshold::*;
pub use vtk_plane::*;
pub use vtk_clip_poly_data::*;
pub use vtk_warp_vector::*;
