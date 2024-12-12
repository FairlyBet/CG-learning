#version 330 core

out vec2 v_texcoord;

void main() {
    const vec2 POSITIONS[] = vec2[](vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    vec2 position = POSITIONS[gl_VertexID];
    v_texcoord = (position + 1) / 2;

    gl_Position = vec4(position, 0.0, 1.0);
}