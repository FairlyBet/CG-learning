use glm::{Mat4, Vec3};
use nalgebra_glm as glm;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: glm::Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Mat4 {
        glm::quat_to_mat4(&self.rotation) * glm::translation(&-self.position)
    }

    pub fn model(&self) -> Mat4 {
        glm::translation(&self.position)
            * glm::quat_to_mat4(&self.rotation)
            * glm::scaling(&self.scale)
    }

    pub fn move_(&mut self, offset: &Vec3) {
        self.position += offset;
    }

    pub fn move_local(&mut self, offset: &Vec3) {
        let (local_right, local_up, local_forward) = self.get_local_axes();
        self.position += local_right * offset.x + local_up * offset.y + local_forward * offset.z;
    }

    pub fn rotate(&mut self, euler: &Vec3) {
        self.rotate_around_axes(euler, &(*Vec3::x_axis(), *Vec3::y_axis(), *Vec3::z_axis()));
    }

    pub fn rotate_local(&mut self, euler: &Vec3) {
        let local_axes = self.get_local_axes();
        self.rotate_around_axes(euler, &local_axes);
    }

    pub fn rotate_around_axes(&mut self, euler: &Vec3, axes: &(Vec3, Vec3, Vec3)) {
        let radians = glm::radians(euler);
        let identity = glm::quat_identity();
        let x_rotation = glm::quat_rotate_normalized_axis(&identity, radians.x, &axes.0);
        let y_rotation = glm::quat_rotate_normalized_axis(&identity, radians.y, &axes.1);
        let z_rotation = glm::quat_rotate_normalized_axis(&identity, radians.z, &axes.2);

        self.rotation =
            glm::quat_normalize(&(z_rotation * y_rotation * x_rotation * self.rotation));
    }

    pub fn get_local_axes(&self) -> (Vec3, Vec3, Vec3) {
        let local_right = glm::quat_rotate_vec3(&self.rotation, &Vec3::x_axis());
        let local_up = glm::quat_rotate_vec3(&self.rotation, &Vec3::y_axis());
        let local_forward = glm::quat_rotate_vec3(&self.rotation, &Vec3::z_axis());

        (local_right, local_up, local_forward)
    }
}
