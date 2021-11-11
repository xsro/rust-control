mod state_space;
mod transfer_function;
mod zero_pole_gain;

pub use state_space::StateSpace;
pub use transfer_function::TransferFunction;
pub use zero_pole_gain::ZeroPoleGain;

pub enum LTIsystem {
    StateSpace,
    TransferFunction,
    ZeroPoleGain,
}
