#version 140
in vec2 position;

void main() {
    float x = (position[0] / 320) - 1;
    float y = ((position[1] / 240) - 1) * -1;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
