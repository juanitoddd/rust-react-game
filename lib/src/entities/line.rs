use crate::entities::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Line<'a> {
  a: &'a Point,          // Borrowed reference
  b: &'a Point,
}

// impl Line {
  // pub fn new(_a:Point, _b: Point) -> Self {
  //     Line { a: _a, b: _b }
  // }

  // pub fn get_points(&self) -> (Point, Point) {
  //   (self.a, self.b)
  // }
// }