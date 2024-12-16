// Define a 2D transformation matrix as a struct
pub struct TransformMatrix {
    pub matrix: [[f64; 3]; 3], // 3x3 matrix for 2D transformations
}

// Trait for applying transformations
pub trait Transformation {
    fn transform(&self, matrix: &TransformMatrix) -> Self;
}
