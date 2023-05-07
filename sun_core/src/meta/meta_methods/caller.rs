use crate::{container::Function, utils::IsSunObject};

/// 是否可用 `()` 进行处理
pub trait CallAble
where
    Self: IsSunObject,
{
    fn call(&self) -> Function;
}
