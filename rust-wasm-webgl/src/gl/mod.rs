mod buffer;
mod program;
pub use crate::error;
pub use buffer::*;
pub use program::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[derive(Clone)]
pub struct Gl {
    pub context: WebGl2RenderingContext,
}

impl Gl {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Gl, String> {
        Ok(Self {
            context: canvas
                .get_context("webgl2")
                .map_err(|e| error::fmt(e, ""))?
                .ok_or("defew")?
                .dyn_into()
                .map_err(|e| error::fmt(e, ""))?,
        })
    }
}
