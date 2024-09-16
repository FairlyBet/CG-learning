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

// Sets the default value for an attribute in case if it is not enabled
gl.vertexAttrib1f(aPointSizeLoc, 10);
gl.vertexAttrib2f(aPositionLoc, 0, 0.5);
gl.vertexAttrib3f(aColorLoc, 0, 0, 0);

// gl.enableVertexAttribArray(aPointSizeLoc);
gl.enableVertexAttribArray(aPositionLoc);
gl.enableVertexAttribArray(aColorLoc);

// Use separate buffers 
const pointSizeData = new Float32Array([20, 10, 30,]);
const positionData = new Float32Array([
    0, 0.5,
    0.5, -0.3,
    -.5, .7,
]);
const colorData = new Float32Array([
    1, 0, 0,
    0, 1, 0,
    0, 0, 1,
]);

const pointSizeBuffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, pointSizeBuffer);
gl.bufferData(gl.ARRAY_BUFFER, pointSizeData, gl.STATIC_DRAW);

const positionBuffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
gl.bufferData(gl.ARRAY_BUFFER, positionData, gl.STATIC_DRAW);

const colorBuffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
gl.bufferData(gl.ARRAY_BUFFER, colorData, gl.STATIC_DRAW);

gl.bindBuffer(gl.ARRAY_BUFFER, pointSizeBuffer);
gl.vertexAttribPointer(aPointSizeLoc, 1, gl.FLOAT, false, 0, 0);
gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
gl.vertexAttribPointer(aPositionLoc, 2, gl.FLOAT, false, 0, 0);
gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
gl.vertexAttribPointer(aColorLoc, 3, gl.FLOAT, false, 0, 0);

gl.drawArrays(gl.POINTS, 0, 3);
