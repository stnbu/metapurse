use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! l {
    ($($t:tt)*) => (console::log(&format_args!($($t)*).to_string()))
}

pub(crate) use l;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
