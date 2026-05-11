# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an OpenGL learning repository written in Rust, containing multiple example programs demonstrating advanced graphics techniques like environment mapping, cubemap reflections, and shadow mapping. The project is structured as a library (`cgl`) with multiple binary targets.

## Build and Run Commands

Build the library and all binaries:
```bash
cargo build
```

Run specific example programs:
```bash
cargo run --bin environment_mapping
cargo run --bin cube-map_reflections
cargo run --bin shadow-mapping
```

Build in release mode for better performance:
```bash
cargo build --release
cargo run --release --bin <binary-name>
```

Note: Development dependencies are compiled with optimizations (`opt-level = 3`) to improve performance during development.

## Project Architecture

### Library Structure (`src/`)

The core library `cgl` provides safe wrappers around raw OpenGL calls:

- **`lib.rs`**: Core OpenGL wrapper functions (shader compilation, buffer/texture/VAO/framebuffer creation and binding, drawing functions)
- **`camera.rs`**: Camera abstraction with `Transform` and projection matrix, provides view-projection matrices
- **`transform.rs`**: 3D transform system using position, quaternion rotation, and scale. Supports world-space and local-space movement/rotation, generates model and view matrices
- **`utils.rs`**: Mesh loading utilities using `russimp` for loading 3D models (`.glb` files), automatic VAO/VBO setup for position, normal, and texture coordinates

### Binary Targets

Each binary is in its own directory with a `main.rs`:

- **`environment_mapping/`**: Demonstrates cubemap environment backgrounds. Renders full-screen triangle with inverse view-projection to sample cubemap
- **`cube-map_reflections/`**: Shows dynamic cubemap reflections on objects. Renders scene into cubemap faces using geometry shader or multiple passes
- **`shadow-mapping/`**: Implements shadow mapping technique with depth buffer rendering

### Shader Organization

Shaders are in `shaders/` directory:
- Vertex shaders: `.vert` extension
- Fragment shaders: `.frag` extension
- Common shaders: `cubemap_background.{vert,frag}`, `cubemap_reflection.frag`, `shadowmap.frag`, `pos_norm.vert`, etc.

### Assets

`assets/` contains 3D models and textures:
- `cube.glb`, `sphere.glb`: 3D mesh models
- `skybox/`: Cubemap texture faces (front, back, left, right, top, bottom)

## Important Technical Notes

### Coordinate System Handling

This codebase deals with coordinate system conversions between OpenGL's right-handed and cubemap's left-handed systems:

- **Cubemap sampling**: OpenGL cubemaps use left-handed coordinates. When sampling, invert the Z component of the direction vector (see `environment_mapping/synopsis.md`)
- **Rendering to cubemap**: Cubemaps flip the V coordinate. Multiply MVP matrix by scale(1, -1, 1) to compensate (see `cube-map_reflections/synopsis.md`)

### Cubemap Face Rotations

When rendering to each cubemap face with inverted clip-space Z (for left-handed sampling), use these camera rotations:
- Right: +90° around Y
- Left: -90° around Y
- Top: -90° around X
- Bottom: +90° around X
- Front: 0° around Y
- Back: 180° around Y

### Graphics Features

- Enable `gl::TEXTURE_CUBE_MAP_SEAMLESS` for smooth cubemap filtering across faces
- Environment mapping works best on curved/small objects; avoid flat surfaces like cubes due to perspective issues
- Full-screen rendering: Use triangle with clip-space coords `[-1,-1]`, `[3,-1]`, `[-1,3]`, set `gl_Position.z = 0.999` for far depth

## Dependencies

- `gl`: OpenGL bindings
- `glfw`: Window creation and input handling
- `nalgebra-glm`: GLM-style math library for vectors, matrices, quaternions
- `russimp`: Assimp bindings for 3D model loading (with `prebuilt` feature)
- `image`: Image loading for textures

## Code Conventions

- Library functions are safe wrappers that handle `unsafe` OpenGL calls internally
- Shader sources are embedded using `include_str!()` macro
- Mesh data stored as `Vec<(u32, i32)>` where tuple is `(vao, index_count)`
- Transform uses quaternions for rotation to avoid gimbal lock
- Matrix multiplication order: projection * view * model (standard OpenGL convention)
