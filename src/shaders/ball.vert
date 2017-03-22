#version 140
in vec2 position;

void main() {
    gl_Position = vec4((position[0] / 320) - 1, ((position[1] / 240) - 1) * -1, 0.0, 1.0);
}
