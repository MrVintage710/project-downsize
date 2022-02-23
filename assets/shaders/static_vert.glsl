#version 410

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

out vec2 uv_pos;

uniform vec3 test;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(pos + test/100, 1.0);
    uv_pos = uv;
}
