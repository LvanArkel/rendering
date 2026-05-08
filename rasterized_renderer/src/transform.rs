use cgmath::{Matrix4, SquareMatrix, Vector3};

pub struct Transform {
    pub position: Vector3<f32>
}

impl Transform {
    pub fn offset(&self) -> Vector3<f32> {
        self.position
    }
}
