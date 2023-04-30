use std::f64::consts::E;

use crate::{sun_lib::sun_value::SunValue, utils::err::SunError};

pub fn complex_2_euler(cpx: &SunValue) -> Result<(f64, f64), SunError> {
    if let SunValue::Complex(r, i) = cpx {
        let rho = (r.powi(2) + i.powi(2)).powf(0.5);
        let theta = (i / r).atan();
        Ok((rho, theta))
    } else {
        Err(SunError::TypeError(format!(
            "expect `complex` but got wrong type"
        )))
    }
}

pub fn complex_ln(cpx: &SunValue) -> Result<(f64, f64), SunError> {
    let (rho, theta) = complex_2_euler(cpx)?;
    Ok((rho.ln(), theta))
}

pub fn complex_index(c1: (f64, f64), c2: (f64, f64)) -> (f64, f64) {
    let (a, b, c, d) = (c1.0, c1.1, c2.0, c2.1);
    let mag = (a.powi(2) + b.powi(2)).powf(0.5);
    let ang = (b / a).atan();
    let r = mag * E.powf(-d * ang) * (c * ang).cos();
    let i = mag * E.powf(-d * ang) * (c * ang).sin();
    (r, i)
}
