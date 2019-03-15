#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out VS_OUTPUT {
    vec3 color;
} OUT;
//uniform mat4 MVPmatrix;

void main() {
    gl_Position = vec4(position, 1.0);
    OUT.color = color;
}
