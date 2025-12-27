#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_actor.h");
        include!("vtk_mapper.h");

        type vtkActor;
        type vtkMapper;
        type vtkProperty;

        fn actor_new() -> *mut vtkActor;
        fn actor_delete(actor: Pin<&mut vtkActor>);
        unsafe fn actor_set_mapper(actor: Pin<&mut vtkActor>, mapper: *mut vtkMapper);
        unsafe fn actor_get_property(actor: Pin<&mut vtkActor>) -> *mut vtkProperty;

        // Position methods
        fn actor_set_position(actor: Pin<&mut vtkActor>, x: f64, y: f64, z: f64);
        fn actor_get_position(actor: &vtkActor, x: &mut f64, y: &mut f64, z: &mut f64);
        fn actor_add_position(actor: Pin<&mut vtkActor>, x: f64, y: f64, z: f64);

        // Rotation methods
        fn actor_rotate_x(actor: Pin<&mut vtkActor>, angle: f64);
        fn actor_rotate_y(actor: Pin<&mut vtkActor>, angle: f64);
        fn actor_rotate_z(actor: Pin<&mut vtkActor>, angle: f64);
        fn actor_set_orientation(actor: Pin<&mut vtkActor>, x: f64, y: f64, z: f64);
        fn actor_get_orientation(actor: &vtkActor, x: &mut f64, y: &mut f64, z: &mut f64);

        // Scale methods
        fn actor_set_scale(actor: Pin<&mut vtkActor>, x: f64, y: f64, z: f64);
        fn actor_get_scale(actor: &vtkActor, x: &mut f64, y: &mut f64, z: &mut f64);

        // Visibility methods
        fn actor_set_visibility(actor: Pin<&mut vtkActor>, visible: bool);
        fn actor_get_visibility(actor: &vtkActor) -> bool;

        // Pickability methods
        fn actor_set_pickable(actor: Pin<&mut vtkActor>, pickable: bool);
        fn actor_get_pickable(actor: &vtkActor) -> bool;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkActor.html",
    @name Actor, ffi::vtkActor,
    @new ffi::actor_new,
    @delete ffi::actor_delete
);

impl Actor {
    pub fn set_mapper(&mut self, mapper: &mut crate::PolyDataMapper) {
        unsafe {
            // Cast PolyDataMapper to vtkMapper (base class)
            let mapper_ptr = mapper.as_mut_ptr() as *mut ffi::vtkMapper;
            unsafe { ffi::actor_set_mapper(self.ptr.as_mut(), mapper_ptr) }
        }
    }

    /// Get the property for this actor.
    /// Returns a non-owning reference to the property managed by the actor.
    pub fn get_property(&mut self) -> PropertyRef {
        let ptr = unsafe { ffi::actor_get_property(self.ptr.as_mut()) };
        PropertyRef { ptr }
    }

    // Position methods
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        ffi::actor_set_position(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_position(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::actor_get_position(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn add_position(&mut self, x: f64, y: f64, z: f64) {
        ffi::actor_add_position(self.ptr.as_mut(), x, y, z);
    }

    // Rotation methods
    pub fn rotate_x(&mut self, angle: f64) {
        ffi::actor_rotate_x(self.ptr.as_mut(), angle);
    }

    pub fn rotate_y(&mut self, angle: f64) {
        ffi::actor_rotate_y(self.ptr.as_mut(), angle);
    }

    pub fn rotate_z(&mut self, angle: f64) {
        ffi::actor_rotate_z(self.ptr.as_mut(), angle);
    }

    pub fn set_orientation(&mut self, x: f64, y: f64, z: f64) {
        ffi::actor_set_orientation(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_orientation(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::actor_get_orientation(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    // Scale methods
    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) {
        ffi::actor_set_scale(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_scale(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::actor_get_scale(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    // Visibility methods
    pub fn set_visibility(&mut self, visible: bool) {
        ffi::actor_set_visibility(self.ptr.as_mut(), visible);
    }

    pub fn get_visibility(&self) -> bool {
        ffi::actor_get_visibility(&self.ptr.as_ref())
    }

    // Pickability methods
    pub fn set_pickable(&mut self, pickable: bool) {
        ffi::actor_set_pickable(self.ptr.as_mut(), pickable);
    }

    pub fn get_pickable(&self) -> bool {
        ffi::actor_get_pickable(&self.ptr.as_ref())
    }
}

/// A non-owning reference to a Property managed by an Actor.
/// This does not delete the property when dropped.
pub struct PropertyRef {
    ptr: *mut ffi::vtkProperty,
}

impl PropertyRef {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::vtkProperty {
        self.ptr
    }

    /// Set the color of the object. Values should be in the range [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_color(std::pin::Pin::new_unchecked(property_ref), r, g, b);
        }
    }

    /// Get the color of the object.
    pub fn get_color(&mut self) -> (f64, f64, f64) {
        use crate::vtk_property::ffi as property_ffi;
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_get_color(
                std::pin::Pin::new_unchecked(property_ref),
                &mut r,
                &mut g,
                &mut b
            );
        }
        (r, g, b)
    }

    /// Set the opacity of the object. Value should be in the range [0.0, 1.0].
    pub fn set_opacity(&mut self, opacity: f64) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_opacity(std::pin::Pin::new_unchecked(property_ref), opacity);
        }
    }

    /// Get the opacity of the object.
    pub fn get_opacity(&mut self) -> f64 {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_get_opacity(std::pin::Pin::new_unchecked(property_ref))
        }
    }

    /// Set the representation type (points, wireframe, or surface).
    pub fn set_representation(&mut self, representation: crate::RepresentationType) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_representation(
                std::pin::Pin::new_unchecked(property_ref),
                representation as i32
            );
        }
    }

    /// Get the representation type.
    pub fn get_representation(&mut self) -> crate::RepresentationType {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            match
                property_ffi::property_get_representation(
                    std::pin::Pin::new_unchecked(property_ref)
                )
            {
                0 => crate::RepresentationType::Points,
                1 => crate::RepresentationType::Wireframe,
                2 => crate::RepresentationType::Surface,
                _ => crate::RepresentationType::Surface,
            }
        }
    }

    /// Set whether edges are visible (useful for surface representation).
    pub fn set_edge_visibility(&mut self, visible: bool) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_edge_visibility(
                std::pin::Pin::new_unchecked(property_ref),
                visible
            );
        }
    }

    /// Set the line width for wireframe rendering.
    pub fn set_line_width(&mut self, width: f64) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_line_width(
                std::pin::Pin::new_unchecked(property_ref),
                width
            );
        }
    }

    /// Set the point size for point rendering.
    pub fn set_point_size(&mut self, size: f64) {
        use crate::vtk_property::ffi as property_ffi;
        unsafe {
            let property_ref = &mut *(self.ptr as *mut property_ffi::vtkProperty);
            property_ffi::property_set_point_size(std::pin::Pin::new_unchecked(property_ref), size);
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkActor`](https://vtk.org/doc/nightly/html/classvtkActor.html)
#[allow(non_camel_case_types)]
pub trait vtkActor: private::Sealed {}
