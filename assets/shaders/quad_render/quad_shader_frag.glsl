#version 410
out vec4 fragColor;

in vec2 texCoords;

uniform sampler2D texture;

void main() {
    fragColor = texture(texture, texCoords);
}
