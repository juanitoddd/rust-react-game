// Setup related
mod macros;
mod utils;
mod entities;

// Game related
use entities::grid::Grid;
use entities::point::Point;
use entities::line::Line;

use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Error, Reflect};

#[wasm_bindgen(raw_module = "../../js/greeting.js")]
extern "C" {

    fn callback();
    fn name() -> String;
    
    type Greeting;

    #[wasm_bindgen(constructor)]
    fn new(msg: &str, recipient: &str) -> Greeting;

    #[wasm_bindgen(method)]
    fn greet(this: &Greeting);
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
  // Initialize the grid
  let mut grid = Grid::new();
  grid.print_grid();

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let width = canvas.client_width();
  let height = canvas.client_height();
  log!("canvas width {:?}", width);
  log!("canvas height {:?}", height);
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

  // Clear canvas
  context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

  context.begin_path();
  context.set_line_width(1.0);

  let v1 = Point::from_i32(width/2, 0);
  let v2 = Point::from_i32(width/2, height);

  let h1 = Point::from_i32(0, height/2);
  let h2 = Point::from_i32(width, height/2);

  // Create a line using the new function
  let line_v = Line::new(v1, v2);
  let line_h = Line::new(h1, h2);          
  // Display the line
  line_h.display();

  // Draw the mouth.
  context.move_to(v1.clone().x, v1.clone().y);
  context.line_to(v2.x, v2.y);

  context.move_to(h1.x, h1.y);
  context.line_to(h2.x, h2.y);

  context.stroke();  

  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {    
    let x = event.offset_x() as f64;
    let y = event.offset_y() as f64;
    log!("x {:?}", x);
    log!("y {:?}", y);
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