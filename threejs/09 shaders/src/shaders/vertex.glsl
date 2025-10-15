attribute vec3 position;
attribute vec2 uv;

varying vec2 vUv;
varying float vElevation;

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;
uniform vec2 uFrequency;
uniform float uTime;

void main() {
    vec4 modelPosition = modelMatrix * vec4(position, 1.0);
    vElevation = sin(modelPosition.x * uFrequency.x - uTime) * 0.1 + sin(modelPosition.y * uFrequency.y - uTime) * 0.1;
    modelPosition.z += vElevation;
    vUv = uv;
    gl_Position = projectionMatrix * viewMatrix * modelPosition;
}