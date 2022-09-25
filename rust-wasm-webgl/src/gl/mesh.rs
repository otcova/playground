use crate::gl::vao::Attrib;

use super::{buffer::GlBuffer, vao::GlVAO, *};

pub struct GlMesh {
    context: WebGl2RenderingContext,
    instances_data: Vec<f32>,
    instances_buffer: GlBuffer,
    vertices_buffer: GlBuffer,
    vao: GlVAO,
    vertices_count: i32,
    instances_count: i32,
}

pub struct InstanceProperties([f32; 11]);
impl InstanceProperties {
    pub fn new() -> Self {
        Self([0.; 11])
    }
    pub fn position(&mut self, position: &[f32; 3]) -> &mut Self {
        self.0[0..3].copy_from_slice(position);
        self
    }
    pub fn color(&mut self, color: &[f32; 4]) -> &mut Self {
        self.0[3..7].copy_from_slice(color);
        self
    }
    pub fn matrix(&mut self, matrix: &[f32; 4]) -> &mut Self {
        self.0[7..11].copy_from_slice(matrix);
        self
    }
}

impl GlMesh {
    pub fn create_instance(&mut self, instance: &InstanceProperties) {
        self.instances_data.extend_from_slice(&instance.0);
        self.instances_count += 1;
    }

    pub fn draw(&mut self) {
        self.instances_buffer.update(&self.instances_data);

        self.vao.bind();

        const PRIMITIVE: u32 = WebGl2RenderingContext::TRIANGLES;
        self.context
            .draw_arrays_instanced(PRIMITIVE, 0, self.vertices_count, self.instances_count);

        self.instances_count = 0;
        self.instances_data.truncate(0);
    }

    pub(super) fn from_vertices(
        context: &WebGl2RenderingContext,
        vertices: &[f32],
    ) -> Result<Self, String> {
        const VERTEX_LEN: i32 = 2;

        if vertices.len() % 3 != 0
            || (vertices.len() as i32 / 3) % VERTEX_LEN != 0
            || vertices.len() < 3
        {
            Err(format!(
                "Expected [3 vertices with {} coordinates] for each triangle but found {} coordinates",
                VERTEX_LEN,
                vertices.len()
            ))
        } else {
            let vertices_buffer = GlBuffer::new_static(&context, vertices)?;
            let instances_buffer = GlBuffer::new(context)?;

            let vao = GlVAO::new(context)?;
            vao.link_buffer(&vertices_buffer, &[Attrib::VecF32(0, 2)]);
            vao.link_instance_buffer(
                &instances_buffer,
                &[
                    Attrib::VecF32(1, 3),
                    Attrib::VecF32(2, 4),
                    Attrib::MatF32(3, 2, 2),
                ],
            );

            Ok(Self {
                vao,
                vertices_buffer,
                vertices_count: vertices.len() as i32 / VERTEX_LEN,
                instances_data: vec![0.; 2097152],
                instances_buffer,
                context: context.clone(),
                instances_count: 0,
            })
        }
    }
}
