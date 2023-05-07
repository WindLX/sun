use crate::{container::Function, utils::IsSunObject};

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
