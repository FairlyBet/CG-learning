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

The most popular in CG is cubic Bezier Curve. It is more flexible than quadratic curve yet not very complicated and it can be in several planes in 3d while quadratic curve is always contained by only 1 plane.
![[Pasted image 20240919203314.png]]
Curve always remains inside convex hull so it is easy to tell curve's position by its control points.
![[Pasted image 20240919204538.png]]

Curve can be **transformed** by applying transformation to its control points.

Derivative of a cubic curve is quadratic curve that represents direction of a curve.
Derivative of a quadratic curve is linear curve.
![[Pasted image 20240919205335.png]]

**Piecewise Polynomial curve continuity**
C0 - when the beginning of one curve is the end of another curve.
C1 - when C0 and also derivatives in the point of connection have the same direction and magnitude.
G1 - like C1 but they have only same direction
C2 - like C1 but with second derivatives 
G2 - like G1 but with second derivatives

C1 and G1 talks about direction of a curve
![[Pasted image 20240923142238.png]]
C2 and G2 talks about curvature of a curve
![[Pasted image 20240923142310.png]]

**Barycentric coordinates** represent a position of a point inside a plane represented by a triangle with [p0 p1 p2] vertices.

![[Pasted image 20240926173728.png]]

Sum of barycentric coordinates is **always** 1 as it is linear interpolation.
![[Pasted image 20240926174654.png]]

For when the point is **inside** the triangle:
![[Pasted image 20240926175011.png]]

Triangle Strip uses two previous and one new vertex to draw a triangle
The most efficient way to draw a mesh is Triangle Strip with `drawElements` function
![[Pasted image 20240926185511.png]]

How the Bilinear interpolation works on **texture sampling**
![[Pasted image 20240927131815.png]]
Difference between **UV** and **ST** coordinates (specifically in OpenGL)
UV goes from 0 to value of image resolution
ST goes from 0 to 1 (normalized UV)
![[Pasted image 20240927151100.png]]
Normal matrix applies transformation on normal 
**Normal matrix** production:
1. Take Model matrix and make it 3x3 as translation component isn't needed
2. Apply inverse
3. Apply transpose
![[Pasted image 20240928191537.png]]
Matrix is represented as Rotation2 * Scale * Rotation1. Applying inverse gets the desired Scale but also inverses Rotation component. As rotation matrices are orthogonal they may be inversed by transposing and transpose doesn't affect scale component. So after these operations the desired matrix that adjusts normals for non-uniform scale is acquired.

