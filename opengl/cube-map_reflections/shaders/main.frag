#version 330 core

in vec3 v_world_pos;
in vec3 v_normal;

out vec4 frag_color;

void main() {
    const vec3 LIGHT_POS = vec3(0.0, 1.0, -5.0);
    const vec3 COLOR = vec3(0.6, 0.4, 0.3);
    const float AMBIENT = 0.3;
    // const vec3 LIGHT_DIRECTION = vec3(0.0, 0.0, -1.0);

    vec3 normal = normalize(v_normal);
    vec3 offset = LIGHT_POS - v_world_pos;
    vec3 dir = normalize(offset);
    float dist = length(offset);
    float attenuation = 10.0 / (dist * dist + 0.01);
    float illumination = max(AMBIENT, dot(dir, normal) * attenuation);
    vec3 color = COLOR * illumination;
    frag_color = vec4(color, 1.0);
}