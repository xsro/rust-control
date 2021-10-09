use crate::config::DType;
use ndarray::Array2;

/// A struct representating State-Space model
#[warn(non_snake_case)]
pub struct StateSpace {
    A: Array2<DType>,
    B: Array2<DType>,
    C: Array2<DType>,
    D: Array2<DType>,
    ts: Option<DType>,
}

impl StateSpace {
    pub fn to_string(self: &StateSpace) {}
}
