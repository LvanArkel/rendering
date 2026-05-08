#version 140

in vec2 position;

uniform vec3 model;

void main() {
    gl_Position = vec4(model + vec3(position, 0.0), 1.0);
}