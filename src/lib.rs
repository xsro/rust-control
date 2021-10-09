mod config;
mod utils;

/// mod for creating control system model
pub mod model;

/// mod for frequent-domain analysis
pub mod freq;
/// mod for time-domain analysis
pub mod time;

/// mod for Control system design
pub mod design;
/// mod for stability analysis
pub mod stability;

/// mod exposing matlab-like functions
pub mod matlab;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> Vec<f64> {
    alert("Hello, wasm-game-of-life!");
    vec![1.0, 1.0]
}

use config::DType;

#[wasm_bindgen]
pub fn sine(num: Vec<DType>, den: Vec<DType>, omega: DType) -> Vec<DType> {
    let g = model::TransferFunction::from(num, den, None);
    let r = g.frequency_response(omega);
    vec![r.re, r.im, r.norm(), r.arg()]
}
