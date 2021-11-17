use crate::model;

/// # matlab-like API of rust-Control
///
/// ## Data Types for Computing
///
/// In this lib, We use **f64** for all computing
pub type DType = f64;

/// ## model creation

/// ### transfer-function model
///
/// Create a transfer function system model.
///
pub fn tf(
    num: Vec<DType>,
    den: Vec<DType>,
    ts: Option<DType>,
) -> model::TransferFunction<DType, DType> {
    model::TransferFunction::from(num, den, ts)
}

/// ### Zero-pole-gain model
///
///Create a zero-pole-gain system
pub fn zpk(
    zeros: Vec<DType>,
    poles: Vec<DType>,
    gain: DType,
    ts: Option<DType>,
) -> model::ZeroPoleGain<DType, DType> {
    model::ZeroPoleGain::from(zeros, poles, gain, ts)
}

/// ### State-Space model
///
/// Create a state space model
pub fn ss() {}
