use num;
use num::Complex;

/// Bairstow Method for finding roots of nonlinear quations
///
/// This method split polynomial A to polynomial B * (x^2-rx-s)
#[allow(non_snake_case)]
fn bairstow<T>(A: &Vec<T>, r0: T, s0: T, epsilon_s: T) -> Result<(Vec<T>, T, T), T>
where
    T: num::traits::Num + Copy + num::Signed + core::cmp::PartialOrd,
{
    let n: usize = A.len() - 1;
    let mut r = r0;
    let mut s = s0;
    loop {
        let mut B: Vec<T> = A.clone();
        B[n] = A[n];
        B[n - 1] = A[n - 1] + r * B[n];
        for index in 0..=n - 2 {
            let i = n - 2 - index;
            B[i] = A[i] + r * B[i + 1] + s * B[i + 2];
        }
        let mut C: Vec<T> = B.clone();
        C[n - 1] = B[n - 1] + r * C[n];
        for index in 1..=n - 2 {
            let i = n - 2 - index + 1;
            C[i] = B[i] + r * C[i + 1] + s * C[i + 2];
        }
        let ds: T = (B[0] * C[2] - B[1] * C[1]) / (C[1] * C[3] - C[2] * C[2]);
        let dr: T = (B[0] * C[3] - C[2] * B[1]) / (C[2] * C[2] - C[1] * C[3]);
        s = s + ds;
        r = r + dr;
        let epsilon_a_s = ds / s;
        let epsilon_a_r = dr / r;
        if num::abs(epsilon_a_r) < epsilon_s && num::abs(epsilon_a_s) < epsilon_s {
            let B = B.splice(2.., []).collect();
            return Ok((B, r, s));
        }
    }
}

///solve quadratic function x^2-r*x-s=0
fn solve_quadratic<T: num::Float + num::Signed>(r: T, s: T) -> (Complex<T>, Complex<T>) {
    let delta = r * r + s + s + s + s;
    if delta.is_negative() {
        let re = r;
        let im = (-delta).sqrt();
        let two = T::one() + T::one();
        (
            Complex::new(re / two, im / two),
            Complex::new(re / two, -im / two),
        )
    } else {
        let re = r;
        let re2 = delta.sqrt();
        let two = T::one() + T::one();
        (
            Complex::from((re + re2) / two),
            Complex::from((re - re2) / two),
        )
    }
}

/// (quotient,remainder)=function(vec)
/// - https://nptel.ac.in/content/storage2/courses/122104019/numerical-analysis/Rathish-kumar/ratish-1/f3node9.html
pub fn solve<T>(poly: &Vec<T>, eps: T) -> Vec<Complex<T>>
where
    T: num::Float + num::Signed + std::fmt::Debug,
{
    let mut a: Vec<T> = poly.clone();
    let mut result: Vec<Complex<T>> = Vec::new();
    loop {
        let (b, r, s) = bairstow(&a, T::zero(), T::zero(), eps).unwrap();
        let (root1, root2) = solve_quadratic(r, s);
        result.push(root1);
        result.push(root2);
        match b.len() {
            2 => {
                result.push(Complex::from(-b[0] / b[1]));
                break;
            }
            3 => {
                let r = -b[1] / b[2];
                let s = -b[0] / b[2];
                let (root1, root2) = solve_quadratic(r, s);
                result.push(root1);
                result.push(root2);
                break;
            }
            _ => a = b,
        };
    }
    result
}

#[cfg(test)]
mod bairstow_method_tests {
    use super::*;
    #[test]
    fn test_bairstow() {
        let mut a: Vec<f64> = vec![1.0, -5.0, 10.0, -10.0, 4.0];
        a.reverse();
        let (b, r, s) = bairstow(&a, 0.5, -0.5, 0.01).unwrap();
        assert_eq!(b, vec![2.0002748946839852, -2.0001913816133645, 1.0]);
        assert_float_absolute_eq!(r, 3.0);
        assert_float_absolute_eq!(s, -2.0);
    }
    #[test]
    fn test_solve() {
        let mut a: Vec<f64> = vec![1.0, -5.0, 10.0, -10.0, 4.0];
        a.reverse();
        let result = solve(&a, 1e-6);
        assert_eq!(result.len(), a.len() - 1);
        let should = vec![
            Complex::from(2.0),
            Complex::from(1.0),
            Complex::new(1.0, 1.0),
            Complex::new(1.0, -1.0),
        ];
        for i in 0..result.len() {
            println!("{}", result[i]);
            assert_float_absolute_eq!(result[i].re, should[i].re);
            assert_float_absolute_eq!(result[i].im, should[i].im);
        }
    }
}
