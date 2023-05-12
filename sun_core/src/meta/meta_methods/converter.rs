use crate::{container::Function, meta::OwnSunMeta};

/// 是否可以转换成 `Table`
pub trait Converter
where
    Self: OwnSunMeta,
{
    // 返回从 `Table` 转换成对应类型的函数
    fn from() -> Function;
    // 返回转换成 `Table` 的函数
    fn to() -> Function;
}
