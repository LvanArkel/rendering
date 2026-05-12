use cgmath::{Deg, Matrix4, Point3, Vector3, perspective};

use crate::transform::Transform;

pub struct Camera {
    position: Point3<f32>,
    target: Point3<f32>,
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        fov_y: Deg<f32>,
        aspect_ratio: f32,
        near_z: f32,
        far_z: f32
    ) -> Self {
        let projection = perspective(fov_y, aspect_ratio, near_z, far_z);
        Self { position, target, projection }
    }

    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        let view_matrix = Matrix4::look_at_rh(self.position, self.target, Vector3::unit_y());
        self.projection * view_matrix
    }
}