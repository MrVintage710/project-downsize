#version 410

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec3 norm;

out vec2 uv_pos;

uniform mat4 transform;
uniform mat4 perspective;

void main() {
    gl_Position = perspective * transform * vec4(pos, 1.0);
    uv_pos = uv;
}
