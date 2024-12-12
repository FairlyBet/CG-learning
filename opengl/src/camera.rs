use crate::transform::Transform;
use glm::Mat4;
use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub projection: Mat4,
}

impl Camera {
    pub fn new_perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Default::default(),
            projection: glm::perspective(aspect, fovy, near, far),
        }
    }

    pub fn from_transform_projection(transform: Transform, projection: Mat4) -> Self {
        Self {
            transform,
            projection,
        }
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection * self.transform.view()
    }
}
