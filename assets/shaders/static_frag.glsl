#version 410

out vec4 frag_color;

in vec4 vertex_color;
in vec2 uv_pos;

uniform sampler2D our_texture;

void main() {
//    frag_color = vec4(1.0, 1.0, 1.0, 1.0);
    frag_color = texture(our_texture, uv_pos) + vec4(color_shift, 1.0);
}
