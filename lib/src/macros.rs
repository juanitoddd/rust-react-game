#![macro_use]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// A macro to provide `println!(..)`-style syntax for `console.error` logging.
#[macro_export]
macro_rules! err {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

macro_rules! make_error {
    ( $( $t:tt )* ) => {
        // js_sys::Error => wasm_bindgen::JsValue
        Error::new( &format!( $( $t )* ) ).unchecked_into::<JsValue>()
    }
}