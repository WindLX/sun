use crate::{container::Function, meta::OwnSunMeta};

/// 是否可用 `[]` 处理
pub trait IndexAble
where
    Self: OwnSunMeta,
{
    /// 返回处理 `[]` 的函数
    fn index() -> Function;
}
