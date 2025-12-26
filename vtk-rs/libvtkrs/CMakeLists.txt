cmake_minimum_required(VERSION 3.12)

project(vtkrs)

find_package(VTK COMPONENTS
  CommonColor
  CommonCore
  CommonDataModel
  FiltersSources
  InteractionStyle
  InteractionWidgets
  RenderingAnnotation
  RenderingContextOpenGL2
  RenderingCore
  RenderingFreeType
  RenderingGL2PSOpenGL2
  RenderingOpenGL2
  REQUIRED
)

if (NOT VTK_FOUND)
    message(FATAL_ERROR "Vtk: Unable to find the VTK build folder.")
endif()

# Prevent a "command line is too long" failure in Windows.
set(CMAKE_NINJA_FORCE_RESPONSE_FILE "ON" CACHE BOOL "Force Ninja to use response files.")

add_library(vtkrs STATIC
   ${PROJECT_SOURCE_DIR}/include/cxx.h
   ${PROJECT_SOURCE_DIR}/include/vtk_abstract_mapper.h
   ${PROJECT_SOURCE_DIR}/include/vtk_abstract_mapper_3d.h
   ${PROJECT_SOURCE_DIR}/include/vtk_algorithm.h
   ${PROJECT_SOURCE_DIR}/include/vtk_algorithm_output.h
   ${PROJECT_SOURCE_DIR}/include/vtk_axes_actor.h
   ${PROJECT_SOURCE_DIR}/include/vtk_camera.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cell_array.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cell_data.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cell_picker.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cone_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cylinder_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cube_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cube_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_plane_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_disk_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_arrow_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_regular_polygon_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_cube_axes_actor.h
   ${PROJECT_SOURCE_DIR}/include/vtk_data_object.h
   ${PROJECT_SOURCE_DIR}/include/vtk_prop_picker.h
   ${PROJECT_SOURCE_DIR}/include/vtk_double_array.h
   ${PROJECT_SOURCE_DIR}/include/vtk_executive.h
   ${PROJECT_SOURCE_DIR}/include/vtk_information.h
   ${PROJECT_SOURCE_DIR}/include/vtk_information_vector.h
   ${PROJECT_SOURCE_DIR}/include/vtk_int_array.h
   ${PROJECT_SOURCE_DIR}/include/vtk_interactor_style_custom.h
   ${PROJECT_SOURCE_DIR}/include/vtk_interactor_style_trackball_camera.h
   ${PROJECT_SOURCE_DIR}/include/vtk_line_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_mapper.h
   ${PROJECT_SOURCE_DIR}/include/vtk_named_colors.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object_base.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object_base.rs.h
   ${PROJECT_SOURCE_DIR}/include/vtk_orientation_marker_widget.h
   ${PROJECT_SOURCE_DIR}/include/vtk_points.h
   ${PROJECT_SOURCE_DIR}/include/vtk_point_data.h
   ${PROJECT_SOURCE_DIR}/include/vtk_poly_data.h
   ${PROJECT_SOURCE_DIR}/include/vtk_poly_data_algorithm.h
   ${PROJECT_SOURCE_DIR}/include/vtk_poly_data_mapper.h
   ${PROJECT_SOURCE_DIR}/include/vtk_property.h
   ${PROJECT_SOURCE_DIR}/include/vtk_sphere.h
   ${PROJECT_SOURCE_DIR}/include/vtk_sphere_source.h
   ${PROJECT_SOURCE_DIR}/include/vtk_tube_filter.h
   ${PROJECT_SOURCE_DIR}/include/vtk_glyph_3d.h
   ${PROJECT_SOURCE_DIR}/include/vtk_world_point_picker.h
)

if (VTK094)
    target_compile_definitions(vtkrs PUBLIC -DVTK094=1)
endif()

include_directories(${PROJECT_SOURCE_DIR}/include/)

target_sources(vtkrs
    PRIVATE
        ${PROJECT_SOURCE_DIR}/src/vtk_init.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_abstract_mapper.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_abstract_mapper_3d.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_actor.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_algorithm.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_algorithm_output.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_axes_actor.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_camera.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cell_array.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cell_data.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cell_picker.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cone_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cylinder_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cube_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_plane_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_disk_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_arrow_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_regular_polygon_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_cube_axes_actor.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_data_object.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_prop_picker.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_double_array.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_executive.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_information.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_information_vector.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_int_array.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_interactor_style_custom.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_interactor_style_trackball_camera.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_line_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_mapper.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_named_colors.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_object.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_object_base.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_orientation_marker_widget.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_points.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_point_data.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_poly_data.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_poly_data_algorithm.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_poly_data_mapper.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_property.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_render_window.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_render_window_interactor.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_renderer.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_sphere.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_sphere_source.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_tube_filter.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_glyph_3d.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_world_point_picker.cpp
)
set_target_properties(vtkrs PROPERTIES LINKER_LANGUAGE CXX)

# Add VTK autoinit compile definitions
target_compile_definitions(vtkrs PRIVATE
    "vtkRenderingContext2D_AUTOINIT=1(vtkRenderingContextOpenGL2)"
    "vtkRenderingCore_AUTOINIT=5(vtkInteractionStyle,vtkRenderingAnnotation,vtkRenderingFreeType,vtkRenderingOpenGL2,vtkInteractionWidgets)"
    "vtkRenderingOpenGL2_AUTOINIT=1(vtkRenderingGL2PSOpenGL2)"
)

target_link_libraries(vtkrs PRIVATE ${VTK_LIBRARIES})

install(TARGETS vtkrs DESTINATION .)

vtk_module_autoinit(
  TARGETS vtkrs
  MODULES ${VTK_LIBRARIES}
)

# add_executable(main main.cpp vtkPolyDataMapper.cpp)
# set_target_properties(main PROPERTIES LINKER_LANGUAGE CXX)
# target_link_libraries(main PRIVATE ${VTK_LIBRARIES})
