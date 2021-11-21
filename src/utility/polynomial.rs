//! # Use Vec as Polynomial
//!
//! There some crates for polynomial like following list,
//! But they are all in development, So I implement what I need in this Module
//!
//! - [philippeitis/rustnomial](https://github.com/philippeitis/rustnomial)
//! - [cargodog/polynomials](https://github.com/cargodog/polynomials)

use num::traits::sign::Signed;
use num::traits::Num;
use num::Complex;
use num::{One, Zero};

pub trait Polynomial<T: Num> {
    ///Compute the value for x
    fn eval(&self, x: T) -> T;
    fn eval_complex(&self, x: Complex<T>) -> Complex<T>;
}

impl<T: Num + Copy> Polynomial<T> for Vec<T> {
    fn eval(&self, x: T) -> T {
        let mut value = T::zero();
        let mut mult = T::one();
        for &item in self.iter().rev() {
            value = value + item * mult;
            mult = mult * x;
        }
        value
    }
    fn eval_complex(&self, x: Complex<T>) -> Complex<T> {
        let mut value = Complex::zero();
        let mut mult = Complex::one();
        for &item in self.iter().rev() {
            let index = Complex::from(item);
            value = value + index * mult;
            mult = mult * x;
        }
        value
    }
}

pub trait PolynomialStringfy<T> {
    ///Convert Polynomial to string
    fn to_string(&self, val_str: &str) -> String;
}

impl<T: std::fmt::Display + Signed + Copy> PolynomialStringfy<T> for Vec<T> {
    fn to_string(&self, val_str: &str) -> String {
        let mut output = String::new();
        for (i, &item) in self.iter().rev().enumerate() {
            let str = match i {
                0 => format!("{}", item.abs()),
                _ => {
                    format!("{}{}^{}", item.abs(), val_str, i)
                }
            };
            if item.is_positive() {
                if i == self.len() - 1 {
                    output = format!("{} {}", str, output);
                } else {
                    output = format!("+ {} {}", str, output);
                }
            }
            if item.is_negative() {
                output = format!("- {} {}", str, output);
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::polynomial::Polynomial;
    use crate::utility::polynomial::PolynomialStringfy;
    #[test]
    fn polynomial_test_f64() {
        let v: Vec<f64> = vec![1.0, -2.0, 3.0];
        //test toString
        let str = v.to_string("s");
        assert_eq!(str, "1s^2 - 2s^1 + 3 ");
        //test
        let x = 4.0;
        let y = 11.0;
        assert_eq!(v.eval(x), y);
        let x = num::Complex::from(4.0);
        let y = num::Complex::from(11.0);
        assert_eq!(v.eval_complex(x), y);
        let x = num::Complex::new(1.0, 1.0);
        let y = num::Complex::from(1.0);
        assert_eq!(v.eval_complex(x), y);
        let x = num::Complex::new(0.0, 1.0);
        let y = num::Complex::new(2.0, -2.0);
        assert_eq!(v.eval_complex(x), y);
    }
    #[test]
    fn polynomial_test_i32() {
        let v: Vec<i32> = vec![1, 2];
        let str = v.to_string("s");
        assert_eq!(str, "1s^1 + 2 ");
        assert_eq!(v.eval(4), 6);
    }
}
