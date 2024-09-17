**Difference between `drawArrays` and `drawElements`**
When using `drawElements` the produced image is smoothed because normal interpolation between faces. This is beneficial use case of `drawElements`.
![[Pasted image 20240917222704.png]]

But for low poly meshes this would lead to increased memory consumption and give no benefit. So it is better to use `drawArrays` in this case.
![[Pasted image 20240917224602.png]]

