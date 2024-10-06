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
uniform vec2 uCenter;            // Center of the twirl (normalized coordinates)
uniform float uRadius;           // Radius of the twirl effect
uniform float uTwirlStrength;    // Twirl strength (positive or negative for direction)
uniform float uAspect;

in vec2 vTexCoord;
out vec4 fragColor;

void main() {
    vec2 coord = vTexCoord - uCenter;
    float distance = length(vec2(coord.x, coord.y / uAspect));
    if (distance < uRadius) {
        float percent = (uRadius - distance) / uRadius;
        float angle = uTwirlStrength * percent * percent;
        float sinAngle = sin(angle);
        float cosAngle = cos(angle);
        coord = vec2(
            coord.x * cosAngle - coord.y * sinAngle,
            coord.x * sinAngle + coord.y * cosAngle
        );
    }
    coord += uCenter;
    fragColor = texture(uImage, coord);
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
const centerLocation = gl.getUniformLocation(program, 'uCenter');
const radiusLocation = gl.getUniformLocation(program, 'uRadius');
const twirlStrengthLocation = gl.getUniformLocation(program, 'uTwirlStrength');
const aspectLocation = gl.getUniformLocation(program, 'uAspect');
gl.uniform2f(centerLocation, 0.5, 0.5);
gl.uniform1f(radiusLocation, 0.2);
gl.uniform1f(twirlStrengthLocation, 3.0);
gl.uniform1f(aspectLocation, 600.0 / 336.0 );

const image = document.getElementById('image');
const texture = gl.createTexture();
gl.bindTexture(gl.TEXTURE_2D, texture);
gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
gl.generateMipmap(gl.TEXTURE_2D);

gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

const indexData = new Uint8Array([0, 1, 2]);
const indexBuffer = gl.createBuffer();
gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indexData, gl.STATIC_DRAW);

gl.drawElements(gl.TRIANGLES, 3, gl.UNSIGNED_BYTE, 0);
