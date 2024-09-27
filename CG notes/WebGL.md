**Difference between `drawArrays` and `drawElements`**
Beneficial case for using `drawElements` is when there's need to produce smoothed image. Normals are shared and interpolated between faces thus giving smooth shape.
![[Pasted image 20240917222704.png]]

But for low poly meshes this would lead to increased memory consumption and give no benefit. So it is better to use `drawArrays` in this case.
![[Pasted image 20240917224602.png]]

**VAOs**
Vertex Array Object takes snapshot of attribute pointers configuration. Functions that affect VAO:
1. vertexAttribPointer
2. enableVertexAttribArray
3. disableVertexAttribArray
4. vertexAttribDivisor
5. bindBuffer(*ELEMENT_ARRAY_BUFFER*, ...)

**Mat4 attribute** takes 4 attribute locations
``` GLSL
layout(location=0) mat4 m;
// layout(location=1) float f; this does not compile
layout(location=4) mat4 m; // this does as every row of mat4 takes one location
```
In order to set mat4 attribute:
![[Pasted image 20240927181306.png]]