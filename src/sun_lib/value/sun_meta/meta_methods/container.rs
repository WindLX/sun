use super::super::super::sun_function::Function;
use super::super::super::sun_object::IsSunObject;

pub trait IndexAble
where
    Self: IsSunObject,
{
    fn index() -> Function;
}
