#[derive(Debug)]
pub struct Grid {
  width: i32,
  height: i32,    
}

impl Grid {
  pub fn new() -> Self {
    Grid { width: 10, height: 10 }
  }

  fn set_width(&mut self, _width: i32) {
    self.width = _width;
  }

  fn get_width(&self) -> i32 {
    self.width
  }
  pub fn print_grid(&self) {
    log!("width {:?}", self.get_width());
  }
}