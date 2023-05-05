use super::super::super::sun_function::Function;
use super::super::super::sun_object::IsSunObject;

/// `+`
pub trait AddAble
where
    Self: IsSunObject,
{
    fn add() -> Function;
}

/// `-` 二元操作
pub trait SubAble
where
    Self: IsSunObject,
{
    fn sub() -> Function;
}

/// `*` 二元操作
pub trait MulAble
where
    Self: IsSunObject,
{
    fn mul() -> Function;
}

/// `/`
pub trait DivAble
where
    Self: IsSunObject,
{
    fn div() -> Function;
}

/// `%`
pub trait RemAble
where
    Self: IsSunObject,
{
    fn rem() -> Function;
}

/// `^`
pub trait PowAble
where
    Self: IsSunObject,
{
    fn pow() -> Function;
}

/// `!` 一元操作
pub trait FacAble
where
    Self: IsSunObject,
{
    fn fac() -> Function;
}

/// `-` 一元操作
pub trait NegAble
where
    Self: IsSunObject,
{
    fn neg() -> Function;
}

/// `*` 一元操作
pub trait ConjAble
where
    Self: IsSunObject,
{
    fn conj() -> Function;
}

/// `&&`
pub trait AndAble
where
    Self: IsSunObject,
{
    fn and() -> Function;
}

/// `||`
pub trait OrAble
where
    Self: IsSunObject,
{
    fn or() -> Function;
}

/// `~`
pub trait NotAble
where
    Self: IsSunObject,
{
    fn not() -> Function;
}

/// `^^`
pub trait XorAble
where
    Self: IsSunObject,
{
    fn xor() -> Function;
}

/// 是否可比较
pub trait CompareAble
where
    Self: IsSunObject,
{
    /// `<`
    fn less() -> Function;
    /// `>`
    fn greater() -> Function;
    /// `==`
    fn eq() -> Function;
    /// `~=`
    fn noteq() -> Function;
    /// `>=`
    fn ge() -> Function;
    /// `<=`
    fn le() -> Function;
}

/// 是否可用 `()` 进行处理
pub trait CallAble
where
    Self: IsSunObject,
{
    fn call() -> Function;
}

/// 批量处理 Number 的二元操作符
#[macro_export]
macro_rules! double_op {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1 $op n2;
                    *self_value = SunValue::from(value);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}

/// 批量处理 Number 的一元操作符
#[macro_export]
macro_rules! single_op {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let mut self_value = args[0].borrow_mut();
            let res = match arg {
                SunValue::Number(n) => {
                    *self_value = SunValue::from($op n);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}

/// 批量处理 Bool 的二元操作符
#[macro_export]
macro_rules! double_op_b {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let mut self_value = args[0].borrow_mut();
            let res = match (arg_0, arg_1) {
                (SunValue::Boolean(b1), SunValue::Boolean(b2)) => {
                    let value = b1 $op b2;
                    *self_value = SunValue::from(value);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}

/// 批量处理 Bool 的一元操作符
#[macro_export]
macro_rules! single_op_b {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg = args[0].get();
            let mut self_value = args[0].borrow_mut();
            let res = match arg {
                SunValue::Boolean(b) => {
                    *self_value = SunValue::from($op b);
                    vec![args[0].clone()]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}

/// 批量处理 Bool 的比较操作符
#[macro_export]
macro_rules! compare_op_b {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let res = match (arg_0, arg_1) {
                (SunValue::Boolean(b1), SunValue::Boolean(b2)) => {
                    let value = b1 $op b2;
                    vec![SunPointer::new(SunValue::from(value))]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}

/// 批量处理 Number 的比较操作符
#[macro_export]
macro_rules! compare_op {
    ($op:tt) => {{
        use crate::sun_lib::value::sun_function::Function;
        let f = |args: Vec<SunPointer>| {
            let arg_0 = args[0].get();
            let arg_1 = args[1].get();
            let res = match (arg_0, arg_1) {
                (SunValue::Number(n1), SunValue::Number(n2)) => {
                    let value = n1 $op n2;
                    vec![SunPointer::new(SunValue::from(value))]
                }
                _ => Vec::new(),
            };
            res
        };
        f as Function
    }};
}
