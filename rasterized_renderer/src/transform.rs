use cgmath::{Matrix4, Vector3};

pub struct Transform {
    pub position: Vector3<f32>
}

impl Transform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
    }
}
