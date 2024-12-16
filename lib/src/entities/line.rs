use crate::entities::point::Point;
use crate::entities::transformation::*;
  
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

  /// Displays the line's points
  pub fn display(&self) {
    log!("Line: ({}, {}) to ({}, {})", self.a.x, self.a.y, self.b.x, self.b.y);
    // println!("Line: ({}, {}) to ({}, {})", self.a.x, self.a.y, self.b.x, self.b.y);
  }  
}

// Implement the Transformation trait for Line
impl Transformation for Line {
  fn transform(&self, matrix: &TransformMatrix) -> Self {
      Line {
          a: self.a.transform(matrix),
          b: self.b.transform(matrix),          
      }
  }
}