mod buffer;
mod mesh;
mod program;
mod shaders;
mod vao;
use crate::error;
pub use mesh::*;
pub use program::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Gl {
    context: WebGl2RenderingContext,
    program: GlProgram,
}

impl Gl {
    pub fn init() -> Result<Gl, String> {
        let canvas = load_canvas()?;
        let context = canvas
            .get_context("webgl2")
            .map_err(|e| error::fmt(e, ""))?
            .ok_or("defew")?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|e| error::fmt(e, ""))?;
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        Ok(Self {
            program: GlProgram::new(&context, shaders::VERTEX_SOURCE, shaders::FRAGMENT_SOURCE)?,
            context,
        })
    }

    pub fn create_mesh(&self, vertices: &[f32]) -> Result<GlMesh, String> {
        GlMesh::from_vertices(&self.context, vertices)
    }

    pub fn clear_canvas(&self, color: &[f32; 4]) {
        self.context
            .clear_color(color[0], color[1], color[2], color[3]);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }
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
