const canvas = document.querySelector('canvas');
const gl = canvas.getContext('webgl2');

gl.viewport(0, 0, canvas.width, canvas.height);

const vsource = /*glsl*/ `#version 300 es
layout(location=0) in float aPointSize;
layout(location=1) in vec2 aPosition;
layout(location=2) in vec3 aColor;

out vec3 vColor;

uniform vec2 offset;

void main() {
    vColor = aColor;
    gl_PointSize = aPointSize;
    gl_Position = vec4(offset + aPosition, 0.0, 1.0);
}`;
const fsource = /*glsl*/ `#version 300 es
precision mediump float;
in vec3 vColor;

out vec4 fragColor;

void main() {
    fragColor = vec4(vColor, 1.0f);
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

if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.log(gl.getShaderInfoLog(vertexShader));
    console.log(gl.getShaderInfoLog(fragmentShader));
    console.log(gl.getProgramInfoLog(program));
}

gl.useProgram(program);

const offsetLoc = gl.getUniformLocation(program, 'offset');
gl.uniform2f(offsetLoc, 0.1, -0.3);

const aPointSizeLoc = 0;
const aPositionLoc = 1;
const aColorLoc = 2;
gl.enableVertexAttribArray(aPointSizeLoc);
gl.enableVertexAttribArray(aPositionLoc);
gl.enableVertexAttribArray(aColorLoc);

const data = new Float32Array([
    100, 0, 0, 1, 0, 0,
    10, .5, .5, 0, 1, 0,
    30, -.7, .4, 0, 0, 1,
]);
const buffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
gl.bufferData(gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);

const floatSize = 4;
const stride = floatSize * 6;
gl.vertexAttribPointer(aPointSizeLoc, 1, gl.FLOAT, false, stride, 0);
gl.vertexAttribPointer(aPositionLoc, 2, gl.FLOAT, false, stride, floatSize);
gl.vertexAttribPointer(aColorLoc, 3, gl.FLOAT, true, stride, floatSize * 3);

gl.drawArrays(gl.POINTS, 0, 3);
