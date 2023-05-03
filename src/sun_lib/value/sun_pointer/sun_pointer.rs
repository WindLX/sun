use super::super::sun_object::SunValue;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

pub struct SunPointer {
    pointer: Rc<RefCell<SunValue>>,
}

impl SunPointer {
    pub fn new(obj: SunValue) -> SunPointer {
        SunPointer {
            pointer: Rc::new(RefCell::new(obj)),
        }
    }

    pub fn borrow(&self) -> Ref<SunValue> {
        self.pointer.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<SunValue> {
        self.pointer.borrow_mut()
    }

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
