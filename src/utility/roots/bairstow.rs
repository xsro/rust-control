use num::traits::Zero;
use num::Complex;

pub fn solve<T>(poly: Vec<T>) -> Vec<Complex<T>>
where
    T: Clone + num::Num,
{
    vec![Complex::zero()]
}

/// (quotient,remainder)=function(vec)
/// - https://nptel.ac.in/content/storage2/courses/122104019/numerical-analysis/Rathish-kumar/ratish-1/f3node9.html
pub fn bairstow<T>(A: Vec<T>) -> (Vec<T>, Option<(T, T)>) {
    (A, None)
}
