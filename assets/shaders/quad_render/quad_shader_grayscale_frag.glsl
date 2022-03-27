#version 410
out vec4 fragColor;

in vec2 texCoords;

uniform sampler2D tex;

void main() {
    fragColor = texture(tex, texCoords);
    float average = 0.2126 * fragColor.r + 0.7152 * fragColor.g + 0.0722 * fragColor.b;
    fragColor = vec4(average, average, average, 1.0);

    //fragColor = texture(tex, texCoords);
}
