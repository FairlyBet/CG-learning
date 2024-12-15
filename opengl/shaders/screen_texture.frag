#version 330 core

in vec2 v_texcoord;

uniform sampler2D tex;

out vec4 frag_color;

void main() {
    float depth = texture(tex, v_texcoord).r;
    frag_color = vec4(vec3(depth), 1.0);
}
