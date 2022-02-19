#version 410

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

out vec2 uv_pos;

uniform vec3 test;

void main() {
    gl_Position = vec4(pos + test, 1.0);
    uv_pos = uv;
}
