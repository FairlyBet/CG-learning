const canvas = document.querySelector('canvas');
const gl = canvas.getContext('webgl2');

gl.viewport(0, 0, canvas.width, canvas.height);

const vsource = /*glsl*/ `#version 300 es
layout(location=0) in vec2 aPosition;
layout(location=1) in vec3 aColor;

out vec3 vColor;

void main() {
    gl_PointSize = 10.0;
    vColor = aColor;
    gl_Position = vec4(aPosition, 0.0, 1.0);
}`;
const fsource = /*glsl*/ `#version 300 es
precision mediump float;
in vec3 vColor;

out vec4 fragColor;

void main() {
    fragColor = vec4(vColor, 1.0);
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

const positionIndex = 0;
const colorIndex = 1;

const data1 = new Float32Array([
    .4, .2,
    -.7, .6,
    -.9, -.1,
]);

const buffer1 = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, buffer1);
gl.bufferData(gl.ARRAY_BUFFER, data1, gl.STATIC_DRAW);

const vao1 = gl.createVertexArray();
gl.bindVertexArray(vao1);
gl.vertexAttribPointer(positionIndex, 2, gl.FLOAT, false, 0, 0);
gl.enableVertexAttribArray(positionIndex);

const data2 = new Float32Array([
    .2, .4,
    -.6, .7,
    .9, -.5,
]);

const buffer2 = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, buffer2);
gl.bufferData(gl.ARRAY_BUFFER, data2, gl.STATIC_DRAW);

const vao2 = gl.createVertexArray();
gl.bindVertexArray(vao2);
gl.vertexAttribPointer(positionIndex, 2, gl.FLOAT, false, 0, 0);
gl.enableVertexAttribArray(positionIndex);

gl.bindVertexArray(vao1);
gl.vertexAttrib3f(colorIndex, 1.0, 0.0, 0.0);
gl.drawArrays(gl.POINTS, 0, 3);

gl.bindVertexArray(vao2);
gl.vertexAttrib3f(colorIndex, 0.0, 0.0, 1.0);
gl.drawArrays(gl.POINTS, 0, 3);
