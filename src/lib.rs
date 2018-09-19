extern crate cfg_if;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod gol;
mod utils;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}