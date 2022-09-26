use web_sys::*;

use crate::error;

#[derive(Debug)]
pub(super) enum Uniform<'a> {
    Vec1F32(&'a [f32]),
    Vec2F32(&'a [f32]),
    Vec3F32(&'a [f32]),
    Vec4F32(&'a [f32]),
    Vec1I32(&'a [i32]),
    Vec2I32(&'a [i32]),
    Vec3I32(&'a [i32]),
    Vec4I32(&'a [i32]),
}

pub struct GlProgram {
    gl_context: WebGl2RenderingContext,
    pub(super) program: WebGlProgram,
}

const VERTEX_SHADER: u32 = WebGl2RenderingContext::VERTEX_SHADER;
const FRAGMENT_SHADER: u32 = WebGl2RenderingContext::FRAGMENT_SHADER;

impl GlProgram {
    pub(super) fn new(
        gl: &WebGl2RenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<GlProgram, String> {
        let vertex_shader = compile_shader(gl, VERTEX_SHADER, vertex_shader_src)?;
        let fragment_shader = compile_shader(gl, FRAGMENT_SHADER, fragment_shader_src)?;

        Ok(GlProgram {
            gl_context: gl.clone(),
            program: link_shaders(gl, &vertex_shader, &fragment_shader)?,
        })
    }

    fn set_uniform(&self, name: &str, value: Uniform) {
        let location = self.gl_context.get_uniform_location(&self.program, name);
        let loc = location.as_ref();
        match value {
            Uniform::Vec1F32(data) => self.gl_context.uniform1fv_with_f32_array(loc, data),
            Uniform::Vec2F32(data) => self.gl_context.uniform2fv_with_f32_array(loc, data),
            Uniform::Vec3F32(data) => self.gl_context.uniform3fv_with_f32_array(loc, data),
            Uniform::Vec4F32(data) => self.gl_context.uniform4fv_with_f32_array(loc, data),
            Uniform::Vec1I32(data) => self.gl_context.uniform1iv_with_i32_array(loc, data),
            Uniform::Vec2I32(data) => self.gl_context.uniform2iv_with_i32_array(loc, data),
            Uniform::Vec3I32(data) => self.gl_context.uniform3iv_with_i32_array(loc, data),
            Uniform::Vec4I32(data) => self.gl_context.uniform4iv_with_i32_array(loc, data),
        }
    }

    pub(super) fn bind(&self) {
        self.gl_context.use_program(Some(&self.program));
    }
}

fn link_shaders(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Couldn't create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        // context.use_program(Some(&program));
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .map(|e| error::fmt(e, "Couldn't link shaders"))
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Couldn't create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let shader_name = if shader_type == VERTEX_SHADER {
            "vertex"
        } else {
            "fragment"
        };
        Err(context
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

impl Drop for GlProgram {
    fn drop(&mut self) {
        self.gl_context.delete_program(Some(&self.program));
    }
}
