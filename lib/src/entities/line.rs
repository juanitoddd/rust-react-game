use crate::entities::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Line<'a> {
  a: &'a Point,          // Borrowed reference
  b: &'a Point,
}

impl<'a> Line<'a> {  

  /// Creates a new Line with two borrowed Points
  pub fn new(a: &'a Point, b: &'a Point) -> Self {
    Line { a, b }
  }

  /// Displays the line's points
  pub fn display(&self) {
    println!("Line: ({}, {}) to ({}, {})", self.a.x, self.a.y, self.b.x, self.b.y);
  }  
}