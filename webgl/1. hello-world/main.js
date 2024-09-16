const canvas = document.querySelector('canvas');
const gl = canvas.getContext('webgl2');

const pixelRatio = window.devicePixelRatio || 1;
canvas.width = pixelRatio * canvas.clientWidth;
canvas.height = pixelRatio * canvas.clientHeight;
gl.viewport(0, 0, canvas.width, canvas.height);

const vsource = /*glsl*/ `#version 300 es
void main() {
    gl_PointSize = 150.0;
    gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}`;
const fsource = /*glsl*/ `#version 300 es
precision mediump float;
out vec4 fragColor;
void main() {
    fragColor = vec4(0.5f, 0.5f, 0.5f, 1.0f);
}`;

const vertexShader = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vertexShader, vsource);
gl.compileShader(vertexShader);

const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fragmentShader, fsource);
gl.compileShader(fragmentShader);

const program = gl.createProgram();
gl.attachShader(program, vertexShader);
gl.attachShader(program, fragmentShader);
gl.linkProgram(program);
gl.useProgram(program);

if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.log(gl.getShaderInfoLog(vertexShader));
    console.log(gl.getShaderInfoLog(fragmentShader));
}

gl.drawArrays(gl.POINTS, 0, 1);
