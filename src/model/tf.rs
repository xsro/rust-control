//! Transfer function representation of Control System
//!
//! references:
//! - [matlab tf function](https://www.mathworks.com/help/control/ref/tf.html)

use crate::utility::polynomial::{Polynomial, PolynomialStringfy};
use num::complex::Complex;
use num::{Signed, Zero};
use std::fmt;

/// # Transfer function model
///
/// This struct representing a transfer function model.
///
/// $$
/// sys(s)=\frac{numerator(s)}{denominator(s)}
/// $$
///
/// $$
/// sys(z)=\frac{numerator(z)}{denominator(z)}
/// $$
pub struct TransferFunction<T, U> {
    ///Numerator coefficients
    pub numerator: Vec<T>,
    ///Denominator coefficients
    pub denominator: Vec<T>,
    ///Sample time
    /// - For continuous system, set property sample_time as None
    /// - For discrete system, set property sample_time as its value
    pub ts: Option<U>,
}

impl<T, U> TransferFunction<T, U> {
    /// create a new transfer-function model
    pub fn new(numerator: Vec<T>, denominator: Vec<T>, ts: Option<U>) -> TransferFunction<T, U> {
        TransferFunction {
            numerator,
            denominator,
            ts,
        }
    }

    /// create a contimuous-time transfer-function model
    pub fn from(numerator: Vec<T>, denominator: Vec<T>) -> TransferFunction<T, U> {
        TransferFunction::new(numerator, denominator, None)
    }

    /// the response to a sinusoidal input
    /// replace $s$ with $j\omega$ and compute the result
    pub fn frequency_response(&self, omega: T) -> Complex<T>
    where
        T: num::Num + std::marker::Copy,
    {
        let mut s: Complex<T> = Complex::zero();
        s.im = omega;
        self.numerator.eval_complex(s) / self.denominator.eval_complex(s)
    }

    ///format to a human-readable string
    /// set `tex` to true if you want to use a tex style
    pub fn to_string(&self, tex: bool) -> String
    where
        T: std::fmt::Display + Signed + std::marker::Copy,
        U: std::fmt::Display,
    {
        let val_str = match &self.ts {
            Some(_ts) => "z",
            None => "s",
        };
        let sample_time_message = match &self.ts {
            Some(ts) => format!("discrete system with sample time:{}", ts),
            None => format!("continuous system"),
        };
        match tex {
            true => format!(
                "\\frac{{{}}}{{{}}}\n{}",
                self.numerator.to_string(val_str),
                self.denominator.to_string(val_str),
                sample_time_message
            ),
            false => format!(
                "
    {}
    -------------
    {}
{}
    ",
                self.numerator.to_string(val_str),
                self.denominator.to_string(val_str),
                sample_time_message
            ),
        }
    }
}

impl<T, U> fmt::Display for TransferFunction<T, U>
where
    T: std::fmt::Display + Signed + std::marker::Copy,
    U: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "transfer function system model:\n{}",
            self.to_string(false)
        )
    }
}
