use super::Gl;
use web_sys::*;

pub struct Buffer {
    gl: Gl,
    gl_buffer: WebGlBuffer,
}

impl Gl {
    pub fn create_buffer(&self) -> Result<Buffer, String> {
        Ok(Buffer {
            gl: self.clone(),
            gl_buffer: self
                .context
                .create_buffer()
                .ok_or("Unable to create gl buffer")?,
        })
    }

    pub fn create_static_buffer(&self, data: &[f32]) -> Result<Buffer, String> {
        let buffer = self.create_buffer()?;
        buffer.allocate_static(data);
        Ok(buffer)
    }
}

const TARGET: u32 = WebGl2RenderingContext::ARRAY_BUFFER;

impl Buffer {
    pub fn bind(&self) {
        self.gl.context.bind_buffer(TARGET, Some(&self.gl_buffer));
    }

    pub fn allocate(&self, data: &[f32]) {
        self.array_buffer_data(data, WebGl2RenderingContext::DYNAMIC_DRAW);
    }

    /// faster than allocate if you don't want to update frequently
    pub fn allocate_static(&self, data: &[f32]) {
        self.array_buffer_data(data, WebGl2RenderingContext::STATIC_DRAW);
    }

    /// always faster than allocate
    pub fn update_slice(&self, data: &[f32], dst_byte_offset: i32) {
        self.bind();

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(&data);

            self.gl.context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                dst_byte_offset,
                &array_buffer_view,
            );
        }
    }

    fn array_buffer_data(&self, data: &[f32], usage: u32) {
        self.bind();

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(&data);

            self.gl.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                usage,
            );
        }
    }
}
