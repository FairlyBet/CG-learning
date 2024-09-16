const canvas = document.querySelector('canvas');
const gl = canvas.getContext('webgl2');

gl.viewport(0, 0, canvas.width, canvas.height);

const vsource = /*glsl*/ `#version 300 es
layout(location=0) in vec2 aPosition;
layout(location=1) in vec2 aTexCoord;

out vec2 vTexCoord;

void main() {
    vTexCoord = aTexCoord;
    gl_Position = vec4(aPosition, 0.0, 1.0);
}`;
const fsource = /*glsl*/ `#version 300 es
precision mediump float;

in vec2 vTexCoord;
uniform sampler2D uTex;

out vec4 fragColor;

void main() {
    fragColor = texture(uTex, vTexCoord);
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

const aPositionLoc = 0;
const aTexCoordLoc = 1;

gl.enableVertexAttribArray(aPositionLoc);
gl.enableVertexAttribArray(aTexCoordLoc);

const positionData = new Float32Array([
    -1, -1,
    1, -1,
    0, 1,
]);
const texCoordData = new Float32Array([
    0, 0,
    1, 0,
    0.5, 1
]);

const positionBuffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
gl.bufferData(gl.ARRAY_BUFFER, positionData, gl.STATIC_DRAW);
gl.vertexAttribPointer(aPositionLoc, 2, gl.FLOAT, false, 0, 0);

const texCoordBuffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, texCoordBuffer);
gl.bufferData(gl.ARRAY_BUFFER, texCoordData, gl.STATIC_DRAW);
gl.vertexAttribPointer(aTexCoordLoc, 2, gl.FLOAT, false, 0, 0);

const loadImage = () => new Promise(resolve => {
    const image = new Image();
    image.src = 'kitten.jpg';
    image.addEventListener('load', () => resolve(image));
});

// const pixels = new Uint8Array([
//     255, 255, 255, 230, 25, 75, 60, 180, 75, 255, 225, 25,
//     67, 99, 216, 245, 130, 49, 145, 30, 180, 70, 240, 240,
//     240, 50, 230, 188, 246, 12, 250, 190, 190, 0, 128, 128,
//     230, 190, 255, 154, 99, 36, 255, 250, 200, 0, 0, 0,
// ]);

const run = async () => {
    const image = await loadImage();
    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, 500, 300, 0, gl.RGB, gl.UNSIGNED_BYTE, image);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
};
run();
