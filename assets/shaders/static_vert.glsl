#version 410

layout (location = 0) in vec3 pos;

out vec4 vertex_color;

uniform vec3 color_shift;

void main() {
    gl_Position = vec4(pos, 1.0);
    vertex_color = vec4(pos.x, pos.y, 0.5, 1.0) + vec4(color_shift, 1.0);
}
