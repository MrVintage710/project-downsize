#version 410
out vec4 fragColor;

in vec2 texCoords;

uniform sampler2D tex;

void main() {
    fragColor = texture(tex, texCoords);
}
