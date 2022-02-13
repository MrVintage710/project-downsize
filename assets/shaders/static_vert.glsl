#version 410

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

out vec4 vertex_color;
out vec2 uv_pos;

uniform vec3 color_shift;

void main() {
    gl_Position = vec4(pos, 1.0);
    uv_pos = uv;
    vertex_color = vec4(pos.x, pos.y, 0.5, 1.0) + vec4(color_shift, 1.0);
}
