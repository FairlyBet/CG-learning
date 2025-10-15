varying vec2 vUv;

float rand(vec2 st) {
    return fract(sin(dot(st, vec2(12.9898, 78.233))) * 43758.5453123);
}

void main() {
    // float strength = floor(vUv.x * 10.0) / 10.0 * floor(vUv.y * 10.0) / 10.0;
    // vec2 coord = vec2(floor(vUv.x * 10.0) / 10.0, floor((vUv.y + vUv.x * 0.5) * 10.0) / 10.0);
    // float strength = rand(coord);
    float strength = 1.0 - length(vUv - 0.5);
    gl_FragColor = vec4(strength, strength, strength, 1.0);
}