pub const VERTEX_SOURCE: &str = r##"#version 300 es
precision mediump float;
layout(location=0) in vec2 vertex_coord;
layout(location=1) in vec3 position;
layout(location=2) in vec4 color;
layout(location=3) in mat2 matrix;

out vec4 fr_color;

void main() {
	gl_Position = vec4(vec3(vertex_coord * matrix, 0.) + position, 1.);
	fr_color = color;
}
"##;

pub const FRAGMENT_SOURCE: &str = r##"#version 300 es
precision mediump float;

in vec4 fr_color;
out vec4 outColor;

void main() {
	outColor = fr_color;
}
"##;
