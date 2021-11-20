#[macro_use]
extern crate assert_float_eq;

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
mod wasm;
pub use wasm::*;

/// some utility may be useful
pub mod utility;
