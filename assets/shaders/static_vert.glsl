#version 410

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

out vec2 uv_pos;

uniform vec3 test;
uniform vec3 un_3;

void main() {
    gl_Position = vec4(pos + test/100 + (un_3*0), 1.0);
    uv_pos = uv;
}
