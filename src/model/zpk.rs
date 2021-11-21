//! Zero-pole-gain model
//!
//! ## references
//!
//! - [matlab zpk function](https://www.mathworks.com/help/control/ref/zpk.html)

use super::TransferFunction;
use crate::utility::roots;
use num::Complex;

/// Zero-pole-gain model
#[derive(Debug)]
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
    pub fn from_tf(tf: TransferFunction<T, U>) -> ZeroPoleGain<T, U>
    where
        T: num::Float + num::Signed + num::Bounded + std::fmt::Debug,
    {
        let z = roots::bairstow::solve2(&tf.numerator);
        let p = roots::bairstow::solve2(&tf.denominator);
        let k = tf.numerator[0] / tf.denominator[0];
        ZeroPoleGain { z, p, k, ts: tf.ts }
    }
}

impl<T, U> std::cmp::PartialEq for ZeroPoleGain<T, U>
where
    T: std::cmp::PartialEq,
    U: std::cmp::PartialEq,
{
    fn eq(&self, other: &ZeroPoleGain<T, U>) -> bool {
        if self.z != other.z {
            return false;
        };
        if self.p != other.p {
            return false;
        };
        if self.k != other.k {
            return false;
        };
        true
    }
}

#[cfg(test)]
mod bairstow_method_tests {
    use super::super::TransferFunction;
    use super::*;
    #[test]
    fn test_from_tf() {
        let g: TransferFunction<f32, f32> =
            TransferFunction::from(vec![2.0, -4.0, 2.0], vec![1.0, -2.0, 2.0]);
        let g = ZeroPoleGain::from_tf(g);
        let g2: ZeroPoleGain<f32, f32> = ZeroPoleGain::from(
            vec![Complex::from(1.0), Complex::from(1.0)],
            vec![Complex::new(1.0, 1.0), Complex::new(1.0, -1.0)],
            2.0,
        );
        assert_eq!(g, g2);
    }
}
