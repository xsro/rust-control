//! Zero-pole-gain model
//!
//! ## references
//!
//! - [matlab zpk function](https://www.mathworks.com/help/control/ref/zpk.html)

use num::Complex;

/// Zero-pole-gain model
pub struct ZeroPoleGain<T, U> {
    ///system zeros
    pub z: Vec<Complex<T>>,
    ///system poles
    pub p: Vec<Complex<T>>,
    ///system gain(s)
    pub k: T,
    ///Sample time
    ///- `Some(U)`: A positive scalar representing the sampling period of a discrete-time system
    ///- `None`: for continuous-time systems.
    pub ts: Option<U>,
}

impl<T, U> ZeroPoleGain<T, U> {
    pub fn new(z: Vec<Complex<T>>, p: Vec<Complex<T>>, k: T, ts: Option<U>) -> ZeroPoleGain<T, U> {
        ZeroPoleGain { z, p, k, ts }
    }
    pub fn from(z: Vec<Complex<T>>, p: Vec<Complex<T>>, k: T) -> ZeroPoleGain<T, U> {
        ZeroPoleGain { z, p, k, ts: None }
    }
}
