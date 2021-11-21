//! # matlab-like API of rust-Control
//!
//! This mod provides some functions which is like MATLAB's implementation
//! in [Control System Toolbox](https://www.mathworks.com/products/control.html)
//!
//! - The Module uses `DType = f64` for most math computation
use crate::*;
use num::Complex;

/// ## Data Types for Computing
///
/// In this lib, We use **f64** for all computing
pub type DType = f64;

/// ## transfer-function model
///
/// Create a transfer function system model.
///
pub fn tf(
    num: Vec<DType>,
    den: Vec<DType>,
    ts: Option<DType>,
) -> model::TransferFunction<DType, DType> {
    model::TransferFunction::new(num, den, ts)
}

/// ## Zero-pole-gain model
///
/// Create a zero-pole-gain system
pub fn zpk(
    zeros: Vec<Complex<DType>>,
    poles: Vec<Complex<DType>>,
    gain: DType,
    ts: Option<DType>,
) -> model::ZeroPoleGain<DType, DType> {
    model::ZeroPoleGain::new(zeros, poles, gain, ts)
}

/// ## State-Space model
///
/// Create a state space model
pub fn ss() {}
