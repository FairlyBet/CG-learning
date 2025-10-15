precision mediump float;

varying vec2 vUv;
varying float vElevation;

uniform vec3 uColor;
uniform sampler2D uTexture;

void main() {
    vec4 texcolor = texture2D(uTexture, vUv) * (vElevation * 2.0 + 0.5);
    gl_FragColor = vec4(texcolor.rgb, 1.0);
}