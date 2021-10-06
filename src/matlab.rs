use crate::config::DType;
use crate::model;

/// # matlab like functions
/// 
/// ## model creation
///
/// ### transfer-function model
///
/// Create a transfer function system model.
///
pub fn tf(
    num: Vec<DType>,
    den: Vec<DType>,
    ts: Option<DType>,
) -> model::transfer_function::TransferFunction {
    model::transfer_function::TransferFunction::from(num, den, ts)
}

/// ### Zero-pole-gain model
///
///Create a zero-pole-gain system
pub fn zpk(
    zeros: Vec<DType>,
    poles: Vec<DType>,
    gain: DType,
    ts: Option<DType>,
) -> model::zero_pole_gain::ZeroPoleGainModel {
    model::zero_pole_gain::ZeroPoleGainModel::from(zeros, poles, gain, ts)
}

/// ### State-Space model
///
/// Create a state space model
pub fn ss() {}
