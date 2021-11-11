use crate::utility::polynomial::{Polynomial, PolynomialStringfy};
use num::complex::Complex;
use num::{Signed, Zero};
use std::fmt;

pub struct TransferFunction<T, U> {
    numerator: Vec<T>,
    denominator: Vec<T>,
    sample_time: Option<U>,
}

impl<T, U> TransferFunction<T, U> {
    pub fn from(num: Vec<T>, den: Vec<T>, ts: Option<U>) -> TransferFunction<T, U> {
        TransferFunction {
            numerator: num,
            denominator: den,
            sample_time: ts,
        }
    }

    pub fn frequency_response(&self, omega: T) -> Complex<T>
    where
        T: num::Num + std::marker::Copy,
    {
        let mut s: Complex<T> = Complex::zero();
        s.im = omega;
        self.numerator.eval_complex(s) / self.denominator.eval_complex(s)
    }

    pub fn to_string(&self, tex: bool) -> String
    where
        T: std::fmt::Display + Signed + std::marker::Copy,
        U: std::fmt::Display,
    {
        let val_str = match &self.sample_time {
            Some(_ts) => "z",
            None => "s",
        };
        let sample_time_message = match &self.sample_time {
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
