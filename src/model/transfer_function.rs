use crate::config::DType;
use num::complex::Complex;
use std::convert::TryInto;
use std::fmt;

pub struct TransferFunction {
    numerator: Vec<DType>,
    denominator: Vec<DType>,
    sample_time: Option<DType>,
}

impl TransferFunction {
    pub fn from(num: Vec<DType>, den: Vec<DType>, ts: Option<DType>) -> TransferFunction {
        TransferFunction {
            numerator: num,
            denominator: den,
            sample_time: ts,
        }
    }

    pub fn frequency_response(self: &TransferFunction, omega: DType) -> Complex<DType> {
        let s = Complex::new(0.0, omega);
        let num = vec2num(&self.numerator, s);
        let den = vec2num(&self.denominator, s);
        num / den
    }

    pub fn to_string(self: &TransferFunction, tex: bool) -> String {
        let val_str = match self.sample_time {
            Some(_ts) => "z",
            None => "s",
        };
        let sample_time_message = match self.sample_time {
            Some(ts) => format!("discrete system with sample time:{}", ts),
            None => format!("continuous system"),
        };
        match tex {
            true => format!(
                "\\frac{{{}}}{{{}}}\n{}",
                vec2string(&self.numerator, &val_str),
                vec2string(&self.denominator, val_str),
                sample_time_message
            ),
            false => format!(
                "
    {}
    -------------
    {}
{}
    ",
                vec2string(&self.numerator, &val_str),
                vec2string(&self.denominator, val_str),
                sample_time_message
            ),
        }
    }
}

impl fmt::Display for TransferFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "transfer function system model:\n{}",
            self.to_string(false)
        )
    }
}

//convert cofix to string
fn vec2string(vec: &Vec<DType>, val: &str) -> String {
    let mut main_str = String::new();
    for (i, &term) in vec.iter().enumerate() {
        let index = vec.len() - i - 1;
        if index == 0 {
            main_str.push_str(&format!(" {}", term));
        } else {
            main_str.push_str(&format!(" {}{}^{} ", term, val, index));
            main_str.push('+');
        }
    }
    main_str
}

fn vec2num(vec: &Vec<DType>, val: Complex<DType>) -> Complex<DType> {
    let mut value = Complex::new(0.0, 0.0);
    for (i, &term) in vec.iter().enumerate() {
        let index = vec.len() - i - 1;
        let index = index.try_into().unwrap();
        value = value + val.powu(index) * term;
    }
    value
}
