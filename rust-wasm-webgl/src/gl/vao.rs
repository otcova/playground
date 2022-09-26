use web_sys::*;

use super::buffer::GlBuffer;

pub struct GlVAO {
    context: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
}

#[derive(Debug)]
pub enum Attrib {
    VecF32(u32, i32),      // location, vec len
    VecI32(u32, i32),      // location, vec len
    MatF32(u32, i32, i32), // location, rows, columns
    Offset(i32),           // len in bytes
}

impl GlVAO {
    pub fn bind(&self) {
        self.context.bind_vertex_array(Some(&self.vao));
    }
    pub fn new(context: &WebGl2RenderingContext) -> Result<GlVAO, String> {
        Ok(Self {
            vao: context.create_vertex_array().ok_or("Couldn't create vao")?,
            context: context.clone(),
        })
    }

    pub fn link_instance_buffer(&self, buffer: &GlBuffer, attributes: &[Attrib]) {
        self.context.bind_vertex_array(Some(&self.vao));
        buffer.bind();

        let mut stride = 0;
        for attrib in attributes {
            stride += attrib.bytes_count();
        }

        let mut offset = 0;
        for attrib in attributes {
            attrib.vertex_attrib_pointer(&self.context, stride, offset);
            attrib.vertex_attrib_divisor(&self.context, 1);
            attrib.enable_vertex_attrib_array(&self.context);

            offset += attrib.bytes_count();
        }
    }
    pub fn link_buffer(&self, buffer: &GlBuffer, attributes: &[Attrib]) {
        self.context.bind_vertex_array(Some(&self.vao));
        buffer.bind();

        let mut stride = 0;
        for attrib in attributes {
            stride += attrib.bytes_count();
        }

        let mut offset = 0;
        for attrib in attributes {
            attrib.vertex_attrib_pointer(&self.context, stride, offset);
            attrib.enable_vertex_attrib_array(&self.context);

            offset += attrib.bytes_count();
        }
    }
}

impl Attrib {
    fn bytes_count(&self) -> i32 {
        match self {
            Attrib::VecF32(_, len) => len * 4,
            Attrib::VecI32(_, len) => len * 4,
            Attrib::MatF32(_, r, c) => r * c * 4,
            Attrib::Offset(len) => *len,
        }
    }

    fn vertex_attrib_pointer(&self, ctx: &WebGl2RenderingContext, stride: i32, offset: i32) {
        match self {
            Attrib::VecF32(location, len) => ctx.vertex_attrib_pointer_with_i32(
                *location,
                *len,
                WebGl2RenderingContext::FLOAT,
                false,
                stride,
                offset,
            ),
            Attrib::VecI32(location, len) => ctx.vertex_attrib_i_pointer_with_i32(
                *location,
                *len,
                WebGl2RenderingContext::INT,
                stride,
                offset,
            ),
            Attrib::MatF32(location, rows, columns) => {
                for row in 0..*rows {
                    ctx.vertex_attrib_pointer_with_i32(
                        location + row as u32,
                        *columns,
                        WebGl2RenderingContext::FLOAT,
                        false,
                        stride,
                        offset + row * columns * 4,
                    );
                }
            }
            Attrib::Offset(_) => {}
        }
    }

    fn vertex_attrib_divisor(&self, ctx: &WebGl2RenderingContext, divisor: u32) {
        match self {
            Attrib::VecF32(location, _) => ctx.vertex_attrib_divisor(*location, divisor),
            Attrib::VecI32(location, _) => ctx.vertex_attrib_divisor(*location, divisor),
            Attrib::MatF32(location, rows, _) => {
                for row in 0..*rows {
                    ctx.vertex_attrib_divisor(location + row as u32, divisor);
                }
            }
            Attrib::Offset(_) => {}
        }
    }

    fn enable_vertex_attrib_array(&self, ctx: &WebGl2RenderingContext) {
        match self {
            Attrib::VecF32(location, _) => ctx.enable_vertex_attrib_array(*location),
            Attrib::VecI32(location, _) => ctx.enable_vertex_attrib_array(*location),
            Attrib::MatF32(location, rows, _) => {
                for row in 0..*rows {
                    ctx.enable_vertex_attrib_array(location + row as u32);
                }
            }
            Attrib::Offset(_) => {}
        }
    }
}

impl Drop for GlVAO {
    fn drop(&mut self) {
        self.context.delete_vertex_array(Some(&self.vao));
    }
}