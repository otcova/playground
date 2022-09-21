use super::*;
use web_sys::*;

pub struct Program {
    gl: Gl,
    gl_program: WebGlProgram,
    vao: WebGlVertexArrayObject,
}

pub enum ParameterType {
    VecF32(i32),
    VecI32(i32),
}

pub enum Uniform<'a> {
    Vec1F32(&'a [f32]),
    Vec2F32(&'a [f32]),
    Vec3F32(&'a [f32]),
    Vec4F32(&'a [f32]),
    Vec1I32(&'a [i32]),
    Vec2I32(&'a [i32]),
    Vec3I32(&'a [i32]),
    Vec4I32(&'a [i32]),
}

impl Gl {
    pub fn create_program(
        &self,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<Program, String> {
        let vertex_shader =
            self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_src)?;
        let fragment_shader =
            self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_src)?;

        Ok(Program {
            gl: self.clone(),
            gl_program: self.link_shaders(&vertex_shader, &fragment_shader)?,
            vao: self
                .context
                .create_vertex_array()
                .ok_or("Couldn't create a vao")?,
        })
    }

    fn link_shaders(
        &self,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self
            .context
            .create_program()
            .ok_or_else(|| String::from("Couldn't create shader object"))?;

        self.context.attach_shader(&program, vertex_shader);
        self.context.attach_shader(&program, fragment_shader);
        self.context.link_program(&program);

        if self
            .context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self
                .context
                .get_program_info_log(&program)
                .map(|s| format!("Could not link shaders because of {}", s))
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = self
            .context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Couldn't create shader object"))?;
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);

        if self
            .context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let shader_name = if shader_type == WebGl2RenderingContext::VERTEX_SHADER {
                "vertex"
            } else {
                "fragment"
            };
            Err(self
                .context
                .get_shader_info_log(&shader)
                .map(|s| {
                    format!(
                        "Could not compile {} shaders because of:\n{}",
                        shader_name, s
                    )
                })
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}

impl Program {
    pub fn link_data(&self, buffer: &Buffer, parameters: &[(&str, ParameterType)]) {
        self.bind();
        buffer.bind();

        let mut stride = 0;

        for (_, parameter_type) in parameters {
            stride += parameter_type.size();
        }

        let mut offset = 0;

        for (parameter_name, parameter_type) in parameters {
            let param_location =
                self.gl
                    .context
                    .get_attrib_location(&self.gl_program, parameter_name) as u32;

            match parameter_type {
                ParameterType::VecF32(len) => self.gl.context.vertex_attrib_pointer_with_i32(
                    param_location,
                    *len,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    stride,
                    offset,
                ),
                ParameterType::VecI32(len) => self.gl.context.vertex_attrib_i_pointer_with_i32(
                    param_location,
                    *len,
                    WebGl2RenderingContext::INT,
                    stride,
                    offset,
                ),
            }
            offset += parameter_type.size();

            self.gl.context.enable_vertex_attrib_array(param_location);
        }
    }
    pub fn bind(&self) {
        self.gl.context.use_program(Some(&self.gl_program));
        self.gl.context.bind_vertex_array(Some(&self.vao));
    }
    pub fn set_uniform(&self, name: &str, value: &Uniform) {
        let location = self.gl.context.get_uniform_location(&self.gl_program, name);
        let loc = location.as_ref();
        match value {
            Uniform::Vec1F32(data) => self.gl.context.uniform1fv_with_f32_array(loc, data),
            Uniform::Vec2F32(data) => self.gl.context.uniform2fv_with_f32_array(loc, data),
            Uniform::Vec3F32(data) => self.gl.context.uniform3fv_with_f32_array(loc, data),
            Uniform::Vec4F32(data) => self.gl.context.uniform4fv_with_f32_array(loc, data),
            Uniform::Vec1I32(data) => self.gl.context.uniform1iv_with_i32_array(loc, data),
            Uniform::Vec2I32(data) => self.gl.context.uniform2iv_with_i32_array(loc, data),
            Uniform::Vec3I32(data) => self.gl.context.uniform3iv_with_i32_array(loc, data),
            Uniform::Vec4I32(data) => self.gl.context.uniform4iv_with_i32_array(loc, data),
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.context.delete_program(Some(&self.gl_program));
    }
}

impl ParameterType {
    fn size(&self) -> i32 {
        match self {
            ParameterType::VecF32(len) => len * 4,
            ParameterType::VecI32(len) => len * 4,
        }
    }
}
