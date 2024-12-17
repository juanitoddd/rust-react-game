use web_sys::CanvasRenderingContext2d;
use crate::entities::line::Line;
use crate::entities::transformation::TransformMatrix;

#[derive(Debug)]
pub struct Tile {
  lines: (Line, Line, Line, Line),  
}

impl Tile {
  pub fn new(origin: (i32, i32), size: (i32, i32)) -> Self {

    let a = (origin.0, origin.1);
    let b = (origin.0 + size.0, origin.1);
    let c = (origin.0, origin.1 + size.1);
    let d = (origin.0 + size.0, origin.1 + size.1);

    // Create a line using the iso fn
    let l1 = Line::iso(a, b);
    let l2 =Line::iso(a, c);
    let l3 =Line::iso(d, b);
    let l4 =Line::iso(d, c);

    Tile { lines: (l1, l2, l3 ,l4) }
  }

  /// Transforms the line using a matrix
  pub fn transform(&self, matrix: &TransformMatrix) -> Self {
    Tile { lines: 
        (
            self.lines.0.transform(matrix), 
            self.lines.1.transform(matrix), 
            self.lines.2.transform(matrix),
            self.lines.3.transform(matrix)
        )         
    }    
  }

  pub fn render(&self, context: &CanvasRenderingContext2d) {
    self.lines.0.render(context);
    self.lines.1.render(context);
    self.lines.2.render(context);
    self.lines.3.render(context);
  }
}