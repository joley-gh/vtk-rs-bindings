// AreaPicker actor refs demo
// Drag to select actors; the demo will print actor positions and nudge each picked actor outward.

use vtk_rs::*;

struct PickerState {
    area_picker: AreaPicker,
    renderer_ptr: usize,
    render_window_ptr: usize,
    style_ptr: usize,
    start_x: i32,
    start_y: i32,
    base_frame: Option<Vec<u8>>,
}

impl PickerState {
    fn new(
        area_picker: AreaPicker,
        renderer_ptr: usize,
        render_window_ptr: usize,
        style_ptr: usize
    ) -> Self {
        Self {
            area_picker,
            renderer_ptr,
            render_window_ptr,
            style_ptr,
            start_x: 0,
            start_y: 0,
            base_frame: None,
        }
    }
}

fn main() {
    println!("AreaPicker ActorRef demo - drag to select actors");

    let mut renderer = Renderer::new();
    renderer.set_background(0.15, 0.18, 0.2);

    // create a few actors
    let mut actors = Vec::new();
    for i in 0..5 {
        let mut sphere = SphereSource::new();
        sphere.set_center([(i as f64) * 2.0 - 4.0, 0.0, 0.0]);
        sphere.set_radius(0.8);
        sphere.set_theta_resolution(16);
        sphere.set_phi_resolution(12);

        let mut mapper = PolyDataMapper::new();
        let out = SphereSource::get_output_port(&mut sphere);
        mapper.set_input_connection(out);

        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        let mut prop = actor.get_property();
        prop.set_color(0.8 - (i as f64) * 0.1, 0.3 + (i as f64) * 0.12, 0.4);
        renderer.add_actor(&mut actor);
        actors.push(actor);
    }

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(900, 600);
    render_window.set_window_name("AreaPicker ActorRef Demo");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let area_picker = AreaPicker::new();

    let renderer_ptr = &mut renderer as *mut Renderer as usize;
    let render_window_ptr = &mut render_window as *mut RenderWindow as usize;

    let state = Box::new(PickerState::new(area_picker, renderer_ptr, render_window_ptr, 0));
    let state_ptr = Box::into_raw(state) as usize;

    let on_left_press = move |x: i32, y: i32| {
        unsafe {
            let st = &mut *(state_ptr as *mut PickerState);
            st.start_x = x;
            st.start_y = y;
            // Cache current front buffer for smooth rubber-band drawing
            let window = &mut *(st.render_window_ptr as *mut RenderWindow);
            window.render();
            st.base_frame = Some(window.get_pixel_data());
            println!("Started selection at ({}, {})", x, y);
        }
    };

    let on_mouse_move = move |x: i32, y: i32| {
        unsafe {
            let st = &mut *(state_ptr as *mut PickerState);
            let style_ref = &*(st.style_ptr as *mut InteractorStyleCustom);
            let window = &mut *(st.render_window_ptr as *mut RenderWindow);
            if style_ref.is_moving() {
                if let Some(ref base_frame) = st.base_frame {
                    interactor_style_custom::draw_rubber_band_rectangle_cached(
                        window,
                        base_frame,
                        st.start_x,
                        st.start_y,
                        x,
                        y,
                        (255, 255, 0),
                        2
                    );
                }
            }
        }
    };

    let on_left_release = move |x: i32, y: i32| {
        unsafe {
            let st = &mut *(state_ptr as *mut PickerState);
            let window = &mut *(st.render_window_ptr as *mut RenderWindow);
            let size = window.get_size();
            let window_height = size.1;

            let y0 = window_height - st.start_y;
            let y1 = window_height - y;
            let x0 = st.start_x;
            let x1 = x;

            println!("Selection: ({}, {}) to ({}, {})", x0, y0, x1, y1);

            let renderer_ref = &mut *(st.renderer_ptr as *mut Renderer);
            let picked = st.area_picker.area_pick(
                x0 as f64,
                y0 as f64,
                x1 as f64,
                y1 as f64,
                renderer_ref
            );

            if !picked {
                println!("No objects picked");
            } else {
                println!("Objects picked - collecting actor refs...");
                // Use the new safe helper
                let mut actors = st.area_picker.get_actor_refs();
                println!("Picked {} actor(s)", actors.len());
                for (idx, mut aref) in actors.iter_mut().enumerate() {
                    let pos = aref.position();
                    println!(
                        " Actor {} position = ({:.2}, {:.2}, {:.2})",
                        idx,
                        pos.0,
                        pos.1,
                        pos.2
                    );
                    // Nudge actor upwards a bit to visualize selection
                    aref.set_position(pos.0, pos.1 + 0.5, pos.2);
                }
                // Re-render to show updated positions
                window.render();
            }
        }
    };

    let mut style = InteractorStyleCustom::new();
    style.set_selection_mode(true);
    unsafe {
        let st = &mut *(state_ptr as *mut PickerState);
        st.style_ptr = &mut style as *mut InteractorStyleCustom as usize;
    }

    style.set_left_button_press_callback(on_left_press);
    style.set_mouse_move_callback(on_mouse_move);
    style.set_left_button_release_callback(on_left_release);
    interactor.set_interactor_style_custom(&mut style);

    render_window.render();
    interactor.start();

    unsafe {
        let _ = Box::from_raw(state_ptr as *mut PickerState);
    }
}
