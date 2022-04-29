#version 410
out vec4 frag_color;

in vec4 out_norm;
in vec2 uv_pos;

uniform sampler2D our_texture;

// Global Light
uniform vec3 global_light_color;
uniform vec3 global_light_direction;
uniform float global_ambient;

void main() {
    float global_difference = max(dot(normalize(out_norm), normalize(vec4(global_light_direction, 1.0))), 0.0);
    vec3 ambient = global_light_color * global_ambient;
    vec3 diffuse = global_difference * global_light_color;
    vec4 albeto = texture(our_texture, uv_pos);
    frag_color = vec4((diffuse + ambient), 1.0);
}