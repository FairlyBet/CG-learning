# Rendering into a cubemap

When rendering into a cubemap keep in mind that it flips V coordinate.
So everything will appear upside down. Pretty easy way to deal with it is to
multiply MVP matrix with [1, -1, 1] scale matrix.

``` rust
let flip_v = scale(1, -1, 1);
let mvp = flip_v * mvp
```

Camera rotation for each face of a cubemap,
assuming that sign of clip-space Z coordinate
will be inverted to match left-handed coordinate system
when sampling result cubemap:

``` rust
let rotations = [
  // right
  rotation(  90.0.to_radians(), Vec3::y_axis() ),
  // left
  rotation( -90.0.to_radians(), Vec3::y_axis() ),
  // top
  rotation( -90.0.to_radians(), Vec3::x_axis() ),
  // bottom
  rotation(  90.0.to_radians(), Vec3::x_axis() ),
  // front
  rotation(   0.0.to_radians(), Vec3::y_axis() ),
  // back
  rotation( 180.0.to_radians(), Vec3::y_axis() ),
];
```

# Cubemap reflections

Cubemap reflection work fine with some roundy or not flat objects.
It completely doesn't work with plane-like surfaces like cube, because of
unnatural perspective. So if surface's normals direction changes smoothly
or object is small relatively to other objects in the scene or relfective surface
is small it probably will work well.

## OpenGL specific

There is a way to render entire cubemap with a single draw call using geometry shader 
and attaching the whole cubemap to a framebuffer as layered texture.
