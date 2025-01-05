// Setup related
mod macros;
mod utils;
mod entities;

// Game related
use entities::{grid::Grid, transformation::TransformMatrix};
use entities::point::Point;
use entities::line::Line;
use entities::tile::Tile;

use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Error, Reflect};
use std::collections::HashMap;

#[wasm_bindgen(raw_module = "../../js/greeting.js")]
unsafe extern "C" {

    fn callback();
    fn name() -> String;
    
    type Greeting;

    #[wasm_bindgen(constructor)]
    unsafe fn new(msg: &str, recipient: &str) -> Greeting;

    #[wasm_bindgen(method)]
    unsafe fn greet(this: &Greeting);
}

fn extract_name(key: &JsValue) -> String {
  match key.as_string() {
      Some(name) => name,
      None => "?".into()
  }
}

#[wasm_bindgen]
pub fn greet(property_key: &JsValue, method_key: &JsValue) -> Result<(), JsValue> {

    let self_ =
        web_sys::window().ok_or_else(|| {
            make_error!("Can't access Window object")
        })?;

    let object =
        match Reflect::get(&self_, property_key) {
            Ok(value) if value.is_object() => {
                Ok(value)
            },
            _ => Err(make_error!("Window object doesn't have a suitable \"{}\" property", extract_name(property_key)))
        }?;

    let method: web_sys::js_sys::Function =
        match Reflect::get(&object, method_key) {
            Ok(value) if value.is_function() => {
                // wasm_bindgen::JsValue => js_sys::Function
                Ok(value.into())
            },
            _ => Err(make_error!("\"{}\" object doesn't have a suitable \"{}\" method", extract_name(property_key), extract_name(method_key)))
        }?;

    let arguments = web_sys::js_sys::Array::new();
    match Reflect::apply(&method, &object, &arguments) {
        Ok(_result) => {
            log!("Applied method successfully.");
            Ok(())
        },
        Err(error) => {
            log!("Attempt to apply method failed.");
            Err(error)
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn start_game() {

  let tile_size = 100;
  let mut line_color = "#888";

  #[wasm_bindgen]
  pub fn set_line_color(param: String) {
    log!("Param {:?}", param);
    // line_color = param.to_string()
  }  

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let width = canvas.client_width();
  let height = canvas.client_height();
  log!("canvas width {:?}", width);
  log!("canvas height {:?}", height);  

  let center_matrix = [[1.0, 0.0, (width/2) as f64],
                                  [0.0, 1.0, (height/2) as f64],
                                  [0.0, 0.0, 1.0]];
  let transform_matrix = TransformMatrix {
    matrix: center_matrix
  };

  let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

  let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();      

  // Create grid
  let render_grid = |color: &str| {
    // Clear canvas
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    context.begin_path();
    context.set_line_width(1.0);
    context.set_stroke_style_str(color);    
    
    let grid = Grid::iso((500,100), 5, 5, 50);
    grid.render(&context);

    let grid2 = Grid::ortho((700,300), 5, 5, 50);
    grid2.render(&context);

    // Tile::iso((0,0), (tile_size, tile_size)).transform(&transform_matrix).render(&context);
    // Tile::iso((tile_size, tile_size), (tile_size, tile_size)).transform(&transform_matrix).render(&context);
    // Tile::iso((tile_size, 0), (tile_size, tile_size)).transform(&transform_matrix).render(&context);
    // Tile::iso((0, tile_size), (tile_size, tile_size)).transform(&transform_matrix).render(&context);      
  
    context.stroke();
  };  

  // Create minimap
  let render_minimap = |color: &str| {

    let minimap_tile_size = 50;
    let minimap_origin = (200, 200);
    
    context.begin_path();
    context.set_line_width(1.0);
    context.set_stroke_style_str(color);    
    
    // Tile::new(minimap_origin, (minimap_tile_size, minimap_tile_size)).transform(&transform_matrix).render(&context);    
  
    context.stroke();
  };

  render_grid(line_color);
  render_minimap(line_color);

  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {    
    let x = event.offset_x() as f64;
    let y = event.offset_y() as f64;
    let pt = Point::iso(x, y);//.transform(&transform_matrix);
    log!("x {:?}", pt.x);
    log!("y {:?}", pt.y);
    // log!("Hello from {:?}", name());
    callback();
  }) as Box<dyn FnMut(_)>);

  canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
    .unwrap();
  closure.forget();  

}

#[wasm_bindgen]
pub fn get_info() {
  log!("get_info");
}

#[wasm_bindgen]
pub fn set_param(param: i8) {
  log!("Param {:?}", param);
}

#[wasm_bindgen]
pub fn set_color(param: String) {
  log!("Param {:?}", param);
}