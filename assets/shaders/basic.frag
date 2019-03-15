#version 450

in VS_OUTPUT {
	vec3 color;
} IN;

out vec4 color;

void main() {
	color = vec4(IN.color, 1.0);
}
