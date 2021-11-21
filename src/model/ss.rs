use ndarray::Array2;
use std::fmt;

/// A struct representating State-Space model
pub struct StateSpace<T, U> {
    a: Array2<T>,
    b: Array2<T>,
    c: Array2<T>,
    d: Array2<T>,
    sample_time: Option<U>,
}

#[allow(non_snake_case)]
impl<T, U> StateSpace<T, U> {
    pub fn from(
        A: Array2<T>,
        B: Array2<T>,
        C: Array2<T>,
        D: Array2<T>,
        ts: Option<U>,
    ) -> StateSpace<T, U> {
        StateSpace {
            a: A,
            b: B,
            c: C,
            d: D,
            sample_time: ts,
        }
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> fmt::Display for StateSpace<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tsmsg = match &self.sample_time {
            Some(ts) => format!("discrete system with sample time Ts={}", ts),
            None => format!("continuous system"),
        };
        write!(
            f,
            "
State Space Model
    x'=Ax+Bu
    y =Cx+Du
with A={}.B={},C={},D={}
{}
        ",
            self.a, self.b, self.c, self.d, tsmsg
        )
    }
}
