use web_sys::CanvasRenderingContext2d;
use crate::entities::point::Point;
use crate::entities::line::Line;
use crate::entities::tile::Tile;

#[derive(Debug)]
pub struct Grid {
  origin: (i32, i32),
  width: i32,
  height: i32,
  size: i32,
  tiles: Vec<Tile>
}

impl Grid {
  pub fn new() -> Self {
    Grid {
      origin: (0, 0),      
      width: 10,
      height: 10,
      size: 100,
      tiles: vec![]
    }
  }

  pub fn iso(origin: (i32, i32), width: i32, height: i32, size: i32) -> Self {

    let mut tiles: Vec<Tile> = vec![];
    for i in 0..width as i32 {
      for j in 0..height as i32 {
        log!("Iteration ({} {})", (i*size) + origin.0, (j*size) + origin.1);
        let tile = Tile::iso(((i*size) + origin.0, (j*size) + origin.1), (size, size));
        tiles.push(tile);  
      }
    }    

    Grid {
      origin: origin,      
      width: width,
      height: height,
      size: size,
      tiles: tiles
    }
  }

  pub fn ortho(origin: (i32, i32), width: i32, height: i32, size: i32) -> Self {
    let mut tiles: Vec<Tile> = vec![];
    for i in 0..width as i32 {
      for j in 0..height as i32 {
        log!("Iteration ({} {})", (i*size) + origin.0, (j*size) + origin.1);
        let tile = Tile::ortho(((i*size) + origin.0, (j*size) + origin.1), (size, size));
        tiles.push(tile);  
      }
    }    

    Grid {
      origin: origin,      
      width: width,
      height: height,
      size: size,
      tiles: tiles
    }
  }

  pub fn render(&self, context: &CanvasRenderingContext2d) {
    for item in &self.tiles {
      item.render(context);      
    }    
  }
}