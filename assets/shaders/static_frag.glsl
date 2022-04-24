#version 410
out vec4 frag_color;

in vec4 vertex_color;
in vec2 uv_pos;

uniform sampler2D our_texture;

//Global Lighting
uniform float ambient_min;
uniform vec3 global_lighting_color;

void main() {
    vec3 ambient = ambient_min * global_lighting_color;

    vec3 result_color = texture(our_texture, uv_pos) * ambient;
    frag_color = vec4(result_color, 1.0);
}
