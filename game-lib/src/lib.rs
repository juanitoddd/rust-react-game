// Setup related
mod macros;
mod utils;
mod entities;

// Game related
use entities::grid::Grid;

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

  // Draw the outer circle.
  context
      .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
      .unwrap();

  // Draw the mouth.
  context.move_to(110.0, 75.0);
  context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

  // Draw the left eye.
  context.move_to(65.0, 65.0);
  context
      .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
      .unwrap();

  // Draw the right eye.
  context.move_to(95.0, 65.0);
  context
      .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
      .unwrap();

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