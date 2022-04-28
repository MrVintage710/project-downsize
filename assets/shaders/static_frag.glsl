#version 410
out vec4 frag_color;

in vec4 vertex_color;
in vec2 uv_pos;

uniform sampler2D our_texture;

// Global Light
uniform vec3 global_light_color;
uniform float global_ambient;

void main() {
    vec4 albeto = texture(our_texture, uv_pos);
    //    frag_color = vec4(1.0, 1.0, 1.0, 1.0);
    frag_color = albeto * vec4(global_light_color * global_ambient, 1.0);
}