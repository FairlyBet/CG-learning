const canvas = document.querySelector('canvas');
const gl = canvas.getContext('webgl2');

gl.viewport(0, 0, canvas.width, canvas.height);

const vsource = /*glsl*/ `#version 300 es

out vec2 vTexCoord;

void main() {
    int x = gl_VertexID & 1;
    int y = gl_VertexID / 2;
    vTexCoord = vec2(float(x) * 2.0, float(y) * 2.0);
  
    gl_Position = vec4(
        vTexCoord.x * 2.0 - 1.0,
        vTexCoord.y * 2.0 - 1.0,
        0.0,
        1.0
    );
}`;
const fsource = /*glsl*/ `#version 300 es

precision mediump float;

uniform sampler2D uImage;
uniform vec2 uTexelSize;

in vec2 vTexCoord;
out vec4 fragColor;

void main() {
    const float KERNEL[] = float[]( 
        1.0,  1.0, 1.0, 
        1.0, -8.0, 1.0,
        1.0,  1.0, 1.0
    );
    
    const vec2 OFFSETS[] = vec2[](
        vec2(-1.0,  1.0), vec2(0.0,  1.0), vec2(1.0,  1.0),
        vec2(-1.0,  0.0), vec2(0.0,  0.0), vec2(1.0,  0.0),
        vec2(-1.0, -1.0), vec2(0.0, -1.0), vec2(1.0, -1.0)
    );
    
    vec4 sum = vec4(0.0);
    for (int i = 0; i < 9; i++) {
        vec2 offset = vTexCoord + OFFSETS[i] * uTexelSize;
        vec4 pixel = texture(uImage, offset);
        sum += pixel * KERNEL[i];
    }
    fragColor = vec4(sum.rgb, 1.0);
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
const texelSizeLocation = gl.getUniformLocation(program, 'uTexelSize');
gl.uniform2f(texelSizeLocation, 1.0 / 600.0, 1.0 / 336.0);

const image = document.getElementById('image');
const texture = gl.createTexture();
gl.bindTexture(gl.TEXTURE_2D, texture);
gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
gl.generateMipmap(gl.TEXTURE_2D);

const indexData = new Uint8Array([0, 1, 2]);
const indexBuffer = gl.createBuffer();
gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indexData, gl.STATIC_DRAW);

gl.drawElements(gl.TRIANGLES, 3, gl.UNSIGNED_BYTE, 0);