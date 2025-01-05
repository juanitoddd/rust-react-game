use web_sys::CanvasRenderingContext2d;

use crate::entities::point::Point;
use crate::entities::transformation::TransformMatrix;
  
#[derive(Debug)]
pub struct Line {
  a: Point, // Line owns its points
  b: Point,
}

impl Line {  

  /// Creates a new Line with two borrowed Points
  pub fn new(a: Point, b: Point) -> Self {
    Line { a, b }
  }

  pub fn ortho(a: (i32, i32), b: (i32, i32)) -> Self {
    let ortho_a = Point::from_i32(a.0, a.1);
    let ortho_b = Point::from_i32(b.0, b.1);
    Line { a: ortho_a, b: ortho_b }
  }

  pub fn iso(a: (i32, i32), b: (i32, i32)) -> Self {
    let trx: [[f64; 3]; 3] = 
      [[1.0, -1.0, 0.0],
      [0.5, 0.5, 0.0],
      [0.0, 0.0, 1.0]];
    // Define a translation matrix
    let transform_matrix = TransformMatrix {
      matrix: trx
    };
    let trx_a = Point::from_i32(a.0, a.1).transform(&transform_matrix);
    let trx_b = Point::from_i32(b.0, b.1).transform(&transform_matrix);
    Line { a: trx_a, b: trx_b }
  }

  pub fn iso_center(a: (i32, i32), b: (i32, i32), width: i32, height:i32) -> Self {
    let trx: [[f64; 3]; 3] = 
      [[1.0, -1.0, (width/2) as f64],
      [0.5, 0.5, (height/2) as f64],
      [0.0, 0.0, 1.0]];
    // Define a translation matrix
    let transform_matrix = TransformMatrix {
      matrix: trx
    };
    let trx_a = Point::from_i32(a.0, a.1).transform(&transform_matrix);
    let trx_b = Point::from_i32(b.0, b.1).transform(&transform_matrix);
    Line { a: trx_a, b: trx_b }
  }

  /// Displays the line's points
  pub fn display(&self) {
    log!("Line: ({}, {}) to ({}, {})", self.a.x, self.a.y, self.b.x, self.b.y);    
  }

  /// Transforms the line using a matrix
  pub fn transform(&self, matrix: &TransformMatrix) -> Self {
    Line {
        a: self.a.transform(matrix),
        b: self.b.transform(matrix),
    }
  }

  pub fn render(&self, context: &CanvasRenderingContext2d) {
    // log!("Line: ({}, {}) to ({}, {})", self.a.x, self.a.y, self.b.x, self.b.y);    
    context.move_to(self.a.x, self.a.y);
    context.line_to(self.b.x, self.b.y);
  }
}