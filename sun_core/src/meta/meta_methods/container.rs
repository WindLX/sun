use crate::{container::Function, utils::IsSunObject};

/// 是否可用 `[]` 处理
pub trait IndexAble
where
    Self: IsSunObject,
{
    /// 返回处理 `[]` 的函数
    fn index() -> Function;
}
