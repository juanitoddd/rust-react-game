use web_sys::CanvasRenderingContext2d;

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