**Difference between `drawArrays` and `drawElements`**
Beneficial case for using `drawElements` is when there's need to produce smoothed image. Normals are shared and interpolated between faces thus giving smooth shape.
![[Pasted image 20240917222704.png]]

But for low poly meshes this would lead to increased memory consumption and give no benefit. So it is better to use `drawArrays` in this case.
![[Pasted image 20240917224602.png]]

TODO: Write about how VAOs are created and edited.