#[derive(Debug)]
pub struct Point {
  x: f64,
  y: f64,    
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

  fn display(&self) {
    println!("Point({}, {})", self.x, self.y);
  }

  pub fn get_coords(&self) -> (f64, f64) {
    (self.x, self.y)    
  }
}

// Implement Copy
// impl Copy for Point {}

// Implement Clone (required for Copy)
// impl Clone for Point {
//   fn clone(&self) -> Self {
//       *self // Simply dereference `self` to copy the value
//   }
// }