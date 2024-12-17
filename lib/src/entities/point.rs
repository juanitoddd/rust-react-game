use crate::entities::transformation::TransformMatrix;

#[derive(Debug, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,    
}

impl Point {
  pub fn new(_x:f64, _y: f64) -> Self {
    Point { x: _x, y: _y }
  }

  pub fn origin() -> Self {
    Point { x: 0.0, y: 0.0 }
  }  

  pub fn from_i32(_x:i32, _y: i32) -> Self {
    Point { x: f64::from(_x), y: f64::from(_y) }
  }

  pub fn display(&self) {
    println!("Point({}, {})", self.x, self.y);
  }

  pub fn iso(_x:f64, _y: f64) -> Self {
    let trx: [[f64; 3]; 3] = 
      [[1.0, -1.0, 0.0],
      [0.5, 0.5, 0.0],
      [0.0, 0.0, 1.0]];
    // Define a translation matrix
    let transform_matrix = TransformMatrix {
      matrix: trx
    };
    let trx_pt = Point::new(_x, _y).transform(&transform_matrix);    
    Point { x: trx_pt.x, y: trx_pt.y }    
  }

  pub fn get_coords(&self) -> (f64, f64) {
    (self.x, self.y)    
  }

  /// Transforms the coords using a matrix
  pub fn transform(&self, matrix: &TransformMatrix) -> Self {
    let x = self.x * matrix.matrix[0][0] + self.y * matrix.matrix[0][1] + matrix.matrix[0][2];
    let y = self.x * matrix.matrix[1][0] + self.y * matrix.matrix[1][1] + matrix.matrix[1][2];
    Point { x, y }
}
}