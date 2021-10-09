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

#[cfg(test)]
mod tests {
    #[test]
    fn model_creation() {
        let g = crate::model::transfer_function::TransferFunction::from(
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            None,
        );
        print!("{}", g);
    }
}
