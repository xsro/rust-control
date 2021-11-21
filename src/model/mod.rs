mod ss;
mod tf;
mod zpk;

pub use ss::StateSpace;
pub use tf::TransferFunction;
pub use zpk::ZeroPoleGain;

pub enum LTIsystem {
    StateSpace,
    TransferFunction,
    ZeroPoleGain,
}
