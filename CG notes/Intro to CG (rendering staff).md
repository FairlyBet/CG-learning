### Rendering Equation
![[Pasted image 20241010173511.png]]
This is an integral over a hemisphere of a point that represent object's surface.
**Li** is intensity of a light
**ω**i is direction of light
**cosθ** is geometry term ( cosine of angle between light direction and normal )
fr - bidirectional reflectance distribution function, (describes material property and energy conservation)

### Rendering algorithms

* Rasterization
* Ray Tracing

---
### 1. **Rasterization**  
**Rasterization** is the process of converting 3D objects into 2D pixels on the screen. It’s the method traditionally used by GPUs in real-time graphics, especially in video games.

#### How It Works:
1. **Vertex Processing**: The 3D scene is transformed into a 2D image by projecting 3D vertices onto a 2D screen using transformations like perspective projection.
2. **Triangle Setup**: 3D objects are broken down into triangles (or other primitives), which are then processed one by one.
3. **Rasterization**: Each triangle is filled in, and pixels (fragments) within the triangle are calculated.
4. **Fragment Shading**: Lighting and texture calculations are applied to each pixel.
5. **Depth Testing**: The depth (z-value) of each pixel is calculated to determine visibility (which triangle is in front).

#### Pros of Rasterization:
- **Real-time Performance**: Rasterization is very fast and highly optimized for real-time applications like video games, due to hardware acceleration on GPUs.
- **Simple Implementation**: It’s relatively easy to implement, especially for basic lighting and shading.
- **Efficient for Simple Scenes**: It works well for scenes that don’t require complex lighting effects like reflections or shadows.
  
#### Cons of Rasterization:
- **Limited Lighting Effects**: Rasterization handles basic lighting (like shadows, reflections, and refractions) in an approximate way. More complex lighting effects require hacks or approximations, such as shadow maps and reflection probes, which can become less realistic.
- **No Global Illumination**: Rasterization lacks a natural way to simulate global illumination (the way light bounces around a scene), leading to less realistic results in some cases.
- **Overdraw and Artifacts**: When multiple triangles overlap the same pixel, the GPU may need to reprocess the same pixel multiple times (overdraw), which can affect performance in dense scenes.
- **Problematic to handle transparent objects**: Transparent and semi-transparent object requires tricks in implementation to be rendered correctly.

---

### 2. **Ray Tracing**  
**Ray Tracing** simulates the way light interacts with objects in a more physically accurate way by tracing the path of light rays from the camera through the scene. It can create more realistic lighting effects, like reflections, refractions, and shadows.

#### How It Works:
1. **Primary Ray Generation**: Rays are cast from the camera through each pixel into the scene.
2. **Intersection Testing**: Each ray is tested to see if it intersects with any objects in the scene.
3. **Shading**: Once a ray hits a surface, shading is calculated. Rays are then cast to light sources to calculate shadows, and secondary rays are cast to handle reflections and refractions.
4. **Recursive Ray Casting**: Rays bounce around the scene (e.g., reflecting off mirrors, refracting through glass) until they either hit a light source or are absorbed, simulating the complex interactions of light.

#### Pros of Ray Tracing:
- **Realistic Lighting and Shadows**: Ray tracing excels at producing realistic lighting, shadows, reflections, refractions, and caustics, since it simulates light as it would behave in the real world.
- **Global Illumination**: Ray tracing can naturally simulate global illumination, leading to more realistic images where light bounces around and affects all surfaces in the scene.
- **Accurate Reflections and Refractions**: It handles complex light interactions, like reflections off curved surfaces or light passing through transparent objects, more accurately than rasterization.

#### Cons of Ray Tracing:
- **Performance Cost**: Ray tracing is computationally expensive because it requires calculating the paths of many rays per pixel, making it traditionally slower than rasterization. This is especially true for real-time applications.
- **Hardware Demands**: Although modern GPUs (like NVIDIA’s RTX series) have specialized cores for ray tracing, it’s still more resource-intensive than rasterization, often requiring high-end hardware for real-time performance.
- **Complex Implementation**: Ray tracing involves more complex algorithms, especially when handling optimizations (e.g., using bounding volume hierarchies to reduce the number of intersection tests).

---

### Comparison of Rasterization and Ray Tracing

| **Aspect**            | **Rasterization**                          | **Ray Tracing**                             |
|-----------------------|--------------------------------------------|--------------------------------------------|
| **Speed**             | Fast, ideal for real-time rendering         | Slower, but modern hardware acceleration helps |
| **Lighting**          | Approximate lighting, good for basic effects | Accurate lighting, including reflections and refractions |
| **Real-time Suitability** | Widely used in real-time applications (games) | Traditionally used in offline rendering, but becoming more viable for real-time |
| **Realism**           | Limited realism, hacks required for complex effects | High realism, natural global illumination and shadows |
| **Complexity**        | Easier to implement, well-supported on GPUs | More complex, requires optimizations for performance |
| **Global Illumination** | Not directly supported, needs tricks (e.g., light maps) | Naturally supports global illumination |
| **Hardware Support**  | Highly optimized on modern GPUs            | Newer GPUs (RTX) have specialized ray tracing cores |

 