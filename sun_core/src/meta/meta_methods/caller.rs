use crate::{container::Function, meta::OwnSunMeta};

/// 是否可用 `()` 进行处理
pub trait CallAble
where
    Self: OwnSunMeta,
{
    fn call(&self) -> Function;
}
