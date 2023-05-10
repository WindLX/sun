use crate::{container::Function, meta::OwnSunMeta};

/// `+`
pub trait AddAble
where
    Self: OwnSunMeta,
{
    fn add() -> Function;
}

/// `-` 二元操作
pub trait SubAble
where
    Self: OwnSunMeta,
{
    fn sub() -> Function;
}

/// `*` 二元操作
pub trait MulAble
where
    Self: OwnSunMeta,
{
    fn mul() -> Function;
}

/// `/`
pub trait DivAble
where
    Self: OwnSunMeta,
{
    fn div() -> Function;
}

/// `%`
pub trait RemAble
where
    Self: OwnSunMeta,
{
    fn rem() -> Function;
}

/// `^`
pub trait PowAble
where
    Self: OwnSunMeta,
{
    fn pow() -> Function;
}

/// `!` 一元操作
pub trait FacAble
where
    Self: OwnSunMeta,
{
    fn fac() -> Function;
}

/// `-` 一元操作
pub trait NegAble
where
    Self: OwnSunMeta,
{
    fn neg() -> Function;
}

/// `*` 一元操作
pub trait ConjAble
where
    Self: OwnSunMeta,
{
    fn conj() -> Function;
}

/// `&&`
pub trait AndAble
where
    Self: OwnSunMeta,
{
    fn and() -> Function;
}

/// `||`
pub trait OrAble
where
    Self: OwnSunMeta,
{
    fn or() -> Function;
}

/// `~`
pub trait NotAble
where
    Self: OwnSunMeta,
{
    fn not() -> Function;
}

/// `^^`
pub trait XorAble
where
    Self: OwnSunMeta,
{
    fn xor() -> Function;
}

/// 是否可比较
pub trait CompareAble
where
    Self: OwnSunMeta,
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
