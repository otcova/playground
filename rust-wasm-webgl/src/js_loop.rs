use crate::error;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Result<(), String> {
    web_sys::window()
        .ok_or("Couldn't get window")?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|e| error::fmt(e, "Request animation frame failed"))?;
    Ok(())
}
pub fn get_current_time() -> Result<f32, String> {
    Ok(window()
        .ok_or("Couldn't get window")?
        .performance()
        .ok_or("Couldn't get window.performance")?
        .now() as f32
        / 1000.)
}

pub fn create_draw_loop<F>(mut draw_loop: F) -> Result<(), String>
where
    F: FnMut(FrameTime) -> Result<(), String> + 'static,
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut frame_count = 0;
    let mut past_time = get_current_time()?;
    let mut render_time = 0.;

    let mut render_time_store = [0.; 30];
    let mut render_average = 0.;
    let mut render_time_store_index = 0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        frame_count += 1;

        error::report((|| {
            let start_time = get_current_time()?;
            let delta_time = start_time - past_time;

            draw_loop(FrameTime {
                fps: 1. / delta_time,
                delta: delta_time,
                frame_count,
                render: render_time,
                render_average,
                seconds: start_time,
            })?;

            render_time = get_current_time()? - start_time;
            past_time = start_time;

            render_time_store[render_time_store_index] = render_time;
            render_time_store_index += 1;
            if render_time_store_index >= render_time_store.len() {
                render_time_store_index = 0;
            }
            
            render_average = render_time_store.iter().sum::<f32>()
                / render_time_store.len().min(frame_count as usize) as f32;
                
            request_animation_frame(f.borrow().as_ref().unwrap())
        })());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap())?;
    Ok(())
}

pub struct FrameTime {
    pub fps: f32,
    pub seconds: f32,
    pub render: f32,
    pub render_average: f32,
    pub delta: f32,
    pub frame_count: i32,
}
