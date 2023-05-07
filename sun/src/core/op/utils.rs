use crate::sun_lib::sun_value::SunValue;
use std::f64::INFINITY;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Add for SunValue {
    type Output = SunValue;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SunValue::Number(n1), SunValue::Number(n2)) => SunValue::Number(n1 + n2),
            (SunValue::Number(n), SunValue::Complex(r, i)) => SunValue::Complex(r + n, i),
            (SunValue::Complex(r, i), SunValue::Number(n)) => SunValue::Complex(r + n, i),
            (SunValue::Complex(r1, i1), SunValue::Complex(r2, i2)) => {
                SunValue::Complex(r1 + r2, i1 + i2)
            }
            (_, _) => SunValue::Nil,
        }
    }
}

impl Sub for SunValue {
    type Output = SunValue;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SunValue::Number(n1), SunValue::Number(n2)) => SunValue::Number(n1 - n2),
            (SunValue::Number(n), SunValue::Complex(r, i)) => SunValue::Complex(n - r, -i),
            (SunValue::Complex(r, i), SunValue::Number(n)) => SunValue::Complex(r - n, i),
            (SunValue::Complex(r1, i1), SunValue::Complex(r2, i2)) => {
                SunValue::Complex(r1 - r2, i1 - i2)
            }
            (_, _) => SunValue::Nil,
        }
    }
}

impl Mul for SunValue {
    type Output = SunValue;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SunValue::Number(n1), SunValue::Number(n2)) => SunValue::Number(n1 * n2),
            (SunValue::Number(n), SunValue::Complex(r, i)) => SunValue::Complex(r * n, i * n),
            (SunValue::Complex(r, i), SunValue::Number(n)) => SunValue::Complex(r * n, i * n),
            (SunValue::Complex(r1, i1), SunValue::Complex(r2, i2)) => {
                SunValue::Complex(r1 * r2 - i1 * i2, i1 * r2 + r1 * i2)
            }
            (_, _) => SunValue::Nil,
        }
    }
}

impl Div for SunValue {
    type Output = SunValue;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SunValue::Number(n1), SunValue::Number(n2)) => {
                let res = n1 / n2;
                if res == INFINITY {
                    SunValue::Nil
                } else {
                    SunValue::Number(res)
                }
            }
            (SunValue::Number(n), SunValue::Complex(r, i)) => {
                let bottom = r.powi(2) + i.powi(2);
                if 1_f64 / bottom == INFINITY {
                    SunValue::Nil
                } else {
                    SunValue::Complex(n * r / bottom, -n * i / bottom)
                }
            }
            (SunValue::Complex(r, i), SunValue::Number(n)) => {
                if r / n == INFINITY {
                    SunValue::Nil
                } else {
                    SunValue::Complex(r / n, i / n)
                }
            }
            (SunValue::Complex(r1, i1), SunValue::Complex(r2, i2)) => {
                let bottom = r2.powi(2) + i2.powi(2);
                if 1_f64 / bottom == INFINITY {
                    SunValue::Nil
                } else {
                    SunValue::Complex((r1 * r2 + i1 * i2) / bottom, (i1 * r2 - r1 * i2) / bottom)
                }
            }
            (_, _) => SunValue::Nil,
        }
    }
}

impl Rem for SunValue {
    type Output = SunValue;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SunValue::Number(n1), SunValue::Number(n2)) => SunValue::Number(n1 % n2),
            (_, _) => SunValue::Nil,
        }
    }
}
