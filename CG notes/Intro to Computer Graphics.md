**Swizzling** — changing image memory layout for better cache locality when accessing neighbor pixels.
![[Pasted image 20240913163331.png]]


**Homogeneous coordinate**
If homogeneous coordinate is set to 1 then the *translation* matrix will be applied
If homogeneous coordinate is set to 0 then the translation transformation will be ignored which is handful for direction vectors
![[Pasted image 20240913203832.png]]

**Example of implicit and parametric representation of a curve**

![[Pasted image 20240916170759.png]]

**Bezier Curve definition**
![[Pasted image 20240919201724.png]]p0, p1, p2 are control points, that form the curve

The most popular is cubic Bezier Curve
![[Pasted image 20240919203314.png]]
Curve always remains inside convex hull so it is easy to tell curve's position by its control points.
![[Pasted image 20240919204538.png]]

Curve can be **transformed** by applying transformation to its control points.

Derivative of a cubic curve is quadratic curve that represents direction of a curve
![[Pasted image 20240919205335.png]]