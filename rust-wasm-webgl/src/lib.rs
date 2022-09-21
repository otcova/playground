#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
mod console;
pub mod error;
mod gl;
use gl::*;

pub fn body() -> Option<web_sys::HtmlElement> {
    web_sys::window()?.document()?.body()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    if let Err(error) = setup() {
        console::log!("[ERROR]\n{}", error);
    }

    Ok(())
}

fn setup() -> Result<(), String> {
    let gl = Gl::new(&load_canvas()?)?;

    let program = gl.create_program(
        r##"#version 300 es
 
        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1);
        }
        "##,
        r##"#version 300 es
        precision mediump float;
        
        uniform vec4 color;
        out vec4 outColor;
        
        void main() {
            outColor = color;
        } "##,
    )?;

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = gl.create_static_buffer(&vertices)?;

    program.link_data(&buffer, &[("position", ParameterType::VecF32(3))]);

    program.set_uniform("color", &Uniform::Vec4F32(&[1., 1., 0.5, 1.]));
    let vert_count = (vertices.len() / 3) as i32;

    draw(&gl.context, vert_count);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

fn load_canvas() -> Result<web_sys::HtmlCanvasElement, String> {
    let window = web_sys::window().ok_or("Couldn't get window")?;
    let document = window.document().ok_or("Couldn't get document")?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Couldn't get canvas")?;
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|e| error::fmt(e, "Invalid canvas"))?;

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    Ok(canvas)
}

fn request_animation_frame(f: &::js_sys::Function) -> Result<(), String> {
    web_sys::window()
        .ok_or("Couldn't get window")?
        .request_animation_frame(&f)
        .map_err(|e| error::fmt(e, "Request animation frame failed"))?;
    Ok(())
}
