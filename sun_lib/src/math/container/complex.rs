use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Neg, Sub},
};
use sun_core::{
    container::{Class, IsSunClass, SunValue, Table},
    utils::{log::error_output, SunError, SunPointer},
};

/// `Complex` 的数据容器
pub struct Complex(Class);

impl Complex {
    /// 新建复数
    pub fn new(r: f64, i: f64) -> Self {
        let mut class = Class::new("Complex", HashMap::new());
        class.set_attribute("r", SunPointer::new(SunValue::from(r)));
        class.set_attribute("i", SunPointer::new(SunValue::from(i)));
        Complex(class)
    }

    /// 获取实部
    pub fn get_real(&self) -> f64 {
        if let SunValue::Number(r) = self.get_class().get_attribute("r").unwrap().get() {
            r
        } else {
            unreachable!()
        }
    }

    /// 获取虚部
    pub fn get_imag(&self) -> f64 {
        if let SunValue::Number(i) = self.get_class().get_attribute("i").unwrap().get() {
            i
        } else {
            unreachable!()
        }
    }

    /// 取模
    pub fn get_mag(&self) -> f64 {
        (self.get_imag().powi(2) + self.get_real().powi(2)).sqrt()
    }

    /// 取共轭
    pub fn get_conj(&self) -> Self {
        Self::new(self.get_real(), -self.get_imag())
    }

    /// 转换成Euler形式
    pub fn to_euler(&self) -> (f64, f64) {
        let (r, i) = (self.get_real(), self.get_imag());
        let rho = (r.powi(2) + i.powi(2)).sqrt();
        let theta = (i / r).atan();
        (rho, theta)
    }

    /// 从Euler形式转化成向量形式
    pub fn from_euler(euler: (f64, f64)) -> Self {
        let r = euler.0 * euler.1.cos();
        let i = euler.0 * euler.1.sin();
        Complex::new(r, i)
    }

    /// 复数的复数次幂
    pub fn get_power(&self, index: Self) -> Self {
        let (c, d) = (index.get_real(), index.get_imag());
        let (rho, theta) = self.to_euler();
        let res = (
            rho * ((-d * theta).exp()) * ((c * theta).cos()),
            rho * ((-d * theta).exp()) * ((c * theta).sin()),
        );
        Complex::from(res)
    }

    /// 复数的自然对数
    pub fn get_ln(&self) -> Self {
        let (rho, theta) = self.to_euler();
        let res = (0.5 * (rho.ln()), theta);
        Complex::from(res)
    }

    /// 复数的任意底对数
    pub fn get_log(&self, base: Self) -> Self {
        let top = self.get_ln();
        let bottom = base.get_ln();
        top / bottom
    }
}

impl IsSunClass for Complex {
    fn get_class(&self) -> &Class {
        &self.0
    }
}

impl From<(f64, f64)> for Complex {
    fn from(value: (f64, f64)) -> Self {
        Complex::new(value.0, value.1)
    }
}

impl From<SunValue> for Complex {
    fn from(value: SunValue) -> Self {
        match value {
            SunValue::Class(class) => match class.get_name() {
                "Complex" => {
                    let (i, r) = (class.get_attribute("i"), class.get_attribute("r"));
                    if let (SunValue::Number(i), SunValue::Number(r)) =
                        (i.unwrap().get(), r.unwrap().get())
                    {
                        Complex::new(r, i)
                    } else {
                        unreachable!("not complex")
                    }
                }
                other => {
                    let e = SunError::TypeError(format!("expect `Complex` but got `{other}`"));
                    error_output(e);
                }
            },
            SunValue::Number(num) => Complex::new(num, 0.0),
            SunValue::Table(t) => {
                let (rho, theta) = (t.get_by_key("rho"), t.get_by_key("theta"));
                match (rho, theta) {
                    (Some(rho), Some(theta)) => match (rho.get(), theta.get()) {
                        (SunValue::Number(rho), SunValue::Number(theta)) => {
                            Self::from_euler((rho, theta))
                        }
                        _ => {
                            let e = SunError::TypeError(format!(
                                "this `Table` can't convert to `Complex`"
                            ));
                            error_output(e);
                        }
                    },
                    (None, None) => {
                        let (r, i) = (t.get_by_key("r"), t.get_by_key("i"));
                        match (r, i) {
                            (Some(r), Some(i)) => match (r.get(), i.get()) {
                                (SunValue::Number(r), SunValue::Number(i)) => Self::from((r, i)),
                                _ => {
                                    let e = SunError::TypeError(format!(
                                        "this `Table` can't convert to `Complex`"
                                    ));
                                    error_output(e);
                                }
                            },
                            _ => {
                                let e = SunError::TypeError(format!(
                                    "this `Table` can't convert to `Complex`"
                                ));
                                error_output(e);
                            }
                        }
                    }
                    _ => {
                        let e =
                            SunError::TypeError(format!("this `Table` can't convert to `Complex`"));
                        error_output(e);
                    }
                }
            }
            other => {
                let e = SunError::TypeError(format!("expect `Complex` but got `{other}`"));
                error_output(e);
            }
        }
    }
}

impl From<Complex> for SunValue {
    fn from(value: Complex) -> Self {
        let (r, i) = (value.get_real(), value.get_imag());
        let mut res = Table::new();
        res.append_kv("r".to_string(), SunValue::from(r));
        res.append_kv("i".to_string(), SunValue::from(i));
        SunValue::from(res)
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let res = (
            self.get_real() + rhs.get_real(),
            self.get_imag() + rhs.get_imag(),
        );
        Complex::from(res)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        let res = (
            self.get_real() - rhs.get_real(),
            self.get_imag() - rhs.get_imag(),
        );
        Complex::from(res)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b, c, d) = (
            self.get_real(),
            self.get_imag(),
            rhs.get_real(),
            rhs.get_imag(),
        );
        let res = (a * c - b * d, a * d + c * b);
        Complex::from(res)
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        let (a, b, c, d) = (
            self.get_real(),
            self.get_imag(),
            rhs.get_real(),
            rhs.get_imag(),
        );
        let bottom = c.powi(2) + d.powi(2);
        let res = ((a * c + b * d) / bottom, (b * c - a * d) / bottom);
        Complex::from(res)
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        let res = (-self.get_real(), -self.get_imag());
        Complex::from(res)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.get_real() == other.get_real() && other.get_imag() == self.get_imag()
    }
}

impl Eq for Complex {}
