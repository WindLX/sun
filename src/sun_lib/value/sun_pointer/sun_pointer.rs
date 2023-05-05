use super::super::sun_object::SunValue;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

/// 指向类型数据的智能指针
pub struct SunPointer {
    pointer: Rc<RefCell<SunValue>>,
}

impl SunPointer {
    /// 创建新指针
    pub fn new(obj: SunValue) -> SunPointer {
        SunPointer {
            pointer: Rc::new(RefCell::new(obj)),
        }
    }

    /// 获取引用
    pub fn borrow(&self) -> Ref<SunValue> {
        self.pointer.borrow()
    }

    /// 获取可变引用
    pub fn borrow_mut(&self) -> RefMut<SunValue> {
        self.pointer.borrow_mut()
    }

    /// 获取指向类型数据的拷贝
    pub fn get(&self) -> SunValue {
        self.pointer.borrow().clone()
    }
}

impl Deref for SunPointer {
    type Target = Rc<RefCell<SunValue>>;

    fn deref(&self) -> &Self::Target {
        &self.pointer
    }
}

impl Clone for SunPointer {
    /// 增加引用计数
    fn clone(&self) -> Self {
        SunPointer {
            pointer: Rc::clone(&self.pointer),
        }
    }
}

impl fmt::Debug for SunPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.pointer.borrow())
    }
}

impl PartialEq for SunPointer {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.pointer, &other.pointer)
            || *self.pointer.borrow() == *other.pointer.borrow()
    }
}

impl Eq for SunPointer {}
