use web_sys::*;

pub struct GlBuffer {
    context: WebGl2RenderingContext,
    buffer: WebGlBuffer,
    len: usize,
}

const TARGET: u32 = WebGl2RenderingContext::ARRAY_BUFFER;

impl GlBuffer {
    pub fn new(gl_context: &WebGl2RenderingContext) -> Result<GlBuffer, String> {
        Ok(GlBuffer {
            context: gl_context.clone(),
            buffer: gl_context
                .create_buffer()
                .ok_or("Unable to create gl buffer")?,
            len: 0,
        })
    }

    pub fn new_static(
        gl_context: &WebGl2RenderingContext,
        data: &[f32],
    ) -> Result<GlBuffer, String> {
        let mut buffer = Self::new(gl_context)?;
        buffer.allocate_static(data);
        Ok(buffer)
    }

    pub fn bind(&self) {
        self.context.bind_buffer(TARGET, Some(&self.buffer));
    }

    pub fn update(&mut self, data: &[f32]) {
        if self.len < data.len() {
            self.allocate_data(data, WebGl2RenderingContext::DYNAMIC_DRAW);
        } else {
            self.update_slice(data, 0);
        }
    }

    /// faster than allocate if you don't want to update data frequently
    pub fn allocate_static(&mut self, data: &[f32]) {
        self.allocate_data(data, WebGl2RenderingContext::STATIC_DRAW);
    }

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

            self.context.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                dst_byte_offset,
                &array_buffer_view,
            );
        }
    }

    fn allocate_data(&mut self, data: &[f32], usage: u32) {
        self.bind();
        self.len = data.len();

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

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                usage,
            );
        }
    }
}
