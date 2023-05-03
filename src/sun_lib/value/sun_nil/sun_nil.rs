use super::super::sun_object::{IsSunObject, SunObject};

#[derive(Clone, Debug)]
pub struct SunNil {
    obj: SunObject,
}

impl IsSunObject for SunNil {
    fn get_obj(&self) -> SunObject {
        self.obj.clone()
    }
}

impl SunNil {
    pub fn new() -> SunNil {
        let obj = SunObject::new("nil");
        SunNil { obj }
    }
}
